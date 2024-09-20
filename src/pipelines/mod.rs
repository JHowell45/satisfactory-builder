use std::{fmt::Display, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{machines::manufacturers::ProductionBuilding, recipes::Recipe};

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineNode {
    building: Rc<ProductionBuilding>,
    parent: Option<Rc<PipelineNode>>,
}

impl Display for PipelineNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d: String = format!("{}", &self.building);
        match &self.parent {
            Some(parent) => {
                let mut node = parent;
                while let Some(parent) = &node.parent {
                    node = &parent;
                    d.push_str(format!("{}", &node.building).as_str());
                }
            },
            None => {},
        };
        write!(f, "{}", d)
    }
}

impl PipelineNode {
    pub fn new(building: Rc<ProductionBuilding>) -> Self {
        Self {
            building,
            parent: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline {
    root: Rc<PipelineNode>,
}

impl Display for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d: String = String::new();
        write!(f, "{}", d)
    }
}

impl Pipeline {}
