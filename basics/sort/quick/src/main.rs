use rand::Rng;

fn quick_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    let pivot_idx = partition(nums);
    quick_sort(&mut nums[0..pivot_idx]);
    quick_sort(&mut nums[pivot_idx + 1..]);
}

fn partition(nums: &mut [i32]) -> usize {
    let len = nums.len();

    let mut rng = rand::rng();
    let pivot_idx = rng.random_range(0..len);
    nums.swap(pivot_idx, len - 1);

    let pivot = nums[len - 1];
    let mut i = 0;

    for j in 0..len - 1 {
        if nums[j] <= pivot {
            nums.swap(i, j);
            i += 1;
        }
    }

    nums.swap(i, len - 1);
    i
}

fn main() {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (0..10)
        .map(|_| rng.random_range(0..1000))
        .collect();

    println!("BEFORE: {:?}", nums);
    quick_sort(&mut nums);
    println!("AFTER: {:?}", nums);
}

