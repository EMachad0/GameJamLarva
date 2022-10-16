mod aabb;
mod biome;
mod camera;
mod cursor_world_position;
mod desktop;
mod drag_and_drop;
mod game_state;
mod image_spawner;
mod ui;

use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use image_spawner::ImageTimer;
use iyes_loopless::prelude::*;

use game_state::despawn;
use game_state::GameState;

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
        .init_resource::<cursor_world_position::CursorWorldPosition>()
        .init_resource::<drag_and_drop::DraggingState>()
        .insert_resource(ImageTimer(Timer::from_seconds(2.0, true)));

    // Types
    app.register_type::<aabb::AABB>();

    // Events
    app.add_event::<drag_and_drop::ClickEntity>()
        .add_event::<drag_and_drop::HoverEntity>()
        .add_event::<drag_and_drop::StartDragEntity>()
        .add_event::<drag_and_drop::EndDragEntity>();

    // Stages
    app.add_loopless_state(GameState::InGame);

    // Plugins
    app.add_plugins(DefaultPlugins);

    // Setup Systems
    app.add_startup_system(camera::camera_setup);
    app.add_startup_system_set(
        SystemSet::new()
            .with_system(ui::main_menu::main_menu_background_load)
            .with_system(ui::loading::loading_background_load)
            .with_system(image_spawner::load_images),
    );

    // Enter Systems
    app.add_enter_system_set(
        GameState::MainMenu,
        SystemSet::new()
            .with_system(ui::main_menu::main_menu_background_setup)
            .with_system(ui::main_menu::main_menu_ui_setup),
    )
    .add_enter_system_set(
        GameState::MainDialog,
        SystemSet::new()
            .with_system(ui::loading::loading_background_setup)
            .with_system(ui::main_dialog::main_dialog_ui_setup),
    )
    .add_enter_system_set(
        GameState::InGame,
        SystemSet::new()
            .with_system(desktop::spawn_desktop_background)
            .with_system(desktop::spawn_folders)
            .with_system(desktop::spawn_recycle_bin),
    );

    // Exit Systems
    app.add_exit_system_set(
        GameState::MainMenu,
        SystemSet::new()
            .with_system(despawn::<ui::main_menu::MainMenuBackground>)
            .with_system(despawn::<ui::main_menu::MainMenuUi>),
    )
    .add_exit_system_set(
        GameState::MainDialog,
        SystemSet::new()
            .with_system(despawn::<ui::loading::LoadingBackground>)
            .with_system(despawn::<ui::main_dialog::MainDialogUi>),
    );

    // Systems
    // MainMenu
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::MainMenu)
            .with_system(ui::button::button_interaction_update)
            .with_system(
                game_state::to_main_dialog
                    .run_if(ui::button::on_button_interaction::<ui::main_menu::StartGameButton>),
            )
            .into(),
    );
    // MainDialog
    app.add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::MainDialog)
            .with_system(ui::typewriter::typewriter_update)
            .with_system(ui::typewriter::finished_typewriter_update)
            .with_system(ui::typewriter::typewriter_skip_input)
            .with_system(
                game_state::to_in_game
                    .run_if(ui::typewriter::after_typewriter_finish::<ui::main_dialog::MainDialog>),
            )
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
    )
    .add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::InGame)
            .with_system(drag_and_drop::mouse_click)
            .with_system(drag_and_drop::draggable_update)
            .with_system(image_spawner::spawn_image)
            .with_system(image_spawner::image_drag)
            .with_system(desktop::hover_folder)
            .with_system(image_spawner::image_drop)
            .into(),
    );

    // Debug
    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    // Run
    app.run();
}
