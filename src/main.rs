use bevy::prelude::*;
use bevy::render::camera::WindowOrigin;
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    // Resources
    app.insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            title: "TODO: ADD TITLE".to_string(),
            canvas: Some("#bevy".to_string()),
            ..default()
        });

    // Stages

    // Plugins
    app.add_plugins(DefaultPlugins);

    // Types

    // Setup Systems
    app.add_startup_system(camera_setup);

    // Enter Systems

    // Exit Systems

    // Systems

    // Debug
    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    // Run
    app.run();
}

pub fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            window_origin: WindowOrigin::BottomLeft,
            ..default()
        },
        ..default()
    });
}
