fn main() {
    let mut msg = String::from("Hello");

    let slice = &msg[2..4]; //index 2->3
    // H E L L O
    // 0 1[2 3]4

    let slice2 = &msg[2..=4]; //index 2->4

    // H E L L O
    // 0 1[2 3 4]

    println!("{}", slice);
    println!("{}", slice.len());

    println!("{}", slice2);
    println!("{}", slice2.len());
}
