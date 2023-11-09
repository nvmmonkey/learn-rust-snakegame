fn main() {
    //always display decimal
    let custom_num: i32 = 98_000; //98000
    let hex_num: i32 = 0xfa; //hexdecimal 16bytes 
    let bin_num: i32 = 0b0010_1011; //binary 2bytes
    let byte_num: u8 = b'A'; //UTF encoding A=>0x41 heximal

    println!("{}", custom_num);
    println!("{}", hex_num);
    println!("{}", bin_num);
    println!("{}", byte_num);
}
