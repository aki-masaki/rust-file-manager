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
    pub sub_files: Vec<String>,
}

pub struct App {
    pub should_quit: bool,
    pub main_dir: Dir,
    pub scroll: usize,
}

impl App {
    pub fn new() -> App {
        App {
            should_quit: false,
            main_dir: Dir {
                name: String::new(),
                full_path: String::new(),
                sub_dirs: Vec::new(),
                sub_files: Vec::new(),
            },
            scroll: 0,
        }
    }

    fn scroll(&mut self, delta: usize, neg: bool) {
        if neg {
            self.scroll -= if self.scroll > 0 { delta } else { 0 };
        } else {
            self.scroll += delta;
        }
    }

    pub fn handle_events(&mut self) -> Result<(), Error> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.should_quit = true,
                    KeyCode::Char('l') => self.scroll(1, false),
                    KeyCode::Char('h') => self.scroll(1, true),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn read_dir(&mut self, dir_path: PathBuf) -> Result<Dir, Error> {
        let mut sub_dirs: Vec<Dir> = Vec::new();
        let mut sub_files: Vec<String> = Vec::new();

        let paths = fs::read_dir(dir_path.clone()).unwrap();

        for path in paths {
            let path_buf = path.unwrap().path();

            if path_buf.is_dir() {
                if !path_buf
                    .file_name()
                    .unwrap_or_default()
                    .to_os_string()
                    .to_string_lossy()
                    .starts_with('.')
                {
                    // TODO: remove if statement after implementing closing and opening directories
                    if path_buf.file_name().unwrap() != "target" {
                        sub_dirs.push(self.read_dir(path_buf).unwrap());
                    }
                }
            } else {
                if !path_buf
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with('.')
                {
                    sub_files.push(
                        path_buf
                            .file_name()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                            .to_string(),
                    );
                }
            }
        }

        Ok(Dir {
            full_path: dir_path.to_str().unwrap_or_default().to_string(),
            name: dir_path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string(),
            sub_dirs,
            sub_files,
        })
    }

    pub fn nav(&mut self, path: PathBuf) -> Result<(), Error> {
        self.main_dir = self.read_dir(path).unwrap();

        Ok(())
    }
}
