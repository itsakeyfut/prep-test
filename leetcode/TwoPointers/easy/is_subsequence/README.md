# Is Subsequence

```rust
struct Solution {}

impl Solution {
    pub fn is_subsequence(sub: String, main: String) -> bool {
        let mut sub_index = 0;
        let mut main_index = 0;
        let sub_chars: Vec<char> = sub.chars().collect();
        let main_chars: Vec<char> = main.chars().collect();

        // main を走査しながら、sub の文字列が順番に含まれているか確認
        while sub_index < sub.len() && main_index < main.len() {
            // sub の現在の文字と main の現在の文字が一致する場合、sub の次の文字へ進む
            if sub_chars[sub_index] == main_chars[main_index] {
                sub_index += 1;
            }
            // main の次の文字へ進む
            main_index += 1;
        }
        // sub の全ての文字が main 内で順番通りに現れていれば sub_index は sub の長さと等しければ true
        sub_index == sub.len()
    }

    pub fn is_subsequence2(sub: String, main: String) -> bool {
        // sub の文字イテレータを作成
        let mut sub_iter = sub.chars();
        // sub の最初の文字を取得
        let mut current_char = sub_iter.next();

        for main_char in main.chars() {
            if let Some(sub_char) = current_char {
                // sub の現在文字と main の現在の文字が一致する場合、sub の次の文字へ進む
                if sub_char == main_char {
                    current_char = sub_iter.next();
                }
            } else {
                // sub のすべての文字が見つかった場合ループを終了
                break;
            }
        }
        // current_char が None になっていれば、すべての文字が main 内で順番通りに現れたことを意味する
        current_char.is_none()
    }

    pub fn is_subsequence3(sub: String, main: String) -> bool {
        let mut main_iter = main.chars();

        for sub_char in sub.chars() {
            match main_iter.find(|&main_char| main_char == sub_char) {
                Some(_) => (),
                None => return false,
            }
        }
        true
    }

    pub fn is_subsequence4(sub: String, main: String) -> bool {
        use std::collections::HashMap;

        let mut char_pos: HashMap<char, Vec<usize>> = HashMap::new();

        // main の各文字の出現回数を記録するハッシュマップを作成
        for (index, main_char) in main.chars().enumerate() {
            match char_pos.get_mut(&main_char) {
                // 既に存在するキーなら追加
                Some(pos) => { pos.push(index); },
                // 新規キーなら作成
                None => { char_pos.insert(main_char, vec![index]); }
            }
        }

        // sub の各文字を main で検索する際の開始位置
        let mut prev_index = 0;

        for sub_char in sub.chars() {
            match char_pos.get(&sub_char) {
                // main に sub_char が含まれていなければ false
                None => return false,
                Some(pos) => {
                    match pos.binary_search(&prev_index) {
                        // 見つかった場合、次の位置へ
                        Ok(idx) => prev_index = pos[idx] + 1,
                        Err(insert_index) => {
                            if insert_index == pos.len() {
                                // 適切な位置が見つからなければ false
                                return false;
                            }
                            // 次に検索する位置を更新
                            prev_index = pos[insert_index] + 1;
                        }
                    }
                }
            }
        }

        true
    }
}

fn main() {
    /*
     * two pointers
     */
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence(s, t);
    println!("result = {}", result);

    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence(s, t);
    println!("result = {}", result);

    /*
     * iterator 1
     */
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence2(s, t);
    println!("result = {}", result);

    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence2(s, t);
    println!("result = {}", result);

    /*
     * iterator 2
     */
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence3(s, t);
    println!("result = {}", result);

    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence3(s, t);
    println!("result = {}", result);

    /*
     * binary search
     */
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence4(s, t);
    println!("result = {}", result);

    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence4(s, t);
    println!("result = {}", result);
}
```

```bash
result = true
result = false

result = true
result = false

result = true
result = false

result = true
result = false
```
