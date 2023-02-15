use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone, Copy)]
pub enum Rule {
    #[default]
    Vanilla,
    Chunk,
    Tripod,
}

#[derive(Debug, Deserialize, Default, Clone, Copy)]
pub struct StageRule {
    hex_per_side: usize,
    rule: Rule,
}
