extern crate communicator;

fn main() {
    communicator::client::connect();
    communicator::network::connect();
    privacy_example();
    series_of_nested_modules_example();
}

fn privacy_example() {
    communicator::outermost::middle_function();
    //communicator::outermost::middle_secret_function();
    communicator::outermost::inside::inner_function();
    //communicator::outermost::inside::secret_function();
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;
use a::series::of::nested_modules;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;
use TrafficLight::{Red, Yellow}; //or glob it
fn series_of_nested_modules_example() {
    //instead of a::series::of::nested_modules()
    //use 'use'
    of::nested_modules();
    //use statements don't bring in children into scope, thus the use of 'of'
    //could just bring in the function itself in too
    nested_modules();
    //this also works with enums
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
    let green = Green; //From glob
}

// Summary of rules for item visibility
// - If an item is public, it can be access through any of its parent modules.
// - If an item is private, it can be accessed only bt its immediate parent module
// and any of the parent's child modules.
