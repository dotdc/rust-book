enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

// Example from "std::net":
// https://doc.rust-lang.org/std/net/enum.IpAddr.html

// the name of each enum variant that we define also becomes a function that
// constructs an instance of the enum. IpAddr::V4() is a function call that
// takes a String argument and returns an instance of the IpAddr type.
// We automatically get this constructor function defined as a result of 
// defining the enum.
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));

// A message enum like this:
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// is almost the same thab these 4 structs:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
// but the enum is both more consice and easy to use only one type has 
// been created and need to be used.

// We can use the method notation for enums too:
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();

// Rust does not have nulls, but it does have an enum that can encode the
// concept of a value being present or absent. This enum is Option<T>, and
// it is defined by the standard library as follows:
enum Option<T> {
    None,
    Some(T),
}
// The Option<T> enum is included in the prelude;
// The Option<T> enum is still just a regular enum, and
// Some(T) and None are still variants of type Option<T>.

// The <T> syntax is a generic type parameter
// https://doc.rust-lang.org/std/option/enum.Option.html

////////////////////////////////////////////////////////////////////////////////
// Using the match control flow
////////////////////////////////////////////////////////////////////////////////

// Matches in Rust are exhaustive!

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Matching with Option<T>

// A function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);


// Combining match and enums is useful in many situations. You’ll see this
// pattern a lot in Rust code: match against an enum, bind a variable to the
// data inside, and then execute code based on it. It’s a bit tricky at first,
// but once you get used to it, you’ll wish you had it in all languages.
// It’s consistently a user favorite.

// Compile because other handle all other cases
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

// catch-all pattern "_" is also valid, we decided to ignore them specificaly
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}


// The if let syntax lets you combine if and let into a less verbose way to
// handle values that match one pattern while ignoring the rest.


// This:
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}

// Can be rewritten using if let:
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}

// Using if let, we lose the exhaustive checking that match enforces.

// The syntax if let takes a pattern and an expression separated by 
// an equal sign. If you have a situation in which your program has
// logic that is too verbose to express using a match, remember that
// if let is in your Rust toolbox as well.

