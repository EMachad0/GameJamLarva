use bevy::prelude::*;
use iyes_loopless::prelude::NextState;
use bevy::ecs::system::Resource;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    MainDialog,
    InGame,
}

/// Despawn all entities with a given component type
pub fn despawn<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn to_main_dialog(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::MainDialog));
}

pub fn to_in_game(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::InGame));
}

pub fn init_resource<R: Resource + Default>(mut commands: Commands) {
    commands.insert_resource(R::default());
}
