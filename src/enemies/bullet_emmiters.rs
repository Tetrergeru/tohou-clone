use crate::{
    geometry::{Circle, Vector},
    world::World,
};

use super::BulletEmmiter;

#[derive(Clone)]
pub struct CombinatorEmmiter<First, Second>(First, Second);

impl<First, Second> CombinatorEmmiter<First, Second> {
    pub fn new(first: First, second: Second) -> Self {
        Self(first, second)
    }
}

impl<First: BulletEmmiter + Clone, Second: BulletEmmiter + Clone> BulletEmmiter
    for CombinatorEmmiter<First, Second>
{
    fn tick(
        &mut self,
        enemy: &Circle,
        time: f64,
        delta: f64,
        bullets: &mut Vec<crate::world::Bullet>,
    ) {
        self.0.tick(enemy, time, delta, bullets);
        self.1.tick(enemy, time, delta, bullets);
    }
}

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

#[derive(Clone)]
pub struct HardcodedEmitter {
    timer: f64,
    bullet_cooldown: f64,
    bullets: Vec<(Vector, Vector)>,
}

impl HardcodedEmitter {
    pub fn new(bullet_cooldown: f64, bullets: Vec<(Vector, Vector)>) -> Self {
        Self {
            timer: 0.0,
            bullet_cooldown,
            bullets,
        }
    }

    pub fn wall(bullet_cooldown: f64, bullet_speed: f64, width: f64, bullet_number: usize) -> Self {
        let mut bullets = vec![];
        let mut left = -width / 2.0;
        let step = width / (bullet_number - 1) as f64;
        for _ in 0..bullet_number {
            bullets.push((Vector::new(left, 30.0), Vector::new(0.0, bullet_speed)));
            left += step;
        }
        Self::new(bullet_cooldown, bullets)
    }

    pub fn hearth(bullet_cooldown: f64, bullet_speed: f64, bullet_number: usize) -> Self {
        let mut bullets = vec![];

        let step = std::f64::consts::PI * 2.0 / (bullet_number - 1) as f64;

        let mut t = 0.0_f64;
        let a = 0.2;
        for _ in 0..bullet_number {
            let x = a * (16.0 * t.sin().powi(3));
            let y = -a * (13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos());
            let point = Vector::new(x, y);
            bullets.push((point, point * bullet_speed));//

            t += step;
        }
        Self::new(bullet_cooldown, bullets)
    }
}

impl BulletEmmiter for HardcodedEmitter {
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

        self.timer += self.bullet_cooldown;

        for (position, speed) in self.bullets.iter() {
            bullets.push(World::new_bullet(enemy.coord + *position, *speed))
        }
    }
}
