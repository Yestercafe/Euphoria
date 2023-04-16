use crate::sh_pair::SHPair;

pub struct Doc {
    doc_name: String,
    sh_pair: SHPair,
}

impl Doc {
    pub fn new(doc_name: String, sh_pair: SHPair) -> Self {
        Self {
            doc_name,
            sh_pair,
        }
    }

    pub fn generate(self) {
        
    }
}
