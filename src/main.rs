mod aabb;
mod camera;
mod cursor_world_position;
mod desktop;
mod drag_and_drop;
mod ui;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use iyes_loopless::prelude::*;

use aabb::AABB;
use cursor_world_position::CursorWorldPosition;
use drag_and_drop::{ClickEntity, DraggingState, EndDragEntity, HoverEntity, StartDragEntity};
use ui::button::on_button_interaction;
use ui::main_menu::{MainMenu, StartGameButton};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
}

fn main() {
    let mut app = App::new();

    // Resources
    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "TODO: ADD TITLE".to_string(),
            width: 1280.0,
            height: 720.0,
            resizable: false,
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
    app.add_loopless_state(GameState::InGame);

    // Plugins
    app.add_plugins(DefaultPlugins);

    // Setup Systems
    app.add_startup_system(camera::camera_setup)
        .add_startup_system(drag_and_drop::squares_setup);

    // Enter Systems
    app.add_enter_system(GameState::MainMenu, ui::main_menu::menu_setup)
        .add_enter_system_set(
            GameState::InGame,
            SystemSet::new()
                .with_system(desktop::spawn_desktop_background)
                .with_system(desktop::spawn_folders)
                .with_system(desktop::spawn_recycle_bin),
        );

    // Exit Systems
    app.add_exit_system(GameState::MainMenu, despawn_with::<MainMenu>);

    // Systems
    // MainMenu
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::MainMenu)
            .with_system(ui::button::button_interaction_update)
            .with_system(ui::main_menu::start_game.run_if(on_button_interaction::<StartGameButton>))
            .into(),
    );
    // InGame
    app.add_system_set_to_stage(
        CoreStage::PreUpdate,
        ConditionSet::new()
            .run_in_state(GameState::InGame)
            .with_system(aabb::aabb_update)
            .with_system(cursor_world_position::cursor_world_position_update)
            .into(),
    );
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::InGame)
            .with_system(drag_and_drop::mouse_click)
            .with_system(drag_and_drop::draggable_update)
            .into(),
    );

    // Debug
    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    // Run
    app.run();
}

/// Despawn all entities with a given component type
fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
