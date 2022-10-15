use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct Typewriter {
    finished: bool,
}

#[derive(Debug, Deref, DerefMut)]
pub struct TypingTimer(pub Timer);

#[derive(Debug, Deref, DerefMut)]
pub struct TypewriterFinished {
    entity: Entity,
}

pub fn typewriter_update(
    time: Res<Time>,
    mut timer: ResMut<TypingTimer>,
    mut query: Query<(Entity, &mut Text, &mut Typewriter)>,
    mut finished_evw: EventWriter<TypewriterFinished>,
) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    for (entity, mut text, mut typewriter) in query.iter_mut() {
        while !text.sections[1].value.is_empty() {
            let c = text.sections[1].value.remove(0);
            text.sections[0].value.push(c);
            if !c.is_whitespace() {
                break;
            }
        }

        if text.sections[1].value.is_empty() {
            typewriter.finished = true;
            finished_evw.send(TypewriterFinished { entity });
        }
    }
}

pub fn to_typewriter_sections(original: TextSection) -> [TextSection; 2] {
    let mut invisible_style = original.style.clone();
    invisible_style.color = Color::NONE;

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

pub fn on_typewriter_finish<C: Component>(query: Query<&Typewriter, With<C>>) -> bool {
    let typewriter = query.get_single().expect("entity without typewriter");
    typewriter.finished
}
