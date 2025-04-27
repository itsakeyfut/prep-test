fn main() {
    let mut map = std::collections::HashMap::new();

    map.insert("apple", 3);
    map.insert("banana", 5);
    map.insert("kiwi", 0);

    println!("{:?}", map); // {"kiwi": 0, "apple": 3, "banana": 5}

    if let Some(val) = map.get("apple") {
        if *val != 0 { println!("You have {} apples", val); }
        else { println!("You have no apple"); }
    }
}
