fn main() {
    // Stack - last in first out, data is pushed onto stack
    // data is popped of stack - data must have know fixed size

    let _s = "hello";

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let _s2 = s1;

    // Rust considers s1 to no longer be valid - it is known as a "move"
    // println!("{}, world!", s1);

    // To deeply copy data on heap, use 'clone' method
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack only data, copy
    // copy of value made in memory - x still valid
    // let x = 5;
    // let y = x;

    // println!("x = {}, y = {}", x, y);

    //-------------------------------------
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    println!("x = {}", x);
    // println!("s = {}", s);   not in scope, has been moved to function, drop called after function finished
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
