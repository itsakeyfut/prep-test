fn main() {
    let mut bmap = std::collections::BTreeMap::new();

    bmap.insert(3, "three");
    bmap.insert(1, "one");
    bmap.insert(2, "two");

    for (key, val) in &bmap {
        println!("{}: {}", key, val);
    }
}
