fn main() {
    let float_num: f32 = 3.14;
    let float_num2: f32 = 3.28726387;

    //Tuples
    //A tuple is a collection of values of different types.
    //Tuples are constructed using parentheses (),
    // and each tuple itself is a value with type signature (T1, T2, ...),
    //where T1, T2 are the types of its members.
    //Functions can use tuples to return multiple values, as tuples can hold any number of values.
    let tup: (i32, &str, u8) = (20, "Hello", 1);

    println!("{}", tup.1);

    let (a, b, c) = tup;
    println!("{}", a);

    let x = [1, 3, 5, 7]; //immutable cannot add/delete
    println!("{}", x[2]); //access array index of 2 => 5

    let y = [2; 6]; // [2,2,2,2,2,2]
    println!("{}", y[5]); //return 2
}
