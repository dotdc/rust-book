fn main() {
    // Because if is an expression, we can use it on the right side of a let
    // statement to assign the outcome to a variable.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    //let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    just_a_loop();
    while_not_zero();
    loop_an_array();
    loop_an_array_with_for();
    reverse();
}

// Infinite loop

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

fn just_a_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter * 3;
        }
    };

    println!("The result is {result}");
}

// To close a parent loop form a nested loop, you can use a loop label.
// https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops
// 
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

fn while_not_zero() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_an_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn loop_an_array_with_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn reverse() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}