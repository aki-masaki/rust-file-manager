use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::App;

pub fn ui(frame: &mut Frame<'_>, _app: &mut App) {
    frame.render_widget(
        Paragraph::new("Hello World!").block(Block::bordered().title("Greeting")),
        frame.area(),
    );
}
