fn main() {
    let mut msg = String::from("Hello");
    let msg3 = &msg;
    println!("{}", msg3);
    let msg2 = &mut msg;

    unpredictable_mutate(msg2); //mutable already

    println!("{}", msg);
    // if place before msg2,
    // it think you are trying to immutable a already
    // borrow mutable variable
}

fn unpredictable_mutate(val: &mut String) {
    val.push_str("_unpredictable");
}
