struct Solution {}

impl Solution {
    pub fn climb_stairs1(n: i32) -> i32 {
        // 2つ前のステップの方法数
        let mut prev = 0;
        // 1つ前のステップの方法数
        let mut curr = 1;
        // 現在のステップの方法数
        let mut total_ways = 0;

        // 1段目から n 段目まで順番に計算
        for _ in 1..=n {
            // 現在のステップへの方法数は、1つ前と2つ前の合計
            total_ways = prev + curr;
            // 前回の「1つ前」を「2つ前」に更新
            prev = curr;
            // 今回の結果を「1つ前」として次のループで使用
            curr = total_ways;
        }
        total_ways
    }

    // メモ化再帰（トップダウンDP）
    fn count_ways(steps_remaining: i32, memo: &mut Vec<i32>) -> i32 {
        // 残りの段数が負なら、そのパスは無効 (方法なし)
        if steps_remaining < 0 {
            return 0;
        }
        // 残りの段数が0なら、ちょうどゴールに到達 (1つの方法)
        if steps_remaining == 0 {
            return 1;
        }
        // 既に計算済みの場合、保存された値を返す (メモ化)
        if memo[steps_remaining as usize] != -1 {
            return memo[steps_remaining as usize];
        }
        // 1段登る場合と2段登る場合の合計を計算してメモ化
        memo[steps_remaining as usize] = Self::count_ways(steps_remaining - 1, memo)
            + Self::count_ways(steps_remaining - 2, memo);

        memo[steps_remaining as usize]
    }
    pub fn climb_stairs2(n: i32) -> i32 {
        // 計算済みの結果を保存するためのメモリ (初期値は -1)
        let mut memo = vec![-1; (n + 1) as usize];
        Self::count_ways(n, &mut memo)
    }

    pub fn climb_stairs3(n: i32) -> i32 {
        // 初期値として、1段前の方法数 (prev_ways) を 0 、現在の方法数 (current_ways) を 1 に設定
        (0..n)
            .fold((1, 0), |(curr_ways, prev_ways), _| (curr_ways + prev_ways, curr_ways))
            .0 // 最終的な方法数を返す
    }

    fn cross_product(a: [i32; 3], b: [i32; 3]) -> [i32; 3] {
        [
            a[0] * b[0] + a[1] * b[1],
            a[0] * b[1] + a[1] * b[2],
            a[1] * b[1] + a[2] * b[2]
        ]
    }
    pub fn climb_stairs4(mut n: i32) -> i32 {
        let mut base_matrix = [1, 1, 0]; // フィボナッチ数列を表す数列
        let mut result_matrix = [1, 0, 1]; // 単位行列に相当するもの

        while n > 0 {
            if n & 1 == 1 {
                // 奇数のとき、結果行列に現在の行列を掛ける
                result_matrix = Self::cross_product(result_matrix, base_matrix);
            }
            // 行列を2乗する (指数を半分にする)
            base_matrix = Self::cross_product(base_matrix, base_matrix);
            n >>= 1; // 2 で割る (右シフト)
        }
        result_matrix[0] // フィボナッチ数列の n 番目の値を返す
    }
}

fn main() {
    let n = 2;
    println!("{}", Solution::climb_stairs1(n));

    let n = 2;
    println!("{}", Solution::climb_stairs2(n));

    let n = 2;
    println!("{}", Solution::climb_stairs3(n));

    let n = 2;
    println!("{}", Solution::climb_stairs4(n));
}
