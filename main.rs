// fn main() {
//     // you can place on stack only value with static size
//     let a = 10;
//     let b = a; //copy value
//     let c = 15;
//     let d = add(a, b);

//     let msg = String::from("Hello"); //string move from stack
//     let msg2 = msg; //string borrow from previous stack and message
//     println!("{}", msg);

//     //can not use msg, because it was moved to msg2
// }

// fn add(x: u32, y: u32) -> u32 {
//     let sum = x + y;
//     sum
// }

fn main() {
    let msg = String::from("Hello"); //msg coming into the scope
    print_msg(msg); //msg moved into the print_msg function

    //msg is no longer valid
    println!("{}", msg);
}
//msg is going out of the scope,
//but nothing more will happen because it was moved into print_msg

fn print_msg(a: String) {
    //a comes into the scope
    println!("{}", a);

    let c = a; //c is coming into the scope, a moved into the c

    //a is no longer valid
    println!("{}", a);
}
//a is going out of the scope, but nothing more will happen because it was moved
//c is going out of the scope
//,and, "drop" is called which clears the memory from the HEAP
