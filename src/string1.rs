fn main() {
    // String vs &str
    let mut msg = String::from("Hello");
    let name = "Jack";

    msg.push_str(" World");
    name.push_str(" Smith"); // This line will cause a compile-time error
}
