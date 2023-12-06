fn main() {
   let mut msg = String::from("Hello");
   let name = "Filip"; // str is read-only memory stack

   msg.push_str(" World");
   name.push_str(" Jerga");
}
