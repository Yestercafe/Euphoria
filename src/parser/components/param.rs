use std::fmt::{Debug, Formatter, Write};

#[derive(Clone)]
pub struct Param {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Debug for Param {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            f.write_str(format!("p@{}", name).as_str())?;
        } else {
            f.write_str("p@MISSING_PARAM_NAME")?;
        }
        if let Some(description) = &self.description {
            f.write_str(format!("@description:\n{}", description).as_str())?;
        }

        Ok(())
    }
}

impl Param {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
        }
    }
}
