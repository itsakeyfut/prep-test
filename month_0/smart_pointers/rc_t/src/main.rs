fn main() {
    let a = std::rc::Rc::new(5);
    let b = std::rc::Rc::clone(&a);

    println!("{}", a);
    println!("{}", b);
}
