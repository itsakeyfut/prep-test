use std::collections::{HashMap, HashSet};

struct Solution {}

#[derive(Eq, PartialEq, Hash)]
enum Entry<'a> {
    Letter(u8),
    Word(&'a str),
}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        // パターンのもじすうと、文字列`s`の単語数を比較する
        if pattern.len() != s.matches(' ').count() + 1 {
            return false;
        }

        // 文字 -> 単語の対応を記録するためのHashMap
        let mut map = HashMap::new();
        let mut word_set = HashSet::new();

        // `pattern`の各文字と`s`の単語をペアとして走査する
        for (word, c) in s.split_ascii_whitespace().zip(pattern.chars()) {
            // 現在の文字`c`が`HashMap`に存在する場合
            if let Some(w) = map.insert(c, word) {
                // 既存の単語`w`と現在の単語`word`が異なる場合、`false`を返す
                if w != word {
                    return false;
                }
            // 現在の文字`c`が新規の場合
            } else if !word_set.insert(word) {
                return false;
            }
        }
        true
    }

    pub fn word_pattern2(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split_ascii_whitespace().collect();
        if pattern.len() != words.len() {
            return false;
        }

        let mut map = HashMap::new();
        let mut word_set = HashSet::new();

        for (word, c) in words.into_iter().zip(pattern.chars()) {
            if let Some(&w) = map.get(&c) {
                if w != word {
                    return false;
                }
            } else if !word_set.insert(word) {
                return false;
            } else {
                map.insert(c, word);
            }
        }
        true
    }

    pub fn word_pattern3(pattern: String, s: String) -> bool {
        // `pattern`のバイト (文字) を`Option<u8>`に変換し、`None`を無限に連結
        pattern.bytes().map(|l| Some(l)).chain(std::iter::repeat(None))
            // `s`の単語を`Option<&str>`に変換し、`None`を無限に連結
            .zip(s.split_whitespace().map(|w| Some(w)).chain(std::iter::repeat(None)))
            // 各要素にインデックスを付与
            .enumerate()
            // `HashMap`を使用して、対応を管理
            .scan(HashMap::<Entry, usize>::new(), |map, (index, (letter_opt, word_opt))| 
                match (letter_opt, word_opt) {
                    // 両方が`None`であれば終了
                    (None, None) => return None,
                    // 両方が`Some`であれば対応を確認または登録
                    (Some(letter), Some(word)) => Some(
                        *map.entry(Entry::Letter(letter)).or_insert(index)
                        == *map.entry(Entry::Word(word)).or_insert(index)
                    ),
                    // 片方だけが`None`の場合はパターンに矛盾があるため`false`
                    _ => Some(false),
                }
            // すべての対応が正しい場合のみ`true`
            ).all(|ok| ok)
    }
}

fn main() {
    /*
     * readable
     */
    let pattern = "abba".to_string();
    let s = "dog cat cat dog".to_string();
    println!("result = {}", Solution::word_pattern(pattern, s));

    let pattern = "abba".to_string();
    let s = "dog cat cat fish".to_string();
    println!("result = {}", Solution::word_pattern(pattern, s));

    let pattern = "aaaa".to_string();
    let s = "dog cat cat dog".to_string();
    println!("result = {}", Solution::word_pattern(pattern, s));

    /*
     * readable without extra check
     */
    let pattern = "abba".to_string();
    let s = "dog cat cat dog".to_string();
    println!("result = {}", Solution::word_pattern2(pattern, s));

    let pattern = "abba".to_string();
    let s = "dog cat cat fish".to_string();
    println!("result = {}", Solution::word_pattern2(pattern, s));

    let pattern = "aaaa".to_string();
    let s = "dog cat cat dog".to_string();
    println!("result = {}", Solution::word_pattern2(pattern, s));

    /*
     * complex
     */
    let pattern = "abba".to_string();
    let s = "dog cat cat dog".to_string();
    println!("result = {}", Solution::word_pattern3(pattern, s));

    let pattern = "abba".to_string();
    let s = "dog cat cat fish".to_string();
    println!("result = {}", Solution::word_pattern3(pattern, s));

    let pattern = "aaaa".to_string();
    let s = "dog cat cat dog".to_string();
    println!("result = {}", Solution::word_pattern3(pattern, s));
}
