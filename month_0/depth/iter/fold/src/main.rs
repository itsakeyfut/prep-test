fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // これは全体の合計を求める。
    let sum = v.iter().fold(0, |cum, x| cum + x);
    println!("{:?}", sum); // 15

    // これが累積和を求める。
    let cumsum = v.iter().fold(
        Vec::new(),
        |mut acc, x| {
            let last = *acc.last().unwrap_or(&0);
            acc.push(last + x);
            acc
        }
    );
    println!("{:?}", cumsum); // [1, 3, 6, 10, 15]
}
