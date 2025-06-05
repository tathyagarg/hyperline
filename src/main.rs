extern crate termion;

use std::io::{
    stdout,
    stdin,
    Write,
};
use termion::raw::IntoRawMode;
use termion::input::TermRead;

mod draw;
mod common;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    stdout.flush().unwrap();

    let size = termion::terminal_size().unwrap();
    let mut buffer = vec!["".to_string(); size.1 as usize];

    buffer = draw::draw_box(
        buffer.clone(),
        common::Vec2{ x: 0, y: 0 },
        common::Vec2{ x: size.0 as usize, y: size.1 as usize },
        vec![
            "Hello".to_string(),
            "World".to_string(),
            "This".to_string(),
        ],
        draw::DrawFlags::ALL
    ).unwrap();

    for k in stdin.keys() {
        match k.unwrap() {
            termion::event::Key::Char('q') => break,
            _ => {}
        }

        let final_buffer = buffer.join("\r\n");

        write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), final_buffer).unwrap();

        stdout.flush().unwrap();
    }
}
