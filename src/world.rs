use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::{
    audio::AudioManager,
    enemies::Enemy,
    geometry::{Circle, Rect, Vector},
    level::Level,
    textures::TextureManager,
};

#[derive(Clone, Copy, PartialEq)]
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
    pub level: Level,
    pub player_bullets: usize,
}

#[derive(PartialEq)]
pub enum TickResult {
    None,
    Win,
    Loose,
}

impl World {
    pub fn new(size: Vector, level: Level) -> Self {
        Self {
            player: Circle::new(0.0, size.y / 6.0 * 2.0, 10.0),
            level,
            enemies: vec![],
            bullets: vec![],
            size,
            time: 0.0,
            player_bullets: 1,
        }
    }

    pub fn move_player(&mut self, delta: Vector) {
        self.player.coord += delta;
    }

    pub fn reset(&mut self, next_level: Level) {
        self.bullets.drain(..);
        self.enemies.drain(..);
        self.player.coord = Vector::new(0.0, self.size.y / 6.0 * 2.0);
        self.level = next_level;
    }

    pub fn tick(&mut self, delta: f64, audio: &AudioManager) -> TickResult {
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

        let mut bullet_collision = false;
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
                    bullet_collision = true;
                }
            }
        }
        if bullet_collision {
            audio.play_name("resources/shoot.wav", false, true, 0.3);
        }

        let mut hit_enemy = false;
        for e in self.enemies.iter_mut() {
            for bullet in self.bullets.iter_mut() {
                if e.hitbox().collides_with(&bullet.hitbox) {
                    if bullet.typ == BulletType::PlayerSniper {
                        e.hit(3.0);
                        bullet.marked_for_delete = true;
                        hit_enemy = true;
                    } else if bullet.typ == BulletType::PlayerHeavy {
                        bullet.marked_for_delete = true;
                        hit_enemy = true;
                    }
                }
            }
        }
        if hit_enemy {
            audio.play_name("resources/shoot_2.wav", false, true, 0.3);
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

    pub fn shoot(&mut self, speed: Vector, typ: BulletType, audio: &AudioManager) {
        let r = match &typ {
            BulletType::PlayerSniper => 5.0,
            BulletType::PlayerHeavy => 10.0,
            BulletType::Enemy => 5.0,
        };
        let mut left = -(r + 1.0) * (self.player_bullets - 1) as f64 / 2.0;
        for _ in 0..self.player_bullets {
            self.bullets.push(Bullet::new(
                typ,
                Circle::new(self.player.coord.x + left, self.player.coord.y, r),
                speed,
            ));
            left += r + 1.0;
        }
        audio.play_name("resources/shoot_3.wav", false, true, 0.1);
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d, texture_manager: &TextureManager) {
        let missile = texture_manager.get("resources/missile.png");
        let missile_2 = texture_manager.get("resources/missile_2.png");
        let hearth = texture_manager.get("resources/hearth.png");
        let green_hearth = texture_manager.get("resources/green_hearth.png");

        context.save();
        context.set_global_composite_operation("copy").unwrap();
        context.set_fill_style(&JsValue::from_str("rgba(0,0,0,0)"));
        context.fill_rect(0.0, 0.0, 700.0, 1100.0);
        context.restore();

        self.draw_back(context, texture_manager);

        let player_bounds = Rect::new(
            self.player.coord.x,
            self.player.coord.y,
            green_hearth.width() as f64,
            green_hearth.height() as f64,
        )
        .with_width(self.player.r * 3.5);

        // self.draw_circle(context, &self.player, "gray");
        self.draw_image(context, &player_bounds, green_hearth);

        for enemy in self.enemies.iter() {
            let img = texture_manager.get(&enemy.sprite);
            let center = enemy.hitbox().coord;
            let w = img.width() as f64;
            let h = img.height() as f64;

            let bounds = Rect::new(center.x, center.y, w, h).with_width(enemy.display_width);
            let hearth_bounds = Rect::new(
                center.x,
                center.y,
                hearth.width() as f64,
                hearth.height() as f64,
            )
            .with_width(enemy.hitbox().r * 3.0);

            self.draw_image(context, &bounds, img);
            // self.draw_circle(context, enemy.hitbox(), "purple");
            self.draw_image(context, &hearth_bounds, hearth);
        }
        for bullet in self.bullets.iter() {
            match bullet.typ {
                BulletType::PlayerSniper => {
                    self.draw_bullet(context, missile_2, bullet, 1.5);
                }
                BulletType::Enemy => {
                    self.draw_bullet(context, missile, bullet, 1.5);
                }
                BulletType::PlayerHeavy => self.draw_circle(context, &bullet.hitbox, "cyan"),
            }
        }
    }

    fn draw_back(&self, context: &CanvasRenderingContext2d, texture_manager: &TextureManager) {
        let img = texture_manager.get(&self.level.background);
        
        let t = (self.time * 50.0) % 250.0;
        for i in -3..4 {
            self.draw_image(context, &Rect::new(0.0, 249.0 * i as f64 + t, 600.0, 350.0), img)
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

    fn draw_image(
        &self,
        context: &CanvasRenderingContext2d,
        bounds: &Rect,
        img: &HtmlImageElement,
    ) {
        context
            .draw_image_with_html_image_element_and_dw_and_dh(
                img,
                self.size.x / 2.0 + bounds.center.x - bounds.size.x / 2.0,
                self.size.y / 2.0 + bounds.center.y - bounds.size.y / 2.0,
                bounds.size.x,
                bounds.size.y,
            )
            .unwrap();
    }

    fn draw_bullet(
        &self,
        context: &CanvasRenderingContext2d,
        img: &HtmlImageElement,
        bullet: &Bullet,
        size_mod: f64,
    ) {
        let mut circle = bullet.hitbox.clone();
        circle.r *= size_mod;
        let angle = bullet.speed.norm().angle() - std::f64::consts::PI / 2.0;

        context.save();
        context
            .translate(
                self.size.x / 2.0 + circle.coord.x,
                self.size.y / 2.0 + circle.coord.y,
            )
            .unwrap();
        context.rotate(angle).unwrap();
        context
            .draw_image_with_html_image_element_and_dw_and_dh(
                img,
                -circle.r,
                -circle.r,
                circle.r * 2.0,
                circle.r * 2.0,
            )
            .unwrap();
        context.restore();
    }
}
