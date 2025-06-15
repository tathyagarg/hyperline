use std::io::{Stdout, Write};

use termion::raw::{IntoRawMode, RawTerminal};

use crate::common::{self, Vec2, compile_buffer};
use crate::draw::border;
use crate::draw::boxes::{self, BoxChar, draw_box};

pub struct Container {
    pub size: Vec2,
    pub buffer: Vec<Vec<BoxChar>>,

    pub stdout: RawTerminal<Stdout>,

    objects: Vec<DivOptions>,
}

#[derive(Clone)]
pub struct DivOptions {
    pub id: Option<String>,

    pub position: Vec2<i16>,
    pub size: Vec2,

    pub border_options: border::BorderFlags,
    pub border_style: border::BorderStyle,

    pub border_color: Option<common::Color>,
    pub background_color: Option<common::Color>,
    pub text_color: Option<common::Color>,

    pub content: Option<Vec<String>>,
}

impl Container {
    pub fn new(size: Vec2) -> Self {
        let stdout = std::io::stdout().into_raw_mode().unwrap();

        let buffer = vec![vec![BoxChar::default(); size.x as usize]; size.y as usize];

        Container {
            size,
            buffer,
            stdout,
            objects: Vec::new(),
        }
    }

    pub fn draw_box(&mut self, div_options: DivOptions) -> () {
        let size = &self.size;

        let option_data = div_options.clone();

        let options = boxes::BoxOptions {
            screen_size: size,

            position: option_data.position,
            size: option_data.size,

            border_options: option_data.border_options,
            border_style: option_data.border_style,

            border_color: option_data.border_color,
            background_color: option_data.background_color,
            text_color: option_data.text_color,

            content: option_data.content,
        };

        if div_options.id.is_some() {
            self.objects.push(div_options.clone());
        }

        draw_box(&mut self.buffer, options);
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
