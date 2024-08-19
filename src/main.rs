use std::{error::Error, io, path::PathBuf};

use app::App;
use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use player::Player;
use ratatui::{prelude::CrosstermBackend, Terminal};
use rodio::OutputStream;

mod player;
mod app;


fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let player = Player::new(stream_handle);

    // let file = match File::open("/home/verifiedfemboy/Downloads/13 Hollowheart (feat. Amy Milan).wav") {
    //     Ok(f) => f,
    //     Err(e) => {
    //         eprintln!("Error opening file: {}", e);
    //         return;
    //     }
    // };

    // player.add(file);
    // player.play();
    // player.handle();

    let directory = PathBuf::from("/home/verifiedfemboy/Music");

    let mut app = App::new(player, directory);

    let res = app.run(&mut terminal);

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
    
        if let Err(err) = res {
            println!("{err:?}");
        }

        Ok(())
}
