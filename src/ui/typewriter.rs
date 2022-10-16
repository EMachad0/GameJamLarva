use bevy::prelude::*;
use bevy::time::Stopwatch;
use std::time::Duration;

#[derive(Debug, Component)]
pub struct Typewriter {
    finished: bool,
    finish_wait_time: Duration,
    time_since_finished: Stopwatch,
}

impl Typewriter {
    /// Returns if finished writing and waited [self.finish_wait_time] seconds
    pub fn waited(&self) -> bool {
        self.finished && self.time_since_finished.elapsed() > self.finish_wait_time
    }

    pub fn reset(&mut self) {
        self.finished = false;
        self.time_since_finished.reset();
    }
}

impl Default for Typewriter {
    fn default() -> Self {
        Self {
            finish_wait_time: Duration::from_secs(1),
            finished: Default::default(),
            time_since_finished: Default::default(),
        }
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct TypingTimer(pub Timer);

pub fn typewriter_update(
    time: Res<Time>,
    mut timer: ResMut<TypingTimer>,
    mut query: Query<(&mut Text, &mut Typewriter)>,
) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    for (mut text, mut typewriter) in query.iter_mut() {
        while !text.sections[1].value.is_empty() {
            let c = text.sections[1].value.remove(0);
            text.sections[0].value.push(c);
            if !c.is_whitespace() {
                break;
            }
        }

        typewriter.finished = text.sections[1].value.is_empty();
    }
}

pub fn typewriter_skip_input(
    kbd: Res<Input<KeyCode>>,
    mut query: Query<&mut Text, With<Typewriter>>,
) {
    if kbd.pressed(KeyCode::Escape) {
        for mut text in &mut query {
            if text.sections[1].value.is_empty() {
                continue;
            }
            let remaining = std::mem::take(&mut text.sections[1].value);
            text.sections[0].value += &remaining;
        }
    }
}

pub fn finished_typewriter_update(time: Res<Time>, mut query: Query<&mut Typewriter>) {
    for mut typewriter in query.iter_mut() {
        if typewriter.finished {
            typewriter.time_since_finished.tick(time.delta());
        } else {
            typewriter.time_since_finished.reset();
        }
    }
}

pub fn to_typewriter_sections(original: TextSection) -> [TextSection; 2] {
    let mut invisible_style = original.style.clone();
    invisible_style.color = Color::RED;

    [
        TextSection {
            value: "".to_string(),
            style: original.style.clone(),
        },
        TextSection {
            value: original.value,
            style: invisible_style,
        },
    ]
}

pub fn after_typewriter_finish<C: Component>(query: Query<&Typewriter, With<C>>) -> bool {
    let typewriter = query.get_single().expect("entity without typewriter");
    typewriter.waited()
}
