use raylib::prelude::*;
use crate::player::Player;
use crate::map::Map;
use crate::sprite::SpriteManager;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;
const RAY_COUNT: usize = SCREEN_WIDTH as usize;

pub struct RayCaster {
    z_buffer: [f32; RAY_COUNT],
    coin_texture: Option<Texture2D>,
}

impl RayCaster {
    pub fn new() -> Self {
        Self {
            z_buffer: [0.0; RAY_COUNT],
            coin_texture: None,
        }
    }

    pub fn load_textures(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let texture_paths = [
            "assets/sprites/sprite.png",
            "assets/sprites/coin.png",
            "assets/coin.png",
        ];

        for path in &texture_paths {
            match rl.load_texture(thread, path) {
                Ok(texture) => {
                    self.coin_texture = Some(texture);
                    return;
                }
                Err(_) => continue,
            }
        }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle, player: &Player, map: &Map, sprites: &SpriteManager) {
        for i in 0..RAY_COUNT {
            let camera_x = 2.0 * i as f32 / RAY_COUNT as f32 - 1.0;
            let ray_angle = player.angle + camera_x * player.fov / 2.0;
            
            let (hit_distance, wall_type) = self.cast_ray(player, map, ray_angle);
            
            self.z_buffer[i] = hit_distance;
            
            let wall_height = (SCREEN_HEIGHT as f32 / hit_distance).min(SCREEN_HEIGHT as f32);
            let wall_start = ((SCREEN_HEIGHT as f32 - wall_height) / 2.0) as i32;
            let wall_end = wall_start + wall_height as i32;
            
            let wall_color = self.get_wall_color(wall_type, hit_distance);
            
            d.draw_line(i as i32, wall_start, i as i32, wall_end, wall_color);
            
            d.draw_line(i as i32, wall_end, i as i32, SCREEN_HEIGHT, Color::DARKGRAY);
            
            d.draw_line(i as i32, 0, i as i32, wall_start, Color::DARKBLUE);
        }
        
        self.draw_sprites(d, player, sprites);
    }

    fn cast_ray(&self, player: &Player, map: &Map, angle: f32) -> (f32, u8) {
        let dx = angle.cos();
        let dy = angle.sin();
        
        let mut ray_x = player.x;
        let mut ray_y = player.y;
        
        let step = 0.01;
        let mut distance = 0.0;
        
        loop {
            ray_x += dx * step;
            ray_y += dy * step;
            distance += step;
            
            let grid_x = ray_x as usize;
            let grid_y = ray_y as usize;
            
            if grid_x >= map.width || grid_y >= map.height {
                return (distance, 1);
            }
            
            let wall_type = map.get_cell(grid_x, grid_y);
            if wall_type > 0 {
                return (distance, wall_type);
            }
            
            if distance > 20.0 {
                break;
            }
        }
        
        (distance, 1)
    }

    fn get_wall_color(&self, wall_type: u8, distance: f32) -> Color {
        let base_color = match wall_type {
            1 => Color::RED,
            2 => Color::GREEN,
            3 => Color::BLUE,
            4 => Color::YELLOW,
            5 => Color::PURPLE,
            _ => Color::GRAY,
        };
        
        let brightness = (1.0 / (1.0 + distance * 0.1)).min(1.0);
        Color::new(
            (base_color.r as f32 * brightness) as u8,
            (base_color.g as f32 * brightness) as u8,
            (base_color.b as f32 * brightness) as u8,
            255,
        )
    }

    fn draw_sprites(&self, d: &mut RaylibDrawHandle, player: &Player, sprites: &SpriteManager) {
        let mut sprite_distances: Vec<(usize, f32)> = Vec::new();
        
        for (i, coin) in sprites.coins.iter().enumerate() {
            if !coin.collected {
                let dx = coin.x - player.x;
                let dy = coin.y - player.y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < 15.0 {
                    sprite_distances.push((i, distance));
                }
            }
        }
        
        sprite_distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        for (sprite_idx, distance) in sprite_distances {
            let coin = &sprites.coins[sprite_idx];
            if let Some(texture) = &self.coin_texture {
                self.draw_texture_sprite(d, player, coin.x, coin.y, distance, texture, sprites.get_animation_scale());
            } else {
                self.draw_circle_sprite(d, player, coin.x, coin.y, distance, Color::GOLD, sprites.get_animation_scale());
            }
        }
    }

