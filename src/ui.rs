pub(crate) use crate::app::Dir;
use crossterm::style::Color;
use ratatui::prelude::Rect;
use ratatui::prelude::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use sysinfo::System;

use crate::App;

fn get_icon(ext: String) -> (String, Color) {
    let icon = match ext.as_str() {
        "png" => ("\u{e2a6} ", Color::Red),
        "rs" => ("\u{e7a8} ", Color::Red),
        "go" => ("\u{e627} ", Color::Blue),
        "toml|conf" => ("\u{e615} ", Color::Grey),
        "ts" => ("\u{e628} ", Color::Blue),
        "js" => ("\u{e60c} ", Color::Yellow),
        "java" => ("\u{e738} ", Color::Red),
        "zip" => ("\u{f06eb} ", Color::Magenta),
        _ => ("\u{f4a5} ", Color::White),
    };

    (icon.0.to_string(), icon.1)
}

fn status_bar(frame: &mut Frame<'_>, app: &mut App) {
    if app.terminal_area.is_none() {
        return;
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

pub fn render_dir<'a>(frame: &mut Frame<'_>, dir: &Dir, level: i32) -> Vec<Line<'a>> {
    let mut spans: Vec<Span> = vec![];
    let mut lines: Vec<Line<'a>> = vec![];

    for (i, sub_dir) in dir.sub_dirs.as_slice().into_iter().enumerate() {
        let mut line = Line::default();

        if level > 0 && i > 0 {
            line.push_span(Span::from("  ".repeat(level as usize + 1)));
        } else if level > 0 && i == 0 {
            line.push_span(Span::from("  ".repeat(level as usize)));

            line.push_span(Span::from("└ "));
        }

        line.push_span(Span::from("\u{f024b} "));

        line.push_span(Span::from(sub_dir.name.as_str().to_owned() + "\n"));

        lines.push(line);

        if dir.expanded {
            lines.extend(render_dir(frame, sub_dir, level + 1));
        }
    }

    for sub_file in dir.sub_files.as_slice() {
        spans.push(Span::from("  ".repeat(level as usize + 1)));

        let puf = PathBuf::from(sub_file.1.clone());

        let icon = get_icon(
            Path::new(puf.file_name().unwrap())
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string(),
        );

        spans.push(Span::from(icon.0).bg(icon.1));

        spans.push(Span::from(sub_file.to_owned().1 + "\n"));

        lines.push(Line::from(spans.clone()));
        spans.clear();
    }

    lines
}

fn draw_dir<'a>(frame: &mut Frame<'_>, rendered_dir: Vec<Line<'a>>, app: &mut App) {
    let lines: Vec<Line<'a>> = rendered_dir.into_iter().skip(app.scroll).collect();

    let mut l_lines: Vec<Line> = Vec::new();

    for (i, line) in lines.clone().into_iter().enumerate() {
        if i == 50 {
            break;
        }

        let mut line = Line::from(line.to_string()).fg(Color::White);

        if i == if app.selected_index < app.scroll {
            app.selected_index
        } else {
            app.selected_index - app.scroll
        } {
            line = line.fg(Color::Red);
        }

        l_lines.push(line);
    }

    frame.render_widget(
        Paragraph::new(l_lines),
        Rect::new(
            0,
            1,
            app.terminal_area.unwrap().width,
            app.terminal_area.unwrap().height - 2,
        ),
    );
}

pub fn ui(frame: &mut Frame<'_>, app: &mut App) {
    app.terminal_area = Some(frame.area());

    status_bar(frame, app);

    let rendered_dir = render_dir(frame, &app.main_dir, 0);
    draw_dir(frame, rendered_dir, app);
}
