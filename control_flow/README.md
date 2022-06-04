# Control Flow in Rust
There are a few ways to do Control Flow in Rust.

### 1st way
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
    
### 2nd way
 // 2nd way to do if statements
    msg1 = if num1 == 6 {
        "six"
    } else if num1 == 4 {
        "four"
    } else {
        "other"
    };
    println!("msg1 is {}", msg1);
### 3rd way
   // 3rd, most pretty way to do if statements
    msg1 = if num1 == 6 { "six" } else { "other" };
    println!("msg1 is {}", msg1);
### 4th way, nested if statement
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