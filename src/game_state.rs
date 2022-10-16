use bevy::prelude::*;
use iyes_loopless::prelude::NextState;
use bevy::ecs::system::Resource;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    MainDialog,
    InGame,
    EndMenu,
}

/// Despawn all entities with a given component type
pub fn despawn<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn init_resource<R: Resource + Default>(mut commands: Commands) {
    commands.insert_resource(R::default());
}

pub fn to_main_menu(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::MainMenu));
}

pub fn to_main_dialog(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::MainDialog));
}

pub fn to_in_game(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::InGame));
}

pub fn to_end_menu(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::EndMenu));
}
