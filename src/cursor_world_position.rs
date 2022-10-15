use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::camera::MainCamera;

#[derive(Debug, Default, Deref, DerefMut)]
pub struct CursorWorldPosition {
    pub position: Option<Vec2>,
}

pub fn cursor_world_position_update(
    mut cursor: ResMut<CursorWorldPosition>,
    windows: Res<Windows>,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = query_camera.single();
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    let world_pos = wnd.cursor_position().map(|screen_pos| {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let world_pos: Vec2 = world_pos.truncate();
        world_pos
    });

    cursor.position = world_pos;
}
