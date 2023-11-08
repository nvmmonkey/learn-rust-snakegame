fn main() {
    //** often used
    //mutable variable
    let mut msg = "Hellow World";

    println!("{}", msg);

    msg = "Hi there";
    println!("Some text: {}", msg);

    //** not often used
    //immutable varible double declare is ok in rust
    let age = 10;
    println!("{}", age);

    let age = 30;
    println!("{}", age);
}

//binary executable code or library
//LLVM -> binary code
