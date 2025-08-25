use raylib::prelude::*;
use crate::player::Player;
use crate::map::Map;
use crate::sprite::SpriteManager;

const MINIMAP_SIZE: i32 = 150;
const MINIMAP_X: i32 = 1024 - MINIMAP_SIZE - 10;
const MINIMAP_Y: i32 = 10;

pub struct MiniMap;

impl MiniMap {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, player: &Player, map: &Map, sprites: &SpriteManager) {
        d.draw_rectangle(MINIMAP_X - 2, MINIMAP_Y - 2, MINIMAP_SIZE + 4, MINIMAP_SIZE + 4, Color::WHITE);
        d.draw_rectangle(MINIMAP_X, MINIMAP_Y, MINIMAP_SIZE, MINIMAP_SIZE, Color::BLACK);

        let cell_size = MINIMAP_SIZE / map.width.max(map.height) as i32;

        for y in 0..map.height {
            for x in 0..map.width {
                let screen_x = MINIMAP_X + x as i32 * cell_size;
                let screen_y = MINIMAP_Y + y as i32 * cell_size;

                let cell_value = map.data[y][x];
                if cell_value > 0 {
                    let color = match cell_value {
                        1 => Color::RED,
                        2 => Color::GREEN,
                        3 => Color::BLUE,
                        4 => Color::YELLOW,
                        5 => Color::PURPLE,
                        _ => Color::GRAY,
                    };
                    d.draw_rectangle(screen_x, screen_y, cell_size, cell_size, color);
                }
            }
        }

        for coin in &sprites.coins {
            if !coin.collected {
                let screen_x = MINIMAP_X + (coin.x * cell_size as f32) as i32;
                let screen_y = MINIMAP_Y + (coin.y * cell_size as f32) as i32;
                d.draw_circle(screen_x, screen_y, 3.0, Color::GOLD);
            }
        }

        let player_screen_x = MINIMAP_X + (player.x * cell_size as f32) as i32;
        let player_screen_y = MINIMAP_Y + (player.y * cell_size as f32) as i32;
        
        d.draw_circle(player_screen_x, player_screen_y, 4.0, Color::WHITE);

        let dir_length = 8.0;
        let end_x = player_screen_x + (player.angle.cos() * dir_length) as i32;
        let end_y = player_screen_y + (player.angle.sin() * dir_length) as i32;
        d.draw_line(player_screen_x, player_screen_y, end_x, end_y, Color::WHITE);
        
        d.draw_text("MAP", MINIMAP_X, MINIMAP_Y + MINIMAP_SIZE + 5, 16, Color::WHITE);
    }
}
