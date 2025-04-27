# Add Binary

```rust
struct Solution;
impl Solution {
    pub fn add_binary1(a: String, b: String) -> String {
        let mut result = String::new();
        let mut carry = 0; // 繰り上がり

        let mut i = a.len() as i32 - 1;
        let mut j = b.len() as i32 - 1;

        // 両方の文字列が終わるまで繰り返す
        while i >= 0 || j >= 0 || carry != 0 {
            let mut sum = carry; // 繰り上がりを加える

            if i >= 0 {
                sum += a.chars().nth(i as usize).unwrap().to_digit(2).unwrap() as i32;
                i -= 1;
            }
            if j >= 0 {
                sum += b.chars().nth(j as usize).unwrap().to_digit(2).unwrap() as i32;
                j -= 1;
            }

            // 繰り上がりを更新
            carry = sum / 2;

            // 現在の桁を結果に追加
            result.push_str(&(sum % 2).to_string());
        }

        result.chars().rev().collect()
    }

    pub fn add_binary2(a: String, b: String) -> String {
        use std::iter;

        let mut carry = 0;
        let mut cur_sum = 0;
        let mut result_chars = a
            // a を逆順にし、b は長さが足りない分 '0' を繰り返して連結
            .as_bytes()
            .iter()
            .rev()
            // b が足りない場合は '0' を繰り返す
            .chain(iter::repeat(&b'0'))
            // a と b のバイト列を逆順に処理
            .zip(b.as_bytes().iter().rev().chain(iter::repeat(&b'0')))
            // どちらかの文字列が終わるまで繰り返す
            .take(a.len().max(b.len()))
            // 各桁を加算し、繰り上がりの処理を行う
            .map(|(ac, bc)| {
                // 現在の桁の和
                cur_sum = (*ac - b'0') + (*bc - b'0') + carry;
                // 繰り上がりを更新
                carry = cur_sum / 2;
                // 現在の桁の値を決定 ('0' または '1')
                match cur_sum % 2 {
                    1 => '1',
                    _ => '0',
                }
            })
            .collect::<Vec<_>>();

        // 最後に繰り上がりが残っていた場合、それを追加
        if carry == 1 {
            result_chars.push('1');
        }

        // 結果の文字列を作成 (逆順にした Vec<char> を文字列に変換)
        result_chars.iter().rev().collect()
    }
}

fn main() {
    let a = "11";
    let b = "1";
    println!("{}", Solution::add_binary1(a.to_string(), b.to_string()));

    let a = "1010";
    let b = "1011";
    println!("{}", Solution::add_binary1(a.to_string(), b.to_string()));

    let a = "11";
    let b = "1";
    println!("{}", Solution::add_binary2(a.to_string(), b.to_string()));

    let a = "1010";
    let b = "1011";
    println!("{}", Solution::add_binary2(a.to_string(), b.to_string()));
}
```

```bash
100
10101
100
10101
```
