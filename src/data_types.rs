fn main() {
    // Scalar types

    // Integers

    // Signed
    let x: i32 = -5;
    // Unsigned
    let y: u32 = 5;

    println!("The signed value is {}, and the unsigned value is {}", x, y);

    // Floating Points
    // Default floating point type
    let _a = 2.0; // f64

    let _b: f32 = 3.0; // f32

    // Numeric Operators
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;

    // remainder
    let _remainder = 43 % 5;

    // Boolean
    let _t = true;

    let _f: bool = false; // with explicit type annotation

    // Character
    // Uses single quote as opposed to strings
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Compound types
    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (c, _d, _e) = tup;
    println!("The value of c is: {}", c);

    let five_hundred = tup.0;
    println!("The value of five_hundred is: {}", five_hundred);

    // Array type
    // Arrays must have same type unlike tuples
    // Can't grow or shrink in size - use a vector for this
    let g: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialize array with same value - let i = [3, 3, 3, 3, 3];
    let _i = [3; 5];

    let _first = g[0]; // 1
}
