use std::cmp;

use bitflags::bitflags;

use crate::common;

const HORZ_BORDER: &'static str = "─";
const VERT_BORDER: &'static str = "│";
const TOP_LEFT: &'static str = "╭";
const TOP_RIGHT: &'static str = "╮";
const BOTTOM_LEFT: &'static str = "╰";
const BOTTOM_RIGHT: &'static str = "╯";

const BOX_CHAR_WIDTH: usize = TOP_RIGHT.len();

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

    pub position: common::Vec2,
    pub size: common::Vec2,
    pub border_options: BorderFlags,

    pub background_color: Option<common::Color>,
}

fn take_visible(string: &str, max_length: usize) -> String {
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

pub fn draw_box(
    buffer: &mut Vec<String>,
    options: BoxOptions,
    crash: bool,
) -> Result<(), DrawError> {
    // Part 1: Top border
    let mut top_border = String::new();
    if options.border_options.contains(BorderFlags::TOP) {
        if options.border_options.contains(BorderFlags::LEFT)
            || options
                .border_options
                .contains(BorderFlags::PRESERVE_CORNERS)
        {
            top_border.push_str(TOP_LEFT);
        } else {
            top_border.push(' ');
        }

        let width = options.size.x.saturating_sub(2);
        if width > 0 {
            top_border.push_str(&HORZ_BORDER.repeat(width));
        }

        if options.border_options.contains(BorderFlags::RIGHT)
            || options
                .border_options
                .contains(BorderFlags::PRESERVE_CORNERS)
        {
            top_border.push_str(TOP_RIGHT);
        } else {
            top_border.push(' ');
        }
    }

    top_border = top_border
        .chars()
        .take(options.screen_size.x - options.position.x)
        .collect::<String>();

    let top_prefix = take_visible(&buffer[options.position.y], options.position.x);
    let top_suffix = &buffer[options.position.y]
        .get(top_prefix.len() + top_border.len()..)
        .unwrap_or("");

    buffer[options.position.y] = format!("{}{}{}", top_prefix, top_border, top_suffix);

    // Part 2: Bottom border
    if options.position.y + options.size.y - 1 < buffer.len() {
        let mut bottom_border = String::new();
        if options.border_options.contains(BorderFlags::BOTTOM) {
            if options.border_options.contains(BorderFlags::LEFT)
                || options
                    .border_options
                    .contains(BorderFlags::PRESERVE_CORNERS)
            {
                bottom_border.push_str(BOTTOM_LEFT);
            } else {
                bottom_border.push(' ');
            }

            let width = options.size.x.saturating_sub(2);
            if width > 0 {
                bottom_border.push_str(&HORZ_BORDER.repeat(width));
            }

            if options.border_options.contains(BorderFlags::RIGHT)
                || options
                    .border_options
                    .contains(BorderFlags::PRESERVE_CORNERS)
            {
                bottom_border.push_str(BOTTOM_RIGHT);
            } else {
                bottom_border.push(' ');
            }

            let bottom_index = options.position.y + options.size.y - 1;

            let bottom_prefix = take_visible(&buffer[bottom_index], options.position.x);
            let bottom_suffix = &buffer[bottom_index]
                .get(bottom_prefix.len() + bottom_border.len()..)
                .unwrap_or("");

            buffer[bottom_index] = format!("{}{}{}", bottom_prefix, bottom_border, bottom_suffix);
        }
    }

    // Part 3: Middle border
    let mut middle_border = String::new();
    if options.border_options.contains(BorderFlags::LEFT) {
        middle_border.push_str(VERT_BORDER);
    } else {
        middle_border.push(' ');
    }

    if options.background_color.is_some() {
        middle_border.push_str(options.background_color.as_ref().unwrap().bg().as_str());
    }

    let width = options.size.x.saturating_sub(2);
    if width > 0 {
        middle_border.push_str(&" ".repeat(width));
    }

    if options.background_color.is_some() {
        middle_border.push_str("\x1b[0m");
    }

    if options.border_options.contains(BorderFlags::RIGHT) {
        middle_border.push_str(VERT_BORDER);
    } else {
        middle_border.push(' ');
    }

    for i in 1..options.size.y.saturating_sub(1) {
        if options.position.y + i < buffer.len() {
            let middle_index = options.position.y + i;

            let middle_prefix = take_visible(&buffer[middle_index], options.position.x);

            let middle_suffix = &buffer[middle_index]
                .get(middle_prefix.len() + middle_border.len()..)
                .unwrap_or("");

            buffer[middle_index] = format!("{}{}{}", middle_prefix, middle_border, middle_suffix);
        }
    }

    // Note: This does not work because it fails to account for options.position.x
    // buffer[options.position.y] = top_border;
    // if options.position.y + options.size.y - 1 < buffer.len() {
    //     buffer[options.position.y + options.size.y - 1] = bottom_border;
    // }

    if crash {
        return Err(DrawError::HeightTooSmall);
    }

    Ok(())
}
