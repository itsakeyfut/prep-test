fn main() {
    let v: Vec<u64> = (0..10_000_000).collect();
    let now = std::time::Instant::now();
    let sum: u64 = v.iter().sum();
    println!("sum = {}, time = {:?}", sum, now.elapsed()); // sum = 49999995000000, time = 31.2907ms
}
