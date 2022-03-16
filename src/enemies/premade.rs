use crate::geometry::{Circle, Vector};

use super::{
    bullet_emmiters::CircleEmitter,
    trajectories::{CircleTrajectory, StayTrajectory},
    Enemy, Phase,
};

pub fn enemy_1(offset: f64, rotation_speed: f64, stay_at: Vector) -> Enemy {
    Enemy::new(
        Circle::new(0.0, 0.0, 30.0),
        30.0,
        vec![
            Phase::new(
                std::f64::consts::PI * 3.0,
                Box::new(CircleTrajectory::new(
                    Circle::new(300.0, 300.0, 200.0),
                    offset + std::f64::consts::PI / 2.0,
                    rotation_speed,
                )),
                Box::new(CircleEmitter::new(0.1, 2, 200.0)),
            ),
            Phase::new(
                3.0,
                Box::new(StayTrajectory::new(stay_at)),
                Box::new(CircleEmitter::new(0.3, 6, 200.0)),
            ),
            Phase::new(
                std::f64::consts::PI * 3.0,
                Box::new(CircleTrajectory::new(
                    Circle::new(300.0, 300.0, 200.0),
                    offset + std::f64::consts::PI / 2.0,
                    -rotation_speed,
                )),
                Box::new(CircleEmitter::new(0.1, 2, 200.0)),
            ),
            Phase::new(
                3.0,
                Box::new(StayTrajectory::new(stay_at)),
                Box::new(CircleEmitter::new(0.3, 6, 300.0)),
            ),
        ],
    )
}
