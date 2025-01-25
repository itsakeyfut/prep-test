struct Solution {}

impl Solution {
    /// 配列から重複を取り除き、重複しない要素数を返す
    /// Time Complexity: O(n) - 配列全体を一回スキャンする
    /// Space Complexity: O(1) - 追加メモリは不要
    pub fn remove_duplicates1(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut write_idx = 0;

        // 1つ目の要素はそのままにして、2つ目以降をチェック
        for read_idx in 1..nums.len() {
            // 重複していない要素を見つけた場合
            if nums[write_idx] != nums[read_idx] {
                write_idx += 1; // 書き込み位置を進める
                nums[write_idx] = nums[read_idx]; // 値を更新
            }
        }

        // 重複を取り除いた後の要素数を返す
        (write_idx + 1) as i32
    }

    /// dedupメソッドを使用して配列から重複を削除する
    /// Time Complexity: O(n) - dedupは配列全体を1回スキャンする
    /// Space Complexity: O(1) - インプレース処理
    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }

    /// matchを使用した場合
    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    pub fn remove_duplicates3(nums: &mut Vec<i32>) -> i32 {
        match nums.is_empty() {
            true => return 0,
            false => {

                let mut write_idx = 0;
                for read_idx in 1..nums.len() {
                    if nums[write_idx] != nums[read_idx] {
                        write_idx += 1;
                        nums[write_idx] = nums[read_idx];
                    }
                }
                (write_idx + 1) as i32
            }
        }
    }
}

fn main() {
    let mut nums = vec![1,1,2];
    let k = Solution::remove_duplicates1(&mut nums);
    println!("{}", k);

    let mut nums = vec![1,1,2];
    let k = Solution::remove_duplicates2(&mut nums);
    println!("{}", k);

    let mut nums = vec![1,1,2];
    let k = Solution::remove_duplicates3(&mut nums);
    println!("{}", k);
}
