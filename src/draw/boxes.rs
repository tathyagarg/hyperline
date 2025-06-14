use std::cmp;

use crate::common;
use crate::draw::border::{self, BorderFlags, determine_edge};

#[derive(Debug)]
pub enum DrawError {
    HeightTooSmall,
    // ContentTooLong,
}

pub struct BoxOptions<'a> {
    pub screen_size: &'a common::Vec2,

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
pub struct BoxChar {
    fg: String,
    bg: String,
    // prefix: String,
    content: String,
}

impl Default for BoxChar {
    fn default() -> Self {
        BoxChar {
            fg: String::new(),
            bg: String::new(),
            // prefix: String::new(),
            content: String::from(" "),
        }
    }
}

impl BoxChar {
    pub fn to_string(&self) -> String {
        // format!("{}{}{}", self.prefix, self.content, self.suffix)
        format!("{}{}{}\x1b[0m", self.fg, self.bg, self.content)
    }
}

fn make_border(left: &str, middle: &str, right: &str, width: usize) -> Vec<BoxChar> {
    let mut border_str = Vec::new();

    border_str.push(BoxChar {
        fg: String::new(),
        bg: String::new(),
        content: left.to_string(),
    });
    for _ in 0..width {
        border_str.push(BoxChar {
            fg: String::new(),
            bg: String::new(),
            content: middle.to_string(),
        });
    }

    border_str.push(BoxChar {
        fg: String::new(),
        bg: String::new(),
        content: right.to_string(),
    });

    border_str
}

fn add_background_color(
    border: &mut Vec<BoxChar>,
    border_style: &border::BorderStyle,
    background_color: &Option<common::Color>,
) {
    if let Some(bg_color) = background_color {
        let border_width = border_style.chars().border_width();
        let bg_ansi = bg_color.bg();

        let skipping = (border.first().unwrap().content.len() == border_width) as usize;
        let taking = border.len()
            - (border.last().unwrap().content.len() == border_width) as usize
            - skipping;

        for char in border.iter_mut().skip(skipping).take(taking) {
            char.bg = bg_ansi.clone();
        }
    }
}

fn add_left_border_color(border: &mut Vec<BoxChar>, border_color: &Option<common::Color>) {
    if let Some(border_color) = border_color {
        let border_ansi = border_color.fg();
        if let Some(first) = border.first_mut() {
            first.fg = border_ansi.clone();
        }
    }
}

fn add_right_border_color(border: &mut Vec<BoxChar>, border_color: &Option<common::Color>) {
    if let Some(border_color) = border_color {
        let border_ansi = border_color.fg();
        if let Some(last) = border.last_mut() {
            last.fg = border_ansi.clone();
        }
    }
}

fn add_edge_border_color(border: &mut Vec<BoxChar>, border_color: &Option<common::Color>) {
    if let Some(border_color) = border_color {
        let border_ansi = border_color.fg();

        for char in border.iter_mut() {
            char.fg = border_ansi.clone();
        }
    }
}

fn add_text_color(
    border: &mut Vec<BoxChar>,
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
            char.fg = text_ansi.clone();
        }
    }
}

fn draw_edge(
    buffer: &mut Vec<Vec<BoxChar>>,
    options: &BoxOptions,
    flags: BorderFlags,
    index: usize,
) {
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
        .map(|c| c.clone())
        .collect::<Vec<_>>();

    let prefix = buffer[index]
        .iter()
        .take(cmp::max(options.position.x, 0) as usize)
        .map(|c| c.clone())
        .collect::<Vec<_>>();

    let suffix = buffer[index]
        .get(prefix.len() + edge.len()..)
        .unwrap_or(&vec![])
        .iter()
        .map(|c| c.clone())
        .collect::<Vec<_>>();

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

    let mut new_row = Vec::new();
    new_row.extend(prefix);
    new_row.extend(edge.into_iter().map(|c| c.clone()));
    new_row.extend(suffix);

    buffer[index] = new_row;
}

pub fn draw_box(buffer: &mut Vec<Vec<BoxChar>>, options: BoxOptions) -> () {
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
    let middle_border_data = make_border(
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
        .iter()
        .skip(cmp::max(0, -options.position.x) as usize)
        .take(cmp::min(
            options.screen_size.x,
            cmp::min(
                options.size.x,
                (options.screen_size.x as i16 - options.position.x) as usize,
            ),
        ))
        .map(|c| c.clone())
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
        && (options.position.x + options.size.x as i16) <= (options.screen_size.x as i16)
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

            let middle_prefix = buffer[middle_index as usize]
                .iter()
                .take(cmp::max(options.position.x, 0) as usize)
                .map(|c| c.clone())
                .collect::<Vec<_>>();

            let middle_suffix = buffer[middle_index as usize]
                .get(middle_prefix.len() + this_line.len()..)
                .unwrap_or(&vec![])
                .iter()
                .map(|c| c.clone())
                .collect::<Vec<_>>();

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
                            && options.position.x >= 0) as usize;

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

            // Combine the prefix, content, and suffix into the final line
            let mut buffer_line = Vec::new();
            buffer_line.extend(middle_prefix);
            buffer_line.extend(this_line.iter().map(|c| c.clone()));
            buffer_line.extend(middle_suffix);
            buffer[middle_index as usize] = buffer_line;
        }
    }
}
