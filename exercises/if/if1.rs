// if1.rs



fn bigger(a: i32, b:i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - return
    // - another function call
    // - additional variables
    // Scroll down for hints.

    if a > b { a } else { b }
}

fn main() {
    assert_eq!(10, bigger(10, 8));
    assert_eq!(42, bigger(32, 42));
}
