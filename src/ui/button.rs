use bevy::prelude::*;

pub struct ButtonImage {
    pub normal: Handle<Image>,
    pub hover: Handle<Image>,
    pub click: Handle<Image>,
}

pub fn button_image_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ButtonImage {
        normal: asset_server.load("img/button/b_normal.png"),
        hover: asset_server.load("img/button/b_hover.png"),
        click: asset_server.load("img/button/b_click.png"),
    });
}

pub fn on_button_interaction<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }
    false
}

pub fn button_interaction_update(
    mut query: Query<(&Interaction, &mut UiImage), (Changed<Interaction>, With<Button>)>,
    images: Res<ButtonImage>,
) {
    for (interaction, mut image) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *image = images.click.clone().into();
            }
            Interaction::Hovered => {
                *image = images.hover.clone().into();
            }
            Interaction::None => {
                *image = images.normal.clone().into();
            }
        }
    }
}
