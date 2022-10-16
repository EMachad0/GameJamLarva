use std::path::Path;

const IMAGE_WIDTH: f32 = 300.0;
const IMAGE_FIRST_LAYER: u32 = 2;

use bevy::{prelude::*, transform};
use rand::{seq::SliceRandom, thread_rng, Rng, distributions::Uniform};

use crate::{biome::Biome, drag_and_drop::{MouseInteractionBundle, StartDragEntity}, image_biome::ImageBiome};

#[derive(Component)]
pub struct ImageSpawned;

pub struct ImageTimer(pub Timer);

pub struct ImagesServer {
    images: Vec<ImagePls>,
    frame: Handle<Image>,
    pub image_layer_count: u32,
}

pub struct ImagePls {
    image: Handle<Image>,
    biome: Biome,
}

pub fn load_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut image_server = ImagesServer {
        images: Vec::new(),
        frame: asset_server.load("img/frame.png"),
        image_layer_count: IMAGE_FIRST_LAYER,
    };

    for biome in Biome::iterator() {
        for i in 1..21 {
            image_server.images.push(ImagePls {
                image: asset_server.load(Path::new(&format!(
                    "img/biome/{}/{}_{:02}.jpg",
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

pub fn drag_image_bring_foward(mut events: EventReader<StartDragEntity>, mut query: Query<(&mut ImageSpawned, &mut Transform)>, mut images_server: ResMut<ImagesServer>) {
    for ev in events.iter() {
        let (_, mut transform) = query.get_mut(ev.entity).unwrap();

        images_server.image_layer_count += 1;
        transform.translation.z = images_server.image_layer_count as f32;
    }
}

pub fn spawn_image(
    mut commands: Commands,
    mut images_server: ResMut<ImagesServer>,
    assets: Res<Assets<Image>>,
    mut timer: ResMut<ImageTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();

        images_server.image_layer_count += 1;

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

        let x_pos = rng.sample(Uniform::new(image_size.x / 2.0, 1280.0 - (image_size.x / 2.0))) as f32;
        let y_pos = rng.sample(Uniform::new(image_size.y / 2.0, 720.0 - (image_size.x / 2.0))) as f32;

        commands
            .spawn_bundle(SpriteBundle {
                texture: image_s.image.clone(),
                transform: Transform::from_xyz(x_pos, y_pos, images_server.image_layer_count as f32),
                sprite: Sprite {
                    custom_size: Some(image_size),
                    ..default()
                },
                ..default()
            })
            .with_children(|image| {
                image.spawn_bundle(SpriteBundle {
                    texture: images_server.frame.clone(),
                    transform: Transform::from_xyz(0.0, 10.0, -0.5),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(image_size.x + 5.0, image_size.y + 25.0)),
                        ..default()
                    },
                    ..default()
                });
            })
            .insert(ImageBiome {
                biome: image_s.biome,
            })
            .insert_bundle(MouseInteractionBundle::default())
            .insert(Name::new("Imagem"))
            .insert(ImageSpawned);
    }
}
