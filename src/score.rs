
use bevy::prelude::*;

use crate::biome::Biome;

#[derive(Default)]
pub struct Score {
    pub right: u32,
    pub wrong: u32,
    pub images_spawned: u32,
    pub biome_score: BiomeScore,
}

impl Score {
    pub fn total_accuracy(&self) -> f32 {
        self.right as f32 / self.images_spawned as f32
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Score Total: {}\nNumber of Instances: {}\nClassified Wrong: {}\nTotal Accuracy: {}\n{}",
               self.right,
               self.images_spawned,
               self.wrong,
               self.total_accuracy(),
               self.biome_score,
        )
    }
}

#[derive(Default)]
pub struct BiomeScore {
    pub amazonia: u32,
    pub caatinga: u32,
    pub cerrado: u32,
    pub mataatlantica: u32,
    pub pampas: u32,
    pub pantanal: u32,
}

impl BiomeScore {
    pub fn array(&self) -> [u32; 6] {
        [
            self.amazonia,
            self.caatinga,
            self.cerrado,
            self.mataatlantica,
            self.pampas,
            self.pantanal
        ]
    }
}

impl std::fmt::Display for BiomeScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Right Guesses for:\n\tamazonia: {}\n\tcaatinga: {}\n\tcerrado: {}\n\tmataatlantica: {}\n\tpampas: {}\n\tpantanal: {}\n\t",
            self.amazonia,
            self.caatinga,
            self.cerrado,
            self.mataatlantica,
            self.pampas,
            self.pantanal
        )
    }
}

impl BiomeScore {
    pub fn increment_biome(&mut self, biome: &Biome) {
        match biome {
            Biome::AMAZONIA => self.amazonia += 1,
            Biome::CAAATINGA => self.caatinga += 1,
            Biome::CERRADO => self.cerrado += 1,
            Biome::MATAATLANTICA => self.mataatlantica += 1,
            Biome::PAMPAS => self.pampas += 1,
            Biome::PANTANAL => self.pantanal += 1,
        }
    }
}

impl Score {
    pub fn reset(&mut self) {
        self.right = 0;
        self.images_spawned = 0;
        self.wrong = 0;
        self.biome_score = BiomeScore::default();
    }
}

pub fn start_score(mut score: ResMut<Score>) {
    score.reset();
}
