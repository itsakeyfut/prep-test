# Search Insert Position

```rust
struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0; // 配列の最初のインデックス
        let mut right = nums.len() as i32 - 1; // 配列の最後のインデックス

        // 二分探索を実行
        while left <= right {
            // 中央のインデックスを計算
            let mid = left + (right - left) / 2;

            // 中央の値がターゲットと一致すれば、そのインデックスを返す
            if nums[mid as usize] == target {
                return mid;
            }
            // 中央の値がターゲットより小さければ、右半分を探索
            else if nums[mid as usize] < target {
                left = mid + 1;
            }
            // 中央の値がターゲットより大きければ、左半分を探索
            else {
                right = mid - 1;
            }
        }
        // ターゲット値が見つからなければ、left が挿入すべき位置
        left
    }
}

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    println!("{}", Solution::search_insert(nums, target));

    let nums = vec![1, 3, 5, 6];
    let target = 2;
    println!("{}", Solution::search_insert(nums, target));

    let nums = vec![1, 3, 5, 6];
    let target = 7;
    println!("{}", Solution::search_insert(nums, target));
}
```

```bash
2
1
4
```
