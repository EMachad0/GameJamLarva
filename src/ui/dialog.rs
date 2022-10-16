use bevy::prelude::*;

use crate::ui::typewriter;
use crate::ui::typewriter::{Typewriter, TypingTimer};

#[derive(Component)]
pub struct DialogUi;

#[derive(Debug, Default, Component)]
pub struct Dialog;

pub fn dialog_ui_setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(TypingTimer(Timer::from_seconds(0.1, true)));

    let container_entity = commands
        .spawn()
        .insert_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(DialogUi)
        .id();

    let bg_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(220.0)),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            color: Color::rgba(0.1, 0.1, 0.8, 0.9).into(),
            ..default()
        })
        .id();

    let portrait = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(280.0), Val::Percent(100.0)),
                ..default()
            },
            image: asset_server.load("img/computer/cp_smile.png").into(),
            ..default()
        })
        .id();

    let typewriter_text_entity = commands
        .spawn_bundle(TextBundle {
            style: Style {
                size: Size::new(Val::Px(1000.0), Val::Undefined),
                ..Default::default()
            },
            text: Text {
                sections: typewriter::to_typewriter_sections(TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/computer_pixel-7.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                })
                .into(),
                alignment: TextAlignment::TOP_CENTER,
            },
            ..default()
        })
        .insert(Dialog::default())
        .insert(Typewriter::default())
        .id();

    commands
        .entity(container_entity)
        .push_children(&[bg_entity]);
    commands
        .entity(bg_entity)
        .push_children(&[portrait, typewriter_text_entity]);
}


