mod learn_mod;
fn main() {
    learn_mod::print_hello();
    println!("{}", get_sum_for_loop());
}
fn get_sum_for_loop() -> i32{
    let mut sum = 0;
    for i in 1..=100 { // for in .. loop
        sum += i
    }
    return sum
}

fn get_sum_do_loop() -> i32{
    let mut sum:i32 = 0;

}