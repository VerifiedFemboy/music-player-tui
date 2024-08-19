use std::fs::File;

use player::Player;
use rodio::OutputStream;

mod player;
mod app;
mod tui;


fn main() {
    let _ = tui::init();
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut player = Player::new(stream_handle);

    let file = match File::open("/home/verifiedfemboy/Downloads/13 Hollowheart (feat. Amy Milan).wav") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    player.add(file);
    player.play();
    player.handle();

    let _ = tui::restore();
}
