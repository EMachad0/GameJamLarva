use std::{path::Path, time::Duration};

const IMAGE_WIDTH: f32 = 300.0;
use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use crate::{biome::Biome, drag_and_drop::MouseInteractionBundle, image_biome::ImageBiome};

pub struct ImageTimer(pub Timer);

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
                image: asset_server.load(Path::new(&format!(
                    "img/biomes/{}/{}_{:02}.jpg",
                    biome, biome, i
                ))),
                biome: biome,
            });
        }
    }

    commands.insert_resource(image_server);
}

// Sets the width size of the image and maintains the aspect ratio
pub fn image_sizer(width: f32, height: f32) -> Vec2 {
    Vec2::new(IMAGE_WIDTH, IMAGE_WIDTH / (width / height))
}

pub fn spawn_image(
    mut commands: Commands,
    mut images_server: ResMut<ImagesServer>,
    assets: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<ImageTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();

        let (image_s, image) = loop { 
            let random_image = images_server.images.choose(&mut rng).unwrap();

            match assets.get(&random_image.image) {
                Some(image) => break (random_image, image),
                None => continue,
            }
        }; 

        let width = image.texture_descriptor.size.width as f32;
        let height = image.texture_descriptor.size.height as f32;

        let image_size = image_sizer(width, height);

        commands
            .spawn_bundle(SpriteBundle {
                texture: image_s.image.clone(),
                transform: Transform::from_xyz(360.0, 460.0, 2.0),
                sprite: Sprite {
                    custom_size: Some(image_size),
                    ..default()
                },
                ..default()
            })
            .with_children(|image| {
                image.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("frame.png"),
                    transform: Transform::from_xyz(0.0, 12.0, 2.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(image_size.x, image_size.y + 25.0)),
                        ..default()
                    },
                    ..default()
                });
            })
            .insert(ImageBiome {
                biome: image_s.biome,
            })
            .insert_bundle(MouseInteractionBundle::default())
            .insert(Name::new("Imagem"));

        images_server.runned = true;
    }
}
