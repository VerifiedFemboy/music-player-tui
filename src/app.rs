use std::{fs::File, io};

use crossterm::event::{self, Event};
use ratatui::{prelude::Backend, widgets::Widget, Frame, Terminal};

use crate::player::Player;

struct App {
    player: Player,
    files: Vec<File>,
    input: InputMode
}

impl App {

    const fn new(player: Player) -> Self {
        Self {
            player,
            files: Vec::new(),
            input: InputMode::Playback,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.render_widget(frame))?;
            
            if let Event::Key(key) = event::read()? {
                match self.input {
                    InputMode::Search => {
                        
                    },
                    InputMode::Select => {

                    },
                    InputMode::Playback => {
                        
                    },
                }
            }
        }
    }
    
    fn render_widget(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer){

    }

}

enum InputMode {
    Search,
    Select,
    Playback
}