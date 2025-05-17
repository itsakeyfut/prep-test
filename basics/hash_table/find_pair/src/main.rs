use std::collections::HashSet;

fn get_pair(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut cache = HashSet::new();

    for &num in nums {
        let val = target - num;
        if cache.contains(&val) {
            return Some((val, num));
        }
        cache.insert(num);
    }
    None
}

fn get_pair_half_num(nums: &[i32]) -> Option<(i32, i32)> {
    let sum_nums: i32 = nums.iter().sum();

    if sum_nums % 2 != 0 {
        return None;
    }

    let half_sum = sum_nums / 2;
    let mut cache = HashSet::new();

    for &num in nums {
        cache.insert(num);
        let val = half_sum - num;
        if cache.contains(&val) {
            return Some(if num < val { (num, val) } else { (val, num) });
        }
    }
    None
}

fn main() {
    let arr = vec![11, 2, 5, 9, 10, 3];
    let target = 12;

    // (2, 10)
    match get_pair(&arr, target) {
        Some(pair) => println!("{:?}", pair),
        None => println!("No pair found."),
    }

    // (9, 11)
    let arr = vec![11, 2, 5, 9, 10, 3];

    match get_pair_half_num(&arr) {
        Some(pair) => println!("{:?}", pair),
        None => println!("No pair found."),
    }
}
