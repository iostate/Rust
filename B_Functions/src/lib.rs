// Procedural macros need importing directly
// use rust_decimal_macros::dec;

// If the reexportable feature is enabled, `Decimal` needs to be in scope
// #[cfg(feature = "reexportable")]
// use rust_decimal::Decimal;

/**
 * First calculate the area of the thing.
 */
pub fn area_of(x: i32, y: i32) -> i32 {
    // 2a. Fix this function to correctly compute the area of a rectangle given
    // dimensions x and y by multiplying x and y and returning the result.
    //
    // return x * y;

    // Challenge: It isn't idiomatic (the normal way a Rust programmer would do things) to use
    //            `return` on the last line of a function. Change the last line to be a
    //            "tail expression" that returns a value without using `return`.
    //            Hint: `cargo clippy` will warn you about this exact thing.
    x * y
}

// pub fn data_types() {
//     // let mut number = dec!(1.2345);
//     // let mut result = assert_eq!("1.2345", number.to_string());
//     // println!("Number: {}", number);
//     // number = dec!(-5.4321);
//     // result = assert_eq!("-5.4321", number.to_string());

//     // let number = -1.23;
//     let decimal = dec!(-1.23);
//     let equals = assert_eq!("-1.23", decimal.to_string());
// }
