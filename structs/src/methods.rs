#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Often, but not always, when we give methods with the same name as a field we
// want it to only return the value in the field and do nothing else. Methods
// like this are called getters, and Rust does not implement them automatically
// for struct fields as some other languages do. Getters are useful because you
// can make the field private but the method public and thus enable read-only
// access to that field as part of the type’s public API.

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// We can have multiple impl for a struct but we could 
// just add all methods in a single one
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}