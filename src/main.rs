extern crate termion;

use std::io::{Write};

mod draw;

fn main() {
    let stdout = std::io::stdout();
    let stdin = std::io::stdin();

    _ = draw::draw_box(stdout.lock(), 10, 5, vec![
        "Hello".to_string(),
        "World".to_string(),
        "This".to_string(),
    ], draw::DrawFlags::TOP | draw::DrawFlags::BOTTOM);

    std::io::stdout().flush().unwrap();
}
