use crate::{
    enemies::{
        bullet_emmiters::ForwardEmitter,
        premade::enemy_1,
        trajectories::{FromToTrajectory, StayTrajectory},
        Enemy, Phase,
    },
    geometry::{Circle, Vector},
    world::{Bullet, TickResult},
};

pub struct Level {
    scene: usize,
    pub scenes: Vec<Scene>,
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

pub fn l1() -> Level {
    Level {
        scene: 0,
        scenes: vec![l1s0(), l1s1(), l1s2()],
    }
}

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

fn l1s0() -> Scene {
    Scene {
        enemies: vec![Enemy::new(
            Circle::new(0.0, 0.0, 20.0),
            30.0,
            vec![
                Phase::new(
                    3.0,
                    Box::new(FromToTrajectory::new(
                        Vector::new(0.0, -800.0),
                        Vector::new(0.0, -400.0),
                        3.0,
                    )),
                    Box::new(ForwardEmitter::new(0.3, 2, Vector::new(0.0, 200.0), 0.5)),
                ),
                Phase::new(
                    f64::MAX,
                    Box::new(StayTrajectory::new(Vector::new(0.0, -400.0))),
                    Box::new(ForwardEmitter::new(0.3, 2, Vector::new(0.0, 200.0), 0.5)),
                ),
            ],
        )],
    }
}

fn l1s1() -> Scene {
    let enemies = vec![
        (
            Vector::new(0.0, -600.0),
            Vector::new(-200.0, -50.0),
            Vector::new(50.0, -450.0),
        ),
        (
            Vector::new(0.0, 600.0),
            Vector::new(-150.0, -350.0),
            Vector::new(200.0, -50.0),
        ),
        (
            Vector::new(400.0, -200.0),
            Vector::new(100.0, -300.0),
            Vector::new(-250.0, -400.0),
        ),
    ];
    Scene {
        enemies: enemies
            .iter()
            .cloned()
            .enumerate()
            .map(|(idx, it)| {
                Enemy::new(
                    Circle::new(0.0, 0.0, 20.0),
                    5.0,
                    vec![
                        Phase::new(
                            6.0,
                            Box::new(FromToTrajectory::new(it.0, it.1, 6.0)),
                            Box::new(ForwardEmitter::new(
                                0.3,
                                2 + idx,
                                Vector::new(0.0, 200.0),
                                1.5,
                            )),
                        ),
                        Phase::new(
                            6.0,
                            Box::new(FromToTrajectory::new(it.1, it.2, 6.0)),
                            Box::new(ForwardEmitter::new(
                                0.3,
                                2 + idx,
                                Vector::new(0.0, 200.0),
                                1.5,
                            )),
                        ),
                        Phase::new_jump(
                            6.0,
                            Box::new(FromToTrajectory::new(it.2, it.1, 6.0)),
                            Box::new(ForwardEmitter::new(
                                0.3,
                                2 + idx,
                                Vector::new(0.0, 200.0),
                                1.5,
                            )),
                            1,
                        ),
                    ],
                )
            })
            .collect(),
    }
}

fn l1s2() -> Scene {
    Scene {
        enemies: vec![
            enemy_1(
                Vector::new(-350.0, -550.0),
                std::f64::consts::PI,
                2.0,
                vec![
                    Vector::new(-200.0, -200.0),
                    Vector::new(-20.0, -350.0),
                    Vector::new(-200.0, -200.0),
                ],
            ),
            enemy_1(
                Vector::new(350.0, -550.0),
                0.0,
                2.0,
                vec![
                    Vector::new(200.0, -200.0),
                    Vector::new(20.0, -350.0),
                    Vector::new(200.0, -200.0),
                ],
            ),
        ],
    }
}
