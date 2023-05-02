use std::{io::BufReader, fs::File};

use rodio::{Decoder, OutputStream, source::Source, OutputStreamHandle, Sink};

pub struct ClickPlayer {
    stream_handle: OutputStreamHandle,
}  

impl ClickPlayer {
    pub fn new() -> ClickPlayer {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        ClickPlayer{
            stream_handle,
        }
    }

    pub fn play_once(&self) {
        // self.stream_handle.play_raw(self.source.by_ref().convert_samples());
        let file = BufReader::new(File::open("mixkit-quick-golf-hit-2121.wav").unwrap());
        let source = Decoder::new(file).unwrap();
        self.stream_handle.play_raw(source.convert_samples());
    }
}