const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    // let mut missiles: i32 = 8;
    // let ready: i32 = 2;
    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!(
        "Starting missiles {} and remaining missles {}",
        STARTING_MISSILES, missiles
    );

    let vol = get_cubic_feet(5.0, 3.0);
    println!("{}", vol);
    // println!("Volume is {}", volume);
}

fn get_cubic_feet(sq_ft: f64, height_ft: f64) -> f64 {
    return sq_ft * height_ft;
}
