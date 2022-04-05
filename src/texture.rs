use bevy::{
    asset::{AssetServer, Handle},
    math::Vec2,
    prelude::Res,
    sprite::{Rect, TextureAtlas},
};

const TEXTURE_WIDTH: f32 = 16.;
const HALF_TEXTURE_WIDTH: f32 = TEXTURE_WIDTH / 2.;

pub struct SpriteIndex;
impl SpriteIndex {
    pub const BASE: [usize; 2] = [256, 257];
    pub const BRICK: [usize; 8] = [258, 259, 260, 261, 262, 263, 264, 265];
    pub const IRON: [usize; 1] = [266];
    pub const GRASS: [usize; 1] = [267];
    pub const SNOW: [usize; 1] = [268];
    pub const RIVER: [usize; 3] = [269, 270, 271];
    pub const STAR: [usize; 4] = [272, 273, 274, 275];
    pub const BULLET: [usize; 4] = [276, 277, 278, 279];
    pub const POWER_UP: [usize; 7] = [280, 281, 282, 283, 284, 285, 286];
    pub const EXPLOSION: [usize; 5] = [287, 288, 289, 290, 291];
    pub const SHIELD: [usize; 2] = [292, 293];
}

pub struct Textures {
    pub texture: Handle<TextureAtlas>,
}

