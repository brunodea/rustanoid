use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::{Builder, GameData, SimpleState, StateData, World, WorldExt},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 4.0;
pub const PADDLE_WIDTH: f32 = 16.0;

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

fn initialise_paddle(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut paddle_transform = Transform::default();
    // middle-bottom
    paddle_transform.set_translation_xyz(ARENA_WIDTH * 0.5, PADDLE_HEIGHT * 0.5, 0f32);
    paddle_transform.rotate_2d(std::f32::consts::FRAC_PI_2); // pi/2 radians == 90 degrees

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(Paddle::default())
        .with(paddle_transform)
        .with(sprite_render.clone())
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
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

        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_camera(world);
        initialise_paddle(world, sprite_sheet_handle);
    }
}
