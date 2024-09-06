use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use std::io::Error;

pub struct App {
    pub should_quit: bool,
}

impl App {
    pub fn new() -> App {
        App { should_quit: false }
    }

    pub fn handle_events(&mut self) -> Result<(), Error> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.should_quit = true,
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
