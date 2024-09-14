use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use ratatui::prelude::Rect;
use std::fs;
use std::io::Error;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone)]
pub struct Dir {
    pub id: String,
    pub full_path: String,
    pub name: String,
    pub sub_dirs: Vec<Dir>,
    pub sub_files: Vec<(String, String)>,
    pub expanded: bool,
}

pub struct App {
    pub should_quit: bool,
    pub main_dir: Dir,
    pub scroll: usize,
    pub selectable_items: Vec<(String, PathBuf)>,
    pub selected_index: usize,
    pub selected_item_id: Option<String>,
    pub selected_item: (Option<Dir>, Option<PathBuf>),
    pub all_dirs: Vec<Dir>,
    pub all_files: Vec<(String, String)>,
    pub terminal_area: Option<Rect>,
}

impl App {
    pub fn new() -> App {
        App {
            should_quit: false,
            main_dir: Dir {
                id: Uuid::new_v4().to_string(),
                name: String::new(),
                full_path: String::new(),
                sub_dirs: Vec::new(),
                sub_files: Vec::new(),
                expanded: false,
            },
            scroll: 0,
            selectable_items: Vec::new(),
            selected_index: 0,
            selected_item_id: Option::None,
            selected_item: (Option::None, Option::None),
            all_dirs: Vec::new(),
            all_files: Vec::new(),
            terminal_area: None,
        }
    }

    fn scroll(&mut self, delta: usize, neg: bool) {
        if neg {
            self.scroll -= if self.scroll > 0 { delta } else { 0 };
        } else {
            self.scroll += if self.scroll < self.selectable_items.len() - 1 {
                delta
            } else {
                0
            };
        }

        if self.selected_index < self.scroll {
            self.selected_index += 1
        } else if self.selected_index == self.terminal_area.unwrap().height as usize - 2 + self.scroll {
            self.selected_index -= 1
        }
    }

    fn select_next(&mut self) {
        if self.selected_index == self.selectable_items.len() - 1 {
            return;
        }

        self.select(self.selected_index + 1);
    }

    fn select_previous(&mut self) {
        if self.selected_index == 0 {
            return;
        }

        self.select(self.selected_index - 1);
    }

    fn select(&mut self, index: usize) {
        self.selected_index = index;

        if self.terminal_area.is_some() {
            if self.selected_index < self.scroll && self.scroll != 0 {
                self.scroll -= 1
            } else if self.selected_index == self.terminal_area.unwrap().height as usize - 2 + self.scroll {
                self.scroll += 1
            }
        }

        if self.selectable_items.get(self.selected_index).is_none() {
            return;
        }

        self.selected_item_id = Some(
            self.selectable_items
                .get(self.selected_index)
                .unwrap()
                .0
                .clone(),
        );

        let path_buf = self
            .selectable_items
            .clone()
            .into_iter()
            .find(|x| x.0 == self.selected_item_id.clone().unwrap_or_default())
            .unwrap_or((String::new(), PathBuf::new()));

        let found_dir = self
            .all_dirs
            .iter()
            .map(|x| x.clone())
            .into_iter()
            .find(|x| x.id == path_buf.0);

        let found_file = self
            .all_files
            .iter()
            .map(|x| x.clone())
            .into_iter()
            .find(|x| x.0 == path_buf.0);

        if path_buf.1.is_dir() {
            self.selected_item = (found_dir.clone(), Option::None);
        } else if found_file.is_some() {
            self.selected_item = (Option::None, Some(path_buf.1));
        }
    }

    fn toggle_expand(&mut self) {
        self.selected_item.clone().0.unwrap().expanded = true;
    }

    fn open(&mut self, dir: PathBuf) {
        let _ = self.nav(dir);
    }

    fn open_selected(&mut self) {
        if self.selected_item.0.is_some() {
            self.open(PathBuf::from(self.selected_item.0.clone().unwrap().full_path));
        }
    }

    pub fn handle_events(&mut self) -> Result<(), Error> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.should_quit = true,
                    KeyCode::Char('l') => self.scroll(1, false),
                    KeyCode::Char('h') => self.scroll(1, true),
                    KeyCode::Char('j') => self.select_next(),
                    KeyCode::Char('k') => self.select_previous(),
                    KeyCode::Char('s') => self.toggle_expand(),
                    KeyCode::Char('o') => self.open_selected(),
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn read_dir(&mut self, dir_path: PathBuf, is_root_dir: bool) -> Result<Dir, Error> {
        let mut sub_dirs: Vec<Dir> = Vec::new();
        let mut sub_files: Vec<(String, String)> = Vec::new();

        let paths = fs::read_dir(dir_path.clone()).unwrap();

        let mut result_dir = Dir {
            id: Uuid::new_v4().to_string(),
            full_path: dir_path.to_str().unwrap_or_default().to_string(),
            name: dir_path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap()
                .to_string(),
            sub_dirs: Vec::new(),
            sub_files: Vec::new(),
            expanded: true,
        };

        if !is_root_dir {
            self.selectable_items
                .push((result_dir.id.clone(), dir_path));
        }

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
                        let read_dir = self.read_dir(path_buf.clone(), false).unwrap();


                        self.selectable_items.extend(
                            read_dir
                                .sub_files
                                .clone()
                                .into_iter()
                                .map(|x| (x.0.clone(), PathBuf::from(x.1.clone()))),
                        );


                        sub_dirs.push(read_dir);
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
                    let sub_file = (
                        Uuid::new_v4().to_string(),
                        path_buf
                            .file_name()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                            .to_string(),
                    );

                    sub_files.push(sub_file);
                }
            }
        }

        result_dir.sub_dirs = sub_dirs.clone();
        result_dir.sub_files = sub_files.clone();

        self.all_dirs.extend(sub_dirs.clone());
        self.all_files.extend(sub_files.clone());

        Ok(result_dir)
    }

    pub fn nav(&mut self, path: PathBuf) -> Result<(), Error> {
        self.selectable_items = Vec::new();
        self.selected_item = (None, None);
        self.selected_item_id = None;
        self.scroll = 0;

        self.main_dir = self.read_dir(path.clone(), true).unwrap();

        self.selectable_items.extend(self.main_dir.sub_files.clone().into_iter().map(|x| (x.0, PathBuf::from(x.1))));

        self.select(0);

        Ok(())
    }
}
