use std::io::Cursor;

use rodio::{
    source::{Buffered, Source},
    Decoder, OutputStream, OutputStreamHandle,
};

pub struct ClickPlayer {
    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    source_buffered: Buffered<Decoder<Cursor<&'static [u8]>>>,
}

impl ClickPlayer {
    pub fn new() -> ClickPlayer {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // self.stream_handle.play_raw(self.source.by_ref().convert_samples());

        let sound_data = include_bytes!("../269718__michorvath__ping-pong-ball-hit.wav");

        let cursor = Cursor::new(&sound_data[..]);

        let source = Decoder::new(cursor).unwrap();
        let source_buffered = source.buffered();

        ClickPlayer {
            stream_handle,
            _stream,
            source_buffered,
        }
    }

    pub fn play_once(&self) {
        self.stream_handle
            .play_raw(self.source_buffered.clone().convert_samples())
            .unwrap();
    }

    pub fn play_nth(&self, times: i32) {
        self.stream_handle
            .play_raw(
                self.source_buffered
                    .clone()
                    .speed(times as f32)
                    .convert_samples(),
            )
            .unwrap();
    }
}
