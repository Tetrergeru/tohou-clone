use crate::{
    geometry::{Circle, Vector},
    world::Bullet,
};

use dyn_clone::DynClone;

pub mod bullet_emmiters;
pub mod premade;
pub mod trajectories;

#[derive(Clone)]
pub struct Enemy {
    hitbox: Circle,
    health: f64,
    phases: Vec<Phase>,
    phase: usize,
    time: f64,
}

#[derive(Clone)]
pub struct Phase {
    length: f64,
    trajectory: Box<dyn Trajectory>,
    bullets: Box<dyn BulletEmmiter>,
    next: Option<usize>,
}

impl Phase {
    pub fn new(
        length: f64,
        trajectory: Box<dyn Trajectory>,
        bullets: Box<dyn BulletEmmiter>,
    ) -> Self {
        Self {
            length,
            trajectory,
            bullets,
            next: None,
        }
    }

    pub fn new_jump(
        length: f64,
        trajectory: Box<dyn Trajectory>,
        bullets: Box<dyn BulletEmmiter>,
        jump_to: usize,
    ) -> Self {
        Self {
            length,
            trajectory,
            bullets,
            next: Some(jump_to),
        }
    }
}

impl Enemy {
    pub fn new(hitbox: Circle, health: f64, phases: Vec<Phase>) -> Self {
        Self {
            hitbox,
            health,
            phases,
            time: 0.0,
            phase: 0,
        }
    }

    pub fn tick(&mut self, delta_time: f64, bullets: &mut Vec<Bullet>) {
        self.time += delta_time;

        let current_phase_length = self.phases[self.phase].length;
        if self.time > current_phase_length {
            self.time -= current_phase_length;

            self.phase = match self.phases[self.phase].next {
                Some(next) => next,
                None => (self.phase + 1) % self.phases.len(),
            }
        }

        let phase = &mut self.phases[self.phase];

        self.hitbox.coord = phase.trajectory.location(self.time);
        phase
            .bullets
            .tick(&self.hitbox, self.time, delta_time, bullets);
    }

    pub fn hit(&mut self, damage: f64) {
        self.hitbox.r -= damage * 0.5;
        self.health -= damage;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn hitbox(&self) -> &Circle {
        &self.hitbox
    }
}

pub trait Trajectory: DynClone {
    fn location(&self, time: f64) -> Vector;
}

dyn_clone::clone_trait_object!(Trajectory);

pub trait BulletEmmiter: DynClone {
    fn tick(&mut self, enemy: &Circle, time: f64, delta: f64, bullets: &mut Vec<Bullet>);
}

dyn_clone::clone_trait_object!(BulletEmmiter);