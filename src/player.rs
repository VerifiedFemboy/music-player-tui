use std::{fs::File, io::{self, BufReader, Write}};
use rodio::{Decoder, OutputStreamHandle, Sink};

pub(crate) struct Player {
    current_playing: String,
    paused: bool,
    sink: Sink,
}

impl Player {
    pub fn new(stream_handle: OutputStreamHandle ) -> Self {
        Self { 
            current_playing: "".to_string(), 
            paused: true, 
            sink: Sink::try_new(&stream_handle).unwrap() 
        }
    }

    pub fn add(&mut self, file: File) {
        let source = Decoder::new(BufReader::new(file)).unwrap();
        self.sink.append(source);
        self.current_playing = "Your Track Name".to_string(); // Możesz dodać logikę do ustalania nazwy utworu
        self.paused = true;
    }

    pub fn play(&mut self) {
        if self.paused {
            self.sink.play();
            self.paused = false;
            println!("Resuming playback.");
        } else {
            println!("Already playing.");
        }
    }

    pub fn pause(&mut self) {
        if !self.paused {
            self.sink.pause(); // Użyj `pause`, aby wstrzymać bez zatrzymywania odtwarzania
            self.paused = true;
            println!("Paused.");
        } else {
            println!("Already paused.");
        }
    }

    pub fn handle(&mut self) {
        loop {
            let mut command = String::new();
            print!("Enter command (play/pause/quit): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut command).unwrap();

            match command.trim() {
                "play" => self.play(),
                "pause" => self.pause(),
                "quit" => {
                    self.sink.stop(); // Opcjonalnie zatrzymaj odtwarzanie przy wyjściu
                    break;
                }
                _ => println!("Unknown command!"),
            }
        }
    }
}