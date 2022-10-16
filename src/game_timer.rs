use bevy::ecs::system::Resource;
use bevy::prelude::*;
use std::time::Duration;

pub trait Tickable {
    fn do_tick(&mut self, delta: Duration);
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct PreGameTimer {
    pub timer: Timer,
}

impl Tickable for PreGameTimer {
    fn do_tick(&mut self, delta: Duration) {
        self.tick(delta);
    }
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct GameTimer {
    pub timer: Timer,
}

impl Tickable for GameTimer {
    fn do_tick(&mut self, delta: Duration) {
        self.tick(delta);
    }
}

pub fn pre_game_timer_setup(mut commands: Commands) {
    commands.insert_resource(PreGameTimer {
        timer: Timer::new(Duration::from_secs(10), false),
    });
}

pub fn game_timer_setup(mut commands: Commands) {
    commands.insert_resource(GameTimer {
        timer: Timer::new(Duration::from_secs(300), false),
    });
}

pub fn tick<R: Resource + Tickable>(time: Res<Time>, mut res: ResMut<R>) {
    res.do_tick(time.delta());
}

pub fn pre_game_timer_finished(timer: Res<PreGameTimer>) -> bool {
    debug!("{:?}", timer);
    timer.finished()
}

pub fn game_timer_finished(timer: Res<GameTimer>) -> bool {
    timer.finished()
}
