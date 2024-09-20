use crate::resources::Resource;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};

use super::{Recipe, Recipes};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeNode {
    recipe: Rc<Recipe>,
    children: Vec<Rc<RecipeNode>>,
}

impl RecipeNode {
    pub fn new(recipe: Rc<Recipe>) -> Self {
        Self {
            recipe,
            children: Vec::new(),
        }
    }

    pub fn simple_display(&self, depth: usize) {
        let separator = "   ";
        let msg = format!(
            "{}{} ({})",
            &separator.repeat(depth),
            &self.recipe.name,
            &self.recipe.production_building
        )
        .green();
        println!("{}", msg);

        println!(
            "{}",
            format!("{}|-- {}", &separator.repeat(depth), "Output: ").bold()
        );
        for output in self.recipe.output_items.iter() {
            let msg = format!(
                "{} |-- {}: {}",
                &separator.repeat(depth),
                output.0,
                output.1
            )
            .blue();
            println!("{}", msg);
        }

        println!(
            "{}",
            format!("{}|-- {}", &separator.repeat(depth), "Input: ").bold()
        );
        for input in self.recipe.input_items.iter() {
            let msg = format!("{} |-- {}: {}", &separator.repeat(depth), input.0, input.1).red();
            println!("{}", msg);
        }
        for child in self.children.iter() {
            child.simple_display(depth + 1);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeTree {
    root: Rc<RecipeNode>,
    inputs: HashMap<Resource, f32>,
    outputs: HashMap<Resource, f32>,
}

impl RecipeTree {
    pub fn build(resource: Resource, recipes: &Recipes) -> Self {
        let mut inputs: HashMap<Resource, f32> = HashMap::new();
        let mut outputs: HashMap<Resource, f32> = HashMap::new();
        let root = Rc::new(Self::create_node(
            resource,
            recipes,
            &mut inputs,
            &mut outputs,
        ));
        Self {
            root,
            inputs,
            outputs,
        }
    }

    pub fn simple_display(&self) {
        self.root.as_ref().simple_display(0);
    }

    fn create_node(
        resource: Resource,
        recipes: &Recipes,
        inputs: &mut HashMap<Resource, f32>,
        outputs: &mut HashMap<Resource, f32>,
    ) -> RecipeNode {
        let resource_recipes: &Vec<Recipe>;
        match recipes.get_component_recipes(resource.clone()) {
            Ok(recipes) => resource_recipes = recipes,
            Err(msg) => panic!("{} || {:?}", msg, resource),
        }
        let recipe = resource_recipes.first().unwrap().clone();
        let mut children = Vec::new();
        if !recipe.end {
            for (input_resource, _) in recipe.input_items.iter() {
                children.push(Rc::new(Self::create_node(
                    input_resource.clone(),
                    recipes,
                    inputs,
                    outputs,
                )));
            }
        }
        for (key, amount) in recipe.input_items.iter() {
            inputs
                .entry(key.clone())
                .and_modify(|total| *total += amount)
                .or_insert(*amount);
        }
        for (key, amount) in recipe.output_items.iter() {
            outputs
                .entry(key.clone())
                .and_modify(|total| *total += amount)
                .or_insert(*amount);
        }
        let mut node = RecipeNode::new(Rc::new(recipe));
        node.children = children;
        return node;
    }
}
