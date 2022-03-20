use crate::geometry::{Circle, Vector};

use super::{
    bullet_emmiters::{CircleEmitter, ForwardEmitter},
    trajectories::{CircleTrajectory, FromToTrajectory, StayTrajectory},
    Enemy, Phase,
};

pub fn enemy_1(start: Vector, offset: f64, rotation_speed: f64, points: Vec<Vector>) -> Enemy {
    let mut stages = vec![
        Phase::new(
            5.0,
            Box::new(FromToTrajectory::new(start, points[0], 5.0)),
            Box::new(ForwardEmitter::new(0.2, 6, Vector::new(0.0, 200.0), 1.0)),
        ),
        Phase::new(
            std::f64::consts::PI * 2.0,
            Box::new(CircleTrajectory::new(
                Circle::new(0.0, -200.0, 200.0),
                offset + std::f64::consts::PI / 2.0,
                rotation_speed,
            )),
            Box::new(CircleEmitter::new(0.15, 2, 200.0)),
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
        Box::new(CircleEmitter::new(0.15, 2, 200.0)),
    ));
    stages.push(Phase::new_jump(
        3.0,
        Box::new(StayTrajectory::new(*points.last().unwrap())),
        Box::new(CircleEmitter::new(0.2, 6, 300.0)),
        1,
    ));

    Enemy::new(Circle::new(0.0, 0.0, 30.0), 30.0, stages)
}
