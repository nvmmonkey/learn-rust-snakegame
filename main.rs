fn main() {
    // you can place on stack only value with static size
    let a = 10;
    let b = a;
    let c = 15;
    let d = add(a, b);

    // Store Pointer/Capacity/Length in the Stack Frame
    // String acutally store on HEAP memory
    // same as let msg: &str = "Hello";
    let msg = String::from("Hello");

    println!("{}", msg)
}

fn add(x: u32, y: u32) -> u32 {
    let sum = x + y;
    // return sum;
    sum //same as return sum
}
