use std::{borrow::BorrowMut, collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::resources::Resource;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    name: String,
    pub input_items: HashMap<Resource, f32>,
    pub output_items: HashMap<Resource, f32>,
    alternative: bool,
    pub end: bool,
}

impl Recipe {
    pub fn new(
        name: &str,
        input_items: HashMap<Resource, f32>,
        output_items: HashMap<Resource, f32>,
        alternative: bool,
        end: bool,
    ) -> Self {
        Self {
            name: String::from(name),
            input_items: input_items.clone(),
            output_items: output_items.clone(),
            alternative,
            end,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipes {
    lookup: HashMap<Resource, Vec<Recipe>>,
}

impl Recipes {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::new(),
        }
    }

    pub fn build() -> Self {
        let mut s = Self::new();
        Self::build_iron(s.borrow_mut());
        Self::build_copper(s.borrow_mut());
        Self::build_concrete(s.borrow_mut());
        return s;
    }

    pub fn add(&mut self, recipe: Recipe) -> () {
        for (resource, _) in recipe.output_items.iter() {
            let key = resource.clone();
            self.lookup.insert(key, vec![recipe.clone()]);
        }
    }

    pub fn get_component_recipes(&self, recipe_name: Resource) -> Result<&Vec<Recipe>, &str> {
        match self.lookup.get(&recipe_name) {
            Some(recipes) => Ok(recipes),
            None => Err("Cannot find any recipes for the component"),
        }
    }

    fn build_iron(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Iron Ingot",
            HashMap::from([(Resource::IronOre, 30.0)]),
            HashMap::from([(Resource::IronIngot, 30.0)]),
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Iron Plate",
            HashMap::from([(Resource::IronIngot, 30.0)]),
            HashMap::from([(Resource::IronPlate, 20.0)]),
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Iron Rod",
            HashMap::from([(Resource::IronIngot, 15.0)]),
            HashMap::from([(Resource::IronRod, 15.0)]),
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Screws",
            HashMap::from([(Resource::IronIngot, 10.0)]),
            HashMap::from([(Resource::Screws, 40.0)]),
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Reinforced Iron Plate",
            HashMap::from([(Resource::IronPlate, 30.0), (Resource::Screws, 60.0)]),
            HashMap::from([(Resource::ReinforcedIronPlate, 5.0)]),
            false,
            false,
        ));
    }

    fn build_copper(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Copper Ingot",
            HashMap::from([(Resource::CopperOre, 30.0)]),
            HashMap::from([(Resource::CopperIngot, 30.0)]),
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Copper Sheet",
            HashMap::from([(Resource::CopperIngot, 20.0)]),
            HashMap::from([(Resource::CopperSheet, 10.0)]),
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Wire",
            HashMap::from([(Resource::CopperIngot, 15.0)]),
            HashMap::from([(Resource::Wire, 30.0)]),
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Cable",
            HashMap::from([(Resource::Wire, 60.0)]),
            HashMap::from([(Resource::Cable, 30.0)]),
            false,
            true,
        ));
    }

    fn build_concrete(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Concrete",
            HashMap::from([(Resource::Limestone, 45.0)]),
            HashMap::from([(Resource::Concrete, 15.0)]),
            false,
            true,
        ));
    }

    fn build_biofuel(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Biomass (Leaves)",
            HashMap::from([(Resource::Leaves, 120.0)]),
            HashMap::from([(Resource::Biomass, 60.0)]),
            false,
            true,
        ));

        recipes.add(Recipe::new(
            "Biomass (Wood)",
            HashMap::from([(Resource::Wood, 60.0)]),
            HashMap::from([(Resource::Biomass, 300.0)]),
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Biomass (Alien Protein)",
            HashMap::from([(Resource::AlienProtein, 15.0)]),
            HashMap::from([(Resource::Biomass, 1500.0)]),
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Solid Biofuel",
            HashMap::from([(Resource::Biomass, 120.0)]),
            HashMap::from([(Resource::SolidBiofuel, 60.0)]),
            false,
            true,
        ));
    }

    fn build_quartz(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Quartz Crystal",
            HashMap::from([(Resource::QuartzOre, 37.5)]),
            HashMap::from([(Resource::QuartzCrystal, 22.5)]),
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Silica",
            HashMap::from([(Resource::QuartzOre, 22.5)]),
            HashMap::from([(Resource::QuartzCrystal, 37.5)]),
            false,
            true,
        ));
    }

    fn build_caterium(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Caterium Ingot",
            HashMap::from([(Resource::CateriumOre, 45.0)]),
            HashMap::from([(Resource::CateriumIngot, 15.0)]),
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Quickwire",
            HashMap::from([(Resource::CateriumIngot, 12.0)]),
            HashMap::from([(Resource::Quickwire, 60.0)]),
            false,
            true,
        ));
    }

    fn build_steel(recipes: &mut Recipes) {}
}

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeTree {
    root: Rc<RecipeNode>,
}

impl RecipeTree {
    pub fn build(resource: Resource, recipes: &Recipes) -> Self {
        let root = Self::create_node(resource, recipes);
        Self {
            root: Rc::new(root),
        }
    }

    fn create_node(resource: Resource, recipes: &Recipes) -> RecipeNode {
        let resource_recipes: &Vec<Recipe>;
        match recipes.get_component_recipes(resource) {
            Ok(recipes) => resource_recipes = recipes,
            Err(msg) => panic!("{}", msg),
        }
        let recipe = resource_recipes.first().unwrap().clone();
        let mut children = Vec::new();
        if !recipe.end {
            for (input_resource, _) in recipe.input_items.iter() {
                children.push(Rc::new(Self::create_node(input_resource.clone(), recipes)));
            }
        }
        let mut node = RecipeNode::new(Rc::new(recipe));
        node.children = children;
        return node;
    }
}
