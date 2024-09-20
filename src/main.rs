mod machines;
mod pipelines;
mod recipes;
mod resources;
mod traits;

use inquire::{InquireError, Select};
use recipes::{RecipeTree, Recipes};
use resources::Resource;
use strum::IntoEnumIterator;

fn main() {
    let recipes = Recipes::build();
    let mut options: Vec<Resource> = Resource::iter().collect();
    let ans: Result<Resource, InquireError> =
        Select::new("What's your favorite fruit?", options).prompt();

    match ans {
        Ok(choice) => {
            let pipeline = RecipeTree::build(choice, &recipes);
            // println!("{:#?}", pipeline);
            pipeline.simple_display();
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
