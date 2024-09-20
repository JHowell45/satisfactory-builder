mod resources;
mod traits;
mod pipelines;
mod recipes;
mod machines;

use recipes::{RecipeTree, Recipes};
use resources::Resource;

fn main() {
    let recipes = Recipes::build();
    // println!("{:#?}", recipes);

    let pipeline = RecipeTree::build(Resource::IronPlate, &recipes);
    // println!("{:#?}", pipeline);

    let pipeline = RecipeTree::build(Resource::ReinforcedIronPlate, &recipes);
    // println!("{:#?}", pipeline);

    let pipeline = RecipeTree::build(Resource::Motor, &recipes);
    println!("{:#?}", pipeline);
    pipeline.simple_display();
}
