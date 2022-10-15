
use bevy::prelude::*;

const RECICLE_BIN_POS: Vec2 = Vec2::new(60.0, 670.0);
const FOLDERS_LAYER: f32 = 1.0;
const FOLDERS_SPACING: f32 = 90.0;

#[derive(Component)]
struct DesktopBackgruound;

#[derive(Component)]
struct Folder;

#[derive(Component)]
struct RecycleBin;

pub fn spawn_desktop_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..default()
        },
        texture: asset_server.load("desktop.png"),
        ..default()
    })
    .insert(DesktopBackgruound {})
    .insert(Name::new("DesktopBackground"));
}

pub fn spawn_folders(mut commands: Commands, asset_server: Res<AssetServer>) {
    let x = RECICLE_BIN_POS.x;
    let y = RECICLE_BIN_POS.y - FOLDERS_SPACING;

    for i in 0..5 {
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("folder.png"),
            transform: Transform::from_xyz(x, y - (FOLDERS_SPACING * i as f32), FOLDERS_LAYER),
            ..default()
        }).with_children(|folder| {
            folder.spawn_bundle(Text2dBundle {
                transform: Transform::from_xyz(0.0, -52.0, FOLDERS_LAYER),
                text: Text::from_section(format!("Folder {}", i), TextStyle { 
                    font_size: 18.0,
                    font: asset_server.load("fonts/segoe_ui.ttf"),
                    color: Color::WHITE,
                })
                .with_alignment(TextAlignment::BOTTOM_CENTER),
                ..default()
            });
        })
        .insert(Folder {})
        .insert(Name::new(format!("Folder {}", i)));
    }
}

pub fn spawn_recycle_bin(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(RecycleBin {})
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("recycle-bin.png"),
            transform: Transform::from_xyz(RECICLE_BIN_POS.x, RECICLE_BIN_POS.y, FOLDERS_LAYER),
            sprite: Sprite {
                custom_size: Some(Vec2::new(60.0, 60.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|folder| {
            folder.spawn_bundle(Text2dBundle {
                transform: Transform::from_xyz(0.0, -52.0, FOLDERS_LAYER),
                text: Text::from_section("Recycle Bin", TextStyle { 
                    font_size: 18.0,
                    font: asset_server.load("fonts/segoe_ui.ttf"),
                    color: Color::WHITE,
                })
                .with_alignment(TextAlignment::BOTTOM_CENTER),
                ..default()
            });
        })
        .insert(Name::new("Recycle Bin"));
}