struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // HashMapを作成して、`magazine`の文字とその出現回数を記録する
        let mut dict = std::collections::HashMap::new();

        // `magazine`のすべての文字を走査し、HashMapに登録
        for c in magazine.chars() {
            // エントリーがすでに存在する場合はカウントを増加、存在しない倍は1を挿入
            dict.entry(c).and_modify(|cnt| *cnt += 1).or_insert(1);
        }

        // `rasom_note`の各文字についてチェック
        for c in ransom_note.chars() {
            match dict.get_mut(&c) {
                // `dict`に現在の文字が存在し、かつそのカウントが1以上の場合
                Some(n) if *n > 0 => { *n -= 1 } // 使用済みとしてカウントを1減らす
                // 文字が存在しない、またはカウントが0の場合
                _ => { return false; } // 必要な文字を構築できないためfalseを返す
            }
        }
        // すべての文字が条件を満たした場合、trueを返す
        true
    }

    pub fn can_construct2(ransom_note: String, magazine: String) -> bool {
        // ASCII文字をカウントするためにサイズ256の配列を用意
        let mut counts = [0; 256];

        // `magazine`の各文字の出現回数をカウント
        for c in magazine.chars() {
            counts[c as usize] += 1;
        }

        // `ransome_note`の各文字を確認
        for c in ransom_note.chars() {
            if counts[c as usize] == 0 {
                return false; // 必要な文字が足りない場合
            }
            counts[c as usize] -= 1;
        }
        true
    }
}

fn main() {
    /*
     * HashMap
     */
    let ransom_note = "a".to_string();
    let magazine = "b".to_string();
    println!("result = {}", Solution::can_construct(ransom_note, magazine));

    let ransom_note = "aa".to_string();
    let magazine = "bb".to_string();
    println!("result = {}", Solution::can_construct(ransom_note, magazine));

    let ransom_note = "aa".to_string();
    let magazine = "aab".to_string();
    println!("result = {}", Solution::can_construct(ransom_note, magazine));

    /*
     * array fixed size
     */
    let ransom_note = "a".to_string();
    let magazine = "b".to_string();
    println!("result = {}", Solution::can_construct2(ransom_note, magazine));

    let ransom_note = "aa".to_string();
    let magazine = "bb".to_string();
    println!("result = {}", Solution::can_construct2(ransom_note, magazine));

    let ransom_note = "aa".to_string();
    let magazine = "aab".to_string();
    println!("result = {}", Solution::can_construct2(ransom_note, magazine));
}
