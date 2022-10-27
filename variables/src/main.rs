fn main() {
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // This doesn't work (immutable var)
    // let x = 5;
    // x = 12;

    // This works (mutable var)
    // let mut x = 5;
    // x = 12;

    // This works, it's a shadowed var
    // let x = 5;
    // let x = 12;

    // We can also change a var ONLY in a an inner scope:
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Shadowing is useful to change the type of a variable
    // without having to come up with different names
    let spaces = "    ";
    let spaces = spaces.len();

    println!("{}", spaces)

    // Constants:
    // Rust’s naming convention for constants is to use all uppercase with underscores between words.
    // Constants are valid for the entire time a program runs, within the scope they were declared in.
    // Ex:
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}