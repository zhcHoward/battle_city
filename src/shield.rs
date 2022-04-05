use crate::texture::SpriteIndex;
use bevy::prelude::*;

#[derive(Component)]
pub struct Shield;

pub fn spawn(commands: &mut Commands, tank: Entity, texture: Handle<TextureAtlas>) {
    let shield = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::SHIELD[0]),
            texture_atlas: texture,
            ..Default::default()
        })
        .insert(Shield)
        .insert(Timer::from_seconds(0.05, true))
        .id();
    commands.entity(tank).push_children(&[shield]);
}

pub fn animation(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Timer), With<Shield>>,
) {
    for (mut sprite, mut timer) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            if sprite.index % 2 == 0 {
                sprite.index += 1;
            } else {
                sprite.index -= 1;
            }
        }
    }
}
