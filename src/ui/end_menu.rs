use bevy::prelude::*;

use crate::biome::Biome;
use crate::score::Score;
use crate::ui::root_ui::RootUi;

#[derive(Component)]
pub struct EndMenuUi;

pub fn end_game_ui_setup(
    mut commands: Commands,
    root: Res<RootUi>,
    score: Res<Score>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/segoe_ui.ttf");

    let report_text_style = TextStyle {
        font: font.clone(),
        font_size: 80.0,
        color: Color::BLACK,
    };

    let biome_scores = Biome::iterator().zip(score.biome_score.array());

    let container = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::WHITE.into(),
            ..default()
        })
        .insert(EndMenuUi)
        .id();

    let background = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(600.), Val::Px(600.)),
                padding: UiRect::all(Val::Px(5.)),
                flex_wrap: FlexWrap::WrapReverse,
                ..default()
            },
            color: Color::BLUE.into(),
            ..default()
        })
        .with_children(|bg| {
            bg.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Px(100.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                color: Color::RED.into(),
                ..default()
            })
            .insert(Name::new("Title"))
            .with_children(|title| {
                title.spawn_bundle(TextBundle {
                    text: Text::from_section(
                        "Score",
                        TextStyle {
                            font: font.clone(),
                            font_size: 100.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
            });
            for (biome, score) in biome_scores {
                bg.spawn_bundle(NodeBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(5.)),
                        min_size: Size::new(Val::Percent(30.), Val::Percent(20.)),
                        flex_grow: 1.0,
                        ..default()
                    },
                    color: Color::GREEN.into(),
                    ..default()
                });
            }
            bg.spawn_bundle(NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(5.)),
                    size: Size::new(Val::Percent(100.), Val::Undefined),
                    min_size: Size::new(Val::Undefined, Val::Percent(20.)),
                    flex_grow: 1.0,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                color: Color::GREEN.into(),
                ..default()
            })
            .with_children(|total| {
                total.spawn_bundle(TextBundle {
                    text: Text::from_section("Total: ", report_text_style.clone()),
                    ..default()
                });
                total.spawn_bundle(TextBundle {
                    text: Text::from_section(
                        format!("{}/{}", score.total, score.images_spawned),
                        report_text_style.clone(),
                    ),
                    ..default()
                });
                total.spawn_bundle(TextBundle {
                    text: Text::from_section(
                        format!(" {:.0}%", score.total_accuracy()),
                        report_text_style.clone(),
                    ),
                    ..default()
                });
            });
        })
        .id();

    commands.entity(root.entity).add_child(container);
    commands.entity(container).add_child(background);
}
