use std::cmp::Ordering;

struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }

        let mut lower = 2;
        let mut higher = 46340.min(x / 2); // √(i32の最大値) ≈ 46340

        if x <= 3 {
            return 1; // 1, 2, 3 の場合は √x = 1
        }

        if x >= higher * higher {
            return higher; // 最大の平方数のケースを処理
        }

        while higher - lower > 1 {
            let mid = (higher + lower) / 2;
            let pow = mid * mid;

            match pow.cmp(&x) {
                Ordering::Less => {
                    lower = mid; // mid の平方が x より小さい場合、探索範囲を右に
                }
                Ordering::Greater => {
                    higher = mid; // mid の平方が x より大きい場合、探索範囲を左に
                }
                _ => {
                    return mid; // ぴったり平方根が見つかった場合
                }
            }
        }
        lower // x に最も近い整数平方根を返す
    }
}

fn main() {
    let x = 4;
    println!("{}", Solution::my_sqrt(x));

    let x = 8;
    println!("{}", Solution::my_sqrt(x));

    let x = 9;
    println!("{}", Solution::my_sqrt(x));

    let x = 49;
    println!("{}", Solution::my_sqrt(x));
}