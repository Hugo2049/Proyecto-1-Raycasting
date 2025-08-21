// src/audio.rs
pub struct AudioManager;

impl AudioManager {
    pub fn new(_rl: &mut raylib::prelude::RaylibHandle) -> Self {
        Self
    }

    pub fn update(&mut self) {
        // Do nothing - audio disabled
    }

    pub fn play_coin_sound(&self) {
        // Do nothing - audio disabled
        println!("Coin collected! (Audio disabled)");
    }

    pub fn set_music_volume(&self, _volume: f32) {
        // Do nothing - audio disabled
    }
}
