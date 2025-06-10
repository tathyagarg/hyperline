extern crate termion;

use std::io::{Write, stdin, stdout};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod common;
mod draw;
use draw::border::BorderFlags;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    stdout.flush().unwrap();

    let size = termion::terminal_size().unwrap();
    println!("Terminal size: {}x{}", size.0, size.1);

    let mut buffer = vec![" ".repeat(size.0.into()); size.1 as usize];

    draw::boxes::draw_box(
        &mut buffer,
        draw::boxes::BoxOptions {
            screen_size: common::Vec2::new(size.0 as usize, size.1 as usize),

            position: common::Vec2::new(-2, 0),
            size: common::Vec2::new(10, size.1 as usize),

            border_options: BorderFlags::TOP
                | BorderFlags::RIGHT
                | BorderFlags::BOTTOM
                | BorderFlags::LEFT,
            border_style: draw::border::BorderStyle::Rounded,

            border_color: Some(common::Color::RED),
            background_color: Some(common::Color::BLACK),
            text_color: Some(common::Color::BLUE),

            content: Some(vec![
                "Hello, World!".to_string(),
                "Press 'q' to exit.".to_string(),
            ]),
        },
        false,
    )
    .unwrap();

    draw::boxes::draw_box(
        &mut buffer,
        draw::boxes::BoxOptions {
            screen_size: common::Vec2::new(size.0 as usize, size.1 as usize),

            position: common::Vec2::new(size.0 as i16 - 18, 10),
            size: common::Vec2::new(20, 10 as usize),

            border_options: BorderFlags::TOP
                | BorderFlags::RIGHT
                | BorderFlags::BOTTOM
                | BorderFlags::LEFT,
            border_style: draw::border::BorderStyle::Double,

            border_color: Some(common::Color::GREEN),
            background_color: Some(common::Color::BLACK),
            text_color: Some(common::Color::YELLOW),

            content: Some(vec![
                "This is a double border box.".to_string(),
                "Press 'q' to exit.".to_string(),
            ]),
        },
        false,
    )
    .unwrap();

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
