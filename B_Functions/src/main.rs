// Silence some warnings so they don't distract from the exercise.
// #![allow(unused_variables)]

use b_functions::area_of;
// use b_functions::data_types;

fn main() {
    // let x = (let y = 6);
    // Produces an error

    // let width = 4;
    // let height = 7;
    // let depth = 10;
    let (width, height, depth) = (4, 7, 10);
    let room_volume = volume(width, height, depth);
    println!("The room's volume is {}", room_volume);
    // 1. Try running this code with `cargo run` and take a look at the error.
    //
    // See if you can fix the error. It is right around here, somewhere.  If you succeed, then
    // doing `cargo run` should succeed and print something out.
    // Bonus. Perform variable shadowing
    // Variable shadowing with one type BEGIN
    let size_of_cube = volume(width, height, depth);
    {
        let area = area_of(width, height);

        println!("Area is {}", area);

        let size_of_cube = volume(width, height, depth);
        println!("The area is {}", size_of_cube);
        println!("Variables within this block, area, size_of_cube, immediately go to Garbage collection after the block ends. ");
    }
    println!("The area is {}", size_of_cube); // Variable shadowing with one type END
                                              // let y = {
                                              //     let x = 3;
                                              //     x + 1
                                              // };
                                              // println!("The value of y is: {}", y);
                                              // Prints 4
                                              // println!("The value of x is: {}", x);
                                              // Produces an error
                                              // 2. The area that was calculated is not correct! Go fix the area_of() function below, then run
                                              //    the code again and make sure it worked (you should get an area of 28).

    // 3. Uncomment the line below.  It doesn't work yet because the `volume` function doesn't exist.
    //    Create the `volume` function!  It should:
    //    - Take three arguments of type i32
    //    - Multiply the three arguments together
    //    - Return the result (which should be 280 when you run the program).
    //
    // If you get stuck, remember that this is *very* similar to what `area_of` does.
    //
    println!("Volume is {}", volume(width, height, depth));
}

///     Calculates the volume based on data provided by the user.
///
///     # Examples
///
/// ```
///     let (width, height, depth) = (4, 7, 10);
///     let roomVolume = volume(width, height, depth);
/// ```
///     @width  i32 Width to be added to the volume calculation
///     @height i32 Height to be added to the volume calculation
///     @depth  i32 Depth to be added to the volume calculation
///
fn volume(width: i32, height: i32, depth: i32) -> i32 {
    width * height * depth
}
