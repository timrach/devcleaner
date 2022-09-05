use crate::{components::StatefulList, move_to_bin, scan_dir};
use std::path::Path;

#[derive(PartialEq, Eq)]
pub enum Screen {
    Overview,
    Confirmation,
}

pub struct App {
    pub path: String,
    pub should_quit: bool,
    pub scan_results: StatefulList<String>,
    pub current_screen: Screen,
}

impl App {
    pub fn new(path: &Path) -> App {
        let p = path.to_str().unwrap().to_owned();
        let scan = scan_dir(path);
        App {
            path: p,
            should_quit: false,
            current_screen: Screen::Overview,
            scan_results: StatefulList::with_items(scan),
        }
    }

    fn move_results_to_bin(&mut self) {
        let mut removed: Vec<String> = vec![];
        for scan in self.scan_results.marked_items.iter() {
            if let Some(entry) = &self.scan_results.items.get(*scan) {
                let path = Path::new(entry);
                match move_to_bin(path) {
                    Ok(_) => {
                        removed.push(entry.to_string());
                    }
                    Err(e) => println!("{}", e),
                };
            }
        }

        for entry in removed {
            if let Some(idx) = self.scan_results.items.iter().position(|x| x == &entry) {
                self.scan_results.items.remove(idx);
            }
        }

        self.scan_results.marked_items.clear();
    }

    pub fn on_arrow_up(&mut self) {
        self.scan_results.shift_selection(-1);
    }

    pub fn on_arrow_down(&mut self) {
        self.scan_results.shift_selection(1);
    }

    pub fn on_esc(&mut self) {
        if self.current_screen == Screen::Confirmation {
            self.current_screen = Screen::Overview
        }
    }

    pub fn on_enter(&mut self) {
        if self.current_screen == Screen::Confirmation {
            self.move_results_to_bin();
            self.current_screen = Screen::Overview;
        }
    }

    pub fn on_key(&mut self, c: char) {
        if self.current_screen == Screen::Overview {
            match c {
                'q' => self.should_quit = true,
                'd' => self.current_screen = Screen::Confirmation,
                'x' => self.scan_results.toggle_current(),
                'a' => self.scan_results.toggle_all(),
                'n' => self.scan_results.shift_selection(-10),
                'm' => self.scan_results.shift_selection(10),
                x => println!("{}", x),
            }
        }
    }
}
