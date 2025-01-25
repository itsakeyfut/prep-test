use std::collections::HashMap;

struct Solution {}

impl Solution {
    // ハッシュマップを使ったバージョン
    pub fn majority_element1(nums: Vec<i32>) -> i32 {
        // 要素の出現回数を記録するためのハッシュマップを作成
        let mut map: HashMap<i32, i32> = HashMap::new();
        let len = nums.len() as i32;

        // 各要素の出現回数をカウント
        for num in nums.iter() {
            // キーが存在しない場合は0を挿入
            let cnt = map.entry(*num).or_insert(0);
            // 出現回数をインクリメント
            *cnt += 1;
        }

        // 過半数を占める要素を探す
        for (num, count) in map.iter() {
            // 過半数かどうかを判定
            if *count > len / 2 {
                // 過半数要素を返す
                return *num;
            }
        }

        // 過半数要素が存在しない場合はフォールバック
        -1
    }

    // ボイヤー・ムーア投票アルゴリズム
    pub fn majority_element2(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut elm = 0;

        for &num in &nums {
            // カウントが0の場合、新しい候補を選択
            if cnt == 0 {
                elm = num;
            }
            // 候補と一致する場合、カウントを追加
            if elm == num {
                cnt += 1;
            }
            // 候補と一致しない場合、カウントを減少
            else {
                cnt -= 1;
            }
        }

        // 過半数であるかを検証
        let mut cnt = 0;
        for &num in &nums {
            if num == elm {
                // 最終候補の出現回数をカウント
                cnt += 1;
            }
        }

        // 過半数を占めるかどうかを確認
        if cnt > nums.len() / 2 {
            elm
        } else {
            -1
        }
    }
}

fn main() {
    let arr = vec![6, 5, 5];

    let elem1 = Solution::majority_element1(arr.clone());
    println!("{}", elem1);

    let arr = vec![6, 5, 5];

    let elem2 = Solution::majority_element2(arr.clone());
    println!("{}", elem2);
}
