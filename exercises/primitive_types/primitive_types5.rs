// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
