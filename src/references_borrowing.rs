fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    //------------------------------------
    // References are immutable, but mutable reference is possible
    let mut s = String::from("hello");

    change(&mut s);
    // ------------------------------------
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so 2nd reference is fine

    let _r2 = &mut s;
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
