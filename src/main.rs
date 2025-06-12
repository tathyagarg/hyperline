extern crate termion;

use std::io::{Write, stdin};
use termion::input::TermRead;

mod common;
mod draw;
mod window;

use draw::border::BorderFlags;
use window::Window;

fn main() {
    let stdin = stdin();

    let size = termion::terminal_size().unwrap();

    let mut window = Window::new(common::Vec2::new(size.0 as usize, size.1 as usize));
    window.draw_box(draw::boxes::BoxOptions {
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
    });

    window.draw_box(draw::boxes::BoxOptions {
        screen_size: common::Vec2::new(size.0 as usize, size.1 as usize),

        position: common::Vec2::new(size.0 as i16 - 18, 10),
        size: common::Vec2::new(20, 10 as usize),

        border_options: BorderFlags::TOP
            | BorderFlags::RIGHT
            | BorderFlags::BOTTOM
            | BorderFlags::LEFT,
        border_style: draw::border::BorderStyle::Sharp,

        border_color: Some(common::Color::GREEN),
        background_color: Some(common::Color::BLACK),
        text_color: Some(common::Color::YELLOW),

        content: Some(vec![
            "This is a double border box.".to_string(),
            "Press 'q' to exit.".to_string(),
        ]),
    });

    window.draw_box(draw::boxes::BoxOptions {
        screen_size: common::Vec2::new(size.0 as usize, size.1 as usize),

        position: common::Vec2::new(0, size.1 as i16 - 5),
        size: common::Vec2::new(8, 2),

        border_options: BorderFlags::RIGHT,
        border_style: draw::border::BorderStyle::Rounded,

        border_color: Some(common::Color::RED),
        background_color: Some(common::Color::CYAN),
        text_color: Some(common::Color::MAGENTA),

        content: Some(vec![
            "This is a borderless box.".to_string(),
            "Press 'q' to exit.".to_string(),
        ]),
    });

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
