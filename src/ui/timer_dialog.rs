pub use bevy::prelude::*;

use crate::ui::dialog::{DialogPortrait, DialogUi};
use crate::ui::portrait::PortraitImages;
use crate::ui::typewriter::Typewriter;

const TEXT: [&str; 2] = ["Facil, nao?", "Que tal mais rapido...\ne contra o tempo"];

#[derive(Default)]
pub struct TimerDialogStatus {
    paragraph: usize,
    started: bool,
    finished: bool,
}

pub fn timer_dialog_update(
    mut dialog_portrait_query: Query<&mut UiImage, With<DialogPortrait>>,
    mut dialog_ui_query: Query<&mut Visibility, With<DialogUi>>,
    mut dialog_text_query: Query<(&mut Text, &mut Typewriter)>,
    mut status: ResMut<TimerDialogStatus>,
    portraits: Res<PortraitImages>,
) {
    let mut dialog_ui = dialog_ui_query.get_single_mut().unwrap();
    let mut dialog_portrait = dialog_portrait_query.get_single_mut().unwrap();
    let (mut text, mut typewriter) = dialog_text_query.get_single_mut().unwrap();
    if typewriter.waited() {
        typewriter.reset();
        if !status.started {
            status.started = true;
            dialog_ui.is_visible = true;
        } else {
            *dialog_portrait = portraits.angry.clone_weak().into();
            status.paragraph += 1;
        }
        if status.paragraph >= TEXT.len() {
            *dialog_portrait = portraits.smile.clone_weak().into();
            dialog_ui.is_visible = false;
            status.finished = true;
        } else {
            text.sections[0].value.clear();
            text.sections[1].value = TEXT[status.paragraph].to_string();
        }
    }
}

pub fn timer_dialog_finished(status: Res<TimerDialogStatus>) -> bool {
    status.finished
}
