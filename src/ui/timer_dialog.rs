pub use bevy::prelude::*;

use crate::ui::dialog::DialogUi;
use crate::ui::typewriter::Typewriter;

pub const TUTORIAL_TEXT: [&str; 3] = [
    "",
    "Facil, nao?",
    "Que tal mais rapido...           e contra o tempo",
];

#[derive(Default)]
pub struct TimerDialogStatus {
    paragraph: usize,
    finished: bool,
}

pub fn timer_dialog_update(
    mut dialog_ui_query: Query<&mut Visibility, With<DialogUi>>,
    mut query: Query<(&mut Text, &mut Typewriter)>,
    mut status: ResMut<TimerDialogStatus>,
) {
    let mut dialog_ui = dialog_ui_query.get_single_mut().unwrap();
    let (mut text, mut typewriter) = query.get_single_mut().unwrap();

    if typewriter.waited() {
        status.paragraph += 1;
        if status.paragraph >= TUTORIAL_TEXT.len() {
            dialog_ui.is_visible = false;
            status.finished = true;
        } else {
            dialog_ui.is_visible = true;
            typewriter.reset();
            text.sections[0].value.clear();
            text.sections[1].value = TUTORIAL_TEXT[status.paragraph].to_string();
        }
    }
}

pub fn timer_dialog_finished(status: Res<TimerDialogStatus>) -> bool {
    status.finished
}
