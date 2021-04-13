// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// I AM NOT DONE

// Put your function here!
fn main() {
    println!("{}",calculate_apple_price(35));
    println!("{}",calculate_apple_price(65));

}


fn calculate_apple_price(num_apple : u32) -> u32{

    if num_apple < 40{
        return num_apple*2;
    }
    else{
        return num_apple;
    }

}


// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
