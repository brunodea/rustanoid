use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    prelude::{SimpleState, StateData, GameData, World, WorldExt, Builder},
    ecs::prelude::{Component, DenseVecStorage},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub struct Paddle {
    pub width: f32,
    pub height: f32,
}

impl Default for Paddle {
    fn default() -> Self {
        Paddle {
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_paddle(world: &mut World) {
    let mut paddle_transform = Transform::default();
    // middle-bottom
    paddle_transform.set_translation_xyz((ARENA_WIDTH*0.5)-(PADDLE_WIDTH*0.5), 0f32, 0f32);

    world
        .create_entity()
        .with(Paddle::default())
        .with(paddle_transform)
        .build();
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left. 
    let mut camera_transform = Transform::default();
    camera_transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(camera_transform)
        .build();
}

pub struct Rustanoid;

impl SimpleState for Rustanoid {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Paddle>();

        initialise_camera(world);
        initialise_paddle(world);
    }
}
