#[allow(dead_code)]
/**
 *  <p>
 *  There is a natural point at which we can return the memory our String needs to the allocator:
 *  when s goes out of scope. When a variable goes out of scope,
 *  Rust calls a special function for us. This function is called drop,
 *  and it’s where the author of String can put the code to return the memory.
 *  Rust calls drop automatically at the closing curly bracket.
 *  </p>
*/
pub fn owner_with_block() {
    let s: &str = "hell world";
    println!("s value = {s}");
    {
        let s: &str = "command line heroes!";
        println!("s value ={s}")
    }// mark call drop
    // 开发者无法调用 Drop::drop ,可以使用mem::drop  作为替代。
    println!("s value = {s}");
}



/**
 *   @see String#push_str(&mut self, string: &str)
RAII还有另外一种被称为RRID(Resource Release Is Destruction)的特殊用法，即在构造时没有“获取”资源，但在析构时释放资源。
 */
pub fn owner_whit_string() {
    let mut s: String = String::from("hello ");

    s.push_str("command line heroes!");
    println!("s value ={}", s)
}
//
pub fn learn_drop(){
    struct PrintOnDrop(&'static str);

    impl Drop for PrintOnDrop {
        fn drop(&mut self) {
            println!("{}", self.0);
        }
    }

    let mut overwritten = PrintOnDrop("drops when overwritten");
    overwritten = PrintOnDrop("drops when scope ends");

    let tuple = (PrintOnDrop("Tuple first"), PrintOnDrop("Tuple second"));

    let moved;
// No destructor run on assignment.
    moved = PrintOnDrop("Drops when moved");
// Drops now, but is then uninitialized.
    drop(moved);

// Uninitialized does not drop.
    let uninitialized: PrintOnDrop;

// After a partial move, only the remaining fields are dropped.
    let mut partial_move = (PrintOnDrop("first"), PrintOnDrop("forgotten"));
// Perform a partial move, leaving only `partial_move.0` initialized.
    core::mem::forget(partial_move.1);
// When partial_move's scope ends, only the first field is dropped.
}