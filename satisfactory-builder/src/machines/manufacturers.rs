use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Category {
    Assembler,
    Blender,
    Constructor,
    Manufacturer,
    Packager,
    ParticleAccelerator,
    Refinery,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manufacturer {
    category: Category,
    power_usage: i16,
}

impl Manufacturer {
    pub fn new(category: Category, power_usage: i16) -> Self {
        Self {
            category,
            power_usage,
        }
    }

    pub fn assembler() -> Self {
        Self {
            category: Category::Assembler,
            power_usage: 15
        }
    }

    pub fn blender() -> Self {
        Self {
            category: Category::Blender,
            power_usage: 75
        }
    }

    pub fn constructor() -> Self {
        Self {
            category: Category::Constructor,
            power_usage: 4
        }
    }

    pub fn manufacturer() -> Self {
        Self {
            category: Category::Manufacturer,
            power_usage: 55
        }
    }

    pub fn packager() -> Self {
        Self {
            category: Category::Packager,
            power_usage: 10
        }
    }

    pub fn particle_accelerator() -> Self {
        Self {
            category: Category::ParticleAccelerator,
            power_usage: 1500
        }
    }

    pub fn refinery() -> Self {
        Self {
            category: Category::Refinery,
            power_usage: 30
        }
    }
}
