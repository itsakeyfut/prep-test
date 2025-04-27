struct Solution;
impl Solution {
    pub fn plus_one(mut number_digits: Vec<i32>) -> Vec<i32> {
        for digit in number_digits.iter_mut().rev() {
            match *digit == 9 {
                // 9の場合は盛り上がるので、0に変更
                true => *digit = 0,
                false => {
                    // 9以外なら+1して終了
                    *digit += 1;
                    return number_digits;
                }
            }
        }
        // 全ての桁が9だった場合、最上位桁に1を追加
        number_digits.insert(0, 1);
        number_digits
    }
}

fn main() {
    let digits = vec![1, 2, 3];
    println!("{:?}", Solution::plus_one(digits));

    let digits = vec![4, 3, 2, 1];
    println!("{:?}", Solution::plus_one(digits));
    
    let digits = vec![9];
    println!("{:?}", Solution::plus_one(digits));
}
