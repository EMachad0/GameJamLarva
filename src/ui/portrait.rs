use bevy::prelude::*;

pub struct PortraitImages {
    pub smile: Handle<Image>,
    pub angry: Handle<Image>,
    pub error: Handle<Image>,
}

pub fn portrait_images_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PortraitImages {
        smile: asset_server.load("img/computer/cp_smile.png"),
        angry: asset_server.load("img/computer/cp_angry.png"),
        error: asset_server.load("img/computer/cp_error.png"),
    });
}
