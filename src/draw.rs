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

const MIN_WIDTH: usize = 0;
const MIN_HEIGHT: usize = 0;

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
    pub position: common::Vec2,
    pub size: common::Vec2,
    pub border_options: BorderFlags,
}

fn determine_corner(location: BorderFlags, options: &BorderFlags) -> bool {
    let count = location.bits().count_ones();
    (count == 2) || (count == 1 && options.contains(BorderFlags::PRESERVE_CORNERS))
}

pub fn draw_box(
    buffer: Vec<String>,
    options: BoxOptions,
) -> Result<Vec<String>, DrawError> {
    let mut buffer = buffer;
    let max_size = common::Vec2 {
        x: buffer.iter().map(|s| s.len()).max().unwrap(),
        y: buffer.len(),
    };

    let use_width = cmp::max(MIN_WIDTH, options.size.x - 2);
    let use_height = cmp::max(MIN_HEIGHT, options.size.y - 2);

    let top_left = if determine_corner(BorderFlags::LEFT | BorderFlags::TOP, &options.border_options) { TOP_LEFT } else { " " };
    let top_right = if determine_corner(BorderFlags::RIGHT | BorderFlags::TOP, &options.border_options) { TOP_RIGHT } else { " " };
    let bottom_left = if determine_corner(BorderFlags::LEFT | BorderFlags::BOTTOM, &options.border_options) { BOTTOM_LEFT } else { " " };
    let bottom_right = if determine_corner(BorderFlags::RIGHT | BorderFlags::BOTTOM, &options.border_options) { BOTTOM_RIGHT } else { " " };

    let top = if options.border_options.contains(BorderFlags::TOP) { HORZ_BORDER } else { " " };
    let bottom = if options.border_options.contains(BorderFlags::BOTTOM) { HORZ_BORDER } else { " " };
    let left = if options.border_options.contains(BorderFlags::LEFT) { VERT_BORDER } else { " " };
    let right = if options.border_options.contains(BorderFlags::RIGHT) { VERT_BORDER } else { " " };

    let max_width_edge = (2 + cmp::min(max_size.x - 2 - options.position.x, use_width)) * BOX_CHAR_WIDTH;
    let max_width_mid = (2 * BOX_CHAR_WIDTH) + cmp::min(max_size.x - (2 * BOX_CHAR_WIDTH) - options.position.x, use_width);

    let mut line = buffer[options.position.y].chars().collect::<Vec<char>>();
    let replacement = format!(
        "{}{}{}",
        top_left,
        top.repeat(use_width),
        top_right
    ).chars().take(max_width_edge).collect::<Vec<char>>();

    line.splice(
        options.position.x..(options.size.x + options.position.x),
        replacement,
    );

    buffer[options.position.y] = line.iter().collect::<String>();

    for i in 0..use_height {
        if (options.position.y + 1 + i) < buffer.len() {
            let mut line = buffer[options.position.y + 1 + i].chars().collect::<Vec<char>>();
            let replacement = format!(
                "{}{}{}",
                left,
                " ".repeat(use_width),
                right
            ).chars().take(max_width_mid).collect::<Vec<char>>();

            line.splice(
                options.position.x..(options.size.x + options.position.x),
                replacement,
            );

            buffer[options.position.y + 1 + i] = line.iter().collect::<String>();
        }
    }

    // Don't draw bottom border if overflowing
    if (options.position.y + 1 + use_height) >= buffer.len() {
        return Ok(buffer);
    }

    let mut line = buffer[options.position.y + 1 + use_height].chars().collect::<Vec<char>>();
    let replacement: Vec<char> = format!(
        "{}{}{}",
        bottom_left,
        bottom.repeat(use_width),
        bottom_right
    ).chars().take(max_width_edge).collect();

    line.splice(
        options.position.x..(options.size.x + options.position.x),
        replacement,
    );

    buffer[options.position.y + 1 + use_height] = line.iter().collect::<String>();

    Ok(buffer)
}
