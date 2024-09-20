use serde::{Deserialize, Serialize};

use crate::{
    recipes::{Recipe, Recipes},
    resources::Resource,
};

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
    root: Box<PipelineNode>,
}

impl Pipeline {
    pub fn build(resource: Resource, recipes: &Recipes) -> Self {
        let root = Self::create_node(resource, recipes);
        if !root.recipe.end {
            for (input_resource, amount) in root.recipe.input_items.iter() {
                println!("{:#?}", input_resource);
                println!("{:#?}", amount);
            }
        }
        Self {
            root: Box::new(root),
        }
    }

    fn create_node(resource: Resource, recipes: &Recipes) -> PipelineNode {
        let resource_recipes: &Vec<Recipe>;
        match recipes.get_component_recipes(resource.clone()) {
            Ok(recipes) => resource_recipes = recipes,
            Err(msg) => panic!("{} || {:?}", msg, resource),
        }
        PipelineNode::new(Box::new(resource_recipes.first().unwrap().clone()))
    }
}
