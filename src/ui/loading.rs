use bevy::prelude::*;

pub struct LoadingBackgroundImage {
    image_handle: Handle<Image>,
}

#[derive(Component)]
pub struct LoadingBackground;

pub fn loading_background_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_handle = asset_server.load("img/background/loading_bg.png");
    commands.insert_resource(LoadingBackgroundImage { image_handle });
}

pub fn loading_background_setup(mut commands: Commands, background: Res<LoadingBackgroundImage>) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..default()
            },
            texture: background.image_handle.clone_weak(),
            ..default()
        })
        .insert(LoadingBackground);
}
