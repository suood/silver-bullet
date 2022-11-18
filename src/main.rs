mod learn_mod;
mod learn_loop;
mod learn_owner;
fn main() {
    learn_mod::print_hello();
    println!("{}", learn_loop::get_sum_with_loop());
    println!("{}", learn_loop::get_sum_with_for());
    println!("{}", learn_loop::get_sun_with_while());
    learn_loop::show_loop_jump();
    learn_owner::owner_with_block();
}
