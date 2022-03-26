use crate::{
    enemies::{
        bullet_emmiters::{CircleEmitter, ForwardEmitter, HardcodedEmitter, CombinatorEmitter},
        premade::enemy_1,
        trajectories::{CircleTrajectory, FromToTrajectory, StayTrajectory},
        Enemy, Phase,
    },
    geometry::{Circle, Vector},
};

use super::{Level, Scene};

pub fn l1() -> Level {
    Level {
        scene: 0,
        scenes: vec![l1s0(), l1s1(), l1s2(), l1s3()],
        background: "resources/Floor.png".to_string(),
        sound: "resources/resurrection.mp3".to_string(),
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
            "resources/ghost.png".to_string(),
            100.0,
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
                    "resources/ghost.png".to_string(),
                    100.0,
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

fn l1s3() -> Scene {
    Scene {
        enemies: vec![Enemy::new(
            Circle::new(0.0, 0.0, 20.0),
            30.0,
            vec![
                Phase::new(
                    2.0,
                    Box::new(FromToTrajectory::new(
                        Vector::new(0.0, -700.0),
                        Vector::new(50.0, -250.0),
                        2.0,
                    )),
                    Box::new(HardcodedEmitter::wall(1.5, 200.0, 500.0, 30)),
                ),
                Phase::new(
                    std::f64::consts::PI * 1.0,
                    Box::new(CircleTrajectory::new(
                        Circle::new(-100.0, -250.0, 150.0),
                        std::f64::consts::PI / 2.0,
                        1.0,
                    )),
                    Box::new(CombinatorEmitter::new(
                        HardcodedEmitter::wall(0.5, 200.0, 300.0, 15),
                        CircleEmitter::new(0.3, 7, 200.0),
                    )),
                ),
                Phase::new(
                    std::f64::consts::PI * 2.0,
                    Box::new(CircleTrajectory::new(
                        Circle::new(0.0, -250.0, 250.0),
                        3.0 * std::f64::consts::PI / 2.0,
                        0.5,
                    )),
                    Box::new(HardcodedEmitter::hearth(0.5, 150.0, 30)),
                ),
                Phase::new(
                    std::f64::consts::PI * 1.0,
                    Box::new(CircleTrajectory::new(
                        Circle::new(100.0, -250.0, 150.0),
                        std::f64::consts::PI / 2.0,
                        1.0,
                    )),
                    Box::new(CombinatorEmitter::new(
                        HardcodedEmitter::wall(0.5, 200.0, 300.0, 15),
                        CircleEmitter::new(0.3, 7, 200.0),
                    )),
                ),
                Phase::new(
                    2.0,
                    Box::new(FromToTrajectory::new(
                        Vector::new(-50.0, -250.0),
                        Vector::new(-200.0, -400.0),
                        2.0,
                    )),
                    Box::new(HardcodedEmitter::hearth(0.5, 150.0, 30)),
                ),
                Phase::new(
                    5.0,
                    Box::new(FromToTrajectory::new(
                        Vector::new(-200.0, -400.0),
                        Vector::new(200.0, -400.0),
                        5.0,
                    )),
                    Box::new(HardcodedEmitter::hearth(0.5, 150.0, 30)),
                ),
                Phase::new_jump(
                    2.0,
                    Box::new(FromToTrajectory::new(
                        Vector::new(200.0, -400.0),
                        Vector::new(50.0, -250.0),
                        2.0,
                    )),
                    Box::new(HardcodedEmitter::hearth(0.5, 150.0, 30)),
                    1,
                ),
            ],
            "resources/witch.png".to_string(),
            200.0,
        )],
    }
}
