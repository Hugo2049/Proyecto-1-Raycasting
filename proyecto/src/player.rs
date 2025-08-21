use raylib::prelude::*;
use crate::map::Map;
use std::f32::consts::PI;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub fov: f32,
    move_speed: f32,
    rot_speed: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Self {
            x,
            y,
            angle,
            fov: PI / 3.0, // 60 degrees
            move_speed: 3.0,
            rot_speed: 2.0,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, map: &Map) {
        let dt = rl.get_frame_time();
        
        // Mouse rotation (horizontal only)
        let mouse_delta = rl.get_mouse_delta();
        self.angle += mouse_delta.x * 0.002;

        // Keyboard movement
        let mut move_x = 0.0;
        let mut move_y = 0.0;

        if rl.is_key_down(KeyboardKey::KEY_W) {
            move_x += self.angle.cos();
            move_y += self.angle.sin();
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            move_x -= self.angle.cos();
            move_y -= self.angle.sin();
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            move_x += (self.angle - PI / 2.0).cos();
            move_y += (self.angle - PI / 2.0).sin();
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            move_x += (self.angle + PI / 2.0).cos();
            move_y += (self.angle + PI / 2.0).sin();
        }

        // Keyboard rotation (arrow keys)
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.angle -= self.rot_speed * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.angle += self.rot_speed * dt;
        }

        // Normalize movement vector
        let move_len = (move_x * move_x + move_y * move_y).sqrt();
        if move_len > 0.0 {
            move_x /= move_len;
            move_y /= move_len;
        }

        // Apply movement with collision detection
        let new_x = self.x + move_x * self.move_speed * dt;
        let new_y = self.y + move_y * self.move_speed * dt;

        // Check collision for X movement
        if !map.is_wall(new_x as usize, self.y as usize) {
            self.x = new_x;
        }

        // Check collision for Y movement
        if !map.is_wall(self.x as usize, new_y as usize) {
            self.y = new_y;
        }

        // Keep angle in range [0, 2π]
        while self.angle < 0.0 {
            self.angle += 2.0 * PI;
        }
        while self.angle >= 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
    }

    pub fn get_direction(&self) -> (f32, f32) {
        (self.angle.cos(), self.angle.sin())
    }
}
