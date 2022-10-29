
// Statements are instructions that perform some action and do not return a value. 
// Expressions evaluate to a resulting value. 
// Expressions do not include ending semicolons.

// You can return early from a function by using the return keyword and 
// specifying a value, but most functions return the last expression implicitly.

fn main() {
    println!("Main");

    // use the return valie of plus_one()
    another_function(plus_one(41), 'm');
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // Same as:
    // return x + 1; 
}

fn another_function(i: i32, unit: char) {
    println!("Hey, {i}{unit} left!");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
