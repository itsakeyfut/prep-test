fn main() {
    let x = std::cell::RefCell::new(5);
    *x.borrow_mut() += 1;
}
