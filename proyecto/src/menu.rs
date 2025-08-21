use raylib::prelude::*;

pub struct Menu {
    selected_level: usize,
    title_animation: f32,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            selected_level: 0,
            title_animation: 0.0,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) -> Option<usize> {
        self.title_animation += rl.get_frame_time();
        
        // Handle input
        if rl.is_key_pressed(KeyboardKey::KEY_UP) && self.selected_level > 0 {
            self.selected_level -= 1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) && self.selected_level < 2 {
            self.selected_level += 1;
        }
        
        // Start selected level
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            return Some(self.selected_level);
        }

        // Number keys for quick selection
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            return Some(0);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            return Some(1);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            return Some(2);
        }

        None
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        // Animated background
        let bg_color = Color::new(
            (20.0 + (self.title_animation * 0.5).sin() * 10.0) as u8,
            (30.0 + (self.title_animation * 0.3).sin() * 15.0) as u8,
            (50.0 + (self.title_animation * 0.7).sin() * 20.0) as u8,
            255,
        );
        d.clear_background(bg_color);

        // Title with animation
        let title = "RAY CASTER GAME";
        let title_size = 48;
        let title_width = measure_text(title, title_size);
        let title_y = 150.0 + (self.title_animation * 2.0).sin() * 10.0;
        
        // Title shadow
        d.draw_text(
            title,
            512 - title_width / 2 + 3,
            title_y as i32 + 3,
            title_size,
            Color::BLACK,
        );
        
        // Main title
        d.draw_text(
            title,
            512 - title_width / 2,
            title_y as i32,
            title_size,
            Color::WHITE,
        );

        // Instructions
        let instruction = "Use WASD to move, Mouse to look around";
        let inst_width = measure_text(instruction, 20);
        d.draw_text(
            instruction,
            512 - inst_width / 2,
            250,
            20,
            Color::LIGHTGRAY,
        );

        // Level selection
        let levels = ["Level 1: The Beginning", "Level 2: The Corridors", "Level 3: The Maze"];
        let start_y = 320;

        d.draw_text("Select Level:", 450, start_y - 30, 24, Color::WHITE);

        for (i, level_name) in levels.iter().enumerate() {
            let y = start_y + i as i32 * 40;
            let color = if i == self.selected_level {
                Color::YELLOW
            } else {
                Color::WHITE
            };

            // Selection indicator
            if i == self.selected_level {
                d.draw_text(">", 420, y, 24, Color::YELLOW);
            }

            d.draw_text(level_name, 450, y, 24, color);
        }

        // Controls help
        let controls_y = start_y + 180;
        d.draw_text("Controls:", 450, controls_y, 20, Color::LIGHTGRAY);
        d.draw_text("↑↓ - Select Level", 450, controls_y + 25, 16, Color::GRAY);
        d.draw_text("ENTER - Start Game", 450, controls_y + 45, 16, Color::GRAY);
        d.draw_text("1,2,3 - Quick Select", 450, controls_y + 65, 16, Color::GRAY);
        d.draw_text("ESC - Return to Menu (in game)", 450, controls_y + 85, 16, Color::GRAY);

        // Objective
        d.draw_text("Objective: Collect all coins to win!", 350, controls_y + 120, 20, Color::GOLD);

        // Footer
        d.draw_text("Made with Rust & Raylib", 10, 740, 16, Color::DARKGRAY);
    }
}

// Helper function for text measurement
fn measure_text(text: &str, font_size: i32) -> i32 {
    // Approximate text width calculation
    text.len() as i32 * font_size / 2
}
