use crate::texture::SpriteIndex;
use bevy::prelude::*;

pub struct Shield;

pub fn spawn(commands: &mut Commands, tank: Entity, texture: Handle<TextureAtlas>) {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SpriteIndex::SHIELD[0]),
            texture_atlas: texture,
            ..Default::default()
        })
        .with(Shield)
        .with(Timer::from_seconds(0.05, true));
    let shield = commands.current_entity().expect("entity Shield not found");
    commands.push_children(tank, &[shield]);
}

pub fn animation(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Timer), With<Shield>>,
) {
    for (mut sprite, mut timer) in query.iter_mut() {
        if timer.tick(time.delta_seconds()).just_finished() {
            if sprite.index % 2 == 0 {
                sprite.index += 1;
            } else {
                sprite.index -= 1;
            }
        }
    }
}
