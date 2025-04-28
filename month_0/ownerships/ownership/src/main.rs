fn main() {
    let s = String::from("hello");
    let s2 = s; // move
    println!("{}", s);
    println!("{}", s2);
}
