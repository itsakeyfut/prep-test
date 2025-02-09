struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // fold で、配列 nums の各要素を累積的に XOR 演算で処理
        nums.into_iter().fold(0, |prev, curr| {
            // XOR 演算で累積し、重複する数字をキャンセルしていく
            // XOR の性質：
            // 1. 同じ数同士の XOR は 0 になります
            // 2. 0 と XOR すると元の数がそのまま帰ります
            prev ^ curr
        })
    }
}

fn main() {
    let nums = vec![2, 2, 1];
    println!("{}", Solution::single_number(nums));

    let nums = vec![4,1,2,1,2];
    println!("{}", Solution::single_number(nums));

    let nums = vec![1];
    println!("{}", Solution::single_number(nums));
}