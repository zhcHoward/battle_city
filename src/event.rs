use crate::{explosion, tank::Tank, texture::Textures, utils::Owner};
use bevy::prelude::*;

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
