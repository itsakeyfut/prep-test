struct Solution {}

impl Solution {
    pub fn summary_ranges1(nums: Vec<i32>) -> Vec<String> {
        // 結果を格納するベクター
        let mut result = Vec::new();
        
        // 空の入力の場合は空リストを返す
        if nums.is_empty() {
            return result;
        }
    
        // 最初の要素を範囲の開始点として設定
        let mut start = nums[0];
    
        // numsのインデックス1から順に処理
        for i in 1..nums.len() {
            // 連続している場合
            if nums[i] != nums[i - 1] + 1 {
                // 範囲を追加
                if start == nums[i - 1] {
                    result.push(start.to_string());
                } else {
                    result.push(format!("{}->{}", start, nums[i - 1]));
                }
                // 新しい範囲の開始点を設定
                start = nums[i];
            }
        }
    
        // 最後の範囲を追加
        if start == nums[nums.len() - 1] {
            result.push(start.to_string());
        } else {
            result.push(format!("{}->{}", start, nums[nums.len() - 1]));
        }
    
        result
    }

    pub fn summary_ranges2(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut idx = 0;

        while idx < nums.len() {
            let start = nums[idx];

            // 連続する文字を探す
            while idx < nums.len() - 1 && nums[idx + 1] == nums[idx] + 1 {
                idx += 1;
            }

            // 範囲の終了点 (連続する数字の最後)
            let end = nums[idx];

            // 範囲が1つの数字であれば、start の形式で追加
            // 範囲が複数の数字であれば、start->end の形式で追加
            if start != end {
                result.push(format!("{}->{}", start, end));
            } else {
                result.push(format!("{}", start));
            }

            // 次の範囲に進むためにインデックスを進める
            idx += 1;
        }
        result
    }
}

fn main() {
    let nums = vec![0, 1, 2, 4, 5, 7];
    println!("result = {:?}", Solution::summary_ranges1(nums));
    let nums = vec![0, 2, 3, 4, 6, 8, 9];
    println!("result = {:?}", Solution::summary_ranges1(nums));

    let nums = vec![0, 1, 2, 4, 5, 7];
    println!("result = {:?}", Solution::summary_ranges2(nums));
    let nums = vec![0, 2, 3, 4, 6, 8, 9];
    println!("result = {:?}", Solution::summary_ranges2(nums));
}
