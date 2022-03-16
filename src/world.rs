use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::{
    enemy::Enemy,
    geometry::{Circle, Vector},
};

#[derive(Clone, PartialEq)]
pub enum EntityType {
    Player,
    Enemy,
}

#[derive(Clone)]
pub struct Bullet {
    pub owner: EntityType,
    pub hitbox: Circle,
    pub speed: Vector,
    marked_for_delete: bool,
}

impl Bullet {
    pub fn new(owner: EntityType, hitbox: Circle, speed: Vector) -> Self {
        Self {
            owner,
            hitbox,
            speed,
            marked_for_delete: false,
        }
    }
}

pub struct World {
    player: Circle,
    enemy: Vec<Enemy>,
    bullets: Vec<Bullet>,
    size: Vector,
    pub time: f64,
}

pub enum TickResult {
    None,
    Win,
    Loose,
}

impl World {
    pub fn new(size: Vector) -> Self {
        Self {
            player: Circle::new(size.x / 2.0, size.y / 6.0 * 5.0, 10.0),
            enemy: vec![
                Enemy::new(Box::new(|t| {
                    let t = t * 3.0;
                    Vector::new(300.0, 300.0) + Vector::new(t.sin(), t.cos()) * 200.0
                })),
                Enemy::new(Box::new(|t| {
                    let t = t * 3.0 + 3.14159265;
                    Vector::new(300.0, 300.0) + Vector::new(t.sin(), t.cos()) * 200.0
                })),
            ],
            bullets: vec![],
            size,
            time: 0.0,
        }
    }

    pub fn move_player(&mut self, delta: Vector) {
        self.player.coord += delta;
    }

    pub fn tick(&mut self, delta: f64) -> TickResult {
        self.time += delta;

        for bullet in self.bullets.iter_mut() {
            if self.player.collides_with(&bullet.hitbox) && bullet.owner == EntityType::Enemy {
                return TickResult::Loose;
            }
        }

        for enemy in self.enemy.iter_mut() {
            enemy.tick(self.time, delta, &mut self.bullets);
        }

        for e in self.enemy.iter_mut() {
            for bullet in self.bullets.iter_mut() {
                if e.hitbox.collides_with(&bullet.hitbox) && bullet.owner == EntityType::Player {
                    e.get_hit();
                    bullet.marked_for_delete = true;
                }
            }
        }

        // for i in 0..self.bullets.len() {
        //     let bi = self.bullets[i].clone();
        //     for j in (i + 1)..self.bullets.len() {
        //         let bj = self.bullets[j].clone();
        //         if bi.hitbox.collides_with(&bj.hitbox) && bi.owner != bj.owner {
        //             self.bullets[i].marked_for_delete = true;
        //             self.bullets[j].marked_for_delete = true;
        //         }
        //     }
        // }

        for bullet in self.bullets.iter_mut() {
            bullet.hitbox.coord += bullet.speed * delta;
            if !bullet.hitbox.in_bounds(0.0, 0.0, self.size.x, self.size.y) {
                bullet.marked_for_delete = true;
            }
        }

        self.bullets.retain(|it| !it.marked_for_delete);

        self.enemy.retain(|it| it.is_alive());

        if self.enemy.len() == 0 {
            return TickResult::Win;
        }

        TickResult::None
    }

    pub fn new_bullet(coord: Vector, speed: Vector) -> Bullet {
        Bullet::new(EntityType::Enemy, Circle::new(coord.x, coord.y, 5.0), speed)
    }

    pub fn shoot(&mut self, speed: Vector) {
        self.bullets.push(Bullet::new(
            EntityType::Player,
            Circle::new(self.player.coord.x, self.player.coord.y, 5.0),
            speed,
        ));
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        context.save();
        context.set_global_composite_operation("copy").unwrap();
        context.set_fill_style(&JsValue::from_str("rgba(0,0,0,0)"));
        context.fill_rect(0.0, 0.0, 700.0, 1100.0);
        context.restore();

        draw_circle(context, &self.player, "green");
        for enemy in self.enemy.iter() {
            draw_circle(context, &enemy.hitbox, "red");
        }
        for bullet in self.bullets.iter() {
            let color = match &bullet.owner {
                EntityType::Player => "blue",
                _ => "yellow",
            };
            draw_circle(context, &bullet.hitbox, color)
        }
    }
}

fn draw_circle(context: &CanvasRenderingContext2d, circle: &Circle, color: &str) {
    context.begin_path();
    context.set_fill_style(&JsValue::from_str(color));
    context
        .arc(
            circle.coord.x,
            circle.coord.y,
            circle.r,
            0.0,
            std::f64::consts::PI * 2.0,
        )
        .unwrap();
    context.fill();
    context.close_path();
}
