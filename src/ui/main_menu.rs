use bevy::prelude::*;

use crate::ui::button::ButtonImage;
use crate::ui::root_ui::RootUi;

pub struct MainMenuBackgroundImage {
    image_handle: Handle<Image>,
}

#[derive(Component)]
pub struct MainMenuBackground;

#[derive(Component)]
pub struct MainMenuUi;

#[derive(Component)]
pub struct StartGameButton;

pub fn main_menu_background_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_handle = asset_server.load("img/background/main_menu_bg.png");
    commands.insert_resource(MainMenuBackgroundImage { image_handle });
}

pub fn main_menu_background_setup(
    mut commands: Commands,
    background: Res<MainMenuBackgroundImage>,
) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..default()
            },
            texture: background.image_handle.clone_weak(),
            ..default()
        })
        .insert(MainMenuBackground);
}

pub fn main_menu_ui_setup(
    mut commands: Commands,
    root: Res<RootUi>,
    asset_server: Res<AssetServer>,
    button_images: Res<ButtonImage>,
) {
    let button_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        margin: UiRect::all(Val::Px(4.0)),
        size: Size::new(Val::Px(130.), Val::Px(40.)),
        flex_grow: 1.0,
        ..Default::default()
    };

    let button_text_style = TextStyle {
        font: asset_server.load("fonts/tahoma.ttf"),
        font_size: 24.0,
        color: Color::BLACK,
    };

    let container_entity = commands
        .spawn()
        .insert_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(770.),
                    right: Val::Undefined,
                    top: Val::Px(370.),
                    bottom: Val::Undefined,
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(MainMenuUi)
        .insert(Name::new("Main Menu UI"))
        .id();

    let button_start_game_entity = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            image: button_images.normal.clone().into(),
            ..default()
        })
        .with_children(|btn| {
            btn.spawn_bundle(TextBundle {
                text: Text::from_section("Start Game", button_text_style.clone()),
                ..default()
            });
        })
        .insert(StartGameButton)
        .id();

    commands.entity(root.entity).add_child(container_entity);
    commands
        .entity(container_entity)
        .add_child(button_start_game_entity);
}
