use std::cmp;

use bitflags::bitflags;

use crate::common;

const HORZ_BORDER: &'static str = "─";
const VERT_BORDER: &'static str = "│";
const TOP_LEFT: &'static str = "╭";
const TOP_RIGHT: &'static str = "╮";
const BOTTOM_LEFT: &'static str = "╰";
const BOTTOM_RIGHT: &'static str = "╯";

const MIN_WIDTH: usize = 2;
const MIN_HEIGHT: usize = 1;

const DEFAULT_STRING: &'static str = "";

#[derive(Debug)]
pub enum DrawError {
    HeightTooSmall,
    ContentTooLong,
}

bitflags! {
    pub struct DrawFlags: u8 {
        const NONE = 0b0000_0000;
        const TOP = 0b0000_0001;
        const BOTTOM = 0b0000_0010;
        const LEFT = 0b0000_0100;
        const RIGHT = 0b0000_1000;

        const PRESERVE_CORNERS = 0b0001_0000;

        const ALL = Self::TOP.bits() | Self::BOTTOM.bits() | Self::LEFT.bits() | Self::RIGHT.bits() | Self::PRESERVE_CORNERS.bits();
    }
}

fn determine_corner(location: DrawFlags, options: &DrawFlags) -> bool {
    let count = location.bits().count_ones();
    (count == 2) || (count == 1 && options.contains(DrawFlags::PRESERVE_CORNERS))
}

pub fn draw_box(buffer: Vec<String>, position: common::Vec2, size: common::Vec2, content_lines: Vec<String>, options: DrawFlags) -> Result<Vec<String>, DrawError> {
    let mut buffer = buffer;

    let use_width = cmp::max(MIN_WIDTH, size.x - 2);
    let use_height = cmp::max(MIN_HEIGHT, size.y - 2);

    if content_lines.len() > use_height {
        return Err(DrawError::HeightTooSmall);
    }

    let empty = &DEFAULT_STRING.to_string();

    let top_left = if determine_corner(DrawFlags::LEFT | DrawFlags::TOP, &options) { TOP_LEFT } else { " " };
    let top_right = if determine_corner(DrawFlags::RIGHT | DrawFlags::TOP, &options) { TOP_RIGHT } else { " " };
    let bottom_left = if determine_corner(DrawFlags::LEFT | DrawFlags::BOTTOM, &options) { BOTTOM_LEFT } else { " " };
    let bottom_right = if determine_corner(DrawFlags::RIGHT | DrawFlags::BOTTOM, &options) { BOTTOM_RIGHT } else { " " };

    let top = if options.contains(DrawFlags::TOP) { HORZ_BORDER } else { " " };
    let bottom = if options.contains(DrawFlags::BOTTOM) { HORZ_BORDER } else { " " };
    let left = if options.contains(DrawFlags::LEFT) { VERT_BORDER } else { " " };
    let right = if options.contains(DrawFlags::RIGHT) { VERT_BORDER } else { " " };

    buffer[position.y] = format!("{}{}{}", top_left, top.repeat(use_width), top_right);

    for i in 0..use_height {
        let curr_line = content_lines.get(i).unwrap_or(empty);

        if curr_line.len() > use_width {
            return Err(DrawError::ContentTooLong);
        }

        let content = if curr_line.len() <= use_width {
            let line = curr_line;
            let padded_line = format!("{:<1$}", line, use_width);
            padded_line
        } else {
            " ".repeat(use_width)
        };

        if (position.y + 1 + i) >= buffer.len() {
            continue;
        }
        buffer[position.y + 1 + i] = format!("{}{}{}", left, content, right);
    }
    if (position.y + 1 + use_height) >= buffer.len() {
        return Ok(buffer);
    }
    buffer[position.y + 1 + use_height] = format!("{}{}{}", bottom_left, bottom.repeat(use_width), bottom_right);

    Ok(buffer)
}
