pub use bevy::prelude::*;

use crate::ui::typewriter;
use crate::ui::typewriter::{Typewriter, TypingTimer};

#[derive(Component)]
pub struct MainDialogBackground;

#[derive(Component)]
pub struct MainDialogUi;

#[derive(Component)]
pub struct MainDialog;

pub fn main_dialog_background_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..default()
            },
            texture: asset_server.load("img/background/loading_bg.png"),
            ..default()
        })
        .insert(MainDialogUi);
}

pub fn main_dialog_ui_setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(TypingTimer(Timer::from_seconds(0.15, true)));

    let container_entity = commands
        .spawn()
        .insert_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(MainDialogUi)
        .id();

    let bg_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(600.0), Val::Px(300.0)),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            color: Color::rgb(0.2, 0.2, 0.2).into(),
            ..default()
        })
        .id();

    let typewriter_text_entity = commands
        .spawn_bundle(TextBundle {
            style: Style {
                size: Size { width: Val::Px(580.), ..default()},
                ..Default::default()
            },
            text: Text {
                sections: typewriter::to_typewriter_sections(
                    TextSection {
                        value: "This is some text that runs on for quite a while and occupies multiple lines. A third line would be nice for good measure.".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/segoe_ui.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    }).into(),
                ..default()
            },
            ..default()
        })
        .insert(MainDialog)
        .insert(Typewriter::default()).id();

    commands
        .entity(container_entity)
        .push_children(&[bg_entity]);
    commands
        .entity(bg_entity)
        .push_children(&[typewriter_text_entity]);
}
