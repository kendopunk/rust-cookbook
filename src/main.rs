#![warn(unused_imports)]
#![warn(dead_code)]
mod random_generator_mod;
mod vector_sort_mod;

use common::separator;

use crate::random_generator_mod::random_generator_mod as rando;
use crate::vector_sort_mod::vector_sort_mod as vec_sort;

fn main() {
    separator("main.rs".to_string(), true);

    rando::kitchen_sink(false);
    vec_sort::kitchen_sink(true);
}