    fn draw_texture_sprite(&self, d: &mut RaylibDrawHandle, player: &Player, sprite_x: f32, sprite_y: f32, distance: f32, texture: &Texture2D, scale: f32) {
        let dx = sprite_x - player.x;
        let dy = sprite_y - player.y;
        
        let cos_angle = player.angle.cos();
        let sin_angle = player.angle.sin();
        
        let transform_x = dy * cos_angle - dx * sin_angle;
        let transform_y = dx * cos_angle + dy * sin_angle;
        
        if transform_y <= 0.1 {
            return;
        }
        
        let sprite_screen_x = (SCREEN_WIDTH as f32 / 2.0) * (1.0 + transform_x / transform_y);
        
        let base_sprite_height = (SCREEN_HEIGHT as f32 / transform_y) * 0.5;
        let sprite_height = (base_sprite_height * scale).abs();
        let sprite_width = sprite_height;
        
        if sprite_screen_x < -sprite_width || sprite_screen_x > SCREEN_WIDTH as f32 + sprite_width {
            return;
        }
        
        let draw_x = sprite_screen_x - sprite_width / 2.0;
        let draw_y = (SCREEN_HEIGHT as f32 - sprite_height) / 2.0;
        
        let brightness = (1.0 / (1.0 + distance * 0.05)).min(1.0);
        let tint = Color::new(
            (255.0 * brightness) as u8,
            (255.0 * brightness) as u8,
            (255.0 * brightness) as u8,
            255,
        );
        
        let center_x = sprite_screen_x as i32;
        if center_x >= 0 && center_x < RAY_COUNT as i32 && transform_y < self.z_buffer[center_x as usize] {
            d.draw_texture_ex(
                texture,
                Vector2::new(draw_x, draw_y),
                0.0,
                sprite_width / texture.width as f32,
                tint,
            );
        }
    }

    fn draw_circle_sprite(&self, d: &mut RaylibDrawHandle, player: &Player, sprite_x: f32, sprite_y: f32, distance: f32, color: Color, scale: f32) {
        let dx = sprite_x - player.x;
        let dy = sprite_y - player.y;
        
        let cos_angle = player.angle.cos();
        let sin_angle = player.angle.sin();
        
        let transform_x = dy * cos_angle - dx * sin_angle;
        let transform_y = dx * cos_angle + dy * sin_angle;
        
        if transform_y <= 0.1 {
            return;
        }
        
        let sprite_screen_x = (SCREEN_WIDTH as f32 / 2.0) * (1.0 + transform_x / transform_y);
        
        let base_sprite_height = (SCREEN_HEIGHT as f32 / transform_y) * 0.5;
        let sprite_height = (base_sprite_height * scale).abs();
        let sprite_width = sprite_height * 0.8;
        
        if sprite_screen_x < -sprite_width || sprite_screen_x > SCREEN_WIDTH as f32 + sprite_width {
            return;
        }
        
        let draw_start_x = (sprite_screen_x - sprite_width / 2.0) as i32;
        let draw_end_x = (sprite_screen_x + sprite_width / 2.0) as i32;
        let draw_start_y = ((SCREEN_HEIGHT as f32 - sprite_height) / 2.0) as i32;
        let draw_end_y = draw_start_y + sprite_height as i32;
        
        let center_x = sprite_screen_x as i32;
        let center_y = (draw_start_y + draw_end_y) / 2;
        let radius_squared = ((sprite_height / 2.0) as i32).pow(2) as f32;
        let brightness = (1.0 / (1.0 + distance * 0.05)).min(1.0);
        let final_color = Color::new(
            (color.r as f32 * brightness) as u8,
            (color.g as f32 * brightness) as u8,
            (color.b as f32 * brightness) as u8,
            255,
        );
        
        let x_start = draw_start_x.max(0);
        let x_end = draw_end_x.min(SCREEN_WIDTH);
        let y_start = draw_start_y.max(0);
        let y_end = draw_end_y.min(SCREEN_HEIGHT);
        
        for x in x_start..x_end {
            if x >= 0 && x < RAY_COUNT as i32 && transform_y < self.z_buffer[x as usize] {
                let dx_col = (x - center_x).pow(2) as f32;
                
                for y in y_start..y_end {
                    let dy_row = (y - center_y).pow(2) as f32;
                    let dist_from_center_squared = dx_col + dy_row;
                    
                    if dist_from_center_squared <= radius_squared {
                        d.draw_pixel(x, y, final_color);
                    }
                }
            }
        }
    }
}
