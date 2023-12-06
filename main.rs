fn main() {
    let mut msg = String::from("Hello");
    let slice = &msg[2..4]; //index 2->3
    
    // Already borrow mutable, so cannot borrow again as immutable
    // move_me(msg);
    // msg.clear();

    println!("{}", slice);
}

fn move_me(val:String){

}
