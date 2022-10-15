mod aabb;
mod camera;
mod cursor_world_position;
mod desktop;
mod drag_and_drop;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use aabb::AABB;
use cursor_world_position::CursorWorldPosition;
use drag_and_drop::{ClickEntity, DraggingState, EndDragEntity, HoverEntity, StartDragEntity};

fn main() {
    let mut app = App::new();

    // Resources
    app.insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            title: "TODO: ADD TITLE".to_string(),
            canvas: Some("#bevy".to_string()),
            ..default()
        })
        .init_resource::<CursorWorldPosition>()
        .init_resource::<DraggingState>();

    // Types
    app.register_type::<AABB>();

    // Events
    app.add_event::<ClickEntity>()
        .add_event::<HoverEntity>()
        .add_event::<StartDragEntity>()
        .add_event::<EndDragEntity>();

    // Stages

    // Plugins
    app.add_plugins(DefaultPlugins);

    // Setup Systems
    app.add_startup_system(camera::camera_setup)
        .add_startup_system(drag_and_drop::squares_setup)
        .add_startup_system(desktop::spawn_desktop_background)
        .add_startup_system(desktop::spawn_folders)
        .add_startup_system(desktop::spawn_recycle_bin);

    // Enter Systems

    // Exit Systems

    // Systems
    app.add_system_to_stage(CoreStage::PreUpdate, aabb::aabb_update)
        .add_system_to_stage(
            CoreStage::PreUpdate,
            cursor_world_position::cursor_world_position_update,
        );

    app.add_system(drag_and_drop::mouse_click)
        .add_system(drag_and_drop::draggable_update);

    // Debug
    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    // Run
    app.run();
}
