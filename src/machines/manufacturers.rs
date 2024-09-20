use std::fmt::Display;

use serde::{Deserialize, Serialize};

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
pub struct ProductionBuilding<'a> {
    pub category: Category,
    pub power_usage: i16,
    pub recipe_name: &'a str,
    pub amount: usize,
}

impl<'a> ProductionBuilding<'a> {
    pub fn new(category: Category, power_usage: i16, recipe_name: &'a str, amount: usize) -> Self {
        Self {
            category,
            power_usage,
            recipe_name,
            amount
        }
    }

    pub fn from_category(category: Category, recipe_name: &'a str) -> Self {
        match category {
            Category::Assembler => Self {
                category,
                power_usage: 15,
                recipe_name,
                amount: 0,
            },
            Category::Blender => Self {
                category,
                power_usage: 75,
                recipe_name,
                amount: 0,
            },
            Category::Constructor => Self {
                category,
                power_usage: 4,
                recipe_name,
                amount: 0,
            },
            Category::Foundry => Self {
                category,
                power_usage: 16,
                recipe_name,
                amount: 0,
            },
            Category::Manufacturer => Self {
                category,
                power_usage: 55,
                recipe_name,
                amount: 0,
            },
            Category::Packager => Self {
                category,
                power_usage: 10,
                recipe_name,
                amount: 0,
            },
            Category::ParticleAccelerator => Self {
                category,
                power_usage: 1500,
                recipe_name,
                amount: 0,
            },
            Category::Refinery => Self {
                category,
                power_usage: 30,
                recipe_name,
                amount: 0,
            },
            Category::Smelter => Self {
                category,
                power_usage: 4,
                recipe_name,
                amount: 0,
            },
        }
    }
}
