fn binary_search(nums: &Vec<i32>, target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if nums[mid] == target {
            return Some(mid)
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    None
}

fn main() {
    let num = vec![0,1,5,7,9,11,15,20,24];
    let target = 20;

    match binary_search(&num, target) {
        Some(_) => println!("FOUND"),
        None => println!("NOT FOUND")
    }
}
