fn main() {
    println!("Hello, world!");
}

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// module system, includes:
// - Packages: A Cargo feature that lets you build, test, and share crates
// - Crates: A tree of modules that produces a library or executable (binary)
// - Modules and use: Let you control the organization, scope, and privacy of paths
// - Paths: A way of naming an item, such as a struct, function, or module

// Library crates don’t have a main function and they don’t compile to an 
// executable. Instead, they define functionality intended to be shared
// with multiple projects.

// A crate is the smallest amount of code that the Rust compiler considers at a time
// When Rustaceans say “crate”, they mean library crate, and they use “crate”
// interchangeably with the general programming concept of a “library".

// A package is a bundle of one or more crates that provides a set of
// functionality. A package contains a Cargo.toml file that describes
// how to build those crates.
// A package can contain as many binary crates as you like, but at most
// only one library crate. A package must contain at least one crate,
// whether that’s a library or binary crate.

// Package containing a library crate:
// src/lib.rs

// If a package contains src/main.rs and src/lib.rs, it has two crates: a
// binary and a library, both with the same name as the package. A package
// can have multiple binary crates by placing files in the src/bin directory:
// each file will be a separate binary crate.

// A path can take two forms:

// An absolute path is the full path starting from a crate root; for code from
// an external crate, the absolute path begins with the crate name, and for
// code from the current crate, it starts with the literal crate.
// A relative path starts from the current module and uses self, super, or an
// identifier in the current module.

// Both absolute and relative paths are followed by one or more identifiers
// separated by double colons (::).

// Items in a parent module can’t use the private items inside child modules,
// but items in child modules can use the items in their ancestor modules.
// This is because child modules wrap and hide their implementation details,
// but the child modules can see the context in which they’re defined.

// The privacy rules apply to structs, enums, functions, and methods as well as modules.
// For struct, need to apply pub on the struct itself and all the child fields
// If an enum is pulic, all its fields will be public


fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super is like ..
        super::deliver_order();
    }

    fn cook_order() {}
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// "Use" can import a crate to the current scope

use std::fmt::Result;
use std::io::Result as IoResult;

pub use crate::front_of_house::hosting;


// Regular
use std::cmp::Ordering;
use std::io;

// nested path to bring multiple items with the same prefix into scope
use std::{cmp::Ordering, io};

// Import all public items using the glob operator
use std::collections::*;
