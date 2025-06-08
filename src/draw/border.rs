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
