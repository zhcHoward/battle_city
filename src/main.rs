use bevy::{
    math::const_vec3,
    // sprite::collide_aabb::{collide, Collision},
    prelude::*,
    render::pass::ClearColor,
};

fn main() {
    let mut app = App::build();
    app.add_resource(WindowDescriptor {
        title: "Snake!".to_string(),
        width: GAME_WIDTH as f32,
        height: GAME_HEIGHT as f32,
        ..Default::default()
    })
    .add_resource(ClearColor)
    .add_startup_system(setup.system())
    .add_startup_stage("game_setup", SystemStage::single(spawn_tank.system()))
    .add_system(tank_movement.system())
    // .add_system(animate_sprite_system.system())
    .add_plugins(DefaultPlugins)
    .run();
}

struct Tank {
    direction: Direction,
}
struct Materials {
    tank: Handle<ColorMaterial>,
}

struct Textures {
    texture: Handle<TextureAtlas>,
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world
    let texture_handle = asset_server.load("General Sprites.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 25, 16);
    let texture_handle = texture_atlases.add(texture_atlas);

    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .insert_resource(Materials {
            tank: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        })
        .insert_resource(Textures {
            texture: texture_handle,
        });
    // .spawn(CameraUiBundle::default());
}

// fn animate_sprite_system(
//     time: Res<Time>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
// ) {
//     for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
//         timer.tick(time.delta_seconds());
//         if timer.finished() {
//             // let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
//             if sprite.index % 2 == 0 {
//                 sprite.index += 1;
//             } else {
//                 sprite.index -= 1;
//             }
//             // sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
//         }
//     }
// }

fn spawn_tank(commands: &mut Commands, textures: Res<Textures>) {
    commands
        .spawn(SpriteSheetBundle {
            // sprite: TextureAtlasSprite::new(200),  // P2
            texture_atlas: textures.texture.clone(),
            transform: Transform {
                translation: TANK1_SPAWN_POSITION,
                scale: Vec3::splat(SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Tank {
            direction: Direction::Up,
        })
        .with(Timer::from_seconds(0.1, true));
}

const BOUNDARY: f32 = GAME_WIDTH / 2. - BLOCK;

fn tank_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut tank_positions: Query<&mut Transform, With<Tank>>,
) {
    for mut transform in tank_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) && transform.translation.x > -BOUNDARY {
            transform.translation.x -= TANK_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Right) && transform.translation.x < BOUNDARY {
            transform.translation.x += TANK_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Down) && transform.translation.y > -BOUNDARY {
            transform.translation.y -= TANK_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Up) && transform.translation.y < BOUNDARY {
            transform.translation.y += TANK_SPEED;
        }
    }
}

const SCALE: f32 = 2.;
const TANK_SPEED: f32 = MIN_BLOCK / 2.;
const MIN_BLOCK: f32 = 4. * SCALE; // unit: px
const BLOCK: f32 = 2. * MIN_BLOCK;
const MAX_BLOCK: f32 = 2. * BLOCK; // tank's size
const GAME_WIDTH: f32 = 13. * MAX_BLOCK;
const GAME_HEIGHT: f32 = GAME_WIDTH;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
const TANK1_SPAWN_POSITION: Vec3 =
    const_vec3!([-2. * MAX_BLOCK, (MAX_BLOCK - GAME_WIDTH) / 2., 0.]);
