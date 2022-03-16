use crate::{
    geometry::{Circle, Vector},
    world::{Bullet, World},
};

pub struct Enemy {
    pub trajectory: Box<dyn Fn(f64) -> Vector>,
    pub hitbox: Circle,
    pub health: f64,
    pub timer: f64,
}

impl Enemy {
    pub fn new(trajectory: Box<dyn Fn(f64) -> Vector>) -> Self {
        Self {
            trajectory,
            hitbox: Circle::new(0.0, 0.0, 30.0),
            timer: 0.0,
            health: 30.0,
        }
    }

    pub fn tick(&mut self, world_time: f64, delta_time: f64, bullets: &mut Vec<Bullet>) {
        self.hitbox.coord = (self.trajectory)(world_time);

        let top = 0.1;
        let angles = 2.0;
        let speed = 200.0;

        self.timer -= delta_time;
        if self.timer >= 0.0 {
            return;
        }
        self.timer += top;
        let mut angle = 1.0;
        let d_angle = std::f64::consts::TAU / angles;
        while angle < std::f64::consts::TAU {
            let t = world_time + angle;
            let speed = Vector::new(t.sin(), t.cos()) * speed;
            bullets.push(World::new_bullet(self.hitbox.coord + speed * 0.1, speed));
            angle += d_angle;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn get_hit(&mut self) {
        self.health -= 3.0;
        self.hitbox.r -= 2.0;
    }
}
