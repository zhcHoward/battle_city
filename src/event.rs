use crate::{
    base::Base,
    bullet::Bullet,
    consts::{BLOCK_WIDTH, MIN_BLOCK_WIDTH},
    explosion, iron,
    shield::Shield,
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
    commands: &mut Commands,
    mut event_reader: Local<EventReader<DestroyAllEnemies>>,
    events: Res<Events<DestroyAllEnemies>>,
    query: Query<(Entity, &Tank, &Transform)>,
    textures: Res<Textures>,
) {
    for event in event_reader.iter(&events) {
        for (entity, tank, transform) in query.iter() {
            if event.by.is_enemy(tank.owner) {
                commands.despawn_recursive(entity); // in case tank has sub entity, like a shield
                explosion::spawn(
                    commands,
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

const BaseWallMin: Vec2 = const_vec2!([-2. * BLOCK_WIDTH, -6.5 * BLOCK_WIDTH]);
const BaseWallMax: Vec2 = const_vec2!([BLOCK_WIDTH, -4.5 * BLOCK_WIDTH]);
const BaseWallPositions: [Vec3; 8] = [
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
    commands: &mut Commands,
    mut event_reader: Local<EventReader<ChangeBaseWall>>,
    events: Res<Events<ChangeBaseWall>>,
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
) {
    let texture = &textures.texture;
    for event in event_reader.iter(&events) {
        for (entity, transform) in query.iter() {
            let pos = transform.translation.truncate();
            if pos.cmpgt(BaseWallMin).all() && pos.cmplt(BaseWallMax).all() {
                commands.despawn_recursive(entity);
            }
        }
        if event.by != Owner::AI {
            for pos in &BaseWallPositions {
                iron::spawn(commands, *pos, texture.clone(), iron::IronType::QuarterIron);
            }
        }
        break;
    }
}
