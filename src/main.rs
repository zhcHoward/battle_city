use bevy::{
    prelude::*,
    render::pass::ClearColor,
    // sprite::collide_aabb::{collide, Collision},
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
    .add_system(position_translation.system())
    .add_plugins(DefaultPlugins)
    .run();
}

struct Tank {
    direction: Direction,
}
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
            sprite: Sprite::new(Vec2::new(FULL_BLOCK as f32, FULL_BLOCK as f32)),
            ..Default::default()
        })
        .with(Tank {
            direction: Direction::Up,
        })
        .with(Position { x: 4, y: 0 });
}

const BOUNDARY: (u32, u32, u32, u32) = (12, 12, 0, 0); // up, right, down, left

fn tank_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut tank_positions: Query<&mut Position, With<Tank>>,
) {
    for mut pos in tank_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) && pos.x as u32 > BOUNDARY.3 {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) && (pos.x as u32) < BOUNDARY.1 {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) && pos.y as u32 > BOUNDARY.2 {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::Up) && (pos.y as u32) < BOUNDARY.0 {
            pos.y += 1;
        }
    }
}

const BLOCK: u32 = 20; // the smallest block in battle city(unit: px)
const FULL_BLOCK: u32 = 2 * BLOCK;
const GAME_WIDTH: u32 = 13 * 2 * BLOCK;
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

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    // fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    //     let tile_size = bound_window / bound_game;
    //     pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    // }
    // let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            ((pos.x - 6) * FULL_BLOCK as i32) as f32,
            ((pos.y - 6) * FULL_BLOCK as i32) as f32,
            0.,
        );
    }
}
