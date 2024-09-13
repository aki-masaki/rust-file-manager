pub(crate) use crate::app::Dir;
use crossterm::style::Color;
use ratatui::prelude::Rect;
use ratatui::prelude::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::text::Text;
use ratatui::text::ToSpan;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use std::ffi::OsString;
use sysinfo::System;

use crate::App;

fn status_bar(frame: &mut Frame<'_>, app: &mut App) {
    if app.terminal_area.is_none() {
        return
    }

    let mut text = app.main_dir.full_path.clone() + " ";

    let selected_item_type: String;
    let selected_item_name: String = if app.selected_item.0.is_some() {
        selected_item_type = String::from("d");

        app.selected_item.0.clone().unwrap().name
    } else if app.selected_item.1.is_some() {
        selected_item_type = String::from("f");

        app.selected_item
            .1
            .clone()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_string()
    } else {
        selected_item_type = String::from("");

        String::from("")
    };

    text += users::get_current_username()
        .unwrap_or(OsString::from("user"))
        .to_str()
        .unwrap_or_default();
    text += "@";
    text += System::host_name().unwrap_or_default().as_str();

    text += " ";
    text += app.terminal_area.unwrap().width.to_string().as_str();
    text += "x";
    text += app.terminal_area.unwrap().height.to_string().as_str();

    text += " /";
    text += app.scroll.to_string().as_str();

    text += " @";
    text += app.selected_index.to_string().as_str();

    text += " #";
    text += app.selectable_items.len().to_string().as_str();
    text += " ";

    text += selected_item_type.as_str();
    text += " { ";
    text += selected_item_name.as_str();
    text += " } ";

    frame.render_widget(
        Paragraph::new(text).bg(Color::DarkGrey),
        Rect::new(0, 0, app.terminal_area.unwrap().width, 1),
    );
}

pub fn render_dir(frame: &mut Frame<'_>, dir: &Dir, level: i32) -> String {
    let mut text = String::new();

    for sub_dir in dir.sub_dirs.as_slice() {
        text += "  ".repeat(level as usize).as_str();
        text += &(sub_dir.name.as_str().to_owned() + "\n");

        if dir.expanded {
            text += render_dir(frame, sub_dir, level + 1).as_str();
        }
    }

    for sub_file in dir.sub_files.as_slice() {
         text += "  ".repeat(level as usize).as_str();
         text += &(sub_file.to_owned().1 + "\n");
    }

    text
}

fn draw_dir(frame: &mut Frame<'_>, rendered_dir: String, app: &mut App) {
    let lines: Vec<Span<'_>> = rendered_dir
        .split('\n')
        .into_iter()
        .skip(app.scroll)
        .map(|x| Span::from(x))
        .collect();

    let mut l_lines: Vec<Line> = Vec::new();

    for (i, line) in lines.clone().into_iter().enumerate() {
        if i == 50 {
            break;
        }

        let mut line = Line::from(line.to_string());

        if i == if app.selected_index < app.scroll {
            app.selected_index
        } else {
            app.selected_index - app.scroll
        } {
            line = line.fg(Color::DarkRed);
        }

        l_lines.push(line);
    }

    let text = Text::from(l_lines);

    frame.render_widget(
        Paragraph::new(text),
        Rect::new(0, 1, app.terminal_area.unwrap().width, app.terminal_area.unwrap().height - 2),
    );
}

pub fn ui(frame: &mut Frame<'_>, app: &mut App) {
    app.terminal_area = Some(frame.area());

    status_bar(frame, app);

    let rendered_dir = render_dir(frame, &app.main_dir, 0);
    draw_dir(frame, rendered_dir, app);
}
