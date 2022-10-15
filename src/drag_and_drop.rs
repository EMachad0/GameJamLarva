use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use rand::Rng;

use crate::aabb::AABB;
use crate::cursor_world_position::CursorWorldPosition;

#[derive(Default, Component)]
pub struct Clickable;

#[derive(Default, Component)]
pub struct Draggable;

#[derive(Default, Component)]
pub struct Hoverable;

#[derive(Default, Bundle)]
pub struct MouseInteractionBundle {
    pub clickable: Clickable,
    pub draggable: Draggable,
    pub hoverable: Hoverable,
    pub aabb: AABB,
}

pub fn squares_setup(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    let mut rng = rand::thread_rng();
    let range_x = rand::distributions::Uniform::new(0.0, window.width());
    let range_y = rand::distributions::Uniform::new(0.0, window.height());
    for i in 0..10 {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    rng.sample(range_x),
                    rng.sample(range_y),
                    1.0,
                )),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    color: Color::RED,
                    ..default()
                },
                ..default()
            })
            .insert_bundle(MouseInteractionBundle::default())
            .insert(Name::new(format!("Square {}", i)));
    }
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct DraggingState {
    entity: Option<Entity>,
}

#[derive(Debug)]
pub struct ClickEntity {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Debug)]
pub struct HoverEntity {
    pub entity: Entity,
}

#[derive(Debug)]
pub struct StartDragEntity {
    pub entity: Entity,
}

#[derive(Debug)]
pub struct EndDragEntity {
    pub entity: Entity,
}

pub fn mouse_click(
    mut mouse_btn_evr: EventReader<MouseButtonInput>,
    mut dragging_state: ResMut<DraggingState>,
    clickable_query: Query<(Entity, Option<&Draggable>, &GlobalTransform, &AABB), With<Clickable>>,
    cursor: Res<CursorWorldPosition>,
    mut click_evw: EventWriter<ClickEntity>,
    mut drag_start_evw: EventWriter<StartDragEntity>,
    mut drag_end_evw: EventWriter<EndDragEntity>,
) {
    let cursor_position = match **cursor {
        None => return,
        Some(p) => p,
    };

    for ev in mouse_btn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                if let MouseButton::Left = ev.button {
                    let clicked = clickable_query
                        .iter()
                        .filter(|(_, _, _, aabb)| aabb.inside(cursor_position))
                        .max_by(|(_, _, a, _), (_, _, b, _)| {
                            a.translation()
                                .z
                                .partial_cmp(&b.translation().z)
                                .expect("Tried to compare NAN value")
                        });

                    if let Some((entity, draggable, _, _)) = clicked {
                        click_evw.send(ClickEntity {
                            entity,
                            position: cursor_position,
                        });
                        if draggable.is_some() {
                            drag_start_evw.send(StartDragEntity { entity });
                            dragging_state.entity = Some(entity);
                        }
                    }
                }
            }
            ButtonState::Released => {
                if let Some(entity) = dragging_state.entity.take() {
                    drag_end_evw.send(EndDragEntity { entity });
                }
            }
        }
    }
}

pub fn draggable_update(
    cursor: Res<CursorWorldPosition>,
    state: Res<DraggingState>,
    mut query: Query<&mut Transform, With<Draggable>>,
) {
    if let Some(entity) = state.entity {
        let mut transform = query
            .get_mut(entity)
            .expect("unable to find dragging entity");
        if let Some(position) = cursor.position {
            transform.translation = position.extend(transform.translation.z);
        }
    }
}
