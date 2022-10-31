fn main() {
    println!("Ownership");

    // Ownership Rules

    // - Each value in Rust has an owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.


    // The Stack

    // The stack stores values in the order it gets them and removes the values
    // in the opposite order. This is referred to as last in, first out.
    // Adding data is called pushing onto the stack, and removing data is 
    // called popping off the stack

    // All data stored on the stack must have a known, fixed size.

    // The Heap

    // The heap is less organized: when you put data on the heap, you request 
    // a certain amount of space. The memory allocator finds an empty spot in 
    // the heap that is big enough, marks it as being in use, and returns a 
    // pointer, which is the address of that location. This process is called 
    // allocating on the heap and is sometimes abbreviated as just allocating 
    // (pushing values onto the stack is not considered allocating). Because 
    // the pointer to the heap is a known, fixed size, you can store the pointer
    // on the stack, but when you want the actual data, you must follow the 
    // pointer. 

    // When your code calls a function, the values passed into the function 
    // (including, potentially, pointers to data on the heap) and the 
    // function’s local variables get pushed onto the stack. When the function
    // is over, those values get popped off the stack.

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let string1 = String::from("hello");
    let string2 = string1; // This will invalidates s1, we cannot use it anymore
    println!("{}, world!", string2); 

    let int1 = 5;
    let _int2 = int1; // This will copy and will NO invalidates i1
    println!("{}, world!", int1); 

    // to copy/clone heap data
    let c1 = String::from("hello");
    let c2 = c1.clone();

    println!("c1 = {}, c2 = {}", c1, c2);

    ////////////////////////////////////////////////////////////////////////////
    // References and Borrowing
    ////////////////////////////////////////////////////////////////////////////
    
    // A reference is like a pointer in that it’s an address we can follow 
    // to access the data stored at that address; that data is owned by some 
    // other variable. Unlike a pointer, a reference is guaranteed to point to
    // a valid value of a particular type for the life of that reference.

    // Note: The opposite of referencing by using & is dereferencing, which 
    // is accomplished with the dereference operator, *. 

    let hello = String::from("Hello David!");

    let len = calculate_length(&hello); // Ampersands represent references: &hello 

    println!("The length of '{}' is {}.", hello, len);

    // A mutable reference
    change(&mut s);

    // When functions have references as parameters instead of the actual 
    // values, we won’t need to return the values in order to give back 
    // ownership, because we never had ownership.

    // We call the action of creating a reference borrowing.

    // Mutable references have one big restriction: if you have a mutable
    // reference to a value, you can have no other references to that value.

    // We also cannot have a mutable reference while we have an immutable
    // one to the same value.

    // Note that a reference’s scope starts from where it is introduced and
    // continues through the last time that reference is used. For instance, 
    // this code will compile because the last usage of the immutable
    // references, the println!, occurs before the mutable reference is 
    // introduced:
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // The ability of the compiler to tell that a reference is no longer
    // being used at a point before the end of the scope is called 
    // Non-Lexical Lifetimes (NLL for short)
    // Doc: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes

}

// Ampersands represent references: &String 
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.


// A mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
