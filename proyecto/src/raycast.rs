use raylib::prelude::*;
use crate::player::Player;
use crate::map::Map;
use crate::sprite::SpriteManager;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;
const RAY_COUNT: usize = SCREEN_WIDTH as usize;

pub struct RayCaster {
    z_buffer: Vec<f32>,
}

impl RayCaster {
    pub fn new() -> Self {
        Self {
            z_buffer: vec![0.0; RAY_COUNT],
        }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle, player: &Player, map: &Map, sprites: &SpriteManager) {
        // Cast rays and draw walls
        for i in 0..RAY_COUNT {
            let camera_x = 2.0 * i as f32 / RAY_COUNT as f32 - 1.0;
            let ray_angle = player.angle + camera_x * player.fov / 2.0;
            
            let (hit_distance, wall_type) = self.cast_ray(player, map, ray_angle);
            
            // Store distance for sprite rendering
            self.z_buffer[i] = hit_distance;
            
            // Calculate wall height
            let wall_height = (SCREEN_HEIGHT as f32 / hit_distance).min(SCREEN_HEIGHT as f32);
            let wall_start = ((SCREEN_HEIGHT as f32 - wall_height) / 2.0) as i32;
            let wall_end = wall_start + wall_height as i32;
            
            // Get wall color based on type
            let wall_color = self.get_wall_color(wall_type, hit_distance);
            
            // Draw vertical line for this ray
            d.draw_line(i as i32, wall_start, i as i32, wall_end, wall_color);
            
            // Draw floor
            d.draw_line(i as i32, wall_end, i as i32, SCREEN_HEIGHT, Color::DARKGRAY);
            
            // Draw ceiling
            d.draw_line(i as i32, 0, i as i32, wall_start, Color::DARKBLUE);
        }
        
        // Draw sprites (coins)
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
                return (distance, 1); // Hit boundary
            }
            
            let wall_type = map.get_cell(grid_x, grid_y);
            if wall_type > 0 {
                return (distance, wall_type);
            }
            
            if distance > 20.0 { // Max render distance
                break;
            }
        }
        
        (distance, 1)
    }

    fn get_wall_color(&self, wall_type: u8, distance: f32) -> Color {
        // Different colors for different wall types
        let base_color = match wall_type {
            1 => Color::RED,
            2 => Color::GREEN,
            3 => Color::BLUE,
            4 => Color::YELLOW,
            5 => Color::PURPLE,
            _ => Color::GRAY,
        };
        
        // Apply distance-based shading
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
        
        // Calculate distances to all sprites
        for (i, coin) in sprites.coins.iter().enumerate() {
            if !coin.collected {
                let dx = coin.x - player.x;
                let dy = coin.y - player.y;
                let distance = (dx * dx + dy * dy).sqrt();
                sprite_distances.push((i, distance));
            }
        }
        
        // Sort by distance (farthest first)
        sprite_distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Draw sprites
        for (sprite_idx, distance) in sprite_distances {
            let coin = &sprites.coins[sprite_idx];
            self.draw_sprite(d, player, coin.x, coin.y, distance, Color::GOLD, sprites.get_animation_scale());
        }
    }

    fn draw_sprite(&self, d: &mut RaylibDrawHandle, player: &Player, sprite_x: f32, sprite_y: f32, distance: f32, color: Color, scale: f32) {
        // Calculate sprite position relative to player
        let dx = sprite_x - player.x;
        let dy = sprite_y - player.y;
        
        // Transform to player's coordinate system (corrected)
        let inv_det = 1.0 / (player.angle.cos() * player.angle.sin() + player.angle.sin() * player.angle.cos());
        
        let transform_x = inv_det * (player.angle.sin() * dx - player.angle.cos() * dy);
        let transform_y = inv_det * (-player.angle.cos() * dx - player.angle.sin() * dy);
        
        // Don't draw if behind player
        if transform_y <= 0.1 {
            return;
        }
        
        // Calculate sprite screen position
        let sprite_screen_x = (SCREEN_WIDTH as f32 / 2.0) * (1.0 + transform_x / transform_y);
        
        // Calculate sprite dimensions
        let sprite_height = (SCREEN_HEIGHT as f32 / transform_y).abs() * scale;
        let sprite_width = sprite_height * 0.5; // Aspect ratio
        
        let draw_start_x = (sprite_screen_x - sprite_width / 2.0) as i32;
        let draw_end_x = (sprite_screen_x + sprite_width / 2.0) as i32;
        let draw_start_y = ((SCREEN_HEIGHT as f32 - sprite_height) / 2.0) as i32;
        let draw_end_y = draw_start_y + sprite_height as i32;
        
        // Draw sprite
        for x in draw_start_x.max(0)..draw_end_x.min(SCREEN_WIDTH) {
            // Check if this vertical strip is visible (not behind a wall)
            if x >= 0 && x < RAY_COUNT as i32 && transform_y < self.z_buffer[x as usize] {
                for y in draw_start_y.max(0)..draw_end_y.min(SCREEN_HEIGHT) {
                    // Simple circular sprite shape
                    let center_x = sprite_screen_x as i32;
                    let center_y = (draw_start_y + draw_end_y) / 2;
                    let radius = (sprite_height / 2.0) as i32;
                    
                    let dist_from_center = ((x - center_x).pow(2) + (y - center_y).pow(2)) as f32;
                    if dist_from_center <= (radius * radius) as f32 {
                        let brightness = (1.0 / (1.0 + distance * 0.05)).min(1.0);
                        let final_color = Color::new(
                            (color.r as f32 * brightness) as u8,
                            (color.g as f32 * brightness) as u8,
                            (color.b as f32 * brightness) as u8,
                            255,
                        );
                        d.draw_pixel(x, y, final_color);
                    }
                }
            }
        }
    }
}
