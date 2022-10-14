use bevy::prelude::*;
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
