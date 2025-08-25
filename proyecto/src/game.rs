use raylib::prelude::*;
use crate::player::Player;
use crate::map::Map;
use crate::sprite::SpriteManager;
use crate::raycast::RayCaster;
use crate::minimap::MiniMap;
use crate::audio::AudioManager;
use crate::menu::Menu;

const SCREEN_WIDTH: i32 = 1024;

pub struct Game {
    pub player: Player,
    pub map: Map,
    pub sprites: SpriteManager,
    pub raycaster: RayCaster,
    pub minimap: MiniMap,
    pub audio: AudioManager,
    pub menu: Menu,
    pub current_level: usize,
    pub in_menu: bool,
    pub game_won: bool,
    pub music_started: bool,
}

impl Game {
    pub fn new(_rl: &mut RaylibHandle, _thread: &RaylibThread) -> Self {
        let level = 0;
        let map = Map::new(level);
        let sprites = SpriteManager::new(&map);
        let mut audio = AudioManager::new();

        Self {
            player: Player::new(1.5, 1.5, 0.0),
            map,
            sprites,
            raycaster: RayCaster::new(),
            minimap: MiniMap::new(),
            audio,
            menu: Menu::new(),
            current_level: level,
            in_menu: true,
            game_won: false,
            music_started: false,
        }
    }

    pub fn load_textures(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.raycaster.load_textures(rl, thread);
        self.audio.load_sounds(thread);
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        // Start music when first entering the menu (delayed start)
        if !self.music_started {
            self.audio.start_background_music();
            self.music_started = true;
        }

        if self.in_menu {
            if let Some(level) = self.menu.update(rl) {
                self.start_level(level);
            }
            
            // Music volume control in menu
            if rl.is_key_pressed(KeyboardKey::KEY_MINUS) {
                self.adjust_music_volume(-0.1);
            }
            if rl.is_key_pressed(KeyboardKey::KEY_EQUAL) {
                self.adjust_music_volume(0.1);
            }
            if rl.is_key_pressed(KeyboardKey::KEY_M) {
                self.toggle_music();
            }
        } else {
            self.player.update(rl, &self.map);
            
            // Check for coin collection (no sound)
            if let Some((x, y)) = self.sprites.check_collision(&self.player) {
                println!("Coin collected at ({:.1}, {:.1})!", x, y);
            }
            
            if self.sprites.all_coins_collected() {
                self.game_won = true;
                println!("All coins collected! Level completed!");
            }
            
            // Game controls
            if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                self.in_menu = true;
                self.game_won = false;
            }
            
            if self.game_won && rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                self.in_menu = true;
                self.game_won = false;
            }
            
            if rl.is_key_pressed(KeyboardKey::KEY_R) {
                self.restart_level();
            }
            
            // Music controls in game
            if rl.is_key_pressed(KeyboardKey::KEY_MINUS) {
                self.adjust_music_volume(-0.1);
            }
            if rl.is_key_pressed(KeyboardKey::KEY_EQUAL) {
                self.adjust_music_volume(0.1);
            }
            if rl.is_key_pressed(KeyboardKey::KEY_M) {
                self.toggle_music();
            }
            
            self.sprites.update_animation(rl.get_frame_time());
        }
        
        self.audio.update();
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        if self.in_menu {
            self.menu.draw(d);
            
            // Draw music controls
            d.draw_text("Music Controls:", 10, 650, 16, Color::WHITE);
            d.draw_text("M: Toggle Music", 10, 670, 14, Color::LIGHTGRAY);
            d.draw_text("-/+: Volume", 10, 690, 14, Color::LIGHTGRAY);
            
            let status = if self.audio.is_music_playing() { "Playing" } else { "Stopped" };
            d.draw_text(&format!("Music: {}", status), 10, 710, 14, Color::LIGHTGRAY);
        } else {
            self.raycaster.render(d, &self.player, &self.map, &self.sprites);
            self.minimap.draw(d, &self.player, &self.map, &self.sprites);
            
            // Game UI
            d.draw_text(&format!("Coins: {}/{}", self.sprites.coins_collected(), self.sprites.total_coins()), 10, 10, 20, Color::WHITE);
            d.draw_text(&format!("Level: {}", self.current_level + 1), 10, 40, 20, Color::WHITE);
            d.draw_text("ESC: Menu  R: Restart  M: Music", 10, 70, 20, Color::WHITE);
            d.draw_text(&format!("FPS: {}", d.get_fps()), SCREEN_WIDTH - 120, 10, 20, Color::WHITE);
            
            if self.game_won {
                d.draw_text("LEVEL COMPLETED!", 350, 300, 40, Color::GOLD);
                d.draw_text("Press ENTER to return to menu", 350, 350, 20, Color::WHITE);
            }
        }
    }

    fn start_level(&mut self, level: usize) {
        self.current_level = level;
        self.map = Map::new(level);
        self.sprites = SpriteManager::new(&self.map);
        
        // Set player starting position based on level
        match level {
            0 => self.player = Player::new(1.5, 1.5, 0.0),
            1 => self.player = Player::new(1.5, 1.5, 0.0),
            2 => self.player = Player::new(1.5, 1.5, 0.0),
            _ => self.player = Player::new(1.5, 1.5, 0.0),
        }
        
        self.game_won = false;
        self.in_menu = false;
        
        println!("Starting level {}", level + 1);
    }

    fn restart_level(&mut self) {
        self.start_level(self.current_level);
    }
    
    fn adjust_music_volume(&mut self, delta: f32) {
        // This is a simplified volume control - you might want to store the current volume
        // For now, we'll just print what we're trying to do
        if delta > 0.0 {
            println!("Increasing music volume");
        } else {
            println!("Decreasing music volume");
        }
        // The actual volume adjustment would need to be implemented based on your audio system
    }
    
    fn toggle_music(&mut self) {
        if self.audio.is_music_playing() {
            self.audio.pause_music();
        } else {
            self.audio.resume_music();
        }
    }
}
