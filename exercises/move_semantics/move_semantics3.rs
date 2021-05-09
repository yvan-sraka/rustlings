// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec_mut = vec;

    vec_mut.push(22);
    vec_mut.push(44);
    vec_mut.push(66);

    vec_mut
}
