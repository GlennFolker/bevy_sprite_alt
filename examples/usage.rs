use bevy::{
    prelude::*,

    asset::{
        LoadState,
        HandleId, AssetPath
    },
    render::texture::ImageSettings,
    window::{
        WindowMode, PresentMode
    }
};

use bevy_sprite_alt::prelude::*;

use std::path::Path;

fn main() {
    App::new()
        .init_resource::<Sprites>()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "'le atlas".to_string(),

            width: 800.0,
            height: 600.0,
            resizable: false,

            position: WindowPosition::Centered(MonitorSelection::Primary),
            mode: WindowMode::Windowed,
            present_mode: PresentMode::AutoNoVsync,

            ..default()
        })
        .add_state(AppState::Setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(SpritePlugin)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup))
        .run();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished
}

#[derive(Deref, DerefMut, Default)]
struct Sprites(Vec<Handle<Image>>);
#[derive(Deref, DerefMut)]
struct Atlas(Handle<TextureAtlas>);

fn load_textures(mut sprites: ResMut<Sprites>, asset_server: Res<AssetServer>) {
    **sprites = vec![
        asset_server.load("sprites/lime.png"),
        asset_server.load("sprites/orange.png"),
        asset_server.load("sprites/pink.png"),
        asset_server.load("sprites/purple.png"),
        asset_server.load("sprites/red.png")
    ];
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprites: ResMut<Sprites>,
    asset_server: Res<AssetServer>
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(sprites.iter().map(|handle| handle.id)) {
        state.set(AppState::Finished).unwrap();
    }
}

fn setup(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut textures: ResMut<Assets<Image>>, mut atlases: ResMut<Assets<TextureAtlas>>
) {
    let mut builder = TextureAtlasBuilder::default();
    for sprite in &**sprites {
        builder.add(sprite.clone_weak(), textures.get(&sprite).unwrap());
    }

    let atlas = builder.finish(&mut textures).unwrap();
    commands.remove_resource::<Sprites>();

    info!(
        "{:?}\n{:?}\n{:?}",
        atlas.pages, atlas.regions, atlas.mappings
    );

    info!(
        "{:?}",
        atlas.get_region(atlas.get_texture_index(
            &Handle::weak(HandleId::AssetPathId(AssetPath::new_ref(
                Path::new("sprites/lime.png"), None
            ).get_id()))
        ).unwrap()).unwrap()
    );

    let atlas = atlases.add(atlas);
}
