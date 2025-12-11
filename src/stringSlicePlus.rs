fn main() {
    // String vs &str
    let mut msg = String::from("Hello");
    let slice = &msg[2..4]; 
    //borrowing msg here, cannot modify msg while slice is in scope
    let slice2 = &msg[2..=4]; 
    let slice3 = &msg[..4]; 
    let slice4 = &msg[..]; 

    // move_me(msg);
    msg.clear();

    println!("{}", slice);
    println!("{}", slice.len());

}


fn move_me(value: String) {

}