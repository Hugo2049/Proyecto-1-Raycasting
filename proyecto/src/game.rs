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
}

impl Game {
    pub fn new(rl: &mut RaylibHandle) -> Self {
        let level = 0;
        let map = Map::new(level);
        
        
        let sprites = SpriteManager::new(&map);
        
        Self {
            player: Player::new(1.5, 1.5, 0.0),
            map: map, 
            sprites: sprites,
            raycaster: RayCaster::new(),
            minimap: MiniMap::new(),
            audio: AudioManager::new(rl),
            menu: Menu::new(),
            current_level: level,
            in_menu: true,
            game_won: false,
        }
    }

    pub fn load_textures(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.raycaster.load_textures(rl, thread);
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if self.in_menu {
            if let Some(level) = self.menu.update(rl) {
                self.start_level(level);
            }
        } else {
            
            self.player.update(rl, &self.map);
            
            if let Some((x, y)) = self.sprites.check_collision(&self.player) {
                self.audio.play_coin_sound();
                println!("Coin collected at ({:.1}, {:.1})!", x, y);
            }
            
            if self.sprites.all_coins_collected() {
                self.game_won = true;
                println!("All coins collected! Level completed!");
            }
            
         
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
            
            self.sprites.update_animation(rl.get_frame_time());
            self.audio.update();
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        if self.in_menu {
            self.menu.draw(d);
        } else {
            
            self.raycaster.render(d, &self.player, &self.map, &self.sprites);
            
            
            self.minimap.draw(d, &self.player, &self.map, &self.sprites);
            
            
            d.draw_text(&format!("Coins: {}/{}", self.sprites.coins_collected(), self.sprites.total_coins()), 10, 10, 20, Color::WHITE);
            d.draw_text(&format!("Level: {}", self.current_level + 1), 10, 40, 20, Color::WHITE);
            d.draw_text("ESC: Menu  R: Restart", 10, 70, 20, Color::WHITE);
            
       
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
        
        
        match level {
            0 => self.player = Player::new(1.5, 1.5, 0.0),
            1 => self.player = Player::new(1.5, 1.5, 0.0),
            2 => self.player = Player::new(1.5, 1.5, 0.0),
            _ => self.player = Player::new(1.5, 1.5, 0.0),
        }
        
        self.game_won = false;
        self.in_menu = false;
        self.audio.start_background_music();
        
        println!("Starting level {}", level + 1);
    }

    fn restart_level(&mut self) {
        self.start_level(self.current_level);
    }
}
