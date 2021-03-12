use bevy::{
    prelude::*,
    render::pass::ClearColor,
    // sprite::collide_aabb::{collide, Collision},
};

fn main() {
    let mut app = App::build();
    app.add_resource(ClearColor)
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_tank.system()))
        .add_system(tank_movement.system())
        .add_plugins(DefaultPlugins)
        .run();
}

struct Tank;
struct Materials {
    tank: Handle<ColorMaterial>,
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    // Add the game's entities to our world
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .insert_resource(Materials {
            tank: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        });
    // .spawn(CameraUiBundle::default());
}

fn spawn_tank(commands: &mut Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteBundle {
            material: materials.tank.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Tank);
}

fn tank_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<Tank>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
    }
}

const CELL: u32 = 50; // the smallest block in battle city(unit: px)
const GAME_WIDTH: u32 = 13 * 2 * CELL;
const GAME_HEIGHT: u32 = GAME_WIDTH;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

