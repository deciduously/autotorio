#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate  pretty_assertions;

mod building;
mod factory;
mod item;

use factory::*;

fn main() {
    ::std::process::exit(0);
    //println!("Recipe!\n{}", IRON_PLATE.show());
}
