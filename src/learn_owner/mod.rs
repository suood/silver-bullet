pub fn owner_with_block() {
    let s: &str = "hell world";
    println!("s value = {s}");
    {
        let s: &str = "command line heroes!";
        println!("s value ={s}")
    }
    println!("s value = {s}");
}

/**
 *   @see String#push_str(&mut self, string: &str)
 */
pub fn owner_whit_string() {
    let mut s: String = String::from_utf8("hello ");

    s.push_str("command line heroes!");
    println!("s value ={}", s)
}