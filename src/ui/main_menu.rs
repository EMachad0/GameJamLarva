use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct StartGameButton;

pub fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    let main_menu_entity = commands
        .spawn_bundle(NodeBundle {
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
        .insert(MainMenu)
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
        .entity(main_menu_entity)
        .push_children(&[button_start_game_entity]);
}
