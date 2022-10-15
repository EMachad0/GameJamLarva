use std::path::Path;

const IMAGE_WIDTH: f32 = 300.0;
use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use crate::{biome::Biome, image_biome::ImageBiome, drag_and_drop::MouseInteractionBundle};

pub struct ImagesServer {
    images: Vec<ImagePls>,
    runned: bool,
}

pub struct ImagePls {
    image: Handle<Image>,
    size: Option<Size>,
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
                size: None,
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
    mut ev_asset: EventReader<AssetEvent<Image>>,
    asset_server: Res<AssetServer>,
) {
    if !images_server.runned {
        for ev in ev_asset.iter() {
            if images_server.runned { continue; }
            match ev {
                AssetEvent::Created { handle } => {
                    for image_s in &images_server.images {
                        if *handle == image_s.image {
                            let image = assets.get(&handle).unwrap();

                            let width = image.texture_descriptor.size.width as f32;
                            let height = image.texture_descriptor.size.height as f32;

                            let image_size = image_sizer(width, height);

                            commands
                                .spawn_bundle(SpriteBundle {
                                    texture: handle.clone(),
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
                                    image.spawn_bundle(SpriteBundle {
                                        transform: Transform::from_xyz(0.0, 12.0, 0.0),
                                        sprite: Sprite {
                                            color: Color::BLUE,
                                            custom_size: Some(Vec2::new(image_size.x, image_size.y + 20.0)),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                })
                                .insert(ImageBiome { biome: image_s.biome })
                                .insert_bundle(MouseInteractionBundle::default())
                                .insert(Name::new("Imagem"));

                            images_server.runned = true;
                            break;
                        }
                    }
                },
                _ => {}
            }
            if images_server.runned { break; }
        }
    }
}
