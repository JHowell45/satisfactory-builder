mod machines;
mod pipelines;
mod recipes;
mod resources;
mod traits;

use inquire::{InquireError, Select};
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
            let pipeline = RecipeTree::build(choice, &recipes);
            println!("{:#?}", pipeline);
            pipeline.simple_display();
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
