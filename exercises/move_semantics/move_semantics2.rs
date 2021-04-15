// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    let vec0 = fill_vec();
    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    let mut vec1 = vec0;
    vec1.push(88);

    let len = vec1.len();

    println!("{} has length {} content `{:?}`", "vec1", len, vec1);
}

fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
