use std::{fmt::Display, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{machines::manufacturers::ProductionBuilding, recipes::Recipe};

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineNode {
    recipe: Rc<ProductionBuilding>,
    parent: Option<Rc<PipelineNode>>,
}

impl Display for PipelineNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut node = self;
        let d: String = format!("{}", &node.recipe);
        while let Some(parent) = node.parent {
            node = parent;
            d.push_str(format!("{}", &node.recipe).as_str());
        }
        write!(f, "{}", d)
    }
}

impl PipelineNode {
    pub fn new(recipe: Rc<Recipe>) -> Self {
        Self {
            recipe,
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
