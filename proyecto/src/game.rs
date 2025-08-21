// src/game.rs
use raylib::prelude::*;
use crate::player::Player;
use crate::raycast::RayCaster;
use crate::map::Map;
use crate::minimap::MiniMap;
use crate::sprite::SpriteManager;
use crate::audio::AudioManager;
use crate::menu::Menu;

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Won,
}

pub struct Game {
    pub state: GameState,
    pub current_level: usize,
    pub player: Player,
    pub raycast: RayCaster,
    pub map: Map,
    pub minimap: MiniMap,
    pub sprites: SpriteManager,
    pub audio: AudioManager,
    pub menu: Menu,
    pub fps_counter: f32,
    pub frame_time: f32,
}

impl Game {
    pub fn new(rl: &mut RaylibHandle) -> Self {
        let map = Map::new(0);
        let player = Player::new(1.5, 1.5, 0.0);
        let raycast = RayCaster::new();
        let minimap = MiniMap::new();
        let sprites = SpriteManager::new(&map);
        let audio = AudioManager::new(rl);
        let menu = Menu::new();

        Self {
            state: GameState::Menu,
            current_level: 0,
            player,
            raycast,
            map,
            minimap,
            sprites,
            audio,
            menu,
            fps_counter: 0.0,
            frame_time: 0.0,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        self.frame_time = rl.get_frame_time();
        self.fps_counter = 1.0 / self.frame_time;

        // Update audio (does nothing in current implementation)
        self.audio.update();

        match self.state {
            GameState::Menu => {
                if let Some(level) = self.menu.update(rl) {
                    self.load_level(level);
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => {
                self.player.update(rl, &self.map);
                self.sprites.update_animation(self.frame_time);
                
                // Check coin collection
                if let Some(_coin_pos) = self.sprites.check_collision(&self.player) {
                    self.audio.play_coin_sound();
                    if self.sprites.all_coins_collected() {
                        self.state = GameState::Won;
                    }
                }

                // ESC to menu
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    self.state = GameState::Menu;
                }
            }
            GameState::Won => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    self.state = GameState::Menu;
                } else if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    // Restart level
                    self.load_level(self.current_level);
                    self.state = GameState::Playing;
                }
            }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        match self.state {
            GameState::Menu => {
                self.menu.draw(d);
            }
            GameState::Playing => {
                // Draw 3D view
                self.raycast.render(d, &self.player, &self.map, &self.sprites);
                
                // Draw minimap in top-right corner
                self.minimap.draw(d, &self.player, &self.map, &self.sprites);
                
                // Draw FPS counter
                d.draw_text(
                    &format!("FPS: {:.1}", self.fps_counter),
                    10,
                    10,
                    20,
                    Color::WHITE,
                );

                // Draw coins collected
                let collected = self.sprites.coins_collected();
                let total = self.sprites.total_coins();
                d.draw_text(
                    &format!("Coins: {}/{}", collected, total),
                    10,
                    40,
                    20,
                    Color::YELLOW,
                );
            }
            GameState::Won => {
                // Draw the game state first
                self.raycast.render(d, &self.player, &self.map, &self.sprites);
                self.minimap.draw(d, &self.player, &self.map, &self.sprites);

                // Draw win screen overlay
                d.draw_rectangle(0, 0, 1024, 768, Color::new(0, 0, 0, 180));
                
                let win_text = "LEVEL COMPLETE!";
                let text_width = measure_text(win_text, 60);
                d.draw_text(
                    win_text,
                    512 - text_width / 2,
                    300,
                    60,
                    Color::GOLD,
                );

                let continue_text = "Press ENTER for menu or SPACE to restart";
                let continue_width = measure_text(continue_text, 20);
                d.draw_text(
                    continue_text,
                    512 - continue_width / 2,
                    400,
                    20,
                    Color::WHITE,
                );
            }
        }
    }

    fn load_level(&mut self, level: usize) {
        self.current_level = level;
        self.map = Map::new(level);
        self.player = Player::new(1.5, 1.5, 0.0);
        self.sprites = SpriteManager::new(&self.map);
    }
}

// Helper function for text measurement
fn measure_text(text: &str, font_size: i32) -> i32 {
    // Approximate text width calculation
    text.len() as i32 * font_size / 2
}
