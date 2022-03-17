use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::{
    enemies::Enemy,
    geometry::{Circle, Vector},
    level::{l1, Level},
};

#[derive(Clone, PartialEq)]
pub enum BulletType {
    PlayerSniper,
    PlayerHeavy,
    Enemy,
}

#[derive(Clone)]
pub struct Bullet {
    pub typ: BulletType,
    pub hitbox: Circle,
    pub speed: Vector,
    marked_for_delete: bool,
}

impl Bullet {
    pub fn new(typ: BulletType, hitbox: Circle, speed: Vector) -> Self {
        Self {
            typ,
            hitbox,
            speed,
            marked_for_delete: false,
        }
    }
}

pub struct World {
    player: Circle,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    size: Vector,
    pub time: f64,
    level: Level,
}

#[derive(PartialEq)]
pub enum TickResult {
    None,
    Win,
    Loose,
}

impl World {
    pub fn new(size: Vector) -> Self {
        Self {
            player: Circle::new(0.0, size.y / 6.0 * 2.0, 10.0),
            level: l1(),
            enemies: vec![],
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

        let level_tick = self.level.tick(&mut self.enemies, &mut self.bullets);
        if level_tick != TickResult::None {
            return level_tick;
        }

        // === Collisions ===

        if self.player.coord.x < -self.size.x / 2.0 + self.player.r {
            self.player.coord.x = -self.size.x / 2.0 + self.player.r;
        }
        if self.player.coord.x > self.size.x / 2.0 - self.player.r {
            self.player.coord.x = self.size.x / 2.0 - self.player.r;
        }
        if self.player.coord.y < -self.size.y / 2.0 + self.player.r {
            self.player.coord.y = -self.size.y / 2.0 + self.player.r;
        }
        if self.player.coord.y > self.size.y / 2.0 - self.player.r {
            self.player.coord.y = self.size.y / 2.0 - self.player.r;
        }

        for bullet in self.bullets.iter_mut() {
            if self.player.collides_with(&bullet.hitbox) && bullet.typ == BulletType::Enemy {
                return TickResult::Loose;
            }
        }

        for i in 0..self.bullets.len() {
            let bi = self.bullets[i].clone();
            for j in (i + 1)..self.bullets.len() {
                let bj = self.bullets[j].clone();
                if bi.typ == BulletType::PlayerSniper || bj.typ == BulletType::PlayerSniper {
                    continue;
                }
                if bi.hitbox.collides_with(&bj.hitbox) && bi.typ != bj.typ {
                    self.bullets[i].marked_for_delete = true;
                    self.bullets[j].marked_for_delete = true;
                }
            }
        }

        for e in self.enemies.iter_mut() {
            for bullet in self.bullets.iter_mut() {
                if e.hitbox().collides_with(&bullet.hitbox) {
                    if bullet.typ == BulletType::PlayerSniper {
                        e.hit(3.0);
                        bullet.marked_for_delete = true;
                    } else if bullet.typ == BulletType::PlayerHeavy {
                        bullet.marked_for_delete = true;
                    }
                }
            }
        }

        // === Delete enemies and bullets ===

        for enemy in self.enemies.iter_mut() {
            enemy.tick(delta, &mut self.bullets);
        }

        for bullet in self.bullets.iter_mut() {
            bullet.hitbox.coord += bullet.speed * delta;
            if !bullet.hitbox.in_bounds(
                -self.size.x / 2.0,
                -self.size.y / 2.0,
                self.size.x / 2.0,
                self.size.y / 2.0,
            ) {
                bullet.marked_for_delete = true;
            }
        }

        self.bullets.retain(|it| !it.marked_for_delete);

        self.enemies.retain(|it| it.is_alive());

        TickResult::None
    }

    pub fn new_bullet(coord: Vector, speed: Vector) -> Bullet {
        Bullet::new(BulletType::Enemy, Circle::new(coord.x, coord.y, 5.0), speed)
    }

    pub fn shoot(&mut self, speed: Vector, typ: BulletType) {
        let r = match &typ {
            BulletType::PlayerSniper => 5.0,
            BulletType::PlayerHeavy => 10.0,
            BulletType::Enemy => 5.0,
        };
        self.bullets.push(Bullet::new(
            typ,
            Circle::new(self.player.coord.x, self.player.coord.y, r),
            speed,
        ));
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        context.save();
        context.set_global_composite_operation("copy").unwrap();
        context.set_fill_style(&JsValue::from_str("rgba(0,0,0,0)"));
        context.fill_rect(0.0, 0.0, 700.0, 1100.0);
        context.restore();

        self.draw_circle(context, &self.player, "green");
        for enemy in self.enemies.iter() {
            self.draw_circle(context, enemy.hitbox(), "red");
        }
        for bullet in self.bullets.iter() {
            let color = match &bullet.typ {
                BulletType::PlayerSniper => "blue",
                BulletType::PlayerHeavy => "cyan",
                _ => "orange",
            };
            self.draw_circle(context, &bullet.hitbox, color)
        }
    }

    fn draw_circle(&self, context: &CanvasRenderingContext2d, circle: &Circle, color: &str) {
        context.begin_path();
        context.set_fill_style(&JsValue::from_str(color));
        context
            .arc(
                self.size.x / 2.0 + circle.coord.x,
                self.size.y / 2.0 + circle.coord.y,
                circle.r,
                0.0,
                std::f64::consts::PI * 2.0,
            )
            .unwrap();
        context.fill();
        context.close_path();
    }
}
