// Happy Number
// 1² + 9² = 1 + 81 = 82
// 8² + 2² = 64 + 4 = 68
// 6² + 8² = 36 + 64 = 100
// 1² + 0² + 0² = 1

struct Solution {}

impl Solution {
    pub fn is_happy1(mut n: i32) -> bool {
        // 1桁の数で「ハッピーな数」は1か7のみ (7はループを経て1になる)
        if n < 10 {
            return n == 1 || n == 7;
        }

        // 各桁の2乗の合計を計算する変数
        let mut squared_sum = 0;

        // 数字の各桁を2乗して合計を求める
        while n > 0 {
            // 1の位の桁を取得
            let digit = n % 10;
            // 2乗して合計に加える
            squared_sum += digit * digit;
            // 1桁減らす
            n /= 10;
        }
        // 再帰的に次の数で同じ処理を繰り返す
        Solution::is_happy1(squared_sum)
    }

    pub fn is_happy2(mut n: i32) -> bool {
        // 数値の各桁をイテレータとして返す
        let extra_digits = |mut num| {
            std::iter::from_fn(move || {
                if num != 0 {
                    // 一番右の桁を取得
                    let last_digit = num % 10;
                    // 次の桁へ
                    num /= 10;
                    Some(last_digit)
                } else {
                    // 全ての桁を取得し終えたら終了
                    None
                }
            })
        };

        // 収束しない無限ループを防ぐために最大のイテレーション数
        let max_iterations = 2 * 2 + 9 * 9 * 9; // ループ回数の上限

        // 
        for _ in 0 ..= max_iterations {
            // 各桁の2乗の合計を求める
            n = extra_digits(n).map(|x| x * x).sum();
            if n == 1 {
                // ハッピーな数
                return true;
            }
        }
        // ループの上限に達した場合はハッピーではない
        return false;
    }
}

fn main() {
    let n1 = 19;
    let n2 = 2;
    println!("{}", Solution::is_happy1(n1));
    println!("{}", Solution::is_happy1(n2));

    let n1 = 19;
    let n2 = 2;
    println!("{}", Solution::is_happy2(n1));
    println!("{}", Solution::is_happy2(n2));
}