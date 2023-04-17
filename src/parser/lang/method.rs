use crate::parser::components::desc::Desc;
use crate::parser::components::param::Param;
use crate::parser::components::returns::Returns;
use std::fmt::{format, Debug, Formatter, Write};

#[derive(Clone)]
pub struct Method {
    pub signature: Option<String>,
    pub desc: Option<Desc>,
    pub returns: Option<Returns>,
    pub params: Vec<Param>,
}

impl Debug for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(signature) = &self.signature {
            f.write_str(format!("@Signautre:\n{}\n", signature).as_str())?;
        }
        if let Some(desc) = &self.desc {
            f.write_str(format!("@Desc:\n{}\n", desc.description).as_str())?;
        }
        if let Some(returns) = &self.returns {
            f.write_str(format!("@Returns:\n{}\n", returns.description).as_str())?;
        }

        f.write_str("@Params: \n")?;
        for param in &self.params {
            f.write_str(format!("{:?}\n", param).as_str())?;
        }

        Ok(())
    }
}

impl Method {
    pub fn new() -> Self {
        Self {
            signature: None,
            desc: None,
            returns: None,
            params: vec![],
        }
    }
}
