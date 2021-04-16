// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string_slice("red".to_string());
    string_slice(String::from("hi"));
    string_slice("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string_slice(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string_slice("Happy Monday!".to_string().replace("Mon", "Tues"));
    string_slice("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
