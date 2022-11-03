fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

// [0..5] bit. We create slices using a range within brackets by specifying [starting_index..ending_index], where starting_index is the first position in the slice and ending_index is one more than the last position in the slice.
let s = String::from("hello");

// If you want to start at index zero 0 can be ommited with Rust’s .. range syntax. 
// [0..5] and [..5] are the same
let hello = &s[0..5]; 
let world = &s[6..11];

// In the same way, if your slice includes the last byte of the String, 
// you can drop the trailing number.
let len = s.len();

// Both the same:
let slice = &s[3..len];
let slice = &s[3..];

// Note: String slice range indices must occur at valid UTF-8 character 
// boundaries. If you attempt to create a string slice in the middle of
// a multibyte character, your program will exit with an error. 

// or the full string :
let len = s.len();

// Both the same
let slice = &s[0..len];
let slice = &s[..];

// Other slices

// refer to part of an array.
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);