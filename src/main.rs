#![warn(unused_imports)]
#![warn(dead_code)]
mod random_generator_mod;
mod string_fun_mod;
mod vector_sort_mod;

use common::separator;

use crate::random_generator_mod::random_generator_mod as rando;
use crate::string_fun_mod::string_fun_mod as sfun;
use crate::vector_sort_mod::vector_sort_mod as vec_sort;

fn main() {
    separator("main.rs".to_string(), true);

    // set to "true" to view each
    rando::kitchen_sink(false);
    vec_sort::kitchen_sink(false);
    sfun::kitchen_sink(false);
}
