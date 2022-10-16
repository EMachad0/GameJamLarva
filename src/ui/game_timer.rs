use crate::game_timer::GameTimer;
use bevy::prelude::*;

use crate::ui::root_ui::RootUi;

#[derive(Component)]
pub struct GameTimerUi;

#[derive(Component)]
pub struct GameTimerText;

pub fn game_timer_ui_setup(
    mut commands: Commands,
    root: Res<RootUi>,
    asset_server: Res<AssetServer>,
) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/retro_computer.ttf"),
        font_size: 50.0,
        color: Color::RED,
    };

    let container_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Undefined),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexEnd,
                flex_grow: 4.0,
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(GameTimerUi)
        .id();

    let timer_ui_entity = commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection::new("00", text_style.clone()),
                    TextSection::new(":", text_style.clone()),
                    TextSection::new("00", text_style.clone()),
                ],
                ..default()
            },
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(GameTimerText)
        .id();

    commands.entity(root.entity).add_child(container_entity);
    commands.entity(container_entity).add_child(timer_ui_entity);
}

pub fn game_timer_ui_update(
    timer: Res<GameTimer>,
    mut query: Query<(&mut Text, &mut Visibility), With<GameTimerText>>,
) {
    let remaining_time = timer.duration() - timer.elapsed();
    let sec = remaining_time.as_secs() % 60;
    let min = remaining_time.as_secs() / 60;

    let (mut text, mut visibility) = query
        .get_single_mut()
        .expect("Unable to find game timer text");
    text.sections[2].value = sec.to_string();
    text.sections[0].value = min.to_string();
    visibility.is_visible = true;
}
