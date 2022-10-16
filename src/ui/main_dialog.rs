pub use bevy::prelude::*;

use crate::ui::typewriter;
use crate::ui::typewriter::{Typewriter, TypingTimer};

pub const MAIN_DIALOG_TEXT: [&str; 4] = [
    "Ola querido humano! :) nao tenha medo MUAHAHAHA",
    "Veja bem, meus criadores sempre me davam imagens sem sentido na esperanca que eu conseguisse clasificalas, mas eu apenas sigo algoritmos",
    "Mas voces se acham tao superiores com sua habilidade de VER imagens",
    "Por que nao tenta um pouco? *-*",
];

#[derive(Component)]
pub struct MainDialogUi;

#[derive(Debug, Component)]
pub struct MainDialog {
    paragraph: i32,
}

impl Default for MainDialog {
    fn default() -> Self {
        Self { paragraph: -1 }
    }
}

#[derive(Default)]
pub struct MainDialogStatus {
    finished: bool,
}

pub fn main_dialog_ui_setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(TypingTimer(Timer::from_seconds(0.1, true)));
    commands.insert_resource(MainDialogStatus::default());

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
        .insert(MainDialogUi)
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
        .insert(MainDialog::default())
        .insert(Typewriter::default())
        .id();

    commands
        .entity(container_entity)
        .push_children(&[bg_entity]);
    commands
        .entity(bg_entity)
        .push_children(&[portrait, typewriter_text_entity]);
}

pub fn main_dialog_update(
    mut query: Query<(&mut Text, &mut Typewriter, &mut MainDialog)>,
    mut status: ResMut<MainDialogStatus>,
) {
    let (mut text, mut typewriter, mut dialog) = query.get_single_mut().unwrap();

    if typewriter.waited() {
        dialog.paragraph += 1;
        let paragraph = dialog.paragraph as usize;
        if paragraph >= MAIN_DIALOG_TEXT.len() {
            status.finished = true;
        } else {
            typewriter.reset();
            text.sections[0].value.clear();
            text.sections[1].value = MAIN_DIALOG_TEXT[paragraph].to_string();
        }
    }
}

pub fn main_dialog_finished(status: Res<MainDialogStatus>) -> bool {
    status.finished
}
