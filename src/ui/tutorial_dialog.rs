pub use bevy::prelude::*;

use crate::ui::dialog::DialogUi;
use crate::ui::typewriter::Typewriter;

const TEXT: [&str; 3] = [
    "Hmmmmm vamos ver, que tal esse DATASET?\nAraste as imagens ate as pastas para clasifica-las",
    "Como voce e um ser capaz de APRENDIZADO\na cor da pasta ao guardar uma imagen indica se voce\nERROU ou ACERTOU",
    "Boa Sorte!"
];

#[derive(Default)]
pub struct TutorialDialogStatus {
    paragraph: usize,
    started: bool,
    finished: bool,
}

pub fn tutorial_dialog_update(
    mut dialog_ui_query: Query<&mut Visibility, With<DialogUi>>,
    mut query: Query<(&mut Text, &mut Typewriter)>,
    mut status: ResMut<TutorialDialogStatus>,
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

pub fn tutorial_finished(status: Res<TutorialDialogStatus>) -> bool {
    status.finished
}
