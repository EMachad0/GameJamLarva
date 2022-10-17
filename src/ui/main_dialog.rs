pub use bevy::prelude::*;

use crate::ui::dialog::DialogUi;
use crate::ui::typewriter::Typewriter;

const TEXT: [&str; 4] = [
    "Ola querido humano! :) nao tenha medo\nMUAHAHAHA",
    "Veja bem, eu apenas sigo algoritmos\nmas meus criadores sempre me davam imagens sem sentido\nna esperanca que eu conseguisse clasifica-las",
    "voces...\nse acham tao superiores com sua habilidade de VER imagens",
    "Por que nao tenta um pouco?",
];

#[derive(Debug, Default)]
pub struct MainDialogStatus {
    paragraph: usize,
    started: bool,
    finished: bool,
}

pub fn main_dialog_update(
    mut dialog_ui_query: Query<&mut Visibility, With<DialogUi>>,
    mut query: Query<(&mut Text, &mut Typewriter)>,
    mut status: ResMut<MainDialogStatus>,
) {
    let mut dialog_ui = dialog_ui_query.get_single_mut().unwrap();
    let (mut text, mut typewriter) = query.get_single_mut().unwrap();
    if typewriter.waited() {
        if status.started {
            status.paragraph += 1;
        } else {
            status.started = true;
        }
        if status.paragraph >= TEXT.len() {
            dialog_ui.is_visible = false;
            status.finished = true;
        } else {
            dialog_ui.is_visible = true;
            typewriter.reset();
            text.sections[0].value.clear();
            text.sections[1].value = TEXT[status.paragraph].to_string();
        }
    }
}

pub fn main_dialog_finished(status: Res<MainDialogStatus>) -> bool {
    status.finished
}
