fn main() {
    println!("Data Types");

    ////////////////////////////////////////////////////////////////////////////
    /// Integer Types
    ////////////////////////////////////////////////////////////////////////////

    // | Length  |  Signed  | Unsigned
    //   8-bit       i8	    u8
    //   16-bit      i16	    u16
    //   32-bit      i32	    u32
    //   64-bit      i64	    u64
    //   128-bit     i128 	    u128
    //   arch	      isize	    usize

    // Signed integers can store numbers from -(2n-1) to 2n-1 - 1 inclusive
    
    // Signed integers ranges:
    //
    // i8 : from -128 to 127
    // i16 : from -32768 to 32767
    // i32 : from -2147483648 to 2147483647
    // i64 : from -9223372036854775808 to 9223372036854775807
    // i128 : from -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727

    // Unsigned integers can store numbers from 0 to 2n-1 inclusive

    // Unsigned integers ranges:
    //
    // u8 : from 0 to 255
    // u16 : from 0 to 65535
    // u32 : from 0 to 4294967295
    // u64 : from 0 to 18446744073709551615
    // u128 : from 0 to 340282366920938463463374607431768211455

    ////////////////////////////////////////////////////////////////////////////
    /// Floating-Point Types
    ////////////////////////////////////////////////////////////////////////////

    // All floating-point types are signed.
    // 
    // f32 & f64  

    ////////////////////////////////////////////////////////////////////////////
    /// Numereic operations
    ////////////////////////////////////////////////////////////////////////////

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    ////////////////////////////////////////////////////////////////////////////
    /// Booleans
    ////////////////////////////////////////////////////////////////////////////

    let t = true;

    let f: bool = false;

    ////////////////////////////////////////////////////////////////////////////
    /// Character Type (Rune)
    ////////////////////////////////////////////////////////////////////////////

    /// Note that we specify char literals with single quotes, as opposed to 
    /// string literals, which use double quotes. Rust’s char type is four 
    /// bytes in size and represents a Unicode Scalar Value, which means it 
    /// can represent a lot more than just ASCII. Accented letters; Chinese, 
    /// Japanese, and Korean characters; emoji; and zero-width spaces are all 
    /// valid char values in Rust. Unicode Scalar Values range from 
    /// U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. 

    let ferris: char = '🦀';

    ////////////////////////////////////////////////////////////////////////////
    /// Tuples
    ////////////////////////////////////////////////////////////////////////////

    /// The tuple without any values has a special name, unit.
    /// This value and its corresponding type are both written () and 
    /// represent an empty value or an empty return type. Expressions 
    /// implicitly return the unit value if they don’t return any other value.
    
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Reztrieve values from a tup
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("The first value of 'tup' is: {five_hundred}");

    // Destructuring a tup
    let (x, y, z) = tup;

    // Use y
    println!("The value of y is: {y}");

    ////////////////////////////////////////////////////////////////////////////
    /// Arrays
    ////////////////////////////////////////////////////////////////////////////

    // Unlike a tuple, every element of an array must have the same type.
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.

    // An array isn’t as flexible as the vector type, though.
    // A vector is a similar collection type provided by the standard library 
    // that is allowed to grow or shrink in size.

    let a = [1, 2, 3, 4, 5];
    // Same as:
    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    // You can also initialize an array to contain the same value for each 
    // element by specifying the initial value, followed by a semicolon, and 
    // then the length of the array in square brackets, as shown here:

    let a = [3; 5];

    // The array named a will contain 5 elements that will all be set to the 
    // value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; 
    // but in a more concise way.

    // Accessing Array Elements
    let first = a[0];
    let second = a[1];

    ////////////////////////////////////////////////////////////////////////////
    /// Memory Safety
    ////////////////////////////////////////////////////////////////////////////

    /// When you attempt to access an element using indexing, Rust will check 
    /// that the index you’ve specified is less than the array length. If the 
    /// index is greater than or equal to the length, Rust will panic. 
    /// This check has to happen at runtime, especially in this case, because 
    /// the compiler can’t possibly know what value a user will enter when 
    /// they run the code later.

    println!("Bye!");
}
