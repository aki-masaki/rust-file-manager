use crate::app::Dir;
use crossterm::style::Color;
use ratatui::prelude::Rect;
use ratatui::prelude::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use std::ffi::OsString;
use sysinfo::System;

use crate::App;

fn status_bar(frame: &mut Frame<'_>, app: &mut App) {
    let mut text = app.main_dir.full_path.clone() + " ";

    text += users::get_current_username()
        .unwrap_or(OsString::from("user"))
        .to_str()
        .unwrap_or_default();
    text += "@";
    text += System::host_name().unwrap_or_default().as_str();

    frame.render_widget(
        Paragraph::new(text).bg(Color::Green),
        Rect::new(0, 0, frame.area().width, 1),
    );
}

pub fn render_dir(frame: &mut Frame<'_>, dir: &Dir, level: i32) -> String {
    let mut text = String::new();

    for sub_dir in dir.sub_dirs.as_slice() {
        text += "  ".repeat(level as usize).as_str();
        text += &(sub_dir.name.as_str().to_owned() + "\n");

        text += render_dir(frame, sub_dir, level + 1).as_str();
    }

    for sub_file in dir.sub_files.as_slice() {
        text += "  ".repeat(level as usize).as_str();
        text += &(sub_file.to_owned() + "\n");
    }

    text
}

fn draw_dir(frame: &mut Frame<'_>, rendered_dir: String, scroll: usize) {
    let lines: Vec<&str> = rendered_dir.split('\n').into_iter().skip(scroll).collect();

    let text = lines.join("\n");

    frame.render_widget(
        Paragraph::new(text),
        Rect::new(0, 1, frame.area().width, frame.area().height - 1),
    ); 
}

pub fn ui(frame: &mut Frame<'_>, app: &mut App) {
    status_bar(frame, app);

    let rendered_dir = render_dir(frame, &app.main_dir, 0);
    draw_dir(frame, rendered_dir, app.scroll);
}
