use crate::{
    geometry::{Circle, Vector},
    world::World,
};

use super::BulletEmmiter;

pub struct CircleEmitter {
    timer: f64,
    bullet_cooldown: f64,
    bullet_number: usize,
    bullet_speed: f64,
}

impl CircleEmitter {
    pub fn new(cooldown: f64, bullets: usize, speed: f64) -> Self {
        Self {
            timer: 0.0,
            bullet_cooldown: cooldown,
            bullet_number: bullets,
            bullet_speed: speed,
        }
    }
}

impl BulletEmmiter for CircleEmitter {
    fn tick(
        &mut self,
        enemy: &Circle,
        time: f64,
        delta: f64,
        bullets: &mut Vec<crate::world::Bullet>,
    ) {
        self.timer -= delta;
        if self.timer >= 0.0 {
            return;
        }

        let mut angle = 0.0;

        self.timer += self.bullet_cooldown;
        let d_angle = std::f64::consts::TAU / self.bullet_number as f64;

        while angle <= std::f64::consts::TAU {
            let t = time + angle;
            let speed_direction = Vector::new(t.sin(), t.cos());
            let speed = speed_direction * self.bullet_speed;

            bullets.push(World::new_bullet(
                enemy.coord + speed_direction * enemy.r,
                speed,
            ));
            angle += d_angle;
        }
    }
}
