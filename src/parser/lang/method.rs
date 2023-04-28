use crate::parser::components::desc::Desc;
use crate::parser::components::param::Param;
use crate::parser::components::returns::Returns;
use std::fmt::{Debug, Formatter, Write};
use crate::parser::lang::any::Any;

#[derive(Clone)]
pub struct Method {
    pub desc: Option<Desc>,
    pub returns: Option<Returns>,
    pub params: Vec<Param>,
    pub ufunction: Option<String>,
    pub signature: Option<String>,
    pub name: Option<String>,
}

impl Debug for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(desc) = &self.desc {
            f.write_str(format!("@Desc:\n{}\n", desc.description).as_str())?;
        }

        if let Some(returns) = &self.returns {
            f.write_str(format!("@Returns:\n{:?}\n", returns.desc).as_str())?;
        }

        f.write_str("@Params: \n")?;
        for param in &self.params {
            f.write_str(format!("{:?}\n", param).as_str())?;
        }

        if let Some(signature) = &self.signature {
            f.write_str(format!("@Signautre:\n{}\n", signature).as_str())?;
        }

        Ok(())
    }
}

impl Method {
    pub fn new() -> Self {
        Self {
            desc: None,
            returns: None,
            ufunction: None,
            params: vec![],
            signature: None,
            name: None,
        }
    }

    pub fn from_any(any: Any) -> Self {
        let mut this_method = Self::new();
        this_method.desc = any.desc;
        this_method.returns = any.returns;
        this_method.params = any.params;
        this_method
    }
}
