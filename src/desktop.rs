use bevy::prelude::*;

use crate::{
    aabb::AABB,
    biome::Biome,
    cursor_world_position::CursorWorldPosition,
    drag_and_drop::{DraggingState, MouseInteractionBundle},
};

const RECYCLE_BIN_POS: Vec2 = Vec2::new(60.0, 670.0);
const FOLDERS_LAYER: f32 = 1.0;
const FOLDERS_SPACING: f32 = 90.0;

#[derive(Component)]
struct DesktopBackground;

#[derive(Component)]
pub struct Folder {
    pub biome: Biome,
    pub state: Option<bool>,
    pub state_timer: Option<Timer>,
}

#[derive(Component)]
pub struct RecycleBin;

#[derive(Component)]
pub struct Frame;

pub fn spawn_desktop_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..default()
            },
            texture: asset_server.load("img/background/desktop_bg.png"),
            ..default()
        })
        .insert(DesktopBackground)
        .insert(Name::new("DesktopBackground"));
}

pub fn spawn_folders(mut commands: Commands, asset_server: Res<AssetServer>) {
    let x = RECYCLE_BIN_POS.x;
    let y = RECYCLE_BIN_POS.y - FOLDERS_SPACING;

    let mut i = 0;
    for biome in Biome::iterator() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("img/folder.png"),
                transform: Transform::from_xyz(x, y - (FOLDERS_SPACING * i as f32), FOLDERS_LAYER),
                ..default()
            })
            .with_children(|folder| {
                folder.spawn_bundle(Text2dBundle {
                    transform: Transform::from_xyz(0.0, -52.0, FOLDERS_LAYER),
                    text: Text::from_section(
                        format!("{}", biome),
                        TextStyle {
                            font_size: 18.0,
                            font: asset_server.load("fonts/segoe_ui.ttf"),
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment::BOTTOM_CENTER),
                    ..default()
                });
            })
            .insert(Folder {
                biome,
                state: None,
                state_timer: None,
            })
            .insert_bundle(MouseInteractionBundle::default())
            .insert(Name::new(format!("Folder {}", i)));
        i += 1;
    }
}

pub fn folder_state_coloring(
    mut query: Query<(&mut Folder, &mut Sprite, &AABB)>, 
    time: Res<Time>,
    dragging_state: ResMut<DraggingState>,
    cursor: Res<CursorWorldPosition>,
) {
    let cursor_position = match **cursor {
        None => return,
        Some(p) => p,
    };

    for (mut folder, mut sprite, aabb) in query.iter_mut() {
        match folder.state {
            Some(state) => {
                match folder.state_timer.as_mut() {
                    Some(timer) => {
                        if timer.tick(time.delta()).just_finished() {
                            folder.state = None;
                            folder.state_timer = None;
                        } else {
                            sprite.color = if state {
                                Color::GREEN
                            } else {
                                Color::RED
                            }
                        }
                    },
                    None => {}
                }
            },
            None => {
                sprite.color = if aabb.inside(cursor_position) && dragging_state.entity.is_some() {
                    Color::BLUE
                } else {
                    Color::WHITE
                };
            },
        }
    }
}

pub fn spawn_recycle_bin(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("img/recycle-bin.png"),
            transform: Transform::from_xyz(RECYCLE_BIN_POS.x, RECYCLE_BIN_POS.y, FOLDERS_LAYER),
            sprite: Sprite {
                custom_size: Some(Vec2::new(60.0, 60.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|folder| {
            folder.spawn_bundle(Text2dBundle {
                transform: Transform::from_xyz(0.0, -52.0, FOLDERS_LAYER),
                text: Text::from_section(
                    "Recycle Bin",
                    TextStyle {
                        font_size: 18.0,
                        font: asset_server.load("fonts/segoe_ui.ttf"),
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::BOTTOM_CENTER),
                ..default()
            });
        })
        .insert(RecycleBin)
        .insert_bundle(MouseInteractionBundle::default())
        .insert(Name::new("Recycle Bin"));
}
