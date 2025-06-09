use bitflags::bitflags;

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

const THICK_BORDER_CHARS: BorderChars = BorderChars {
    top: "━",
    left: "┃",
    right: "┃",
    bottom: "━",
    top_left: "┏",
    top_right: "┓",
    bottom_left: "┗",
    bottom_right: "┛",
};

const DOUBLE_BORDER_CHARS: BorderChars = BorderChars {
    top: "═",
    left: "║",
    right: "║",
    bottom: "═",
    top_left: "╔",
    top_right: "╗",
    bottom_left: "╚",
    bottom_right: "╝",
};

const DOTTED_BORDER_CHARS: BorderChars = BorderChars {
    top: "╌",
    left: "╎",
    right: "╎",
    bottom: "╌",
    top_left: "┌",
    top_right: "┐",
    bottom_left: "└",
    bottom_right: "┘",
};

pub enum BorderStyle {
    Block,
    Rounded,
    Sharp,
    Thick,
    Double,
    Dotted,
}

impl BorderStyle {
    pub fn chars(&self) -> &BorderChars {
        match self {
            BorderStyle::Block => &BLOCK_BORDER_CHARS,
            BorderStyle::Rounded => &ROUNDED_BORDER_CHARS,
            BorderStyle::Sharp => &SHARP_BORDER_CHARS,
            BorderStyle::Thick => &THICK_BORDER_CHARS,
            BorderStyle::Double => &DOUBLE_BORDER_CHARS,
            BorderStyle::Dotted => &DOTTED_BORDER_CHARS,
        }
    }
}

bitflags! {
    #[derive(Eq, PartialEq, Clone, Copy, Debug)]
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

pub fn determine_edge(
    flags: &BorderFlags,
    style: &BorderStyle,
    position: BorderFlags,
) -> &'static str {
    let chars = style.chars();

    match (
        position.contains(BorderFlags::TOP),
        position.contains(BorderFlags::BOTTOM),
        position.contains(BorderFlags::LEFT),
        position.contains(BorderFlags::RIGHT),
    ) {
        (true, _, true, _) => {
            if flags.contains(BorderFlags::TOP | BorderFlags::LEFT)
                || (flags.contains(BorderFlags::PRESERVE_CORNERS)
                    && (flags.contains(BorderFlags::TOP) || flags.contains(BorderFlags::LEFT)))
            {
                chars.top_left
            } else if flags.contains(BorderFlags::TOP) {
                chars.top
            } else if flags.contains(BorderFlags::LEFT) {
                chars.left
            } else {
                " "
            }
        }

        (true, _, _, true) => {
            if flags.contains(BorderFlags::TOP | BorderFlags::RIGHT)
                || (flags.contains(BorderFlags::PRESERVE_CORNERS)
                    && (flags.contains(BorderFlags::TOP) || flags.contains(BorderFlags::RIGHT)))
            {
                chars.top_right
            } else if flags.contains(BorderFlags::TOP) {
                chars.top
            } else if flags.contains(BorderFlags::RIGHT) {
                chars.right
            } else {
                " "
            }
        }

        (_, true, true, _) => {
            if flags.contains(BorderFlags::BOTTOM | BorderFlags::LEFT)
                || (flags.contains(BorderFlags::PRESERVE_CORNERS)
                    && (flags.contains(BorderFlags::BOTTOM) || flags.contains(BorderFlags::LEFT)))
            {
                chars.bottom_left
            } else if flags.contains(BorderFlags::BOTTOM) {
                chars.bottom
            } else if flags.contains(BorderFlags::LEFT) {
                chars.left
            } else {
                " "
            }
        }

        (_, true, _, true) => {
            if flags.contains(BorderFlags::BOTTOM | BorderFlags::RIGHT)
                || (flags.contains(BorderFlags::PRESERVE_CORNERS)
                    && (flags.contains(BorderFlags::BOTTOM) || flags.contains(BorderFlags::RIGHT)))
            {
                chars.bottom_right
            } else if flags.contains(BorderFlags::BOTTOM) {
                chars.bottom
            } else if flags.contains(BorderFlags::RIGHT) {
                chars.right
            } else {
                " "
            }
        }

        (true, _, _, _) => {
            if flags.contains(BorderFlags::TOP) {
                chars.top
            } else {
                " "
            }
        }

        (_, true, _, _) => {
            if flags.contains(BorderFlags::BOTTOM) {
                chars.bottom
            } else {
                " "
            }
        }

        (_, _, true, _) => {
            if flags.contains(BorderFlags::LEFT) {
                chars.left
            } else {
                " "
            }
        }

        (_, _, _, true) => {
            if flags.contains(BorderFlags::RIGHT) {
                chars.right
            } else {
                " "
            }
        }

        _ => " ",
    }
}
