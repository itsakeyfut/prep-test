fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v); // [1, 2, 3]

    v.pop();
    v.pop();
    v.pop();
    println!("{:?}", v); // []

    let mut v = vec![1, 2, 3];
    v.insert(1, 10);
    println!("{:?}", v); // [1, 10, 2, 3]

    v.remove(2);
    println!("{:?}", v); // [1, 10, 3]

    let v = vec![1, 2, 3, 4, 5];

    let result = v.iter().map(|elm| elm*2).collect::<Vec<_>>();
    println!("{:?}", result); // [2, 4, 6, 8, 10]
}
