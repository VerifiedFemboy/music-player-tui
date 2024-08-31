use std::{fs, io, path::PathBuf};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{layout::{Constraint, Layout}, prelude::Backend, style::{Color, Style}, text::{Line, Span}, widgets::{Block, List, ListItem, Paragraph}, Frame, Terminal};

use crate::player::Player;

pub struct App {
    player: Player,
    directory: PathBuf,  // Przechowuje ścieżkę do katalogu
    selected_file: Option<PathBuf>,
    index_select: usize,
    input: InputMode,
    search_input: String,
}

impl App {

    pub fn new(player: Player, directory: PathBuf) -> Self {
        Self {
            player,
            directory,
            selected_file: None,
            index_select: 0,
            input: InputMode::Normal,
            search_input: String::new(),
        }
    }


    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|frame| ui(frame, &self))?;
            
            if let Event::Key(key) = event::read()? {
                match self.input {
                    InputMode::Search => match key.code {
                        KeyCode::Esc | KeyCode::Tab => self.change_input(InputMode::Normal),
                        _ => {},
                    },
                    InputMode::Select => match key.code {
                        KeyCode::Esc => self.change_input(InputMode::Normal),
                        KeyCode::Up => self.select_up(),
                        KeyCode::Down => self.select_down(),
                        KeyCode::Enter => self.play_the_selected_file(),
                        _ => {},
                    },
                    InputMode::Playback => match key.code {
                        KeyCode::Esc => self.change_input(InputMode::Normal),
                        _ => {},
                    },
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('s') => self.change_input(InputMode::Select),
                        KeyCode::Char(' ') => self.change_input(InputMode::Playback),
                        KeyCode::Tab => self.change_input(InputMode::Search),
                        _ => {},
                    },
                }
            }
        }
        Ok(())
    }

    fn change_input(&mut self, input: InputMode) {
        self.input = input;
    }

    fn select_up(&mut self) {
        if self.index_select > 0 {
            self.index_select -= 1;
        }
    }

    fn select_down(&mut self) {
        let entries_max = match fs::read_dir(&self.directory) {
            Ok(entries) => entries.count().saturating_sub(1),
            Err(_) => 0,
        };

        if self.index_select < entries_max {
            self.index_select += 1;
        }
    }

    fn play_the_selected_file(&mut self) {
        if let Ok(mut entries) = fs::read_dir(&self.directory) {
            if let Some(entry) = entries.nth(self.index_select) {
                if let Ok(entry) = entry {
                    self.selected_file = Some(entry.path());
                    
                    if let Some(ref path) = self.selected_file {
                        self.player.change(path);
                        self.player.play();
                    }
                }
            }
        }
    }
    

}

fn ui(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([
        Constraint::Length(3), Constraint::Length(10), Constraint::Min(1), Constraint::Max(1)
    ]);

    // Search Area
    let [search_area, select_area, playback_area, help_info_area] = vertical.areas(frame.area());

    let search = Paragraph::new(app.search_input.as_str())
        .style(match app.input {
            InputMode::Search => Style::default().fg(Color::LightMagenta),
            _ => Style::default(),
        })
        .block(Block::bordered().title("Search"));

    frame.render_widget(search, search_area);

    //Files Area
    let files: Vec<ListItem> = match fs::read_dir(&app.directory) {
        Ok(entries) => entries
            .enumerate()
            .map(|(i, entry)| {
                let entry = entry.unwrap();
                let filename = entry.file_name().to_string_lossy().into_owned();
                let content = Line::from(Span::raw(format!("{}", filename)));
            
                let mut item = ListItem::new(content);

                if i == app.index_select {
                    let style = Style::default().fg(Color::LightCyan);
                    item = item.style(style);
                }

                item
            })
            .collect(),
        Err(_) => vec![ListItem::new("Brak plików lub błąd dostępu")],
    };

    let files = List::new(files).block(Block::bordered()
    .style(match app.input {
        InputMode::Select => Style::default().fg(Color::LightMagenta),
        _ => Style::default(),
    })
    .title("Files"));
    frame.render_widget(files, select_area);


    //Playback Area
    let playback_info = match &app.selected_file {
        Some(file) => format!("Playing: {}", file.file_name().unwrap().to_string_lossy()),
        None => "Brak pliku do odtwarzania".to_string(),
    };
    
    let playback = Paragraph::new(playback_info)
        .style(Style::default().fg(Color::LightCyan))
        .block(Block::bordered().title("Playback"));

    frame.render_widget(playback, playback_area);

}

enum InputMode {
    Search,
    Select,
    Playback,
    Normal,
}
