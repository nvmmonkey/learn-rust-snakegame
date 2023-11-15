fn main() {
    let msg = String::from("Hello");
    let msg2: &String = &msg;
    // & using reference, not moved data
    // msg2 is not owner of data
    // msg2 is "borrowing" a reference to msg

    println!("{}", msg);
    println!("{}", msg2);
}

// msg and msg2 going out of scope
// msg2 is not dropped because it does not have ownership of what it refers to
// msg is dropped
