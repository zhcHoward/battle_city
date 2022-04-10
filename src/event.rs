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
