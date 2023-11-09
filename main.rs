fn main() {
    let msg = "Hello World";
    let msg2 = print_welcome(msg);

    println!("{}", msg2)
}

// ------------/
//RETURN METHOD/
// ------------/

// arrow -> &str
fn print_welcome(text: &str) -> &str {
    println!("{}", text);
    let new_msg = "Hi There";

    // method 1:
    // return new_msg;
    new_msg //method 2
}

//binary executable code or library
//LLVM -> binary code
