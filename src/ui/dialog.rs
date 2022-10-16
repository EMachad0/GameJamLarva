use bevy::prelude::*;

use crate::ui::root_ui::RootUi;
use crate::ui::typewriter;
use crate::ui::typewriter::{Typewriter, TypingTimer};

#[derive(Component)]
pub struct DialogUi;

#[derive(Debug, Default, Component)]
pub struct DialogText;

pub fn dialog_ui_setup(
    mut commands: Commands,
    root: Res<RootUi>,
    asset_server: ResMut<AssetServer>,
) {
    commands.insert_resource(TypingTimer(Timer::from_seconds(0.1, true)));

    let bg_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(1280.0), Val::Px(220.0)),
                align_items: AlignItems::FlexEnd,
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            visibility: Visibility { is_visible: false },
            color: Color::rgba(0.1, 0.1, 0.8, 0.9).into(),
            ..default()
        })
        .insert(DialogUi)
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
        .insert(DialogText::default())
        .insert(Typewriter::default())
        .id();

    commands.entity(root.entity).push_children(&[bg_entity]);
    commands
        .entity(bg_entity)
        .push_children(&[portrait, typewriter_text_entity]);
}
