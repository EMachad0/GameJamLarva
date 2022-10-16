pub use bevy::prelude::*;

pub struct RootUi {
    pub(crate) entity: Entity,
}

#[derive(Component)]
pub struct RootUiComponent;

pub fn ui_setup(mut commands: Commands) {
    let entity = commands
        .spawn()
        .insert_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(1280.0), Val::Px(720.0)),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(RootUiComponent)
        .id();

    commands.insert_resource(RootUi { entity });
}
