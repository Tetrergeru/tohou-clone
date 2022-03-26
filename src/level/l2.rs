use crate::{
    enemies::{
        bullet_emmiters::{CircleEmitter, ForwardEmitter, HardcodedEmitter, CombinatorEmitter},
        trajectories::{CircleTrajectory, CombinatorTrajectory, FromToTrajectory, StayTrajectory},
        Enemy, Phase,
    },
    geometry::{Circle, Vector},
};

use super::{Level, Scene};

pub fn l2() -> Level {
    Level {
        scene: 0,
        scenes: vec![l2s1(), l2s3()],
        background: "resources/Floor.png".to_string(),
        sound: "resources/cypis.mp3".to_string(),
    }
}

fn l2s1() -> Scene {
    Scene {
        enemies: vec![
            l2ls1_enemy(
                Vector::new(0.0, -550.0),
                std::f64::consts::PI,
                2.0,
                vec![
                    Vector::new(-200.0, -200.0),
                    Vector::new(-20.0, -350.0),
                    Vector::new(-200.0, -200.0),
                ],
            ),
            l2ls1_enemy(
                Vector::new(0.0, -550.0),
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

pub fn l2ls1_enemy(start: Vector, offset: f64, rotation_speed: f64, points: Vec<Vector>) -> Enemy {
    let mut stages = vec![
        Phase::new(
            3.9,
            Box::new(FromToTrajectory::new(start, points[0], 3.9)),
            Box::new(HardcodedEmitter::forward_hearth(
                1.0,
                5.0,
                Vector::new(0.0, 200.0),
                30,
            )),
        ),
        Phase::new(
            std::f64::consts::PI * 2.0,
            Box::new(CircleTrajectory::new(
                Circle::new(0.0, -200.0, 200.0),
                offset + std::f64::consts::PI / 2.0,
                rotation_speed,
            )),
            Box::new(HardcodedEmitter::hearth(1.5, 100.0, 40)),
        ),
        Phase::new(
            1.0,
            Box::new(StayTrajectory::new(points[0])),
            Box::new(CircleEmitter::new(0.2, 6, 200.0)),
        ),
    ];
    for i in 0..(points.len() - 1) {
        stages.push(Phase::new(
            1.0,
            Box::new(FromToTrajectory::new(points[i], points[i + 1], 1.0)),
            Box::new(ForwardEmitter::new(0.2, 6, Vector::new(0.0, 200.0), 1.0)),
        ));
    }
    stages.push(Phase::new(
        1.0,
        Box::new(StayTrajectory::new(points[0])),
        Box::new(CircleEmitter::new(0.2, 6, 200.0)),
    ));

    stages.push(Phase::new(
        std::f64::consts::PI * 2.0,
        Box::new(CircleTrajectory::new(
            Circle::new(0.0, -200.0, 200.0),
            offset + std::f64::consts::PI / 2.0,
            -rotation_speed,
        )),
        Box::new(HardcodedEmitter::hearth(1.5, 100.0, 40)),
    ));
    stages.push(Phase::new_jump(
        3.0,
        Box::new(StayTrajectory::new(*points.last().unwrap())),
        Box::new(CircleEmitter::new(0.2, 6, 300.0)),
        1,
    ));

    Enemy::new(
        Circle::new(0.0, 0.0, 30.0),
        30.0,
        stages,
        "resources/ghost.png".to_string(),
        100.0,
    )
}

fn l2s3() -> Scene {
    let start = Vector::new(0.0, -1000.0);
    Scene {
        enemies: vec![Enemy::new(
            Circle::new(0.0, 0.0, 30.0),
            100.0,
            vec![
                Phase::new(
                    3.0,
                    Box::new(FromToTrajectory::new(start, Vector::new(0.0, -450.0), 3.0)),
                    Box::new(HardcodedEmitter::forward_hearth(
                        1.0,
                        150.0,
                        Vector::new(0.0, 200.0),
                        80,
                    )),
                ),
                Phase::new(
                    f64::MAX,
                    Box::new(CombinatorTrajectory::new(
                        CircleTrajectory::new(
                            Circle::new(0.0, -200.0, 150.0),
                            std::f64::consts::PI,
                            0.5,
                        ),
                        CircleTrajectory::new(
                            Circle::new(0.0, 0.0, 100.0),
                            std::f64::consts::PI,
                            -1.0,
                        ),
                    )),
                    Box::new(CombinatorEmitter::new(
                        CircleEmitter::new(0.3, 7, 200.0),
                        CircleEmitter::new(0.2, 7, 100.0),
                    )),
                ),
            ],
            "resources/ghost.png".to_string(),
            100.0,
        )],
    }
}
