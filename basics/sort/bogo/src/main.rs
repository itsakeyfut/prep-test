use rand::Rng;
use rand::seq::SliceRandom;

fn is_sorted(nums: &[i32]) -> bool {
    nums.windows(2).all(|w| w[0] <= w[1])
}

fn bogo_sort(nums: &mut Vec<i32>) {
    let mut rng = rand::rng();
    while !is_sorted(nums) {
        nums.shuffle(&mut rng);
    }
}

fn main() {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (0..10)
        .map(|_| rng.random_range(0..1000))
        .collect();

    println!("Before sorting: {:?}", nums);
    bogo_sort(&mut nums);
    println!("After sotring: {:?}", nums);
}
