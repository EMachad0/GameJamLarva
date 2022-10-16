pub use bevy::prelude::*;

use crate::ui::dialog::DialogUi;
use crate::ui::typewriter::Typewriter;

pub const MAIN_DIALOG_TEXT: [&str; 4] = [
    "Ola querido humano! :) nao tenha medo MUAHAHAHA",
    "Veja bem, eu apenas sigo algoritmos, mas meus criadores sempre me davam imagens sem sentido na esperanca que eu conseguisse clasifica-las",
    "Ja voces, seres inteligentes, se acham tao superiores com sua habilidade de VER imagens",
    "Por que nao tenta um pouco? *-*",
];

pub struct MainDialogStatus {
    paragraph: i32,
    finished: bool,
}

impl Default for MainDialogStatus {
    fn default() -> Self {
        Self {
            paragraph: -1,
            finished: false,
        }
    }
}

pub fn main_dialog_update(
    mut dialog_ui_query: Query<&mut Visibility, With<DialogUi>>,
    mut query: Query<(&mut Text, &mut Typewriter)>,
    mut status: ResMut<MainDialogStatus>,
) {
    let mut dialog_ui = dialog_ui_query.get_single_mut().unwrap();
    let (mut text, mut typewriter) = query.get_single_mut().unwrap();
    if typewriter.waited() {
        status.paragraph += 1;
        let paragraph = status.paragraph as usize;
        if paragraph >= MAIN_DIALOG_TEXT.len() {
            dialog_ui.is_visible = false;
            status.finished = true;
        } else {
            dialog_ui.is_visible = true;
            typewriter.reset();
            text.sections[0].value.clear();
            text.sections[1].value = MAIN_DIALOG_TEXT[paragraph].to_string();
        }
    }
}

pub fn main_dialog_finished(status: Res<MainDialogStatus>) -> bool {
    status.finished
}
