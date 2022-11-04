use bevy_asset::*;
use bevy_log::error;
use bevy_math::prelude::*;
use bevy_render::{
    prelude::*,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
    texture::TextureFormatPixelInfo,
};
use bevy_utils::HashMap;

use rectangle_pack::{
    contains_smallest_box, pack_rects, volume_heuristic, GroupedRectsToPlace, PackedLocation,
    RectToInsert, RectanglePackError, TargetBin,
};
use thiserror::Error;

use std::collections::BTreeMap;

use crate::prelude::*;

#[derive(Debug, Error)]
pub enum TextureAtlasBuilderError {
    #[error("could not pack textures into an atlas within the given bounds")]
    NotEnoughSpace,
}

#[derive(Debug)]
pub struct TextureAtlasBuilder {
    pub rects_to_place: GroupedRectsToPlace<Handle<Image>>,
    pub initial_size: Vec2,
    pub max_size: Vec2,
    pub format: TextureFormat,
}

impl Default for TextureAtlasBuilder {
    fn default() -> Self {
        Self {
            rects_to_place: GroupedRectsToPlace::new(),
            initial_size: Vec2::new(256., 256.),
            max_size: Vec2::new(2048., 2048.),
            format: TextureFormat::Rgba8UnormSrgb,
        }
    }
}

pub type TextureAtlasBuilderResult = Result<TextureAtlas, TextureAtlasBuilderError>;

impl TextureAtlasBuilder {
    pub fn add(&mut self, handle: Handle<Image>, texture: &Image) {
        self.rects_to_place.push_rect(
            handle,
            None,
            RectToInsert::new(
                texture.texture_descriptor.size.width,
                texture.texture_descriptor.size.height,
                1,
            ),
        );
    }

    pub fn finish(&self, textures: &mut Assets<Image>) -> TextureAtlasBuilderResult {
        let init_w = self.initial_size.x as u32;
        let init_h = self.initial_size.y as u32;
        let max_w = self.max_size.x as u32;
        let max_h = self.max_size.y as u32;

        let mut cur_w = max_w;
        let mut cur_h = max_h;
        let mut placements = None;
        let mut pages = Vec::new();
        let mut bins = BTreeMap::new();

        while placements.is_none() {
            if cur_w >= max_w || cur_h >= max_h {
                cur_w = init_w;
                cur_h = init_h;

                pages.push((cur_w, cur_h));
                bins.insert(0, TargetBin::new(cur_w, cur_h, 1));
            }

            placements = match pack_rects(
                &self.rects_to_place,
                &mut bins,
                &volume_heuristic,
                &contains_smallest_box,
            ) {
                Ok(placements) => Some(placements),
                Err(RectanglePackError::NotEnoughBinSpace) => {
                    cur_w = (cur_w * 2).clamp(0, max_w);
                    cur_h = (cur_h * 2).clamp(0, max_h);

                    let index = pages.len();
                    pages[index] = (cur_w, cur_h);
                    bins.insert(index, TargetBin::new(cur_w, cur_h, 1));

                    None
                }
            }
        }

        let placements = placements.ok_or(TextureAtlasBuilderError::NotEnoughSpace)?;
        let mut pages: Vec<Image> = pages
            .iter()
            .map(|(w, h)| {
                Image::new(
                    Extent3d {
                        width: *w,
                        height: *h,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    vec![0; self.format.pixel_size() * (*w * *h) as usize],
                    self.format,
                )
            })
            .collect();

        let mut regions = Vec::with_capacity(placements.packed_locations().len());
        let mut mappings = HashMap::default();
        for (handle, (page, loc)) in placements.packed_locations() {
            let texture = textures.get(handle).unwrap();
            let min = Vec2::new(loc.x() as f32, loc.y() as f32);
            let max = min + Vec2::new(loc.width() as f32, loc.height() as f32);

            mappings.insert(handle.clone_weak(), regions.len());
            regions.push(AtlasRegion {
                page_index: *page,
                rect: Rect { min, max },
            });

            Self::copy_texture(&mut pages[*page], texture, self.format, loc);
        }

        let mut handles = Vec::with_capacity(pages.len());
        for page in pages {
            handles.push(textures.add(page));
        }

        Ok(TextureAtlas {
            pages: handles,
            regions,
            mappings,
        })
    }

    fn copy_texture(
        page: &mut Image,
        texture: &Image,
        format: TextureFormat,
        loc: &PackedLocation,
    ) {
        if format == texture.texture_descriptor.format {
            Self::copy_texture_impl(page, texture, loc);
        } else if let Some(conv) = texture.convert(format) {
            Self::copy_texture_impl(page, &conv, loc);
        } else {
            error!(
                "Error converting texture from '{:?}' to '{:?}', ignoring",
                texture.texture_descriptor.format, format
            );
        };
    }

    fn copy_texture_impl(page: &mut Image, texture: &Image, loc: &PackedLocation) {
        let rect_width = loc.width() as usize;
        let rect_height = loc.height() as usize;
        let rect_x = loc.x() as usize;
        let rect_y = loc.y() as usize;
        let atlas_width = page.texture_descriptor.size.width as usize;
        let format_size = page.texture_descriptor.format.pixel_size();

        for (texture_y, bound_y) in (rect_y..rect_y + rect_height).enumerate() {
            let begin = (bound_y * atlas_width + rect_x) * format_size;
            let end = begin + rect_width * format_size;
            let texture_begin = texture_y * rect_width * format_size;
            let texture_end = texture_begin + rect_width * format_size;
            page.data[begin..end].copy_from_slice(&texture.data[texture_begin..texture_end]);
        }
    }
}
