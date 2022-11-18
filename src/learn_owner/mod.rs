// mod learn_owner;
pub  fn owner_with_block(){
    let s :&str ="hell world";
    println!("s value = {s}");
    {
        let s:&str ="command line heroes!";
        println!("s value ={s}")
    }
    println!("s value = {s}");
}