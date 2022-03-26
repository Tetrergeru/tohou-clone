mod l1;
mod l2;

pub use l1::l1;
pub use l2::l2;

use crate::{
    enemies::Enemy,
    world::{Bullet, TickResult},
};

#[derive(Clone)]
pub struct Level {
    scene: usize,
    pub scenes: Vec<Scene>,
    pub background: String,
    pub sound: String,
}

impl Level {
    pub fn tick(&mut self, enemies: &mut Vec<Enemy>, bullets: &mut Vec<Bullet>) -> TickResult {
        if enemies.is_empty() && self.scene == self.scenes.len() {
            return TickResult::Win;
        }
        if enemies.is_empty() {
            log::debug!("Level 1, Scene {}", self.scene);
            bullets.drain(..);
            self.scenes[self.scene].spawn(enemies);
            self.scene += 1;
        }
        TickResult::None
    }
}

#[derive(Clone)]
pub struct Scene {
    enemies: Vec<Enemy>,
}

impl Scene {
    fn spawn(&self, to: &mut Vec<Enemy>) {
        for enemy in self.enemies.iter() {
            to.push(enemy.clone());
        }
    }
}
