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

pub fn render_dir(frame: &mut Frame<'_>, dir: &mut Dir) {
    let mut text = String::new();

    for sub_dir in dir.sub_dirs.as_slice() {
        if sub_dir.name.starts_with('.') {
            continue
        }

        text += &(sub_dir.name.as_str().to_owned() + "\n");
    }

    frame.render_widget(
        Paragraph::new(text),
        Rect::new(0, 1, frame.area().width, frame.area().height - 1),
    );
}

pub fn ui(frame: &mut Frame<'_>, app: &mut App) {
    status_bar(frame, app);

    render_dir(frame, &mut app.main_dir);
}
