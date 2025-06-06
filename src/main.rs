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
    println!("Terminal size: {}x{}", size.0, size.1);

    let mut buffer = vec![" ".repeat(size.0.into()); size.1 as usize];

    let box_coords = [
        common::Vec2 { x: 1, y: 1 },
        common::Vec2 { x: 6, y: 1 },
        common::Vec2 { x: 1, y: 5 },
        common::Vec2 { x: 6, y: 5 },
    ];

    for coord in box_coords.iter() {
        buffer = draw::draw_box(
            buffer.clone(),
            draw::BoxOptions {
                position: common::Vec2 { x: coord.x, y: coord.y },
                size: common::Vec2 { x: 5, y: 4 },
                border_options: draw::BorderFlags::ALL,
            }
        ).unwrap();
    }

    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    let final_buffer = buffer.join("\r\n");
    write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), final_buffer).unwrap();

    stdout.flush().unwrap();

    for k in stdin.keys() {
        let final_buffer = buffer.join("\r\n");

        write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), final_buffer).unwrap();

        stdout.flush().unwrap();
        match k.unwrap() {
            termion::event::Key::Char('q') => break,
            _ => {}
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
