use macroquad::audio::*;


pub struct SoundPlayer{
    pub sound: Sound,
    pub volume: f32,
}

impl SoundPlayer{

    pub fn play(&self, how_often: &i32){
        for _ in [0..*how_often]{
            play_sound(self.sound, PlaySoundParams{
                looped: false,
                volume: self.volume,
            });
        }
    }
}