fn main() {
    let a = 10;
    let b = &a;
    let c = &b;

    // **c =100;

    println!("{}", a == **c);
}
