// fn main() {
//     let a = 10;
//     let b = a;
//     let c = 15;
//     let d = add(a, b);

//     println!("{}", d);
// }

// fn add(x: u32, y: u32) -> u32 {
//     let sum = x + y;
//     // return sum;
//     sum //same as return sum
// }

fn main() {
    c();
    d();
    f();
}

fn a() {
    println!("calling A");
    d();
}
fn b() {
    println!("calling B");
}
fn c() {
    println!("calling C");
}
fn d() {
    println!("calling D");
    a();
}
fn e() {
    println!("calling E");
}
fn f() {
    println!("calling F");
    b();
}
