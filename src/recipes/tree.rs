use crate::{
    machines::manufacturers::ProductionBuilding, pipelines::Pipeline, resources::Resource,
};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

use super::{Recipe, Recipes};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeNode {
    recipe: Rc<Recipe>,
    calculations: HashMap<String, f32>,
    parent: Option<Rc<RefCell<RecipeNode>>>,
    children: Vec<Rc<RefCell<RecipeNode>>>,
}

impl RecipeNode {
    pub fn new(recipe: Rc<Recipe>) -> Self {
        let mut calculations = HashMap::new();
        for (input, input_amount) in recipe.input_items.iter() {
            for (output, output_amount) in recipe.output_items.iter() {
                calculations.insert(
                    format!("{}-{}", input, output),
                    output_amount / input_amount,
                );
            }
        }
        Self {
            recipe,
            calculations,
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn add_parent(&mut self, parent: Rc<RefCell<RecipeNode>>) {
        self.parent = Some(parent);
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

        println!(
            "{}",
            format!("{}|-- {}", &separator.repeat(depth), "Calculations: ").bold()
        );
        for input in self.calculations.iter() {
            let msg = format!("{} |-- {}: {}", &separator.repeat(depth), input.0, input.1).yellow();
            println!("{}", msg);
        }

        for child in self.children.iter() {
            child.as_ref().borrow().simple_display(depth + 1);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeTree {
    root: Rc<RefCell<RecipeNode>>,
    inputs: HashMap<Resource, f32>,
    outputs: HashMap<Resource, f32>,
}

impl RecipeTree {
    pub fn build(resource: Resource, recipes: &Recipes, pipeline: &mut Pipeline) -> Self {
        let mut inputs: HashMap<Resource, f32> = HashMap::new();
        let mut outputs: HashMap<Resource, f32> = HashMap::new();
        let root = Self::create_node(resource, recipes, &mut inputs, &mut outputs, pipeline);
        Self {
            root,
            inputs,
            outputs,
        }
    }

    pub fn simple_display(&self) {
        self.root.as_ref().borrow().simple_display(0);
    }

    fn create_node(
        resource: Resource,
        recipes: &Recipes,
        inputs: &mut HashMap<Resource, f32>,
        outputs: &mut HashMap<Resource, f32>,
        pipeline: &mut Pipeline,
    ) -> Rc<RefCell<RecipeNode>> {
        let resource_recipes: &Vec<Recipe>;
        match recipes.get_component_recipes(resource.clone()) {
            Ok(recipes) => resource_recipes = recipes,
            Err(msg) => panic!("{} || {:?}", msg, resource),
        }
        let recipe = Rc::new(resource_recipes.first().unwrap().clone());

        let node = RecipeNode::new(recipe.clone());
        let node_p = Rc::new(RefCell::new(node));

        let mut children = Vec::new();
        if !recipe.end {
            for (input_resource, _) in recipe.input_items.iter() {
                let child =
                    Self::create_node(input_resource.clone(), recipes, inputs, outputs, pipeline);
                child.as_ref().borrow_mut().add_parent(node_p.clone());
                children.push(child.clone());
            }
        } else {
            for (key, amount) in recipe.input_items.iter() {
                inputs
                    .entry(key.clone())
                    .and_modify(|total| *total += amount)
                    .or_insert(*amount);
            }
            pipeline.add(ProductionBuilding::from_category(
                recipe.production_building.clone(),
                recipe.clone(),
            ));
        }
        for (key, amount) in recipe.output_items.iter() {
            outputs
                .entry(key.clone())
                .and_modify(|total| *total += amount)
                .or_insert(*amount);
        }
        node_p.as_ref().borrow_mut().children = children;
        return node_p;
    }
}
