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
    wall_texture: Option<Texture2D>,
    floor_texture: Option<Texture2D>,
    // Pre-create a render texture for better performance
    wall_strip_texture: Option<RenderTexture2D>,
}

impl RayCaster {
    pub fn new() -> Self {
        Self {
            z_buffer: [0.0; RAY_COUNT],
            coin_texture: None,
            wall_texture: None,
            floor_texture: None,
            wall_strip_texture: None,
        }
    }

    pub fn load_textures(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // Load coin texture (keeping existing logic)
        let coin_texture_paths = [
            "assets/sprites/sprite.png",
            "assets/sprites/coin.png",
            "assets/coin.png",
        ];

        for path in &coin_texture_paths {
            match rl.load_texture(thread, path) {
                Ok(texture) => {
                    println!("Loaded coin texture from: {}", path);
                    self.coin_texture = Some(texture);
                    break;
                }
                Err(_) => continue,
            }
        }

        // Load wall texture
        let wall_texture_paths = [
            "assets/textures/dungeon.jpg",
            "assets/textures/iceDungeon.jpg",
            "dungeon.jpg",
        ];

        for path in &wall_texture_paths {
            match rl.load_texture(thread, path) {
                Ok(texture) => {
                    println!("Loaded wall texture from: {}", path);
                    self.wall_texture = Some(texture);
                    break;
                }
                Err(e) => {
                    println!("Failed to load wall texture from {}: {}", path, e);
                    continue;
                }
            }
        }

        // Load floor texture
        let floor_texture_paths = [
            "assets/textures/ground.jpg",
            "assets/textures/Ground2.jpg",
            "ground.jpg",
        ];

        for path in &floor_texture_paths {
            match rl.load_texture(thread, path) {
                Ok(texture) => {
                    println!("Loaded floor texture from: {}", path);
                    self.floor_texture = Some(texture);
                    break;
                }
                Err(e) => {
                    println!("Failed to load floor texture from {}: {}", path, e);
                    continue;
                }
            }
        }

        // Create a small render texture for wall strips
        match rl.load_render_texture(thread, 1, SCREEN_HEIGHT as u32) {
            Ok(rt) => self.wall_strip_texture = Some(rt),
            Err(e) => println!("Failed to create wall strip render texture: {}", e),
        }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle, player: &Player, map: &Map, sprites: &SpriteManager) {
        for i in 0..RAY_COUNT {
            let camera_x = 2.0 * i as f32 / RAY_COUNT as f32 - 1.0;
            let ray_angle = player.angle + camera_x * player.fov / 2.0;
            
            let (hit_distance, wall_type, wall_x, is_vertical_wall) = self.cast_ray_detailed(player, map, ray_angle);
            
            self.z_buffer[i] = hit_distance;
            
            // Calculate wall height and position
            let wall_height = (SCREEN_HEIGHT as f32 / hit_distance).min(SCREEN_HEIGHT as f32);
            let wall_start = ((SCREEN_HEIGHT as f32 - wall_height) / 2.0) as i32;
            let wall_end = wall_start + wall_height as i32;
            
            // Draw textured wall column
            self.draw_wall_column(d, i as i32, wall_start, wall_end, wall_x, wall_height, hit_distance, wall_type, is_vertical_wall);
            
            // Draw floor and ceiling
            self.draw_floor_and_ceiling(d, i as i32, wall_start, wall_end, player, ray_angle);
        }
        
        self.draw_sprites(d, player, sprites);
    }

    fn cast_ray_detailed(&self, player: &Player, map: &Map, angle: f32) -> (f32, u8, f32, bool) {
        let dx = angle.cos();
        let dy = angle.sin();
        
        let mut ray_x = player.x;
        let mut ray_y = player.y;
        
        let step = 0.005; // Smaller step for more precision
        let mut distance = 0.0;
        let mut last_x = ray_x;
        let mut last_y = ray_y;
        
        loop {
            last_x = ray_x;
            last_y = ray_y;
            ray_x += dx * step;
            ray_y += dy * step;
            distance += step;
            
            let grid_x = ray_x as usize;
            let grid_y = ray_y as usize;
            
            if grid_x >= map.width || grid_y >= map.height {
                return (distance, 1, 0.0, false);
            }
            
            let wall_type = map.get_cell(grid_x, grid_y);
            if wall_type > 0 {
                // Determine if we hit a vertical or horizontal wall
                let last_grid_x = last_x as usize;
                let last_grid_y = last_y as usize;
                
                let is_vertical_wall = last_grid_x != grid_x;
                
                // Calculate texture coordinate
                let wall_x = if is_vertical_wall {
                    ray_y.fract()
                } else {
                    ray_x.fract()
                };
                
                return (distance, wall_type, wall_x, is_vertical_wall);
            }
            
            if distance > 25.0 {
                break;
            }
        }
        
        (distance, 1, 0.0, false)
    }

