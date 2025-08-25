use crate::player::Player;
use crate::map::Map;

#[derive(Clone)]
pub struct Coin {
    pub x: f32,
    pub y: f32,
    pub collected: bool,
}

pub struct SpriteManager {
    pub coins: Vec<Coin>,
    animation_time: f32,
}

impl SpriteManager {
    pub fn new(map: &Map) -> Self {
        let positions = map.get_coin_positions();
        let mut coins = Vec::new();

        for (x, y) in positions {
            coins.push(Coin {
                x,
                y,
                collected: false,
            });
        }

        Self {
            coins,
            animation_time: 0.0,
        }
    }

    pub fn check_collision(&mut self, player: &Player) -> Option<(f32, f32)> {
        let collection_distance = 0.5;

        for coin in &mut self.coins {
            if !coin.collected {
                let dx = coin.x - player.x;
                let dy = coin.y - player.y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance < collection_distance {
                    coin.collected = true;
                    return Some((coin.x, coin.y));
                }
            }
        }

        None
    }

    pub fn all_coins_collected(&self) -> bool {
        self.coins.iter().all(|coin| coin.collected)
    }

    pub fn coins_collected(&self) -> usize {
        self.coins.iter().filter(|coin| coin.collected).count()
    }

    pub fn total_coins(&self) -> usize {
        self.coins.len()
    }

    pub fn get_animation_scale(&self) -> f32 {
        1.0 + (self.animation_time * 3.0).sin() * 0.1
    }

    pub fn update_animation(&mut self, dt: f32) {
        self.animation_time += dt;
    }
}
