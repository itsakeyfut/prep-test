struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle_len = needle.len();
        let haystack_len = haystack.len();

        if needle_len > haystack_len {
            return -1;
        }

        // 0..(haystack_len - needle_len + 1)はhaystack内の部分文字列をneedleと同じ長さで切り取る開始位置を表す。
        // これは、haystackにneedleが存在するかを判定するための必要最小限のループ数を指す。
        for i in 0..(haystack_len - needle_len + 1) {
            // i..i + needle_len は 開始位置..終了位置を指す。
            // つまり、ループで i がインクリメントする毎に、 i + needle_len とすることで検索する文字列も一つ隣にずれて要素の検索を可能にしている。
            if haystack[i..i + needle_len] == needle {
                return i as i32;
            }
        }

        return -1;
    }

    pub fn str_str2(haystack: String, needle: String) -> i32 {
        // find()は、haystack内でneedleが最初に現れる位置を探すメソッドです。
        // 部分文字列が見つかった場合、その開始位置のインデックスを返す。
        // map_orは、Option型を扱う。Noneの場合 -1 を返す。Some(idx)の場合、クロージャ(|idx| idx as i32)を実行して結果を返す。
        haystack.find(&needle).map_or(-1, |idx| idx as i32)
    }
}

fn main() {
    /*
     * procedural
     */
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();
    let result = Solution::str_str(haystack, needle);
    println!("result = {}", result);

    let haystack = "leetcode".to_string();
    let needle = "leeto".to_string();
    let result = Solution::str_str(haystack, needle);
    println!("result = {}", result);

    /*
     * map_or
     */
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();
    let result = Solution::str_str2(haystack, needle);
    println!("result = {}", result);

    let haystack = "leetcode".to_string();
    let needle = "leeto".to_string();
    let result = Solution::str_str2(haystack, needle);
    println!("result = {}", result);
}
