use std::cmp;

use bitflags::bitflags;

use crate::common;

const HORZ_BORDER: &'static str = "─";
const VERT_BORDER: &'static str = "│";
const TOP_LEFT: &'static str = "╭";
const TOP_RIGHT: &'static str = "╮";
const BOTTOM_LEFT: &'static str = "╰";
const BOTTOM_RIGHT: &'static str = "╯";

const BORDER_WIDTH: usize = HORZ_BORDER.len();

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

    pub position: common::Vec2<i8>,
    pub size: common::Vec2,
    pub border_options: BorderFlags,

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

pub fn draw_box(
    buffer: &mut Vec<String>,
    options: BoxOptions,
    crash: bool,
) -> Result<(), DrawError> {
    if options.position.y >= 0 {
        // Part 1: Top border
        let top_border = make_border(
            &options.border_options,
            BorderFlags::TOP,
            TOP_LEFT,
            TOP_RIGHT,
            HORZ_BORDER,
            options.size.x.saturating_sub(2),
        )
        .chars()
        .skip(cmp::max(0, -options.position.x) as usize)
        .take(cmp::min(
            options.screen_size.x,
            cmp::min(
                options.size.x,
                (options.screen_size.x as i8 - options.position.x) as usize,
            ),
        ))
        .collect::<String>();

        let top_index = cmp::max(options.position.y, 0) as usize;
        let top_prefix = take_visible(&buffer[top_index], cmp::max(options.position.x, 0) as usize);
        let top_suffix = &buffer[top_index]
            .get(top_prefix.len() + top_border.len()..)
            .unwrap_or("");

        buffer[top_index] = format!("{}{}{}", top_prefix, top_border, top_suffix);
    }

    // Part 2: Bottom border
    let bottom_index = options.position.y + (options.size.y as i8) - 1;
    if bottom_index >= 0 && bottom_index < (buffer.len() as i8) {
        let bottom_border = make_border(
            &options.border_options,
            BorderFlags::BOTTOM,
            BOTTOM_LEFT,
            BOTTOM_RIGHT,
            HORZ_BORDER,
            options.size.x.saturating_sub(2),
        )
        .chars()
        .skip(cmp::max(0, -options.position.x) as usize)
        .take(cmp::min(
            options.screen_size.x,
            cmp::min(
                options.size.x,
                (options.screen_size.x as i8 - options.position.x) as usize,
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

        buffer[bottom_index as usize] =
            format!("{}{}{}", bottom_prefix, bottom_border, bottom_suffix);
    }

    // Part 3: Middle border
    let mut middle_border = make_border(
        &options.border_options,
        BorderFlags::LEFT | BorderFlags::RIGHT,
        VERT_BORDER,
        VERT_BORDER,
        " ",
        options.size.x.saturating_sub(2),
    )
    .chars()
    .skip(cmp::max(0, -options.position.x) as usize)
    .take(cmp::min(
        options.screen_size.x,
        cmp::min(
            options.size.x,
            (options.screen_size.x as i8 - options.position.x) as usize,
        ),
    ))
    .collect::<String>();

    if options.background_color.is_some() {
        let bg_ansi = options.background_color.as_ref().unwrap().bg();
        middle_border.insert_str(
            if options.position.x >= 0 {
                BORDER_WIDTH
            } else {
                0
            },
            &bg_ansi,
        );

        let last_len = middle_border.chars().last().unwrap().len_utf8();
        middle_border.insert_str(
            middle_border.len() - (BORDER_WIDTH * (last_len == BORDER_WIDTH) as usize),
            "\x1b[0m",
        );
    }

    for i in 1..options.size.y.saturating_sub(1) {
        let middle_index = options.position.y + (i as i8);
        if middle_index >= 0 && middle_index < (buffer.len() as i8) {
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
