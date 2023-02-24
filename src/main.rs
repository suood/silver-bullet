#![allow(dead_code)]

mod learn_mod;
mod learn_loop;
mod learn_owner;
mod learn_slice;
fn main() {
    // 从 hello world.
    // learn_mod::print_hello();
    // println!("{}", learn_loop::get_sum_with_loop());
    // println!("{}", learn_loop::get_sum_with_for());
    // println!("{}", learn_loop::get_sun_with_while());
    // learn_loop::show_loop_jump();
    // learn_owner::owner_with_block();
    // learn_owner::owner_whit_string();
    let mut s = String::from("hello world");

    let word:&str = learn_slice::first_word_str(&s); // word will get the value 5

    println!("word value :{word}");
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    learn_slice::run_first_word();
}
