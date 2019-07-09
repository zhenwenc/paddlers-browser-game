use crate::game::resources::Dt;
use quicksilver::geom::{Vector, Rectangle};
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub area: Rectangle
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub start: Vector,
    pub velocity: Vector,
}


pub struct MoveSystem;
impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        Read<'a, Dt>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, (dt, vel, mut pos): Self::SystemData) {
        let dt = dt.0;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.area.pos = vel.start + vel.velocity * dt as f32;
        }
    }

}

impl From<(Vector, Vector)> for Position {
    fn from(t: (Vector, Vector)) -> Self {
        Position {
            area: Rectangle::new(t.0, t.1),
        }
    }
}

impl Velocity {
    pub fn new (start_pos: impl Into<Vector>, vel: impl Into<Vector>) -> Self {
        Velocity {
            start: start_pos.into(),
            velocity: vel.into(),
        }
    }
}