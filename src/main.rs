mod test_mod;
fn main() {
    test_mod::print_hello();
    println!("{}", get_sum());
}
fn get_sum() -> i32{
    let mut sum = 0;
    for i in 1..=100 {
        sum += i
    }
    return sum
}