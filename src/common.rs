use regex::Regex;

pub struct Vec2<T = usize> {
    pub x: T,
    pub y: T,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Vec2 {
    pub fn new<T>(x: T, y: T) -> Vec2<T> {
        Vec2::<T> { x, y }
    }
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
    pub const YELLOW: Color = Color {
        r: 255,
        g: 255,
        b: 0,
    };
    pub const CYAN: Color = Color {
        r: 0,
        g: 255,
        b: 255,
    };
    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
    };

    pub fn fg(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    pub fn bg(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }
}

pub fn visible_len(s: &str) -> usize {
    let re = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
    let cleaned = re.replace_all(s, "");
    cleaned.chars().count()
}

pub fn take_visible_chars(s: &str, n: usize) -> String {
    let mut result = String::new();
    let mut count = 0;
    let mut i = 0;
    let bytes = s.as_bytes();

    while i < bytes.len() && count < n {
        if bytes[i] == b'\x1b' {
            result.push('\x1b');
            i += 1;

            while i < bytes.len() {
                let c = bytes[i] as char;
                result.push(c);
                i += 1;

                if c == 'm' || c == 'K' || c == 'J' {
                    break;
                }
            }
        } else {
            let ch = s[i..].chars().next().unwrap();
            result.push(ch);

            i += ch.len_utf8();

            count += 1;
        }
    }

    result
}
