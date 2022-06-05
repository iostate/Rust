fn main() {
    // IF STATEMENTS BEGIN
    let num = 5;
    let msg;
    // 1st way to do if statements
    if num == 5 {
        msg = "five";
    } else if num == 4 {
        msg = "four";
    } else {
        msg = "other";
    }
    println!("msg is {}", msg);
    let mut msg1;
    let num1 = 6;
    // 2nd way to do if statements
    msg1 = if num1 == 6 {
        "six"
    } else if num1 == 4 {
        "four"
    } else {
        "other"
    };
    println!("msg1 is {}", msg1);
    // 3rd, most pretty way to do if statements
    msg1 = if num1 == 6 { "six" } else { "other" };
    println!("msg1 is {}", msg1);
    // 4th, nested if statements
    msg1 = if num1 == 6 {
        if num1 != 6 {
            "other"
        } else {
            "six"
        }
    } else {
        "other"
    };
    println!("msg1 is {}", msg1);
    // IF STATEMENTS END
    // LOOPS BEGIN
    'bobs_loop: loop {
        println!("shebang shebang");
        loop {
            println!("oh baby and she move, she move");
            loop {
                break 'bobs_loop;
            }
        }
    }
    'mels_loop: loop {
        println!("shebang shebang");
        loop {
            println!("oh baby and she move, she move");
            loop {
                break 'mels_loop;
            }
        }
    }
    // LOOPS END
    // FOR LOOPS BEGIN
    // range notation is 2 dots
    // start is inclusive end is exclusive
    for num in 0..50 {
        // do stuff with num
        println!("{}", num);
    }
    // start is inclusive end is inclusive as well
    for num in 0..=50 {
        // do stuff with num
        println!("{}", num);
    }
    // FOR LOOPS BEGIN
}
