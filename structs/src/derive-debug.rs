// Rust does include functionality to print out debugging information, 
// but we have to explicitly opt in using : 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Print debug information
    // Regular:      {:?} 
    // Pretty print: {:#?}
    println!("rect1 is {:?}", rect1);

    // Or use the dbg! macro
    dbg!(&rect1);
}