    fn draw_wall_column(&self, d: &mut RaylibDrawHandle, x: i32, wall_start: i32, wall_end: i32, 
                       wall_x: f32, wall_height: f32, distance: f32, wall_type: u8, is_vertical_wall: bool) {
        
        if let Some(wall_texture) = &self.wall_texture {
            let tex_x = (wall_x * wall_texture.width as f32) as i32;
            let tex_x = tex_x.max(0).min(wall_texture.width - 1);
            
            // Calculate brightness based on distance and wall orientation
            let mut brightness = (1.0 / (1.0 + distance * 0.08)).min(1.0);
            if !is_vertical_wall {
                brightness *= 0.7; // Make horizontal walls slightly darker
            }
            
            let tint = Color::new(
                (255.0 * brightness) as u8,
                (255.0 * brightness) as u8,
                (255.0 * brightness) as u8,
                255,
            );
            
            // Draw textured wall strip
            let wall_rect_height = (wall_end - wall_start).max(1);
            
            let source_rect = Rectangle::new(
                tex_x as f32, 
                0.0, 
                1.0, 
                wall_texture.height as f32
            );
            
            let dest_rect = Rectangle::new(
                x as f32, 
                wall_start as f32, 
                1.0, 
                wall_rect_height as f32
            );
            
            d.draw_texture_pro(
                wall_texture,
                source_rect,
                dest_rect,
                Vector2::zero(),
                0.0,
                tint
            );
        } else {
            // Fallback to colored walls if no texture
            let wall_color = self.get_wall_color(wall_type, distance);
            d.draw_line(x, wall_start, x, wall_end, wall_color);
        }
    }

    fn draw_floor_and_ceiling(&self, d: &mut RaylibDrawHandle, x: i32, wall_start: i32, wall_end: i32, 
                             player: &Player, ray_angle: f32) {
        
        // Draw ceiling
        let ceiling_color = Color::new(30, 30, 60, 255);
        d.draw_line(x, 0, x, wall_start.max(0), ceiling_color);
        
        // Draw floor
        if let Some(floor_texture) = &self.floor_texture {
            self.draw_textured_floor(d, x, wall_end, player, ray_angle, floor_texture);
        } else {
            let floor_color = Color::new(60, 40, 30, 255);
            d.draw_line(x, wall_end.max(0), x, SCREEN_HEIGHT, floor_color);
        }
    }

    fn draw_textured_floor(&self, d: &mut RaylibDrawHandle, x: i32, wall_end: i32, 
                          player: &Player, ray_angle: f32, floor_texture: &Texture2D) {
        
        let floor_start = wall_end.max(SCREEN_HEIGHT / 2);
        let step_size = 2; // Draw every 2nd pixel for performance
        
        for y in (floor_start..SCREEN_HEIGHT).step_by(step_size) {
            let distance_to_floor = (SCREEN_HEIGHT as f32 / 2.0 - y as f32);
            if distance_to_floor.abs() < 1.0 {
                continue;
            }
            
            let row_distance = (SCREEN_HEIGHT as f32 / 2.0) / distance_to_floor.abs();
            
            if row_distance > 0.0 && row_distance < 25.0 {
                let floor_x = player.x + ray_angle.cos() * row_distance;
                let floor_y = player.y + ray_angle.sin() * row_distance;
                
                let tex_x = ((floor_x * floor_texture.width as f32) as i32 % floor_texture.width).abs();
                let tex_y = ((floor_y * floor_texture.height as f32) as i32 % floor_texture.height).abs();
                
                let brightness = (1.0 / (1.0 + row_distance * 0.15)).min(0.8);
                let tint = Color::new(
                    (255.0 * brightness) as u8,
                    (255.0 * brightness) as u8,
                    (255.0 * brightness) as u8,
                    255,
                );
                
                // Draw a small rectangle from the texture instead of just a pixel
                let source_rect = Rectangle::new(tex_x as f32, tex_y as f32, 1.0, 1.0);
                let dest_rect = Rectangle::new(x as f32, y as f32, 1.0, step_size as f32);
                
                d.draw_texture_pro(
                    floor_texture,
                    source_rect,
                    dest_rect,
                    Vector2::zero(),
                    0.0,
                    tint
                );
            }
        }
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
