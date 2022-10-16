mod aabb;
mod biome;
mod camera;
mod cursor_world_position;
mod desktop;
mod drag_and_drop;
mod game_state;
mod game_timer;
mod image;
mod score;
mod ui;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
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
        .init_resource::<score::Score>()
        .init_resource::<game_timer::PreGameTimer>()
        .init_resource::<game_timer::GameTimer>()
        .init_resource::<ui::main_dialog::MainDialogStatus>()
        .init_resource::<ui::tutorial_dialog::TutorialDialogStatus>()
        .init_resource::<ui::timer_dialog::TimerDialogStatus>();

    // Types
    app.register_type::<aabb::AABB>();

    // Events
    app.add_event::<drag_and_drop::ClickEntity>()
        .add_event::<drag_and_drop::HoverEntity>()
        .add_event::<drag_and_drop::StartDragEntity>()
        .add_event::<drag_and_drop::EndDragEntity>();

    // Stages
    app.add_loopless_state(GameState::EndMenu);

    // Plugins
    app.add_plugins(DefaultPlugins);

    // Setup Systems
    app.add_startup_system_to_stage(StartupStage::PreStartup, camera::camera_setup)
        .add_startup_system_to_stage(StartupStage::PreStartup, ui::root_ui::ui_setup)
        .add_startup_system(ui::dialog::dialog_ui_setup)
        .add_startup_system_set(
            SystemSet::new()
                .with_system(ui::main_menu::main_menu_background_load)
                .with_system(ui::loading::loading_background_load)
                .with_system(image::load_images),
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
            .with_system(game_state::init_resource::<ui::main_dialog::MainDialogStatus>),
    )
    .add_enter_system_set(
        GameState::InGame,
        SystemSet::new()
            .with_system(game_state::init_resource::<image::ImageTimer>)
            .with_system(game_state::init_resource::<ui::tutorial_dialog::TutorialDialogStatus>)
            .with_system(game_state::init_resource::<ui::timer_dialog::TimerDialogStatus>)
            .with_system(game_timer::pre_game_timer_setup)
            .with_system(game_timer::game_timer_setup)
            .with_system(ui::game_timer::game_timer_ui_setup)
            .with_system(desktop::spawn_desktop_background)
            .with_system(desktop::spawn_folders)
            .with_system(desktop::spawn_recycle_bin)
            .with_system(score::start_score),
    )
    .add_enter_system_set(
        GameState::EndMenu,
        SystemSet::new()
            .with_system(ui::end_menu::end_game_ui_setup)
            .with_system(desktop::spawn_desktop_background),
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
        SystemSet::new().with_system(despawn::<ui::loading::LoadingBackground>),
    )
    .add_exit_system_set(
        GameState::InGame,
        SystemSet::new()
            .with_system(despawn::<desktop::DesktopBackground>)
            .with_system(despawn::<ui::game_timer::GameTimerUi>)
            .with_system(despawn::<image::SpawnedImage>)
            .with_system(despawn::<desktop::Folder>)
            .with_system(despawn::<desktop::RecycleBin>),
    )
    .add_exit_system_set(
        GameState::EndMenu,
        SystemSet::new()
            .with_system(despawn::<desktop::DesktopBackground>)
            .with_system(despawn::<ui::end_menu::EndMenuUi>),
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
            .with_system(ui::main_dialog::main_dialog_update)
            .with_system(game_state::to_in_game.run_if(ui::main_dialog::main_dialog_finished))
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
            .with_system(
                ui::tutorial_dialog::tutorial_dialog_update
                    .run_if_not(ui::tutorial_dialog::tutorial_finished),
            )
            .with_system(ui::typewriter::typewriter_update)
            .with_system(ui::typewriter::finished_typewriter_update)
            .with_system(ui::typewriter::typewriter_skip_input)
            .with_system(drag_and_drop::mouse_click)
            .with_system(drag_and_drop::draggable_update)
            .with_system(image::image_drag)
            .with_system(image::image_drop)
            .with_system(image::sprite_alpha_update)
            .with_system(desktop::folder_state_coloring)
            .into(),
    )
    .add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::InGame)
            .run_if(ui::tutorial_dialog::tutorial_finished)
            .with_system(image::spawn_image)
            .with_system(game_timer::tick::<game_timer::PreGameTimer>)
            .into(),
    )
    .add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::InGame)
            .run_if(game_timer::pre_game_timer_finished)
            .with_system(game_timer::tick::<game_timer::GameTimer>)
            .with_system(ui::game_timer::game_timer_ui_update)
            .with_system(
                ui::timer_dialog::timer_dialog_update
                    .run_if_not(ui::timer_dialog::timer_dialog_finished),
            )
            .with_system(game_state::to_end_menu.run_if(game_timer::game_timer_finished))
            .into(),
    )
    // EndMenu
    .add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::EndMenu)
            .with_system(ui::button::button_interaction_update)
            .with_system(
                game_state::to_main_menu
                    .run_if(ui::button::on_button_interaction::<ui::end_menu::BackToMenuButton>),
            )
            .into(),
    );

    // Debug
    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    // Run
    app.run();
}
