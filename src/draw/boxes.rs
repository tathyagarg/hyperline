use std::cmp;

use crate::common;
use crate::draw::border::BorderFlags;
use crate::draw::border::{self, determine_edge};

#[derive(Debug)]
pub enum DrawError {
    HeightTooSmall,
    // ContentTooLong,
}

pub struct BoxOptions {
    pub screen_size: common::Vec2,

    pub position: common::Vec2<i16>,
    pub size: common::Vec2,

    pub border_options: BorderFlags,
    pub border_style: border::BorderStyle,

    pub border_color: Option<common::Color>,
    pub background_color: Option<common::Color>,
    pub text_color: Option<common::Color>,

    pub content: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoxChar {
    prefix: String,
    content: String,
    suffix: String,
}

impl BoxChar {
    pub fn to_string(&self) -> String {
        format!("{}{}{}", self.prefix, self.content, self.suffix)
    }
}

fn compile_border_string(border_chars: &Vec<&mut BoxChar>) -> String {
    let mut border_string = String::new();
    for border_char in border_chars.iter() {
        border_string.push_str(&border_char.to_string());
    }

    border_string
}

fn make_border(left: &str, middle: &str, right: &str, width: usize) -> Vec<BoxChar> {
    let mut border_str = Vec::new();

    border_str.push(BoxChar {
        prefix: String::new(),
        content: left.to_string(),
        suffix: String::new(),
    });

    for _ in 0..width {
        border_str.push(BoxChar {
            prefix: String::new(),
            content: middle.to_string(),
            suffix: String::new(),
        });
    }

    border_str.push(BoxChar {
        prefix: String::new(),
        content: right.to_string(),
        suffix: String::new(),
    });

    border_str
}

fn add_background_color(
    border: &mut Vec<&mut BoxChar>,
    border_style: &border::BorderStyle,
    background_color: &Option<common::Color>,
) {
    if let Some(bg_color) = background_color {
        let border_width = border_style.chars().border_width();
        let bg_ansi = bg_color.bg();

        if border.first_mut().unwrap().content.len() == border_width {
            if let Some(character) = border.get_mut(1) {
                character.prefix.insert_str(0, &bg_ansi);
            }

            let mut index = border.len() - 1;
            if border.last().unwrap().content.len() == border_width {
                index -= 1;
            }
            if let Some(last) = border.get_mut(index) {
                last.suffix.insert_str(0, "\x1b[0m");
            }
        } else {
            border.first_mut().unwrap().prefix.insert_str(0, &bg_ansi);

            let mut index = border.len() - 1;
            if border.last().unwrap().content.len() == border_width {
                index -= 1;
            }

            border
                .get_mut(index)
                .unwrap()
                .suffix
                .insert_str(0, "\x1b[0m");
        }
    }
}

fn add_left_border_color(border: &mut Vec<&mut BoxChar>, border_color: &Option<common::Color>) {
    if let Some(border_color) = border_color {
        let border_ansi = border_color.fg();
        if let Some(first) = border.first_mut() {
            first.prefix.insert_str(0, &border_ansi);
            first.suffix.insert_str(0, "\x1b[0m");
        }
    }
}

fn add_right_border_color(border: &mut Vec<&mut BoxChar>, border_color: &Option<common::Color>) {
    if let Some(border_color) = border_color {
        let border_ansi = border_color.fg();
        if let Some(last) = border.last_mut() {
            last.prefix.insert_str(0, &border_ansi);
            last.suffix.insert_str(0, "\x1b[0m");
        }
    }
}

fn add_edge_border_color(border: &mut Vec<&mut BoxChar>, border_color: &Option<common::Color>) {
    if let Some(border_color) = border_color {
        let border_ansi = border_color.fg();
        if let Some(first) = border.first_mut() {
            first.prefix.insert_str(0, &border_ansi);
        }

        if let Some(last) = border.last_mut() {
            last.suffix.insert_str(0, "\x1b[0m");
        }
    }
}

fn add_text_color(
    border: &mut Vec<&mut BoxChar>,
    border_style: &border::BorderStyle,
    text_color: &Option<common::Color>,
) {
    if let Some(text_color) = text_color {
        let border_width = border_style.chars().border_width();
        let text_ansi = text_color.fg();

        let mut start_index = 0;
        if border.first().unwrap().content.len() == border_width {
            start_index = 1;
        }

        let mut last_index = border.len() - 1;
        if border.last().unwrap().content.len() == border_width {
            last_index -= 1;
        }

        for char in border
            .iter_mut()
            .skip(start_index)
            .take(last_index - start_index + 1)
        {
            char.prefix.insert_str(0, &text_ansi);
        }
    }
}

fn draw_edge(buffer: &mut Vec<String>, options: &BoxOptions, flags: BorderFlags, index: usize) {
    let (left, middle, right) = (
        determine_edge(
            &options.border_options,
            &options.border_style,
            flags | BorderFlags::LEFT,
        ),
        determine_edge(&options.border_options, &options.border_style, flags),
        determine_edge(
            &options.border_options,
            &options.border_style,
            flags | BorderFlags::RIGHT,
        ),
    );

    let mut edge_data = make_border(left, middle, right, options.size.x.saturating_sub(2));

    let mut edge = edge_data
        .iter_mut()
        .skip(cmp::max(0, -options.position.x) as usize)
        .take(cmp::min(
            options.screen_size.x,
            cmp::min(
                options.size.x,
                (options.screen_size.x as i16 - options.position.x) as usize,
            ),
        ))
        .collect::<Vec<_>>();

    // let prefix = buffer[index]
    //     .chars()
    //     .take(cmp::min(
    //         cmp::max(options.position.x, 0) as usize,
    //         buffer[index].len(),
    //     ))
    //     .collect::<String>();

    let prefix = common::take_visible_chars(
        &buffer[index],
        cmp::min(
            cmp::max(options.position.x, 0) as usize,
            buffer[index].len(),
        ),
    );

    // if options.position.x == 20 {
    //     panic!(
    //         "prefix: '{}', edge: '{:?}', buffer: {:?}, index: {}",
    //         prefix, edge, buffer, index
    //     );
    // }

    let suffix = buffer[index].get(prefix.len() + edge.len()..).unwrap_or("");

    if options.border_options.contains(BorderFlags::TOP)
        || options.border_options.contains(BorderFlags::BOTTOM)
    {
        add_edge_border_color(&mut edge, &options.border_color);
    }

    if !((flags == BorderFlags::TOP && options.border_options.contains(BorderFlags::TOP))
        || (flags == BorderFlags::BOTTOM && options.border_options.contains(BorderFlags::BOTTOM)))
    {
        add_background_color(&mut edge, &options.border_style, &options.background_color);
    }

    let compiled = compile_border_string(&edge);

    buffer[index] = format!("{}{}{}", prefix, compiled, suffix);
}

pub fn draw_box(
    buffer: &mut Vec<String>,
    options: BoxOptions,
    crash: bool,
) -> Result<(), DrawError> {
    if options.position.y >= 0 && options.border_options.contains(BorderFlags::TOP) {
        draw_edge(
            buffer,
            &options,
            BorderFlags::TOP,
            cmp::max(options.position.y, 0) as usize,
        );
    }

    // Part 2: Bottom border
    let bottom_index = options.position.y + (options.size.y as i16) - 1;
    if bottom_index >= 0
        && bottom_index < (buffer.len() as i16)
        && options.border_options.contains(BorderFlags::BOTTOM)
    {
        draw_edge(buffer, &options, BorderFlags::BOTTOM, bottom_index as usize);
    }

    // Part 3: Middle border
    let mut middle_border_data = make_border(
        determine_edge(
            &options.border_options,
            &options.border_style,
            BorderFlags::LEFT,
        ),
        " ",
        determine_edge(
            &options.border_options,
            &options.border_style,
            BorderFlags::RIGHT,
        ),
        options.size.x.saturating_sub(2),
    );

    let mut middle_border = middle_border_data
        .iter_mut()
        .skip(cmp::max(0, -options.position.x) as usize)
        .take(cmp::min(
            options.screen_size.x,
            cmp::min(
                options.size.x,
                (options.screen_size.x as i16 - options.position.x) as usize,
            ),
        ))
        .collect::<Vec<_>>();

    add_background_color(
        &mut middle_border,
        &options.border_style,
        &options.background_color,
    );

    if options.border_options.contains(BorderFlags::LEFT) && options.position.x >= 0 {
        add_left_border_color(&mut middle_border, &options.border_color);
    }

    if options.border_options.contains(BorderFlags::RIGHT)
        && (options.position.x + options.size.x as i16) < (options.screen_size.x as i16)
    {
        add_right_border_color(&mut middle_border, &options.border_color);
    }

    add_text_color(
        &mut middle_border,
        &options.border_style,
        &options.text_color,
    );

    for i in 0..options.size.y {
        if i == 0 && options.border_options.contains(BorderFlags::TOP) {
            continue;
        }

        if i == options.size.y - 1 && options.border_options.contains(BorderFlags::BOTTOM) {
            continue;
        }

        let middle_index = options.position.y + (i as i16);
        if middle_index >= 0 && middle_index < (buffer.len() as i16) {
            let mut this_line = middle_border
                .iter()
                .map(|c| (*c).clone())
                .collect::<Vec<_>>();

            let mut this_line: Vec<&mut BoxChar> = this_line.iter_mut().collect();

            let middle_prefix = common::take_visible_chars(
                &buffer[middle_index as usize],
                cmp::max(options.position.x, 0) as usize,
            );

            let middle_suffix = &buffer[middle_index as usize]
                .get(middle_prefix.len() + middle_border.len()..)
                .unwrap_or("");

            if options.content.is_some()
                && options.content.as_ref().unwrap().len()
                    > i - (options.border_options.contains(BorderFlags::TOP) as usize)
            {
                let content = options
                    .content
                    .as_ref()
                    .unwrap()
                    .get(i - (options.border_options.contains(BorderFlags::TOP) as usize))
                    .unwrap();

                for (j, char) in content
                    .chars()
                    .skip(cmp::max(0, -options.position.x - 1) as usize)
                    .enumerate()
                {
                    let index = j
                        + (options.border_options.contains(BorderFlags::LEFT)
                            && options.position.x > 0) as usize;

                    if index
                        >= this_line.len()
                            - (options.border_options.contains(BorderFlags::RIGHT)
                                && (options.position.x + options.size.x as i16)
                                    < (options.screen_size.x as i16))
                                as usize
                    {
                        continue;
                    } else {
                        this_line[index].content = char.to_string();
                    }
                }
            }

            buffer[middle_index as usize] = format!(
                "{}{}{}",
                middle_prefix,
                compile_border_string(&this_line),
                middle_suffix
            );

            // if options.position.x == 20 {
            //     panic!(
            //         "\rmiddle_prefix: '{}',\r\nline: {:?},\r\nindex: {}",
            //         middle_prefix, this_line, middle_index
            //     );
            // }
        }
    }

    if crash {
        return Err(DrawError::HeightTooSmall);
    }

    Ok(())
}