pub fn load_texture_atlas(asset_server: Res<AssetServer>) -> TextureAtlas {
    let texture_handle = asset_server.load("General Sprites.png");
    let mut sprites = Vec::new();
    // load tank with different color in order: yellow, white, green, red, 16x16px
    // sprite index 0 ~ 255
    for y in 0..16 {
        for x in 0..16 {
            sprites.push(Rect {
                min: Vec2::new(TEXTURE_WIDTH * x as f32, TEXTURE_WIDTH * y as f32),
                max: Vec2::new(
                    TEXTURE_WIDTH * (x + 1) as f32,
                    TEXTURE_WIDTH * (y + 1) as f32,
                ),
            });
        }
    }
    // base (the eagle) 16x16px
    // sprite index 256 ~ 257
    sprites.push(Rect {
        min: Vec2::new(19. * TEXTURE_WIDTH, 2. * TEXTURE_WIDTH),
        max: Vec2::new(20. * TEXTURE_WIDTH, 3. * TEXTURE_WIDTH),
    });
    sprites.push(Rect {
        min: Vec2::new(20. * TEXTURE_WIDTH, 2. * TEXTURE_WIDTH),
        max: Vec2::new(21. * TEXTURE_WIDTH, 3. * TEXTURE_WIDTH),
    });

    // load terrain
    // 16x16 brick
    // sprite index 258
    sprites.push(Rect {
        min: Vec2::new(16. * TEXTURE_WIDTH, 0.),
        max: Vec2::new(17. * TEXTURE_WIDTH, TEXTURE_WIDTH),
    });
    // 8 x 8 brick
    // sprite index 259
    sprites.push(Rect {
        min: Vec2::new(16. * TEXTURE_WIDTH, 4. * TEXTURE_WIDTH),
        max: Vec2::new(16.5 * TEXTURE_WIDTH, 4.5 * TEXTURE_WIDTH),
    });
    // 4 x 8 brick
    // top, index 260
    sprites.push(Rect {
        min: Vec2::new(18. * TEXTURE_WIDTH, 4. * TEXTURE_WIDTH),
        max: Vec2::new(18.5 * TEXTURE_WIDTH, 4.25 * TEXTURE_WIDTH),
    });
    // right, index 261
    sprites.push(Rect {
        min: Vec2::new(16.75 * TEXTURE_WIDTH, 4. * TEXTURE_WIDTH),
        max: Vec2::new(17. * TEXTURE_WIDTH, 4.5 * TEXTURE_WIDTH),
    });
    // bottom, index 262
    sprites.push(Rect {
        min: Vec2::new(17. * TEXTURE_WIDTH, 4.25 * TEXTURE_WIDTH),
        max: Vec2::new(17.5 * TEXTURE_WIDTH, 4.5 * TEXTURE_WIDTH),
    });
    // left, index 263
    sprites.push(Rect {
        min: Vec2::new(17.5 * TEXTURE_WIDTH, 4. * TEXTURE_WIDTH),
        max: Vec2::new(17.75 * TEXTURE_WIDTH, 4.5 * TEXTURE_WIDTH),
    });
    // 4 x 4 brick
    // sprite index 264 ~ 265
    sprites.push(Rect {
        min: Vec2::new(16. * TEXTURE_WIDTH, 4. * TEXTURE_WIDTH),
        max: Vec2::new(16.25 * TEXTURE_WIDTH, 4.25 * TEXTURE_WIDTH),
    });
    sprites.push(Rect {
        min: Vec2::new(16.25 * TEXTURE_WIDTH, 4. * TEXTURE_WIDTH),
        max: Vec2::new(16.5 * TEXTURE_WIDTH, 4.25 * TEXTURE_WIDTH),
    });

    // load iron, grass, snow and river, 8x8px
    // sprite index 266 ~ 271
    for y in 0..2 {
        for x in 0..3 {
            sprites.push(Rect {
                min: Vec2::new(
                    16. * TEXTURE_WIDTH + HALF_TEXTURE_WIDTH * x as f32,
                    4. * TEXTURE_WIDTH + HALF_TEXTURE_WIDTH * (y + 1) as f32,
                ),
                max: Vec2::new(
                    16. * TEXTURE_WIDTH + HALF_TEXTURE_WIDTH * (x + 1) as f32,
                    4. * TEXTURE_WIDTH + HALF_TEXTURE_WIDTH * (y + 2) as f32,
                ),
            });
        }
    }

    // load star which twinkles when spawning player's tank
    // sprite index 272 ~ 275
    for i in 0..4 {
        sprites.push(Rect {
            min: Vec2::new((16 + i) as f32 * TEXTURE_WIDTH, 6. * TEXTURE_WIDTH),
            max: Vec2::new((17 + i) as f32 * TEXTURE_WIDTH, 7. * TEXTURE_WIDTH),
        });
    }

    // load bullets 4x4px
    // sprite index 276 ~ 279
    sprites.push(Rect {
        min: Vec2::new(323., 102.),
        max: Vec2::new(327., 106.),
    });
    sprites.push(Rect {
        min: Vec2::new(330., 101.),
        max: Vec2::new(334., 105.),
    });
    sprites.push(Rect {
        min: Vec2::new(338., 102.),
        max: Vec2::new(342., 106.),
    });
    sprites.push(Rect {
        min: Vec2::new(346., 102.),
        max: Vec2::new(350., 106.),
    });

    // load power ups
    // sprite index 280 ~ 286
    for i in 0..7 {
        sprites.push(Rect {
            min: Vec2::new((16 + i) as f32 * TEXTURE_WIDTH, 7. * TEXTURE_WIDTH),
            max: Vec2::new((17 + i) as f32 * TEXTURE_WIDTH, 8. * TEXTURE_WIDTH),
        })
    }

    // load tank exploded effect
    // sprite index 287 ~ 289
    for i in 0..3 {
        sprites.push(Rect {
            min: Vec2::new((16 + i) as f32 * TEXTURE_WIDTH, 8. * TEXTURE_WIDTH),
            max: Vec2::new((17 + i) as f32 * TEXTURE_WIDTH, 9. * TEXTURE_WIDTH),
        })
    }

    // load 2 large explosion (32 x 32)
    // sprite index 290 ~ 291
    sprites.push(Rect {
        min: Vec2::new(304., 128.),
        max: Vec2::new(336., 160.),
    });
    sprites.push(Rect {
        min: Vec2::new(336., 128.),
        max: Vec2::new(368., 160.),
    });

    // load tank shield
    // sprite index 292 ~ 293
    for i in 0..2 {
        sprites.push(Rect {
            min: Vec2::new((16 + i) as f32 * TEXTURE_WIDTH, 9. * TEXTURE_WIDTH),
            max: Vec2::new((17 + i) as f32 * TEXTURE_WIDTH, 10. * TEXTURE_WIDTH),
        })
    }

    TextureAtlas {
        size: Vec2::new(400., 256.),
        textures: sprites,
        texture: texture_handle,
        texture_handles: None,
    }
}
