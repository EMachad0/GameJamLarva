use bevy::prelude::*;

use crate::ui::portrait::PortraitImages;
use crate::ui::root_ui::RootUi;
use crate::ui::typewriter;
use crate::ui::typewriter::{Typewriter, TypingTimer};

#[derive(Component)]
pub struct DialogUi;

#[derive(Component)]
pub struct DialogText;

#[derive(Component)]
pub struct DialogPortrait;

pub fn dialog_ui_setup(
    mut commands: Commands,
    root: Res<RootUi>,
    asset_server: ResMut<AssetServer>,
    portraits: Res<PortraitImages>,
) {
    commands.insert_resource(TypingTimer(Timer::from_seconds(0.08, true)));

    let bg_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(1280.0), Val::Px(220.0)),
                align_items: AlignItems::FlexEnd,
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            visibility: Visibility { is_visible: false },
            color: Color::BLACK.into(),
            ..default()
        })
        .insert(DialogUi)
        .insert(Name::new("Dialog UI"))
        .id();

    let portrait = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(280.0), Val::Percent(100.0)),
                ..default()
            },
            image: portraits.smile.clone_weak().into(),
            ..default()
        })
        .insert(DialogPortrait)
        .insert(Name::new("Dialog Portrait"))
        .id();

    let text_container = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                margin: UiRect {
                    left: Val::Px(10.),
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: Color::BLACK.into(),
            ..default()
        })
        .id();

    let typewriter_text_entity = commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: typewriter::to_typewriter_sections(TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/computer_pixel-7.ttf"),
                        font_size: 50.0,
                        color: Color::GREEN,
                    },
                })
                .into(),
                alignment: TextAlignment::TOP_CENTER,
            },
            ..default()
        })
        .insert(DialogText)
        .insert(Name::new("Dialog Text"))
        .insert(Typewriter::default())
        .id();

    commands.entity(root.entity).add_child(bg_entity);
    commands
        .entity(bg_entity)
        .push_children(&[portrait, text_container]);
    commands
        .entity(text_container)
        .add_child(typewriter_text_entity);
}
