use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::TypeUuid;
use bevy_render::prelude::*;
use bevy_utils::HashMap;

use crate::prelude::*;

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "7233c597-ccfa-411f-bd59-9af349432ada"]
pub struct TextureAtlas {
    pub pages: Vec<Handle<Image>>,
    pub regions: Vec<AtlasRegion>,
    pub mappings: HashMap<Handle<Image>, usize>
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AtlasRegion {
    pub page_index: usize,
    pub rect: Rect
}

#[derive(Component, Debug, Default, Clone)]
pub struct AtlasSprite {
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub index: usize
}

impl TextureAtlas {
    pub fn get_region(&self, index: usize) -> Option<AtlasRegion> {
        if index >= self.regions.len() {
            None
        } else {
            Some(self.regions[index])
        }
    }

    pub fn get_texture_index(&self, key: &Handle<Image>) -> Option<usize> {
        self.mappings.get(key).copied()
    }
}
