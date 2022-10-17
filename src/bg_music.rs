
use bevy::prelude::*;
use bevy_kira_audio::{AudioControl, AudioSource, AudioChannel};

pub struct Background;

#[derive(Default)]
pub struct MusicState {
    pub intro_music: Handle<AudioSource>,
    pub in_game_music: Handle<AudioSource>,
}

pub fn load_music(asset_server: Res<AssetServer>, mut music_state: ResMut<MusicState>) {
    music_state.intro_music = asset_server.load("sounds/Windows_XP_Tour_intro.mp3");
    music_state.in_game_music = asset_server.load("sounds/Windows_XP_Tour_3.mp3");
}

pub fn play_intro_music(music_state: Res<MusicState>, background: Res<AudioChannel<Background>>) {
    background.stop();
    background
        .play(music_state.intro_music.clone())
        .with_volume(0.2)
        .looped();
}

pub fn play_in_game_music(music_state: Res<MusicState>, background: Res<AudioChannel<Background>>) {
    background.stop();
    background
        .play(music_state.in_game_music.clone())
        .with_volume(0.2)
        .looped();
}
