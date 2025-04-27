struct Solution {}

impl Solution {
    pub fn is_anagram1(s: String, t: String) -> bool {
        // 各文字の出現回数をカウントするためのハッシュマップ
        let mut map = std::collections::HashMap::new();
        // 文字列 s の各文字のカウントを増やす
        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
        // 文字列 t の各文字のカウントを減らす
        t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
        // 全ての文字のカウントが 0 ならアナグラムと判定
        map.into_values().all(|v| v == 0)
    }

    pub fn is_anagram2(s: String, t: String) -> bool {
        let mut s_char: Vec<char> = s.chars().collect();
        let mut t_char: Vec<char> = t.chars().collect();

        s_char.sort();
        t_char.sort();

        s_char == t_char
    }

    pub fn is_anagram3(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        // 英小文字の出現回数を記録する配列
        let mut chars = [0; 26];

        for c in 0..s.len() {
            // 'a' のASCII値 (97) を引くことでインデックスを 0～25 に変換
            chars[s.as_bytes()[c] as usize - 97] += 1;
            chars[t.as_bytes()[c] as usize - 97] -= 1;
        }

        // すべてのカウントが 0 ならアナグラム
        chars.iter().all(|c| *c == 0)
    }
}

fn main() {
    /*
     * HashMap
     */
    let s = "anagram";
    let t = "nagaram";
    println!("{}", Solution::is_anagram1(s.to_string(), t.to_string()));

    let s = "rat";
    let t = "car";
    println!("{}", Solution::is_anagram1(s.to_string(), t.to_string()));

    /*
     * Sort
     */
    let s = "anagram";
    let t = "nagaram";
    println!("{}", Solution::is_anagram2(s.to_string(), t.to_string()));

    let s = "rat";
    let t = "car";
    println!("{}", Solution::is_anagram2(s.to_string(), t.to_string()));

    /*
     * Array
     */
    let s = "anagram";
    let t = "nagaram";
    println!("{}", Solution::is_anagram3(s.to_string(), t.to_string()));

    let s = "rat";
    let t = "car";
    println!("{}", Solution::is_anagram3(s.to_string(), t.to_string()));
}
