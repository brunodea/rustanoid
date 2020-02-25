use amethyst::{
    core::timing::Time,
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::rustanoid::Ball;

#[derive(SystemDesc)]
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ball>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, balls, time): Self::SystemData) {
        for (ball, transform) in (&balls, &mut transforms).join() {
            transform.prepend_translation_x(ball.velocity[0] * time.delta_seconds());
            transform.prepend_translation_y(ball.velocity[1] * time.delta_seconds());
        }
    }
}
