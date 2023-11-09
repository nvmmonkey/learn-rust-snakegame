fn main() {
    let is_it_fun: bool = true;

    //i32 -> signed integer of 32bits
    // -> signed can hold positive or negative value
    let num1: i32 = -10;

    //u8 -> unsigned integer of 8bits
    // -> only positive value
    // HOW many numbers? -> 2^8-1 = 255
    // u32 -> 2^32-1 postive numbers
    let small_num1: u8 = 255;
    let small_num2: u16 = 256;

    // -2^7-1 -> 2^7-1
    // -128 -> 127
    let small_num3: i8 = -128;

    // operating system related types
    let sys_num1: isize = -10;
    //x32 system usize->u32, x64 usize->u64
    let sys_num2: usize = 10;
}
