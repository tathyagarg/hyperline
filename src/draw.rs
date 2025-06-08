use std::cmp;

use bitflags::bitflags;

use crate::common;

pub struct BorderChars {
    pub top: &'static str,
    pub left: &'static str,
    pub right: &'static str,
    pub bottom: &'static str,
    pub top_left: &'static str,
    pub top_right: &'static str,
    pub bottom_left: &'static str,
    pub bottom_right: &'static str,
}

impl BorderChars {
    pub fn border_width(&self) -> usize {
        self.top.len()
    }
}

const BLOCK_BORDER_CHARS: BorderChars = BorderChars {
    top: "▄",
    left: "▐",
    right: "▌",
    bottom: "▀",
    top_left: "▗",
    top_right: "▖",
    bottom_left: "▝",
    bottom_right: "▘",
};

const ROUNDED_BORDER_CHARS: BorderChars = BorderChars {
    top: "─",
    left: "│",
    right: "│",
    bottom: "─",
    top_left: "╭",
    top_right: "╮",
    bottom_left: "╰",
    bottom_right: "╯",
};

const SHARP_BORDER_CHARS: BorderChars = BorderChars {
    top: "─",
    left: "│",
    right: "│",
    bottom: "─",
    top_left: "┌",
    top_right: "┐",
    bottom_left: "└",
    bottom_right: "┘",
};

pub enum BorderStyle {
    Block,
    Rounded,
    Sharp,
}

impl BorderStyle {
    pub fn chars(&self) -> &BorderChars {
        match self {
            BorderStyle::Block => &BLOCK_BORDER_CHARS,
            BorderStyle::Rounded => &ROUNDED_BORDER_CHARS,
            BorderStyle::Sharp => &SHARP_BORDER_CHARS,
        }
    }
}

#[derive(Debug)]
pub enum DrawError {
    HeightTooSmall,
    ContentTooLong,
}

bitflags! {
    pub struct BorderFlags: u8 {
        const NONE = 0b0000_0000;
        const TOP = 0b0000_0001;
        const BOTTOM = 0b0000_0010;
        const LEFT = 0b0000_0100;
        const RIGHT = 0b0000_1000;

        const PRESERVE_CORNERS = 0b0001_0000;

        const ALL = Self::TOP.bits() | Self::BOTTOM.bits() | Self::LEFT.bits() | Self::RIGHT.bits() | Self::PRESERVE_CORNERS.bits();
    }
}

pub struct BoxOptions {
    pub screen_size: common::Vec2,

    pub position: common::Vec2<i16>,
    pub size: common::Vec2,

    pub border_options: BorderFlags,
    pub border_style: BorderStyle,

    pub background_color: Option<common::Color>,
    pub border_color: Option<common::Color>,
}

fn take_visible(string: &str, max_length: usize) -> String {
    if max_length == 0 {
        return String::new();
    }

    let mut out = String::new();
    let mut chars = string.chars().peekable();
    let mut length = 0;

    while let Some(c) = chars.next() {
        out.push(c);
        if c == '\x1b' {
            while let Some(next) = chars.next() {
                out.push(next);
                if next == 'm' {
                    break;
                }
            }
        } else {
            length += 1;
            if length >= max_length {
                break;
            }
        }
    }

    out
}

fn make_border(
    border: &BorderFlags,
    location: BorderFlags,
    left: &str,
    right: &str,
    middle: &str,
    width: usize,
) -> String {
    let mut border_str = String::new();

    let left_char = if ((location.contains(BorderFlags::TOP)
        || location.contains(BorderFlags::BOTTOM))
        && border.contains(BorderFlags::PRESERVE_CORNERS))
        || border.contains(BorderFlags::LEFT)
    {
        left
    } else {
        " "
    };
    border_str.push_str(left_char);

    border_str.push_str(middle.repeat(width).as_str());

    let right_char = if ((location.contains(BorderFlags::TOP)
        || location.contains(BorderFlags::BOTTOM))
        && border.contains(BorderFlags::PRESERVE_CORNERS))
        || border.contains(BorderFlags::RIGHT)
    {
        right
    } else {
        " "
    };
    border_str.push_str(right_char);

    border_str
}

fn add_background_color(
    border: &mut String,
    border_style: &BorderStyle,
    position: &common::Vec2<i16>,
    background_color: &Option<common::Color>,
) {
    if let Some(bg_color) = background_color {
        let border_width = border_style.chars().border_width();
        let bg_ansi = bg_color.bg();
        border.insert_str((position.x >= 0) as usize * border_width, &bg_ansi);

        let ends_with_border = border.chars().last().unwrap().len_utf8() == border_width;
        border.insert_str(
            border.len() - (border_width * ends_with_border as usize),
            "\x1b[0m",
        );
    }
}

