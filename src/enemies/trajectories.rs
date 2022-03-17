use crate::{
    geometry::{Circle, Vector},
};

use super::Trajectory;

pub struct CircleTrajectory {
    pub trajectory: Circle,
    pub timer_offset: f64,
    pub rotation_speed: f64,
}

impl CircleTrajectory {
    pub fn new(trajectory: Circle, timer_offset: f64, rotation_speed: f64) -> Self {
        Self {
            trajectory,
            timer_offset,
            rotation_speed,
        }
    }
}

impl Trajectory for CircleTrajectory {
    fn location(&self, time: f64) -> Vector {
        let t = time * self.rotation_speed + self.timer_offset;
        self.trajectory.coord + Vector::new(t.sin(), t.cos()) * self.trajectory.r
    }
}

pub struct StayTrajectory {
    pub location: Vector,
}

impl StayTrajectory {
    pub fn new(location: Vector) -> Self {
        Self {location
        }
    }
}

impl Trajectory for StayTrajectory {
    fn location(&self, _time: f64) -> Vector {
        self.location
    }
}

pub struct FromToTrajectory {
    pub from: Vector,
    pub speed: Vector,
}

impl FromToTrajectory {
    pub fn new(from: Vector, to: Vector, time: f64) -> Self {
        Self {
            from,
            speed: (to - from) * (1.0 / time),
        }
    }
}

impl Trajectory for FromToTrajectory {
    fn location(&self, time: f64) -> Vector {
        self.from + self.speed * time
    }
}