pub mod tree;

use crate::{machines::manufacturers::Category, resources::Resource};
use serde::{Deserialize, Serialize};
use std::{borrow::BorrowMut, collections::HashMap};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    name: String,
    pub input_items: HashMap<Resource, f32>,
    pub output_items: HashMap<Resource, f32>,
    pub production_building: Category,
    alternative: bool,
    pub end: bool,
}

impl Recipe {
    pub fn new(
        name: &str,
        input_items: HashMap<Resource, f32>,
        output_items: HashMap<Resource, f32>,
        production_building: Category,
        alternative: bool,
        end: bool,
    ) -> Self {
        Self {
            name: String::from(name),
            input_items: input_items.clone(),
            output_items: output_items.clone(),
            production_building: production_building,
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
        Self::build_biofuel(s.borrow_mut());
        Self::build_quartz(s.borrow_mut());
        Self::build_caterium(s.borrow_mut());
        Self::build_steel(s.borrow_mut());
        Self::build_frames(s.borrow_mut());
        Self::build_motors(s.borrow_mut());

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
            Category::Smelter,
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Iron Plate",
            HashMap::from([(Resource::IronIngot, 30.0)]),
            HashMap::from([(Resource::IronPlate, 20.0)]),
            Category::Constructor,
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Iron Rod",
            HashMap::from([(Resource::IronIngot, 15.0)]),
            HashMap::from([(Resource::IronRod, 15.0)]),
            Category::Constructor,
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Screws",
            HashMap::from([(Resource::IronIngot, 10.0)]),
            HashMap::from([(Resource::Screws, 40.0)]),
            Category::Constructor,
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Reinforced Iron Plate",
            HashMap::from([(Resource::IronPlate, 30.0), (Resource::Screws, 60.0)]),
            HashMap::from([(Resource::ReinforcedIronPlate, 5.0)]),
            Category::Assembler,
            false,
            false,
        ));
    }

    fn build_copper(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Copper Ingot",
            HashMap::from([(Resource::CopperOre, 30.0)]),
            HashMap::from([(Resource::CopperIngot, 30.0)]),
            Category::Smelter,
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Copper Sheet",
            HashMap::from([(Resource::CopperIngot, 20.0)]),
            HashMap::from([(Resource::CopperSheet, 10.0)]),
            Category::Constructor,
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Wire",
            HashMap::from([(Resource::CopperIngot, 15.0)]),
            HashMap::from([(Resource::Wire, 30.0)]),
            Category::Constructor,
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Cable",
            HashMap::from([(Resource::Wire, 60.0)]),
            HashMap::from([(Resource::Cable, 30.0)]),
            Category::Constructor,
            false,
            false,
        ));
    }

    fn build_concrete(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Concrete",
            HashMap::from([(Resource::Limestone, 45.0)]),
            HashMap::from([(Resource::Concrete, 15.0)]),
            Category::Constructor,
            false,
            true,
        ));
    }

    fn build_biofuel(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Biomass (Leaves)",
            HashMap::from([(Resource::Leaves, 120.0)]),
            HashMap::from([(Resource::Biomass, 60.0)]),
            Category::Constructor,
            false,
            true,
        ));

        recipes.add(Recipe::new(
            "Biomass (Wood)",
            HashMap::from([(Resource::Wood, 60.0)]),
            HashMap::from([(Resource::Biomass, 300.0)]),
            Category::Constructor,
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Biomass (Alien Protein)",
            HashMap::from([(Resource::AlienProtein, 15.0)]),
            HashMap::from([(Resource::Biomass, 1500.0)]),
            Category::Constructor,
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Solid Biofuel",
            HashMap::from([(Resource::Biomass, 120.0)]),
            HashMap::from([(Resource::SolidBiofuel, 60.0)]),
            Category::Constructor,
            false,
            false,
        ));
    }

    fn build_quartz(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Quartz Crystal",
            HashMap::from([(Resource::QuartzOre, 37.5)]),
            HashMap::from([(Resource::QuartzCrystal, 22.5)]),
            Category::Constructor,
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Silica",
            HashMap::from([(Resource::QuartzOre, 22.5)]),
            HashMap::from([(Resource::QuartzCrystal, 37.5)]),
            Category::Constructor,
            false,
            true,
        ));
    }

    fn build_caterium(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Caterium Ingot",
            HashMap::from([(Resource::CateriumOre, 45.0)]),
            HashMap::from([(Resource::CateriumIngot, 15.0)]),
            Category::Smelter,
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Quickwire",
            HashMap::from([(Resource::CateriumIngot, 12.0)]),
            HashMap::from([(Resource::Quickwire, 60.0)]),
            Category::Constructor,
            false,
            false,
        ));
    }

    fn build_steel(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Steel Ingot",
            HashMap::from([(Resource::IronOre, 45.0), (Resource::Coal, 45.0)]),
            HashMap::from([(Resource::SteelIngot, 45.0)]),
            Category::Foundry,
            false,
            true,
        ));
        recipes.add(Recipe::new(
            "Steel Pipe",
            HashMap::from([(Resource::SteelIngot, 30.0)]),
            HashMap::from([(Resource::SteelPipe, 20.0)]),
            Category::Constructor,
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Steel Beam",
            HashMap::from([(Resource::SteelIngot, 60.0)]),
            HashMap::from([(Resource::SteelBeam, 15.0)]),
            Category::Constructor,
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Encased Industrial Beam",
            HashMap::from([(Resource::SteelBeam, 24.0), (Resource::Concrete, 30.0)]),
            HashMap::from([(Resource::EncasedIndustrialBeam, 6.0)]),
            Category::Assembler,
            false,
            false,
        ));
    }

    fn build_frames(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Modular Frame",
            HashMap::from([
                (Resource::ReinforcedIronPlate, 3.0),
                (Resource::IronRod, 12.0),
            ]),
            HashMap::from([(Resource::ModularFrame, 2.0)]),
            Category::Assembler,
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Heavy Modular Frame",
            HashMap::from([
                (Resource::ModularFrame, 10.0),
                (Resource::EncasedIndustrialBeam, 10.0),
                (Resource::SteelPipe, 30.0),
                (Resource::Screws, 200.0),
            ]),
            HashMap::from([(Resource::HeavyModularFrame, 2.0)]),
            Category::Manufacturer,
            false,
            false,
        ));
    }

    fn build_motors(recipes: &mut Recipes) {
        recipes.add(Recipe::new(
            "Rotor",
            HashMap::from([(Resource::IronRod, 20.0), (Resource::Screws, 100.0)]),
            HashMap::from([(Resource::Rotor, 4.0)]),
            Category::Assembler,
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Stator",
            HashMap::from([(Resource::SteelPipe, 15.0), (Resource::Wire, 40.0)]),
            HashMap::from([(Resource::Stator, 5.0)]),
            Category::Assembler,
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Motor",
            HashMap::from([(Resource::Rotor, 10.0), (Resource::Stator, 10.0)]),
            HashMap::from([(Resource::Motor, 5.0)]),
            Category::Assembler,
            false,
            false,
        ));
        recipes.add(Recipe::new(
            "Turbo Motor",
            HashMap::from([
                (Resource::Motor, 7.5),
                (Resource::CoolingSystem, 7.5),
                (Resource::RadioControlUnit, 3.75),
                (Resource::Rubber, 45.0),
            ]),
            HashMap::from([(Resource::TurboMotor, 1.875)]),
            Category::Assembler,
            false,
            false,
        ));
    }
}
