use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        // pattern の文字数と s 内の単語数が一致しない場合は false
        if pattern.len() != s.matches(' ').count() + 1 {
            return false;
        }
        // pattern_char_to_word はパターンの文字と単語の対応を記録するマップ
        let mut pattern_char_to_word = HashMap::new();
        // used_words は既にマッピングされた単語の集合 (重複チェック用)
        let mut used_words = HashSet::new();
        // pattern の各文字と s の単語をペアにして走査する
        // split_ascii_whitespaceはブランク区切りで単語を取得する
        for (word, pattern_char) in s.split_ascii_whitespace().zip(pattern.chars()) {
            // pattern_char が既にマップに存在する場合
            if let Some(&mapped_word) = pattern_char_to_word.get(&pattern_char) {
                // 既存の単語と現在の単語が異なる場合、不一致なので false
                if mapped_word != word {
                    return false;
                }
            } else {
                // pattern_char が新規の場合
                // すでに別の pattern_char に割り当てられた単語が used_words 存在するなら false
                if !used_words.insert(word) {
                    return false;
                }
                // pattern_char と word のマッピングを記録
                pattern_char_to_word.insert(pattern_char, word);
            }
        }
        true
    }

    pub fn word_pattern2(pattern: String, s: String) -> bool {
        // s を空白区切りで単語に分割し、ベクタに格納
        let words: Vec<&str> = s.split_ascii_whitespace().collect();
        // pattern の文字数と s 内の単語数が一致しない場合は false
        if pattern.len() != words.len() {
            return false;
        }
        // パターンの文字 -> 単語の対応を記録するマップ
        let mut pattern_char_to_word = HashMap::new();
        // 既に対応付けられた単語を記録するセット
        let mut used_words = HashSet::new();
        // pattern の各文字と words の単語をペアとして走査
        for (word, pattern_char) in words.into_iter().zip(pattern.chars()) {
            // 既に対応がある場合、それが現在の単語と一致するかをチェック
            if let Some(&mapped_word) = pattern_char_to_word.get(&pattern_char) {
                if mapped_word != word {
                    return false;
                }
            } else {
                // 既に別のパターン文字と紐づいた単語なら false
                if !used_words.insert(word) {
                    return false;
                }
                // 新しい対応を登録
                pattern_char_to_word.insert(pattern_char, word);
            }
        }
        true
    }

    pub fn word_pattern3(pattern: String, s: String) -> bool {
        #[derive(Hash, Eq, PartialEq)]
        enum Entry {
            Letter(u8),
            Word(String),
        }
        // pattern のバイト (文字) を Option<u8> に変換し、None を無限に連結
        pattern.bytes().map(|ch| Some(ch)).chain(std::iter::repeat(None))
            // s の単語を Option<&str> に変換し、None を無限に連結
            .zip(s.split_whitespace().map(|word| Some(word)).chain(std::iter::repeat(None)))
            // 各要素にインデックスを付与
            .enumerate()
            // HashMap を使用して、文字と単語の対応関係を管理
            .scan(HashMap::<Entry, usize>::new(), |map, (idx, (letter_opt, word_opt))|
                match (letter_opt, word_opt) {
                    // 両方が None であれば修了
                    (None, None) => return None,
                    // 両方が Some であれば対応を確認または登録
                    (Some(letter), Some(word)) => Some(
                        *map.entry(Entry::Letter(letter)).or_insert(idx)
                        == *map.entry(Entry::Word(word.to_string())).or_insert(idx)
                    ),
                    // 片方だけが None の場合はパターンに矛盾があるため false
                    _ => Some(false)
                }
            // すべての対応が正しい場合のみ true
            ).all(|is_matching| is_matching)
    }

    pub fn word_pattern4(pattern: String, s: String) -> bool {
        let mut pattern_map = HashMap::with_capacity(pattern.len());
        let mut seen_words = HashSet::with_capacity(pattern.len());

        let pattern_byte = pattern.as_bytes();
        let s_words = s.split_whitespace().collect::<Vec<_>>();

        if pattern_byte.len() != s_words.len() {
            return false;
        }

        for i in 0..pattern_byte.len() {
            if let Some(mapped_word) = pattern_map.get(&pattern_byte[i]) {
                if *mapped_word != s_words[i] {
                    return false;
                }
            } else {
                if seen_words.contains(s_words[i]) {
                    return false;
                }
                pattern_map.insert(pattern_byte[i], s_words[i]);
                seen_words.insert(s_words[i]);
            }
        }
        true
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

    /*
     * faster
     */
    let pattern = "abba".to_string();
    let s = "dog cat cat dog".to_string();
    println!("result = {}", Solution::word_pattern4(pattern, s));

    let pattern = "abba".to_string();
    let s = "dog cat cat fish".to_string();
    println!("result = {}", Solution::word_pattern4(pattern, s));

    let pattern = "aaaa".to_string();
    let s = "dog cat cat dog".to_string();
    println!("result = {}", Solution::word_pattern4(pattern, s));
}
