use bevy::prelude::*;

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

pub fn main_menu_ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        margin: UiRect::all(Val::Px(4.0)),
        flex_grow: 1.0,
        ..Default::default()
    };

    let button_text_style = TextStyle {
        font: asset_server.load("fonts/segoe_ui.ttf"),
        font_size: 24.0,
        color: Color::BLACK,
    };

    let container_entity = commands
        .spawn()
        .insert_bundle(NodeBundle {
            color: UiColor(Color::rgb(0.5, 0.5, 0.5)),
            style: Style {
                size: Size::new(Val::Auto, Val::Auto),
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::ColumnReverse,
                //align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainMenuUi)
        .id();

    let button_start_game_entity = commands
        .spawn_bundle(ButtonBundle {
            style: button_style.clone(),
            ..Default::default()
        })
        .with_children(|btn| {
            btn.spawn_bundle(TextBundle {
                text: Text::from_section("Start Game", button_text_style.clone()),
                ..Default::default()
            });
        })
        .insert(StartGameButton)
        .id();

    commands
        .entity(container_entity)
        .push_children(&[button_start_game_entity]);
}
