/**
 *  Rust has three kinds of loops: loop, while, and for
 */
//
pub fn get_sum_with_for() -> i32 {
    let mut sum = 0;
    for i in 1..=100 { // for in .. loop
        sum += i
    }
    return sum;
}

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

pub fn get_sun_with_while() -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    while i <= 100 {
        sum += i;
        i+=1;
    }
    return sum;
}