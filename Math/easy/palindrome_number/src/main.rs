struct Solution;
impl Solution {
    pub fn is_palindrome1(x: i32) -> bool {
        x.to_string().chars().rev().eq(x.to_string().chars())
    }

    pub fn is_palindrome2(x: i32) -> bool {
        // 数字を文字列に変換
        let number_str = x.to_string();
        let len = number_str.len();
        // 文字列をキャラクターのベクタに変換
        let number_chars = number_str.chars().collect::<Vec<char>>();
        // 前半部分 (先頭から半分) を取得
        let first_half = number_chars[..len / 2].iter();
        // 後半部分 (後ろから半分) を逆順に取得
        let second_half = number_chars[len / 2..].iter().rev();
        // 前半部分と後半部分を1対1で比較し、すべて一致すれば回文
        first_half.zip(second_half).all(|(a, b)| a == b)
    }

    pub fn is_palindrome3(x: i32) -> bool {
        // 負の数や10で割り切れる数 (ただし0は除く) は回文でない
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        // 数字を反転させるための変数
        let mut x = x;
        let mut rev = 0;

        // 数字が反転した部分よりも小さくなるまで処理を繰り返す
        while x > rev {
            // 最下位の桁を反転部分に追加
            rev = rev * 10 + x % 10;
            // 最下位の桁を削除
            x /= 10;
        }
        // 数字が反転部分と一致するか、反転部分を10で割った結果と一致する場合は回文
        x == rev || x == rev / 10
    }
}

fn main() {
    /*
     * One Linear
     */
    let x = 121;
    println!("{}", Solution::is_palindrome1(x));

    let x = -121;
    println!("{}", Solution::is_palindrome1(x));

    let x = 10;
    println!("{}", Solution::is_palindrome1(x));

    /*
     * Iterator
     */
    let x = 121;
    println!("{}", Solution::is_palindrome2(x));

    let x = -121;
    println!("{}", Solution::is_palindrome2(x));

    let x = 10;
    println!("{}", Solution::is_palindrome2(x));

    /*
     * Iterator
     */
    let x = 121;
    println!("{}", Solution::is_palindrome3(x));

    let x = -121;
    println!("{}", Solution::is_palindrome3(x));

    let x = 10;
    println!("{}", Solution::is_palindrome3(x));
}
