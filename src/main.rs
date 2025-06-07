extern crate termion;

use std::io::{Write, stdin, stdout};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod common;
mod draw;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    stdout.flush().unwrap();

    let size = termion::terminal_size().unwrap();
    println!("Terminal size: {}x{}", size.0, size.1);

    let mut buffer = vec![" ".repeat(size.0.into()); size.1 as usize];

    let box_coords = [
        common::Vec2 { x: 1, y: 1 },
        common::Vec2 { x: 20, y: 1 },
        common::Vec2 { x: 1, y: 8 },
        common::Vec2 { x: 6, y: 14 },
    ];

    for (i, coord) in box_coords.iter().enumerate() {
        draw::draw_box(
            &mut buffer,
            draw::BoxOptions {
                screen_size: common::Vec2 {
                    x: size.0 as usize,
                    y: size.1 as usize,
                },
                position: common::Vec2 {
                    x: coord.x,
                    y: coord.y,
                },
                size: common::Vec2 {
                    x: 10 as usize,
                    y: 4,
                },
                border_options: draw::BorderFlags::TOP
                    | draw::BorderFlags::PRESERVE_CORNERS
                    | draw::BorderFlags::LEFT,
                background_color: Some(common::Color {
                    r: (255 * (i == 0) as u8),
                    g: (255 * (i == 1) as u8),
                    b: (255 * (i == 2) as u8),
                }),
            },
            i == 4, // Alternate crash state for demonstration
        )
        .unwrap();
    }

    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    let final_buffer = buffer.join("\r\n");
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        final_buffer
    )
    .unwrap();

    stdout.flush().unwrap();

    for k in stdin.keys() {
        let final_buffer = buffer.join("\r\n");

        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            final_buffer
        )
        .unwrap();

        stdout.flush().unwrap();
        match k.unwrap() {
            termion::event::Key::Char('q') => break,
            _ => {}
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
