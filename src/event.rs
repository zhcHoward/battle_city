use crate::{
    base::Base,
    bullet::Bullet,
    consts::{BLOCK_WIDTH, MIN_BLOCK_WIDTH},
    explosion,
    game_data::GameData,
    iron,
    shield::Shield,
    state::State,
    tank::Tank,
    texture::Textures,
    utils::Owner,
};
use bevy::{
    math::{const_vec2, const_vec3},
    prelude::*,
};

pub struct DestroyAllEnemies {
    pub by: Owner,
}

pub fn handle_destroy_all_enemies(
    mut commands: Commands,
    mut event_reader: EventReader<DestroyAllEnemies>,
    query: Query<(Entity, &State, &Transform), With<Tank>>,
    textures: Res<Textures>,
) {
    for event in event_reader.iter() {
        for (entity, state, transform) in query.iter() {
            let tank = state.as_tank();
            if event.by.is_enemy(tank.owner) {
                commands.entity(entity).despawn_recursive(); // in case tank has sub entity, like a shield
                explosion::spawn(
                    &mut commands,
                    textures.texture.clone(),
                    transform.translation,
                    true,
                )
            }
        }

        // It is possible that player's tank and AI's tank trigger DestroyAllEnemies event in the same frame.
        // So, to destroy AI's tank or player's tank depends on whose event comes first.
        // The other event is simply ignored.
        break;
    }
}

pub struct ChangeBaseWall {
    pub by: Owner,
}

pub const BaseWallMin: Vec2 = const_vec2!([-2. * BLOCK_WIDTH, -6.5 * BLOCK_WIDTH]);
pub const BaseWallMax: Vec2 = const_vec2!([BLOCK_WIDTH, -4.5 * BLOCK_WIDTH]);
pub const BaseWallPositions: [Vec3; 8] = [
    const_vec3!([-5. * MIN_BLOCK_WIDTH, -25. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([-5. * MIN_BLOCK_WIDTH, -23. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([-5. * MIN_BLOCK_WIDTH, -21. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([-3. * MIN_BLOCK_WIDTH, -21. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([-MIN_BLOCK_WIDTH, -21. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([MIN_BLOCK_WIDTH, -21. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([MIN_BLOCK_WIDTH, -23. * MIN_BLOCK_WIDTH, 0.]),
    const_vec3!([MIN_BLOCK_WIDTH, -25. * MIN_BLOCK_WIDTH, 0.]),
];

pub fn handle_change_base_wall(
    mut commands: Commands,
    mut event_reader: EventReader<ChangeBaseWall>,
    query: Query<
        (Entity, &Transform),
        (
            Without<Tank>,
            Without<Bullet>,
            Without<Base>,
            Without<Shield>,
        ),
    >,
    textures: Res<Textures>,
    mut game_data: ResMut<GameData>,
) {
    let texture = &textures.texture;
    let mut by_ai = true;
    for event in event_reader.iter() {
        for (entity, transform) in query.iter() {
            let pos = transform.translation.truncate();
            if pos.cmpgt(BaseWallMin).all() && pos.cmplt(BaseWallMax).all() {
                commands.entity(entity).despawn_recursive();
            }
        }
        if event.by != Owner::AI {
            by_ai = false;
            for pos in &BaseWallPositions {
                iron::spawn(
                    &mut commands,
                    *pos,
                    texture.clone(),
                    iron::IronType::QuarterIron,
                );
            }
        }
        break;
    }
    game_data.base_changed = true;
    game_data.base_wall_hidden = by_ai;
}
