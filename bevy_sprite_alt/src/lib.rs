pub mod prelude {
    pub use crate::{
        SpritePlugin,
        Sprite,
        TextureAtlas, AtlasRegion, AtlasSprite,
        TextureAtlasBuilder,
        SpriteBundle, AtlasSpriteBundle,
        Rect
    };
}

pub mod render;

mod bundle;
mod rect;
mod sprite;
mod texture_atlas;
mod texture_atlas_builder;

pub use bundle::*;
pub use rect::*;
pub use sprite::*;
pub use texture_atlas::*;
pub use texture_atlas_builder::*;

use bevy_app::prelude::*;
use bevy_asset::prelude::*;

pub struct SpritePlugin;
impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TextureAtlas>();
    }
}
