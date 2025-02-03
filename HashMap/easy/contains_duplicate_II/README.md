# Contains Duplicate II

```rust
struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicates1(nums: Vec<i32>, max_distance: i32) -> bool {
        use std::collections::HashMap;
        // 数列を nums の要素とそのインデックスの組に変換し、
        // ハッシュマップ (数値 -> インデックス) を用いて重複をチェック
        nums.into_iter()
            // (値、インデックス) のペアを作成
            .zip(0..)
            //  HashMap を管理しながら重複チェック
            .scan(HashMap::<i32, i32>::new(), |seen_indices: &mut HashMap<i32, i32>, (value, index)| {
                match seen_indices.insert(value, index) {
                    Some(previous_index) => Some(index - previous_index <= max_distance),
                    None => Some(false), //初めての値なら false
                }
            })
            // どこかで true になれば即終了
            .any(|is_duplicate_within_range| is_duplicate_within_range)
    }

    pub fn contains_nearby_duplicates2(nums: Vec<i32>, max_distance: i32) -> bool {
        use std::collections::VecDeque;

        let size = nums.len();
        if size <= 1 {
            return false; // 要素が 1 以下なら重複はあり得ない
        }

        let max_distance = max_distance as usize;
        let mut recent_values = VecDeque::<i32>::with_capacity(max_distance);

        for index in 0..size {
            // 直近の max_distance 以内に同じ値が存在するかをチェック
            if recent_values.contains(&nums[index]) {
                return true;
            }
            // キューに現在の値を追加
            recent_values.push_back(nums[index]);
            // ウィンドウサイズを max_distance に保つ
            if recent_values.len() > max_distance {
                recent_values.pop_front();
            }
        }
        false
    }

    pub fn contains_nearby_duplicates3(nums: Vec<i32>, max_distance: i32) -> bool {
        use std::collections::HashSet;

        let num_count = nums.len();
        if num_count <= 1 {
            return false; // 要素数が 1 以下なら重複はあり得ない
        }

        let max_distance = max_distance as usize;
        let mut recent_values = HashSet::new();

        for (index, &value) in nums.iter().enumerate() {
            // すでに recent_values に value があれば重複がある
            if recent_values.contains(&value) {
                return true;
            }

            // value をセットに追加
            recent_values.insert(value);

            // max_distance を越えたら古い値を削除 (ウィンドウの維持)
            if recent_values.len() > max_distance {
                recent_values.remove(&nums[index - max_distance]);
            }
        }
        false
    }

    pub fn contains_nearby_duplicates4(nums: Vec<i32>, max_distance: i32) -> bool {
        use std::collections::HashMap;

        let mut recent_values: HashMap<i32, usize> = HashMap::new();
        let mut left_index = 0;

        for right_index in 0..nums.len() {
            // スライディングウィンドウの範囲を max_distance に維持
            if right_index - left_index > max_distance as usize {
                recent_values.remove(&nums[left_index]);
                left_index += 1;
            }

            // すでに recent_values に nums[right_index] が存在すれば重複あり
            if recent_values.contains_key(&nums[right_index]) {
                return true;
            }

            // 現在の値を recent_values に追加
            recent_values.insert(nums[right_index], 1);
        }
        false
    }
}


fn main() {
    /*
     * One Linear with methods: zip, scan, any
     */
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    println!("{}", Solution::contains_nearby_duplicates1(nums, k));

    let nums = vec![1,0,1,1];
    let k = 1;
    println!("{}", Solution::contains_nearby_duplicates1(nums, k));

    let nums = vec![1,2,3,1,2,3];
    let k = 2;
    println!("{}", Solution::contains_nearby_duplicates1(nums, k));

    /*
     * Sliding Window
     */
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    println!("{}", Solution::contains_nearby_duplicates2(nums, k));

    let nums = vec![1,0,1,1];
    let k = 1;
    println!("{}", Solution::contains_nearby_duplicates2(nums, k));

    let nums = vec![1,2,3,1,2,3];
    let k = 2;
    println!("{}", Solution::contains_nearby_duplicates2(nums, k));

    /*
     * HashSet
     */
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    println!("{}", Solution::contains_nearby_duplicates3(nums, k));

    let nums = vec![1,0,1,1];
    let k = 1;
    println!("{}", Solution::contains_nearby_duplicates3(nums, k));

    let nums = vec![1,2,3,1,2,3];
    let k = 2;
    println!("{}", Solution::contains_nearby_duplicates3(nums, k));

    /*
     * HashMap
     */
    let nums = vec![1, 2, 3, 1];
    let k = 3;
    println!("{}", Solution::contains_nearby_duplicates4(nums, k));

    let nums = vec![1,0,1,1];
    let k = 1;
    println!("{}", Solution::contains_nearby_duplicates4(nums, k));

    let nums = vec![1,2,3,1,2,3];
    let k = 2;
    println!("{}", Solution::contains_nearby_duplicates4(nums, k));
}
```

```bash
true
true
false
true
true
false
true
true
false
true
true
false
```
