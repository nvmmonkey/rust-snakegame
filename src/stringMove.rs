fn main() {
    let msg = String::from("Hello"); //msg into the scope
    print_message(msg); //mssg into print_msg func
    //msg no longer valid

}
fn print_message(a: String) { //a comes into the scope
    println!("{}", a);
    let c = a; //c into scope , a moved into c

    //a is no longer valid
} //a is going out of the scope, but nothing more will happen bcuz it was moved
// c is going out of the scope, and 'drop' called to clears the memory from the heap


