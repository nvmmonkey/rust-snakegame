fn main() {
    let msg = String::from("Hello");
    let msg2: &String = &msg; // borrow msg instead of moving it, msg2 is not owning msg
    println!("{}", msg);
    println!("{}", msg2);
}
