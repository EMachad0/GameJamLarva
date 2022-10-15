
use std::path::Path;

use bevy::prelude::*;
use rand::{thread_rng, seq::SliceRandom};

use crate::biome::{Biome};

pub struct ImagesServer {
    images: Vec<ImagePls>,
    runned: bool,
}

pub struct ImagePls {
    image: Handle<Image>,
    biome: Biome,
}

pub fn load_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut image_server = ImagesServer {
        images: Vec::new(),
        runned: false,
    };

    for biome in Biome::iterator() {
        for i in 1..21 {
            image_server.images.push(ImagePls { 
                image: asset_server.load(Path::new(&format!("img/biomes/{}/{}_{:02}.jpg", biome, biome, i))), 
                biome: biome
            });
        }
    }

    commands.insert_resource(image_server);
}

pub fn spawn_image(mut commands: Commands, mut images_server: ResMut<ImagesServer>) {
    let mut rng = thread_rng();

    let image = images_server.images.choose(&mut rng).unwrap().clone();
    if !images_server.runned {
        commands.spawn_bundle(SpriteBundle {
            texture: image.image.clone(),
            transform: Transform::from_xyz(360.0, 460.0, 2.0),
            ..default()
        });

        images_server.runned = true;
    }
}
