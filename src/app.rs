use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub struct Dir {
    pub full_path: String,
    pub name: String,
    pub sub_dirs: Vec<Dir>,
}

pub struct App {
    pub should_quit: bool,
    pub main_dir: Dir,
}

impl App {
    pub fn new() -> App {
        App {
            should_quit: false,
            main_dir: Dir {
                name: String::new(),
                full_path: String::new(),
                sub_dirs: Vec::new(),
            },
        }
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

    pub fn nav(&mut self, path: PathBuf) -> Result<(), Error> {
        let mut sub_dirs: Vec<Dir> = Vec::new();

        let paths = fs::read_dir(path.clone()).unwrap();

        for path in paths {
            let path_buf = path.unwrap().path();

            if path_buf.is_dir() {
                sub_dirs.push(Dir {
                    full_path: path_buf.to_str().unwrap_or_default().to_string(),
                    name: path_buf
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                        .to_string(),
                    sub_dirs: Vec::new(),
                })
            }
        }

        self.main_dir = Dir {
            full_path: path.to_str().unwrap_or_default().to_string(),
            name: path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string(),
            sub_dirs,
        };

        Ok(())
    }
}
