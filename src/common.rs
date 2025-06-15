use crate::draw::boxes::BoxChar;

#[derive(Clone)]
pub struct Vec2<T = usize> {
    pub x: T,
    pub y: T,
}

#[derive(Clone)]
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

pub fn compile_buffer(buffer: &Vec<Vec<BoxChar>>) -> String {
    let mut result = String::new();
    for row in buffer {
        for box_char in row {
            result.push_str(&box_char.to_string());
        }
        result.push('\r');
        result.push('\n');
    }

    result[..result.len() - 2].to_string()
}
