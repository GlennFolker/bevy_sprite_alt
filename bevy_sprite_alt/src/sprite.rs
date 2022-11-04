use bevy_ecs::prelude::*;
use bevy_render::prelude::*;

use crate::prelude::*;

#[derive(Component, Debug, Default, Clone)]
pub struct Sprite {
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub rect: Option<Rect>,
}
