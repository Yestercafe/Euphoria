use crate::source::Source;

#[derive(Debug, Clone)]
pub struct SHPair {
    pub source: Option<Source>,
    pub header: Option<Source>,
}

impl SHPair {
    pub fn new() -> Self {
        Self {
            source: None,
            header: None,
        }
    }
}
