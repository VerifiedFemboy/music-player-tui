use std::{fs, io, path::PathBuf};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{layout::{Constraint, Layout}, prelude::Backend, style::{Color, Style}, text::{Line, Span}, widgets::{Block, List, ListItem, Paragraph}, Frame, Terminal};

use crate::player::Player;

pub struct App {
    player: Player,
    directory: PathBuf,  // Przechowuje ścieżkę do katalogu
    selected_file: Option<PathBuf>,
    input: InputMode,
    search_input: String,
}

impl App {
    // Inicjalizacja aplikacji
    pub fn new(player: Player, directory: PathBuf) -> Self {
        Self {
            player,
            directory,
            selected_file: None,
            input: InputMode::Normal,
            search_input: String::new(),
        }
    }

    // Uruchomienie aplikacji
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|frame| ui(frame, &self))?;
            
            if let Event::Key(key) = event::read()? {
                match self.input {
                    InputMode::Search => {},
                    InputMode::Select => {},
                    InputMode::Playback => match key.code {
                        _ => {}
                    },
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(':') => {},
                        _ => {},
                    },
                }
            }
        }
        Ok(())
    }
}

// Renderowanie interfejsu użytkownika
fn ui(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([
        Constraint::Length(3), Constraint::Length(10), Constraint::Min(1)
    ]);

    // Search Area
    let [search_area, select_area, playback_area] = vertical.areas(frame.area());

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
                let content = Line::from(Span::raw(format!("{i}: {}", filename)));
                ListItem::new(content)
            })
            .collect(),
        Err(_) => vec![ListItem::new("Brak plików lub błąd dostępu")],
    };

    let files = List::new(files).block(Block::bordered().title("Files"));
    frame.render_widget(files, select_area);


    //Playback Area
    let playback_info = match &app.selected_file {
        Some(file) => format!("Odtwarzanie: {}", file.file_name().unwrap().to_string_lossy()),
        None => "Brak pliku do odtwarzania".to_string(),
    };
    
    let playback = Paragraph::new(playback_info)
        .style(Style::default().fg(Color::LightCyan))
        .block(Block::bordered().title("Playback"));

    frame.render_widget(playback, playback_area);
}

// Tryby wprowadzania
enum InputMode {
    Search,
    Select,
    Playback,
    Normal,
}
