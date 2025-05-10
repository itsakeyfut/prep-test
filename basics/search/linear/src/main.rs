pub fn linear_search(nums: &Vec<i32>, target: i32) -> Option<usize> {
    for (idx, &num) in nums.iter().enumerate() {
        if num == target {
            return Some(idx)
        }
    }
    None
}

fn main() {
    let num = vec![0,1,5,7,9,11,15,20,24];
    let target = 11;

    match linear_search(&num, target) {
        Some(_) => println!("FOUND"),
        None => println!("NOT FOUND")
    }
}
