use std::{fmt::Display, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::recipes::Recipe;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Category {
    Assembler,
    Blender,
    Constructor,
    Foundry,
    Manufacturer,
    Packager,
    ParticleAccelerator,
    Refinery,
    Smelter,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductionBuilding {
    pub category: Category,
    pub power_usage: usize,
    pub recipe: Rc<Recipe>,
    pub amount: usize,
}

impl ProductionBuilding {
    pub fn new(category: Category, power_usage: usize, recipe: Rc<Recipe>, amount: usize) -> Self {
        Self {
            category,
            power_usage,
            recipe,
            amount,
        }
    }

    pub fn from_category(category: Category, recipe: Rc<Recipe>) -> Self {
        match category {
            Category::Assembler => Self {
                category,
                power_usage: 15,
                recipe,
                amount: 0,
            },
            Category::Blender => Self {
                category,
                power_usage: 75,
                recipe,
                amount: 0,
            },
            Category::Constructor => Self {
                category,
                power_usage: 4,
                recipe,
                amount: 0,
            },
            Category::Foundry => Self {
                category,
                power_usage: 16,
                recipe,
                amount: 0,
            },
            Category::Manufacturer => Self {
                category,
                power_usage: 55,
                recipe,
                amount: 0,
            },
            Category::Packager => Self {
                category,
                power_usage: 10,
                recipe,
                amount: 0,
            },
            Category::ParticleAccelerator => Self {
                category,
                power_usage: 1500,
                recipe,
                amount: 0,
            },
            Category::Refinery => Self {
                category,
                power_usage: 30,
                recipe,
                amount: 0,
            },
            Category::Smelter => Self {
                category,
                power_usage: 4,
                recipe,
                amount: 0,
            },
        }
    }
}


impl Display for ProductionBuilding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input = self.recipe.input_items.iter().next().unwrap();
        let output = self.recipe.output_items.iter().next().unwrap();
        write!(
            f,
            "({}:{})-[{}]-({}:{})",
            input.0, input.1, self.recipe.name, output.0, output.1
        )
    }
}