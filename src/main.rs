extern crate termion;

use std::io::{Write, stdin};
use termion::input::TermRead;

mod common;
mod draw;
mod window;

use draw::border::BorderFlags;
use window::Container;

fn main() {
    let stdin = stdin();

    let size = termion::terminal_size().unwrap();

    let mut window = Container::new(common::Vec2::new(size.0 as usize, size.1 as usize));

    window.draw_box(window::DivOptions {
        id: Some("block_border_box".to_string()),

        position: common::Vec2::new(0, 0),
        size: common::Vec2::new(size.0 as usize, size.1 as usize),

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
    });

    window
        .draw_box_under(
            &"block_border_box".to_string(),
            window::DivOptions {
                id: Some("block_border_box_under".to_string()),

                position: common::Vec2::new(10, 10),
                size: common::Vec2::new(16, 8),

                border_options: BorderFlags::TOP
                    | BorderFlags::RIGHT
                    | BorderFlags::BOTTOM
                    | BorderFlags::LEFT,
                border_style: draw::border::BorderStyle::Rounded,

                border_color: Some(common::Color::GREEN),
                background_color: Some(common::Color::BLACK),
                text_color: Some(common::Color::WHITE),

                content: Some(vec![
                    "This is an under box.".to_string(),
                    "It is under the main box.".to_string(),
                ]),
            },
        )
        .unwrap();

    write!(window.stdout, "{}", termion::cursor::Hide).unwrap();

    window.render();

    for k in stdin.keys() {
        window.render();

        match k.unwrap() {
            termion::event::Key::Char('q') => break,
            _ => {}
        }
    }

    write!(window.stdout, "{}", termion::cursor::Show).unwrap();
}
