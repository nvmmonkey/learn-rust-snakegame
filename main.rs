fn main() {
    let mut msg = String::from("Hello");
    let msg2 = &mut msg;
    let msg3 = &msg2;

    // (*msg2).push_str(" World");
    // println!("{}", msg2);

    msg3.push_str(" Worl")
}
