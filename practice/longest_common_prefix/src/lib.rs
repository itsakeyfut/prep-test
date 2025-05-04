//! Example 1:
//!   Input: strs = ["flower", "flow", "flight"]
//!   Output: "fl"
//! 
//! Example 2:
//!   Input: strs = ["dog", "racecar", "car"]
//!   Output: ""
//!   Explanation: There is no common prefix among the input strings.
//! 
//! Constraints:
//!   1 <= strs.length <= 200
//!   0 <= strs[i].length <= 200
//!   strs[i] consists of only lowercase English letters if it is non-empty.

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        match strs.is_empty() {
            true => "".to_string(),
            _ => {
                strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
                    // 最初の文字列を初期の接頭辞(acc)として開始
                    // その後、ベクター内の各後続文字列に対して

                    acc
                        .chars()                     // 蓄積された接頭辞を文字イテレータに変換
                        .zip(x.chars())              // 各文字を現在の文字列の対応する文字とペアにする
                        .take_while(|(x, y)| x == y) // 一致する文字ペアのみを保持
                        .map(|(x, _)| x)             // ペアから最初の文字のみを抽出
                        .collect()                   // 残りの文字を文字列に集める
                })
            }
        }
    }

    /// Binary Search
    pub fn longest_common_prefix2(strs: Vec<String>) -> String {
        // 空のベクターの場合は空文字列を返す
        if strs.is_empty() {
            return "".to_string();
        }

        // 最初の文字列の長さを見つける
        let min_len = strs.iter()
            .map(|s| s.len())
            .min()
            .unwrap_or(0);

        // ベクター内のすべての文字列が長さ0の場合は空文字列を返す
        if min_len == 0 {
            return "".to_string();
        }

        // バイナリーサーチで共通接頭辞の長さを見つける
        let mut low = 0;
        let mut high = min_len;

        while low < high {
            let mid = low + (high - low + 1) / 2;

            if Self::is_common_prefix(&strs, mid) {
                // 長さ mid の接頭辞が共通なら、もっと長い接頭辞を探す
                low = mid;
            } else {
                // 長さ mid の接頭辞が共通でないなら、より短い接頭辞を探す
                high = mid - 1;
            }
        }

        strs[0][0..low].to_string()
    }

    /// 指定された長さの接頭辞がすべての文字列で共通化どうかをチェックする
    fn is_common_prefix(strs: &[String], len: usize) -> bool {
        let prefix = &strs[0][0..len];

        for s in strs.iter().skip(1) {
            if !s.starts_with(prefix) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        let expected = "fl";
        assert_eq!(Solution::longest_common_prefix(strs), expected);
    }

    #[test]
    fn example2() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let expected = "";
        assert_eq!(Solution::longest_common_prefix(strs), expected);
    }

    #[test]
    fn example3() {
        let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        let expected = "fl";
        assert_eq!(Solution::longest_common_prefix2(strs), expected);
    }

    #[test]
    fn example4() {
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let expected = "";
        assert_eq!(Solution::longest_common_prefix2(strs), expected);
    }
}