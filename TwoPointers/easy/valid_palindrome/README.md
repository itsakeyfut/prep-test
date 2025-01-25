# Valid Palindrome

```rust
struct Solution {}

impl Solution {
    /// String comparison using filter method
    pub fn is_palindrome(s: String) -> bool {
        // 入力文字列から英数字のみを抽出し、小文字に変換した新しい文字列を生成
        let new_str: String = s.chars() // 入力文字列を1文字ずつイテレート
            .filter(|c| c.is_alphanumeric()) // 英数字のみをフィルタリング
            .map(|c| c.to_ascii_lowercase())// 小文字に変換（大文字と小文字を区別しないため）
            .collect(); // フィルタリングと変換の結果を文字列として収集

        // 新しい文字列がその反転文字列と等しいかを判定
        new_str == new_str.chars().rev().collect::<String>()
    }

    /// Classic Two Pointers
    pub fn is_palindrome2(s: String) -> bool {
        // 入力文字列を1文字ずつのベクタに変換
        let chars: Vec<char> = s.chars().collect();

        // 空文字列は回文とみなすため、trueを返す
        if chars.is_empty() {
            return true;
        }

        // 左ポインタを0 (先頭) 、右ポインタを文字列の最後のインデックスに設定
        let mut l = 0;                  // left
        let mut r = chars.len() - 1;    // right

        // 左右ポインタが交差するまでループ
        while l < r {
            // 左側のポインタを進め、英数字ではない文字をスキップ
            while l < r && !chars[l].is_alphanumeric() {
                l += 1;
            }
            // 右側のポインタを戻し、営為数字ではない文字をスキップ
            while l < r && !chars[r].is_alphanumeric() {
                r = r.saturating_sub(1); // saturating_subを使用することで、r が負の値になることを防ぐ
            }

            // 英数字の文字で比較を行う
            if l < r {
                // 小文字に変換して左右の文字を比較、一致しなければ回文ではない
                if chars[l].to_ascii_lowercase() != chars[r].to_ascii_lowercase() {
                    return false;
                }

                // 左右のポインタをそれぞれ進める/戻す
                l += 1;
                r = r.saturating_sub(1);
            }
        }
        // すべての比較をパスした場合、回文と判定
        true
    }

    pub fn is_palindrome3(s: String) -> bool {
        let iter = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase());

        iter.clone().eq(iter.rev())
    }

}

fn main() {
    /*
     * filter
     */
    let s = "A man, a plan, a canal: Panama".to_string();
    let result = Solution::is_palindrome(s);
    println!("result = {}", result);

    let s = "race a car".to_string();
    let result = Solution::is_palindrome(s);
    println!("result = {}", result);

    let s = " ".to_string();
    let result = Solution::is_palindrome(s);
    println!("result = {}", result);

    /*
     * two pointers
     */
    let s = "A man, a plan, a canal: Panama".to_string();
    let result = Solution::is_palindrome2(s);
    println!("result = {}", result);

    let s = "race a car".to_string();
    let result = Solution::is_palindrome2(s);
    println!("result = {}", result);

    let s = " ".to_string();
    let result = Solution::is_palindrome2(s);
    println!("result = {}", result);

    /*
     * eq
     */
    let s = "A man, a plan, a canal: Panama".to_string();
    let result = Solution::is_palindrome3(s);
    println!("result = {}", result);

    let s = "race a car".to_string();
    let result = Solution::is_palindrome3(s);
    println!("result = {}", result);

    let s = " ".to_string();
    let result = Solution::is_palindrome3(s);
    println!("result = {}", result);
}
```

```bash
result = true
result = false
result = true

result = true
result = false
result = true

result = true
result = false
result = true
```
