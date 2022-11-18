/**
 *  Rust has three kinds of loops: loop, while, and for
 */
// for
pub fn get_sum_with_for() -> i32 {
    let mut sum = 0;
    for i in 1..=100 { // for in .. loop
        sum += i
    }
    return sum;
}

// loop
pub fn get_sum_with_loop() -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    loop {
        sum += i;
        i += 1;
        if i > 100 { break; }
    }
    return sum;
}

// while
pub fn get_sun_with_while() -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    while i <= 100 {
        sum += i;
        i+=1;
    }
    return sum;
}

pub fn show_loop_jump(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;  //MARK break then jump
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}