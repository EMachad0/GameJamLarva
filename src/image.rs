use std::path::Path;

const IMAGE_WIDTH: f32 = 300.0;
const IMAGE_FIRST_LAYER: u32 = 2;

use bevy::prelude::*;

use rand::{distributions::Uniform, seq::SliceRandom, thread_rng, Rng};

use crate::{
    aabb::AABB,
    biome::Biome,
    cursor_world_position::CursorWorldPosition,
    desktop::{Folder, Frame},
    drag_and_drop::{DraggingState, EndDragEntity, MouseInteractionBundle, StartDragEntity},
    score::Score,
};

#[derive(Component)]
pub struct SpawnedImage {
    biome: Biome,
}

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
fn image_sizer(width: f32, height: f32) -> Vec2 {
    Vec2::new(IMAGE_WIDTH, IMAGE_WIDTH / (width / height))
}

pub fn sprite_alpha_update(
    query: Query<(&Sprite, &Children), (Changed<Sprite>, With<SpawnedImage>)>,
    mut frame_query: Query<&mut Sprite, (With<Frame>, Without<SpawnedImage>)>,
) {
    for (sprite, children) in query.iter() {
        for child in children.iter() {
            let mut frame_sprite = frame_query.get_mut(*child).unwrap();

            frame_sprite.color.set_a(sprite.color.a());
        }
    }
}

pub fn image_drag(
    mut events: EventReader<StartDragEntity>,
    mut query: Query<(&mut Sprite, &mut Transform), With<SpawnedImage>>,
    dragging_state: ResMut<DraggingState>,
    mut images_server: ResMut<ImagesServer>,
) {
    for _ in events.iter() {
        if let Some(entity) = dragging_state.entity {
            let (mut sprite, mut transform) = match query.get_mut(entity) {
                Ok(entity) => entity,
                Err(_) => continue,
            };

            images_server.image_layer_count += 1;
            transform.translation.z = images_server.image_layer_count as f32;

            sprite.color.set_a(0.2);
        }
    }
}

pub fn image_drop(
    mut commands: Commands,
    mut query_images: Query<(&SpawnedImage, &mut Sprite, &Children)>,
    query_folders: Query<(&Folder, &AABB)>,
    mut ev_drop: EventReader<EndDragEntity>,
    cursor: Res<CursorWorldPosition>,
    mut score: ResMut<Score>,
) {
    for ev in ev_drop.iter() {
        let cursor_position = match **cursor {
            None => return,
            Some(p) => p,
        };

        let entity = ev.entity;

        let (image, mut sprite, children) = match query_images.get_mut(ev.entity) {
            Ok(entity) => entity,
            Err(_) => continue,
        };

        let mut disappeared = false;
        // TODO: find a way to determine the folder a file will drop in
        for (folder, aabb) in query_folders.iter() {
            if aabb.inside(cursor_position) {
                disappeared = true;

                println!("Image biome: {}", image.biome);
                println!("Folder bione: {}", folder.biome);
                score_bookkeeper(&image.biome, &folder.biome, &mut score);

                for child in children.iter() {
                    commands.entity(*child).despawn();
                }
                commands.entity(entity).despawn();
                break;
            }
        }

        if !disappeared {
            sprite.color.set_a(1.0);
        }
    }
}

fn score_bookkeeper(biome_guessed: &Biome, actual_biome: &Biome, score: &mut Score) {
    if biome_guessed == actual_biome {
        score.total += 1;

        score.biome_score.increment_biome(biome_guessed);
    } else {
        score.mistakes += 1;
    }
    debug!("{}", score);
}

pub fn spawn_image(
    mut commands: Commands,
    mut images_server: ResMut<ImagesServer>,
    assets: Res<Assets<Image>>,
    mut timer: ResMut<ImageTimer>,
    time: Res<Time>,
    mut score: ResMut<Score>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        score.images_spawned += 1;

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

        let x_pos = rng.sample(Uniform::new(
            image_size.x / 2.0,
            1280.0 - (image_size.x / 2.0),
        )) as f32;
        let y_pos = rng.sample(Uniform::new(
            image_size.y / 2.0,
            720.0 - (image_size.x / 2.0),
        )) as f32;

        commands
            .spawn_bundle(SpriteBundle {
                texture: image_s.image.clone(),
                transform: Transform::from_xyz(
                    x_pos,
                    y_pos,
                    images_server.image_layer_count as f32,
                ),
                sprite: Sprite {
                    custom_size: Some(image_size),
                    ..default()
                },
                ..default()
            })
            .with_children(|image| {
                image
                    .spawn_bundle(SpriteBundle {
                        texture: images_server.frame.clone(),
                        transform: Transform::from_xyz(0.0, 10.0, -0.5),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(image_size.x + 5.0, image_size.y + 25.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Frame);
            })
            .insert_bundle(MouseInteractionBundle::default())
            .insert(Name::new("Imagem"))
            .insert(SpawnedImage {
                biome: image_s.biome,
            });
    }
}
