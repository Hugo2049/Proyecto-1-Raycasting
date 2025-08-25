mod game;
mod player;
mod raycast;
mod map;
mod minimap;
mod sprite;
mod audio;
mod menu;

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Ray Caster Game")
        .build();

    rl.set_target_fps(60);
    rl.disable_cursor();

    let mut game = game::Game::new(&mut rl, &thread);
    
    game.load_textures(&mut rl, &thread);

    while !rl.window_should_close() {
        game.update(&mut rl);
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        game.draw(&mut d);
    }
}
