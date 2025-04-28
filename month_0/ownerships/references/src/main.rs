fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("{}", len); // 5

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s); // hello, world
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}