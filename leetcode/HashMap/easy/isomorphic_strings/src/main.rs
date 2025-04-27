struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        // ASCII文字に対応するサイズ128の配列を用意し、文字の対応関係を記録する
        let mut s_to_t_map = vec![0; 128];
        let mut t_to_s_map = vec![0; 128];
        // s と t の文字列を同時に走査する
        for (s_char, t_char) in s.chars().zip(t.chars()) {
            let s_idx = s_char as usize;
            let t_idx = t_char as usize;
            if s_to_t_map[s_idx] == 0 && t_to_s_map[t_idx] == 0 {
                // 初めての対応関係の場合、それぞれのマッピングを記録する
                s_to_t_map[s_idx] = t_char as i32;
                t_to_s_map[t_idx] = s_char as i32;
            } else if s_to_t_map[s_idx] != t_char as i32 || t_to_s_map[t_idx] != s_char as i32 {
                // 既存の対応関係と矛盾している場合、同形ではないので false を返す
                return false;
            }
        }
        // すべての文字の対応関係が矛盾せずに走査できた場合、同形であるため true を返す
        true
    }

    pub fn is_isomorphic2(s: String, t: String) -> bool {
        use std::collections::{HashMap, HashSet};
        // `String`をバイト列(`Vec<u8>`)に変換し、ASCII文字列として扱う
        let (s_bytes, t_bytes) = (s.into_bytes(), t.into_bytes());
        // `HashMap`を初期化して、文字の対応関係を記録する
        let mut s_to_t_map: HashMap<u8, u8> = HashMap::with_capacity(s_bytes.len());
        // 各文字のインデックスを操作
        for idx in 0..s_bytes.len() {
            // HashMap に s_bytes[idx] の対応関係がなければ、t_bytes[idx] を記録する
            let mapped_value = s_to_t_map
                .entry(s_bytes[idx])
                .or_insert(t_bytes[idx]);
            // 既存の対応関係と矛盾している場合は false を返す
            if *mapped_value != t_bytes[idx] {
                return false;
            }
        }
        // 1対1 の対応関係を保証するため、対応関係の値（t の文字）が重複していないか確認
        // values()は全てのキーの値を取得する
        let unique_values: HashSet<_> = s_to_t_map.values().collect();
        unique_values.len() == s_to_t_map.len()
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
}
