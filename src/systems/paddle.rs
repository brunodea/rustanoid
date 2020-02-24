use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::rustanoid::{Paddle};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (_, transform) in (&paddles, &mut transforms).join() {
            let movement = input.axis_value("paddle");
            if let Some(mv_amount) = movement {
                // TODO(brunor): use delta time from amethyst::core::timing::Time for the scale
                // instead of a fixed value, in order for it to be independent from the game's
                // framerate.
                let scaled_amount = 1.2*mv_amount as f32;
                transform.prepend_translation_x(scaled_amount);
            }
        }
    }
}
