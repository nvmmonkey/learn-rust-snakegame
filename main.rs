fn main() {
    let mut msg = String::from("Hello");
    let msg2: &mut String = &mut msg;
    // & using reference, not moved data
    // msg2 is not owner of data
    // msg2 is "borrowing" a reference to msg

    msg2.push_str(" World");

    println!("{}", msg2);
    println!("{}", msg);
}

// msg and msg2 going out of scope
// msg2 is not dropped because it does not have ownership of what it refers to
// msg is dropped
