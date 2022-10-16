pub use bevy::prelude::*;

use crate::ui::typewriter::Typewriter;

pub const TUTORIAL_TEXT: [&str; 2] = [
    "",
    "Hmmmmm vamos ver, que tal esse DATASET? Araste as imagens até as pastas para clasificalas",
];

#[derive(Default)]
pub struct TutorialStatus {
    paragraph: usize,
    finished: bool,
}

pub fn tutorial_update(
    mut query: Query<(&mut Text, &mut Typewriter)>,
    mut status: ResMut<TutorialStatus>,
) {
    let (mut text, mut typewriter) = query.get_single_mut().unwrap();
    debug!("{:?}", typewriter);

    if typewriter.waited() {
        status.paragraph += 1;
        if status.paragraph >= TUTORIAL_TEXT.len() {
            status.finished = true;
        } else {
            typewriter.reset();
            text.sections[0].value.clear();
            text.sections[1].value = TUTORIAL_TEXT[status.paragraph].to_string();
        }
    }
}

pub fn tutorial_finished(status: Res<TutorialStatus>) -> bool {
    status.finished
}
