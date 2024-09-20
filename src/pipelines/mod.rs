use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::recipes::Recipe;

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineNode {
    recipe: Box<Recipe>,
    parents: Vec<Box<PipelineNode>>,
    children: Vec<Box<PipelineNode>>,
}

impl PipelineNode {
    pub fn new(recipe: Box<Recipe>) -> Self {
        Self {
            recipe,
            parents: Vec::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline {
    root: Rc<PipelineNode>,
}

impl Pipeline {}
