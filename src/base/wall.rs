use bevy::{math::const_vec3, prelude::*};

use crate::{
    brick,
    collision::Collider,
    consts::{MIN_BLOCK_WIDTH, SCALE},
    game_data::GameData,
    iron, power_up, state,
    texture::{SpriteIndex, Textures},
    utils::Owner,
};

pub const BASE_WALL_POSITIONS: [Vec3; 8] = [
    const_vec3!([-5. * MIN_BLOCK_WIDTH, -25. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([-5. * MIN_BLOCK_WIDTH, -23. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([-5. * MIN_BLOCK_WIDTH, -21. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([-3. * MIN_BLOCK_WIDTH, -21. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([-MIN_BLOCK_WIDTH, -21. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([MIN_BLOCK_WIDTH, -21. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([MIN_BLOCK_WIDTH, -23. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([MIN_BLOCK_WIDTH, -25. * MIN_BLOCK_WIDTH, 0.]),
];

#[derive(Component)]
pub struct BaseWall;

pub enum BaseWallType {
    Iron,
    Brick,
}

pub fn spawn(commands: &mut Commands, texture: Handle<TextureAtlas>, wtype: BaseWallType) {
    match wtype {
        BaseWallType::Iron => {
            for pos in &BASE_WALL_POSITIONS {
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(SpriteIndex::IRON[0]),
                        texture_atlas: texture.clone(),
                        transform: Transform {
                            translation: *pos,
                            scale: Vec3::splat(SCALE),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(iron::Iron)
                    .insert(Collider::Iron)
                    .insert(state::State::Iron)
                    .insert(BaseWall);
            }
        }
        BaseWallType::Brick => {
            for pos in &BASE_WALL_POSITIONS {
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(SpriteIndex::BRICK[1]),
                        texture_atlas: texture.clone(),
                        transform: Transform {
                            translation: *pos,
                            scale: Vec3::splat(SCALE),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(brick::Brick)
                    .insert(Collider::Brick)
                    .insert(state::State::Brick(brick::State {
                        b_type: brick::BrickType::QuarterBrick,
                    }))
                    .insert(BaseWall);
            }
        }
    }
}

pub struct ChangeBaseWall {
    pub by: Owner,
}

pub fn handle_change_base_wall(
    mut commands: Commands,
    mut event_reader: EventReader<ChangeBaseWall>,
    query: Query<Entity, With<BaseWall>>,
    textures: Res<Textures>,
    mut game_data: ResMut<GameData>,
) {
    let texture = &textures.texture;
    for event in event_reader.iter() {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        if event.by != Owner::AI {
            spawn(&mut commands, texture.clone(), BaseWallType::Iron);
        }

        game_data.base_wall_changed = true;
        game_data.base_wall_changed_by = event.by;
        break;
    }
}

pub fn change_basewall_count_down(
    mut commands: Commands,
    query: Query<Entity, With<BaseWall>>,
    time: Res<Time>,
    textures: Res<Textures>,
    mut game_data: ResMut<GameData>,
) {
    if !game_data.base_wall_changed {
        return;
    }

    let texture = &textures.texture;
    let timer = game_data.restore_timer.tick(time.delta());
    if timer.finished() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        spawn(&mut commands, texture.clone(), BaseWallType::Brick);
        game_data.base_wall_changed = false;
        game_data.base_wall_normal = true;
        return;
    }

    let left = timer.duration() - timer.elapsed();
    if left <= power_up::BLINK_DURATION {
        if game_data.blink_timer.tick(time.delta()).just_finished() {
            if game_data.base_wall_normal {
                for entity in query.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                if game_data.base_wall_changed_by == Owner::AI {
                    unimplemented!();
                } else {
                    spawn(&mut commands, texture.clone(), BaseWallType::Iron);
                }
            } else {
                for entity in query.iter() {
                    commands.entity(entity).despawn();
                }

                spawn(&mut commands, texture.clone(), BaseWallType::Brick);
            }
            game_data.base_wall_normal = !game_data.base_wall_normal;
        }
    }
}
