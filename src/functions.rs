fn main() {
    println!("main");

    another_function(10, -25);

    // Statement - 6 is an expression as it evaluates to 6
    let _y = 6;

    let z = {
        let x = 3;
        x + 1
    };

    let a = ten();
    println!("a value is {}", a);

    let b = add_two(10);
    println!("b value is {}", b);

    // x + 1 returns the value, expressions don't include semicolons

    println!("The value of z is: {}", z);
}

// Snake case
fn another_function(x: u32, y: i32) {
    println!("Argument of function is {}", x);
    println!("Argument of function is {}", y);
}

fn ten() -> u32 {
    10
}

fn add_two(x: u32) -> u32 {
    x + 2
}
