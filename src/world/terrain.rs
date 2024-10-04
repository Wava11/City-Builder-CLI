#[derive(Clone, Copy)]
pub enum TerrainType {
    Ground,
    Water,
}

impl TerrainType {
    pub fn to_char(&self) -> char {
        match self {
            Self::Ground => '.',
            Self::Water => '*',
        }
    }
}
