fn main() {
    // you can place on stack only value with static size
    let a = 10;
    let b = a; //copy value
    let c = 15;
    let d = add(a, b);

    let msg = String::from("Hello"); //string move from stack
    let msg2 = msg; //string borrow from previous stack and message
    println!("{}", msg); 

    //can not use msg, because it was moved to msg2
}

fn add(x: u32, y: u32) -> u32 {
    let sum = x + y;
    sum
}
