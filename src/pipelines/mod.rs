use serde::{Deserialize, Serialize};
use std::{fmt::Display, rc::Rc};

use crate::machines::manufacturers::ProductionBuilding;
use crate::resources::Resource;

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineNode {
    building: ProductionBuilding,
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
            }
            None => {}
        };
        write!(f, "{}", d)
    }
}

impl PipelineNode {
    pub fn new(building: ProductionBuilding) -> Self {
        Self {
            building,
            parent: None,
        }
    }

    pub fn calculate(&self, input: Vec<(Resource, f32)>) -> f32 {
        let result: f32 = 0.0;
        for (resource, amount) in input.iter() {

        }
        return result;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline {
    root: Vec<Rc<PipelineNode>>,
    total_power: usize,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            root: Vec::new(),
            total_power: 0,
        }
    }

    pub fn add(&mut self, building: ProductionBuilding) {
        self.total_power += building.power_usage;
        let pipeline = PipelineNode::new(building);
        self.root.push(Rc::new(pipeline));
    }
}
