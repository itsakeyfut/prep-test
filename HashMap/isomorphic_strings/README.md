# Isomorphic Strings

```rust
use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        // ACII文字に対応するサイズ128の配列を用意し、文字の対応関係を記録する
        let mut array1 = vec![0; 128];
        let mut array2 = vec![0; 128];

        // sとtの文字列を同時に走査する
        for (s_ch, t_ch) in s.chars().zip(t.chars()) {
            if array1[s_ch as usize] == 0 && array2[t_ch as usize] == 0 {
                // 初めての対応関係の場合、それぞれの配列に対応を記録
                array1[s_ch as usize] = t_ch as i32; // sの文字がtのどの文字に対応しているかを記録
                array2[t_ch as usize] = s_ch as i32; // tの文字がsのどの文字に対応しているかを記録
            } else if array1[s_ch as usize] != t_ch as i32 || array2[t_ch as usize] != s_ch as i32 {
                // 既存の対応関係の矛盾している場合、不等形であるためfalseを返す
                return false;
            }
        }
        // すべての文字が対応関係に矛盾せずに走査できた場合、trueを返す
        true
    }

    pub fn is_isomorphic2(s: String, t: String) -> bool {
        // `String`をバイト列(`Vec<u8>`)に変換し、ASCII文字列として扱う
        let (s, t) = (s.into_bytes(), t.into_bytes());

        // `HashMap`を初期化して、文字の対応関係を記録する
        let mut map: HashMap<u8, u8> = HashMap::with_capacity(s.len());

        // 各文字のインデックスを操作
        for idx in 0..s.len() {
            //`HashMap`にs[idx]がなければt[idx]を対応として記録する
            let val = map.entry(s[idx]).or_insert(t[idx]);

            // 現在の対応が既存の対応と矛盾している場合は`false`を返す
            if *val != t[idx] {
                return false;
            }
        }

        // マップ内のキー（sの文字）と値（tの文字）の一意性を確認する
        // 対応が1対1でない場合（値の重複がある場合）もfalseを返す
        map.values().collect::<HashSet<_>>().len() == map.keys().collect::<HashSet<_>>().len()
    }

    pub fn is_isomorphic3(s: String, t: String) -> bool {
        let (s, t) = (s.into_bytes(), t.into_bytes());
        let mut map = HashMap::new();

        for idx in 0..s.len() {
            if map.entry(s[idx]).or_insert(t[idx]) != &t[idx] {
                return false;
            }
        }
        true
    }
}

fn main() {
    /*
     * array
     */
    let s = "egg".to_string();
    let t = "add".to_string();
    println!("result = {}", Solution::is_isomorphic(s, t));

    let s = "foo".to_string();
    let t = "bar".to_string();
    println!("result = {}", Solution::is_isomorphic(s, t));

    let s = "paper".to_string();
    let t = "title".to_string();
    println!("result = {}", Solution::is_isomorphic(s, t));

    /*
     * HashMap
     */
    let s = "egg".to_string();
    let t = "add".to_string();
    println!("result = {}", Solution::is_isomorphic2(s, t));

    let s = "foo".to_string();
    let t = "bar".to_string();
    println!("result = {}", Solution::is_isomorphic2(s, t));

    let s = "paper".to_string();
    let t = "title".to_string();
    println!("result = {}", Solution::is_isomorphic2(s, t));

    /*
     * HashMap (1対1チェックを短縮)
     */
    let s = "egg".to_string();
    let t = "add".to_string();
    println!("result = {}", Solution::is_isomorphic3(s, t));

    let s = "foo".to_string();
    let t = "bar".to_string();
    println!("result = {}", Solution::is_isomorphic3(s, t));

    let s = "paper".to_string();
    let t = "title".to_string();
    println!("result = {}", Solution::is_isomorphic3(s, t));
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
