use bevy::prelude::Component;

use crate::biome::Biome;

#[derive(Component)]
pub struct ImageBiome {
    pub biome: Biome,
}
