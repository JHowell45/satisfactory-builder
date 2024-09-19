use serde::{Deserialize, Serialize};


#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
pub enum Resource {
    // Iron
    IronOre,
    IronIngot,
    IronPlate,
    IronRod,
    Screws,
    ReinforcedIronPlate,
    // Copper
    CopperOre,
    CopperIngot,
    CopperSheet,
    Wire,
    Cable,
    // Concrete
    Limestone,
    Concrete,
    // Biofuel
    Leaves,
    Wood,
    Mycelia,
    AlienProtein,
    Biomass,
    SolidBiofuel,
    // Quartz
    QuartzOre,
    QuartzCrystal,
    Silica,
    // Caterium
    CateriumOre,
    CateriumIngot,
    Quickwire,
    // Steel
    Coal,
    SteelIngot,
    SteelPipe,
    SteelBeam,
    EncasedIndustrialBeam,
    // Frames
    ModularFrame,
    HeavyModularFrame,
    // Motors
    Rotor,
    Stator,
    Motor,
    TurboMotor,
    // Computers
    CircuitBoard,
    AILimiter,
    CrystalOscillator,
    Computer,
    HighSpeedConnector,
    RadioControlUnit,
    // Oil
    Oil,
    Fuel,
    DilutedPackagedFuel,
    HeavyOilResidue,
    PolymerResin,
    TurboFuel,
    PetroleumCoke,
    Plastic,
    Rubber,
    // Aluminium
    BauxiteOre,
    AluminaSolution,
    AluminiumScrap,
    AluminumIngot,
    AluminumCasing,
    AlcladAluminumSheet,
    HeatSink,
    CoolingSystem,
    Battery,
    // Liquids
    Water
}

// #[derive(Debug)]
// pub struct ResourceDeposit {
//     name: ResourceT,
//     size: ResourceSize
// }

// impl ResourceDeposit {
//     pub fn new(name: ResourceT, size: ResourceSize) -> Self {
//         Self { name, size }
//     }

//     pub fn output(&self) -> f32 {
//         match self.size {
//             ResourceSize::Impure => 30.0,
//             ResourceSize::Normal => 60.0,
//             ResourceSize::Pure => 120.0,
//         }
//     }
// }
