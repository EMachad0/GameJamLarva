#[derive(Clone, Copy, PartialEq)]
pub enum Biome {
    AMAZONIA,
    CAAATINGA,
    CERRADO,
    MATAATLANTICA,
    PAMPAS,
    PANTANAL,
}

impl Biome {
    pub fn iterator() -> impl Iterator<Item = Biome> {
        static BIOME: [Biome; 6] = [
            Biome::PAMPAS,
            Biome::CERRADO,
            Biome::AMAZONIA,
            Biome::CAAATINGA,
            Biome::MATAATLANTICA,
            Biome::PANTANAL,
        ];
        BIOME.iter().copied()
    }
}

impl std::fmt::Display for Biome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AMAZONIA => write!(f, "amazonia"),
            Self::CAAATINGA => write!(f, "caatinga"),
            Self::CERRADO => write!(f, "cerrado"),
            Self::MATAATLANTICA => write!(f, "mataatlantica"),
            Self::PAMPAS => write!(f, "pampas"),
            Self::PANTANAL => write!(f, "pantanal"),
        }
    }
}
