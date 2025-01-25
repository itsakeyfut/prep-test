struct Solution {}

impl Solution {
    /// 指定された値を削除し、変更後の長さを返す
    // Time Complexity: O(n) - 配列を1回スキャンするため。
    // Space Complexity: O(1) - 追加のメモリは使用しない
    pub fn remove_element1(nums: &mut Vec<i32>, val: i32) -> i32 {
        // 書き込みインデックスを初期化
        let mut write_idx = 0;

        // 読み込みインデックスをループで処理
        for read_idx in 0..nums.len() {
            // 現在の要素が削除対象でない場合
            if nums[read_idx] != val {
                // 書き込みインデックスに値を保存
                nums[write_idx] = nums[read_idx];
                // 書き込みインデックスを更新
                write_idx += 1; 
            }
        }
        // 更新後の有効な要素数を返す
        write_idx as i32
    }

    /// Vecのretainメソッドを使用して要素をフィルタリング
    /// Time Complexity: O(n) - retainメソッドも全要素をスキャンするため
    /// Space Complexity: O(1) - 元のは配列をインプレースで変更
    pub fn remove_element2(nums: &mut Vec<i32>, val: i32) -> i32 {
        // retainメソッドで、条件を満たす要素だけを保持
        nums.retain(|&x| x != val);
        // 更新後のベクタの長さを返す
        return nums.len() as i32;
    }
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;

    let k = Solution::remove_element1(&mut nums, val);

    println!("{}", k);

    let mut nums = vec![3, 2, 2, 3];
    let val = 3;

    let k = Solution::remove_element2(&mut nums, val);

    println!("{}", k);
}
