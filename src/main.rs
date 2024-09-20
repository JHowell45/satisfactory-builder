mod machines;
mod pipelines;
mod recipes;
mod resources;
mod traits;

use inquire::{InquireError, Select};
use pipelines::Pipeline;
use recipes::{tree::RecipeTree, Recipes};
use resources::Resource;
use strum::IntoEnumIterator;

fn main() {
    let recipes = Recipes::build();
    let options: Vec<Resource> = Resource::iter().collect();
    let ans: Result<Resource, InquireError> =
        Select::new("Enter the component you want to make:", options).prompt();

    match ans {
        Ok(choice) => {
            let mut pipeline = Pipeline::new();
            let recipe_tree = RecipeTree::build(choice, &recipes, &mut pipeline);
            println!("{:#?}", recipe_tree);
            recipe_tree.simple_display();
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
