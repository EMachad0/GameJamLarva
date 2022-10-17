use bevy::prelude::*;

use crate::biome::Biome;
use crate::score::Score;
use crate::ui::button::ButtonImage;
use crate::ui::root_ui::RootUi;

#[derive(Component)]
pub struct BackToMenuButton;

#[derive(Component)]
pub struct EndMenuUi;

pub fn end_game_ui_setup(
    mut commands: Commands,
    root: Res<RootUi>,
    score: Res<Score>,
    asset_server: Res<AssetServer>,
    button_images: Res<ButtonImage>,
) {
    let font = asset_server.load("fonts/tahoma.ttf");

    let biome_report_ts = TextStyle {
        font: font.clone(),
        font_size: 50.0,
        color: Color::BLACK,
    };
    let total_report_ts = TextStyle {
        font: font.clone(),
        font_size: 80.0,
        color: Color::BLACK,
    };

    let button_style = Style {
        align_self: AlignSelf::Center,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        margin: UiRect::all(Val::Px(5.0)),
        max_size: Size::new(Val::Percent(30.), Val::Undefined),
        flex_grow: 1.0,
        ..Default::default()
    };

    let button_text_style = TextStyle {
        font: asset_server.load("fonts/tahoma_bold.ttf"),
        font_size: 40.0,
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
            color: Color::NONE.into(),
            ..default()
        })
        .insert(EndMenuUi)
        .insert(Name::new("EndMenu UI"))
        .id();

    let background = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(800.), Val::Px(640.)),
                padding: UiRect {
                    left: Val::Px(5.),
                    right: Val::Px(5.),
                    bottom: Val::Px(5.),
                    top: Val::Px(45.),
                },
                flex_wrap: FlexWrap::WrapReverse,
                ..default()
            },
            image: asset_server.load("img/frame.png").into(),
            ..default()
        })
        .with_children(|bg| {
            bg.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Px(100.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            })
            .insert(Name::new("Title"))
            .with_children(|title| {
                title.spawn_bundle(TextBundle {
                    text: Text::from_section(
                        "Score",
                        TextStyle {
                            font: asset_server.load("fonts/tahoma_bold.ttf"),
                            font_size: 100.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
            });
            bg.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Auto),
                    margin: UiRect::all(Val::Px(5.)),
                    flex_wrap: FlexWrap::Wrap,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                color: Color::WHITE.into(),
                ..default()
            })
            .insert(Name::new("Box Biome"))
            .with_children(|parent| {
                biome_scores.for_each(|(biome, score)| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(280.0), Val::Px(100.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                margin: UiRect::new(
                                    Val::Px(20.0),
                                    Val::Px(20.0),
                                    Val::Undefined,
                                    Val::Undefined,
                                ),
                                flex_grow: 1.0,
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .insert(Name::new(format!("Box Biome {}", biome)))
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::from_section(
                                    format!("{}:", biome.as_label()),
                                    biome_report_ts.clone(),
                                ),
                                ..default()
                            });
                            parent.spawn_bundle(TextBundle {
                                text: Text::from_section(
                                    format!("{}", score),
                                    biome_report_ts.clone(),
                                ),
                                ..default()
                            });
                        });
                });
            });
            bg.spawn_bundle(NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(5.)),
                    size: Size::new(Val::Percent(100.), Val::Undefined),
                    flex_grow: 1.0,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            })
            .with_children(|total| {
                total.spawn_bundle(TextBundle {
                    text: Text::from_section("Total: ", total_report_ts.clone()),
                    ..default()
                });
                total.spawn_bundle(TextBundle {
                    text: Text::from_section(
                        format!("{}/{}", score.right, score.images_spawned),
                        total_report_ts.clone(),
                    ),
                    ..default()
                });
                total.spawn_bundle(TextBundle {
                    text: Text::from_section(
                        format!(" {:.0}%", score.total_accuracy() * 100.),
                        total_report_ts.clone(),
                    ),
                    ..default()
                });
            });
            bg.spawn_bundle(NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            })
            .insert(Name::new("Button Container"))
            .with_children(|button_container| {
                button_container
                    .spawn_bundle(ButtonBundle {
                        style: button_style.clone(),
                        image: button_images.normal.clone().into(),
                        ..default()
                    })
                    .with_children(|btn| {
                        btn.spawn_bundle(TextBundle {
                            text: Text::from_section("Play Again", button_text_style.clone()),
                            ..default()
                        });
                    })
                    .insert(BackToMenuButton);
            });
        })
        .id();

    commands.entity(root.entity).add_child(container);
    commands.entity(container).add_child(background);
}