fn add_border_color(
    border: &mut String,
    border_style: &BorderStyle,
    position: &common::Vec2<i16>,
    border_color: &Option<common::Color>,
) {
    if let Some(border_color) = border_color {
        let border_width = border_style.chars().border_width();
        let border_ansi = border_color.fg();
        if position.x >= 0 {
            border.insert_str(0, &border_ansi);
            border.insert_str(border_width + border_ansi.len(), "\x1b[0m");
        }

        if border.chars().last().unwrap().len_utf8() == border_width {
            border.insert_str(border.len() - border_width, &border_ansi);
            border.insert_str(border.len(), "\x1b[0m");
        }
    }
}

fn add_edge_border_color(border: &mut String, border_color: &Option<common::Color>) {
    if let Some(border_color) = border_color {
        let border_ansi = border_color.fg();
        border.insert_str(0, &border_ansi);
        border.insert_str(border.len(), "\x1b[0m");
    }
}

pub fn draw_box(
    buffer: &mut Vec<String>,
    options: BoxOptions,
    crash: bool,
) -> Result<(), DrawError> {
    let border_chars = options.border_style.chars();

    if options.position.y >= 0 {
        // Part 1: Top border
        let mut top_border = make_border(
            &options.border_options,
            BorderFlags::TOP,
            border_chars.top_left,
            border_chars.top_right,
            border_chars.top,
            options.size.x.saturating_sub(2),
        )
        .chars()
        .skip(cmp::max(0, -options.position.x) as usize)
        .take(cmp::min(
            options.screen_size.x,
            cmp::min(
                options.size.x,
                (options.screen_size.x as i16 - options.position.x) as usize,
            ),
        ))
        .collect::<String>();

        let top_index = cmp::max(options.position.y, 0) as usize;
        let top_prefix = take_visible(&buffer[top_index], cmp::max(options.position.x, 0) as usize);
        let top_suffix = &buffer[top_index]
            .get(top_prefix.len() + top_border.len()..)
            .unwrap_or("");

        add_edge_border_color(&mut top_border, &options.border_color);

        buffer[top_index] = format!("{}{}{}", top_prefix, top_border, top_suffix);
    }

    // Part 2: Bottom border
    let bottom_index = options.position.y + (options.size.y as i16) - 1;
    if bottom_index >= 0 && bottom_index < (buffer.len() as i16) {
        let mut bottom_border = make_border(
            &options.border_options,
            BorderFlags::BOTTOM,
            border_chars.bottom_left,
            border_chars.bottom_right,
            border_chars.bottom,
            options.size.x.saturating_sub(2),
        )
        .chars()
        .skip(cmp::max(0, -options.position.x) as usize)
        .take(cmp::min(
            options.screen_size.x,
            cmp::min(
                options.size.x,
                (options.screen_size.x as i16 - options.position.x) as usize,
            ),
        ))
        .collect::<String>();

        let bottom_prefix = take_visible(
            &buffer[bottom_index as usize],
            cmp::max(options.position.x, 0) as usize,
        );
        let bottom_suffix = &buffer[bottom_index as usize]
            .get(bottom_prefix.len() + bottom_border.len()..)
            .unwrap_or("");

        add_edge_border_color(&mut bottom_border, &options.border_color);

        buffer[bottom_index as usize] =
            format!("{}{}{}", bottom_prefix, bottom_border, bottom_suffix);
    }

    // Part 3: Middle border
    let mut middle_border = make_border(
        &options.border_options,
        BorderFlags::LEFT | BorderFlags::RIGHT,
        border_chars.left,
        border_chars.right,
        " ",
        options.size.x.saturating_sub(2),
    )
    .chars()
    .skip(cmp::max(0, -options.position.x) as usize)
    .take(cmp::min(
        options.screen_size.x,
        cmp::min(
            options.size.x,
            (options.screen_size.x as i16 - options.position.x) as usize,
        ),
    ))
    .collect::<String>();

    add_background_color(
        &mut middle_border,
        &options.border_style,
        &options.position,
        &options.background_color,
    );

    add_border_color(
        &mut middle_border,
        &options.border_style,
        &options.position,
        &options.border_color,
    );

    for i in 1..options.size.y.saturating_sub(1) {
        let middle_index = options.position.y + (i as i16);
        if middle_index >= 0 && middle_index < (buffer.len() as i16) {
            let middle_prefix = take_visible(
                &buffer[middle_index as usize],
                cmp::max(options.position.x, 0) as usize,
            );

            let middle_suffix = &buffer[middle_index as usize]
                .get(middle_prefix.len() + middle_border.len()..)
                .unwrap_or("");

            buffer[middle_index as usize] =
                format!("{}{}{}", middle_prefix, middle_border, middle_suffix);
        }
    }

    if crash {
        return Err(DrawError::HeightTooSmall);
    }

    Ok(())
}
