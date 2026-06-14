use crate::scanner::{CleanableItem, Scanner};
use crate::scanners::{
    cache::CacheScanner, flatpak::FlatpakScanner, journal::JournalScanner,
    pacman::PacmanScanner, snap::SnapScanner,
};
use crate::sudo::run_with_sudo;

pub enum ModalState {
    None,
    SudoPrompt { input: String },
    Cleaning,
    Error { message: String },
    Success { freed_bytes: u64 },
}

pub enum Action {
    None,
    ExecuteClean(String),
}

pub struct App {
    pub items: Vec<(CleanableItem, bool)>, // (item, selected)
    pub selected_index: usize,
    pub scanners: Vec<Box<dyn Scanner>>,
    pub modal: ModalState,
}

impl App {
    pub fn new() -> App {
        App {
            items: Vec::new(),
            selected_index: 0,
            scanners: vec![
                Box::new(PacmanScanner),
                Box::new(JournalScanner),
                Box::new(CacheScanner),
                Box::new(FlatpakScanner),
                Box::new(SnapScanner),
            ],
            modal: ModalState::None,
        }
    }

    pub async fn load(&mut self) {
        let mut loaded = Vec::new();
        for scanner in &self.scanners {
            if let Ok(res) = scanner.scan().await {
                for item in res {
                    loaded.push((item, true));
                }
            }
        }
        self.items = loaded;
    }

    pub fn next(&mut self) {
        match &self.modal {
            ModalState::None => {
                if !self.items.is_empty() {
                    self.selected_index = (self.selected_index + 1) % self.items.len();
                }
            }
            _ => {}
        }
    }

    pub fn previous(&mut self) {
        match &self.modal {
            ModalState::None => {
                if !self.items.is_empty() {
                    if self.selected_index == 0 {
                        self.selected_index = self.items.len() - 1;
                    } else {
                        self.selected_index -= 1;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn toggle_selection(&mut self) {
        match &self.modal {
            ModalState::None => {
                if !self.items.is_empty() {
                    self.items[self.selected_index].1 = !self.items[self.selected_index].1;
                }
            }
            _ => {}
        }
    }

    pub fn handle_input(&mut self, c: char) {
        if let ModalState::SudoPrompt { input } = &mut self.modal {
            input.push(c);
        }
    }

    pub fn handle_backspace(&mut self) {
        if let ModalState::SudoPrompt { input } = &mut self.modal {
            input.pop();
        }
    }

    pub fn handle_enter(&mut self) -> Action {
        match &self.modal {
            ModalState::None => {
                self.toggle_selection();
                Action::None
            }
            ModalState::SudoPrompt { input } => {
                let pwd = input.clone();
                self.modal = ModalState::Cleaning;
                Action::ExecuteClean(pwd)
            }
            ModalState::Success { .. } | ModalState::Error { .. } => {
                self.modal = ModalState::None;
                Action::None
            }
            _ => Action::None
        }
    }

    pub async fn clean(&mut self) {
        // Just trigger the sudo prompt for now
        self.modal = ModalState::SudoPrompt { input: String::new() };
    }
}
