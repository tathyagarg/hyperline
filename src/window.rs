use std::io::{Stdout, Write};

use termion::raw::{IntoRawMode, RawTerminal};

use crate::common::{Vec2, compile_buffer};
use crate::draw::boxes::{self, BoxChar, draw_box};

pub struct Window {
    pub size: Vec2,
    pub buffer: Vec<Vec<BoxChar>>,

    pub stdout: RawTerminal<Stdout>,
}

impl Window {
    pub fn new(size: Vec2) -> Self {
        let stdout = std::io::stdout().into_raw_mode().unwrap();

        let buffer = vec![vec![BoxChar::default(); size.x as usize]; size.y as usize];

        Window {
            size,
            buffer,
            stdout,
        }
    }

    pub fn draw_box(&mut self, box_options: boxes::BoxOptions) -> () {
        draw_box(&mut self.buffer, box_options)
    }

    fn make_render(&mut self) -> String {
        compile_buffer(&self.buffer)
    }

    pub fn render(&mut self) -> () {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();

        let compiled = self.make_render();
        write!(self.stdout, "{}", compiled).unwrap();

        self.stdout.flush().unwrap();
    }

    pub fn clear(&mut self) -> Result<(), String> {
        self.buffer = vec![vec![BoxChar::default(); self.size.x as usize]; self.size.y as usize];
        Ok(())
    }
}
