use std::{io::BufReader, fs::File};

use rodio::{Decoder, OutputStream, source::{Source, Buffered}, OutputStreamHandle};

pub struct ClickPlayer {
    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    source_buffered: Buffered<Decoder<BufReader<File>>>,
}  

impl ClickPlayer {
    pub fn new() -> ClickPlayer {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // self.stream_handle.play_raw(self.source.by_ref().convert_samples());
        let file = BufReader::new(File::open("mixkit-quick-golf-hit-2121.wav").unwrap());
        let source = Decoder::new(file).unwrap();
        let source_buffered = source.buffered();

        ClickPlayer{
            stream_handle,
            _stream,
            source_buffered,
        }
    }

    pub fn play_once(&self) {
        self.stream_handle.play_raw(self.source_buffered.clone().convert_samples()).unwrap();
    }

    pub fn play_nth(&self, times: i32) {
        self.stream_handle.play_raw(self.source_buffered.clone().speed(times as f32).convert_samples()).unwrap();
    }
}