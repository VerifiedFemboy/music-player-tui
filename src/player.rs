use std::{fs::File, io::{self, BufReader}, path::PathBuf};
use rodio::{Decoder, OutputStreamHandle, Sink};

pub(crate) struct Player {
    paused: bool,
    sink: Sink,
}

impl Player {
    pub fn new(stream_handle: OutputStreamHandle ) -> Self {
        Self { 
            paused: true, 
            sink: Sink::try_new(&stream_handle).unwrap(),
            
        }
    }

    fn open_file(path: &PathBuf) -> io::Result<File> {
        let file = File::open(path)?;
        Ok(file)
    }

    pub fn change(&mut self, path: &PathBuf) {
        match Self::open_file(path) {
            Ok(file) => {
                let source = Decoder::new(BufReader::new(file)).unwrap();
                self.sink.append(source);
                self.paused = false;
            }
            Err(_) => panic!("Something went wrong while opening file..."),
        }
    }
    

    pub fn play(&mut self) {
        if self.paused {
            self.sink.play();
            self.paused = false;
        }
    }

    pub fn pause(&mut self) {
        if !self.paused {
            self.sink.pause();
            self.paused = true;
            println!("Paused.");
        } else {
            println!("Already paused.");
        }
    }

}