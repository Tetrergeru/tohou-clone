use crate::{
    geometry::{Circle, Vector},
    world::World,
};

use super::BulletEmmiter;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct ForwardEmitter {
    timer: f64,
    bullet_cooldown: f64,
    bullet_number: usize,
    bullet_speed: f64,
    forward: Vector,
    spawn_angle: f64,
}

impl ForwardEmitter {
    pub fn new(cooldown: f64, bullets: usize, forward: Vector, spawn_angle: f64) -> Self {
        Self {
            timer: 0.0,
            bullet_cooldown: cooldown,
            bullet_number: bullets,
            bullet_speed: forward.len(),
            forward: forward.norm(),
            spawn_angle,
        }
    }
}

impl BulletEmmiter for ForwardEmitter {
    fn tick(
        &mut self,
        enemy: &Circle,
        _time: f64,
        delta: f64,
        bullets: &mut Vec<crate::world::Bullet>,
    ) {
        self.timer -= delta;
        if self.timer >= 0.0 {
            return;
        }

        let mut angle = self.forward.angle() - self.spawn_angle / 2.0;
        let end_angle = self.forward.angle() + self.spawn_angle / 2.0;

        self.timer += self.bullet_cooldown;
        let d_angle = self.spawn_angle / (self.bullet_number - 1) as f64;

        while angle <= end_angle {
            let speed_direction = Vector::new(angle.cos(), angle.sin());
            let speed = speed_direction * self.bullet_speed;

            bullets.push(World::new_bullet(
                enemy.coord + speed_direction * enemy.r,
                speed,
            ));
            angle += d_angle;
        }
    }
}
