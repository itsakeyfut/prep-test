# Longest Common Prefix

```rust
struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // 入力のVec<String>をイテレートし、リデュース（畳み込み）処理で最長共通接頭辞を計算
        strs.into_iter()
            .reduce(|acc, cur| {
                // 現在の累積値(acc)と現在の文字列(cur)を比較し、
                // 共通部分を収集する
                acc.chars() // 累積値を文字ごとに分割
                    .zip(cur.chars()) // 現在の文字列と対応する文字をペア化
                    .take_while(|(a, c)| a ==c) // 対応する文字が一致する限りペアを取得
                    .map(|(c, _)| c) // 最初の文字（共通部分の文字）を抽出
                    .collect() // 共通部分の文字を新しい文字列として収集
            }).unwrap()
    }

    pub fn longest_common_prefix2(strs: Vec<String>) -> String {
        match strs.is_empty() {
            true => "".to_string(),
            _ => {
                strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
                    acc
                        .chars()
                        .zip(x.chars())
                        .take_while(|(x, y)| x == y)
                        .map(|(x, _)| x)
                        .collect()
                })
            }
        }
    }

    pub fn longest_common_prefix3(strs: Vec<String>) -> String {
        // 配列の最初の文字列を出力の候補に設定
        let mut output = strs[0].clone();
        for s in strs.iter() {
            // 現在の文字列が出力候補で始まらない限り、出力候補を短くしていく
            while !s.starts_with(&output) {
                // 出力候補の末尾の文字を1文字削除
                output.pop();
            }
        }
        output.to_string()
    }
}

fn main() {
    /*
     * iterate
     */
    let strs = ["flower","flow","flight"];
    let strs = strs.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let result = Solution::longest_common_prefix(strs);
    println!("result = {}", result);

    let strs = ["dog","racecar","car"];
    let strs = strs.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let result = Solution::longest_common_prefix(strs);
    println!("result = {}", result);

    let strs = ["rapid","rabbit","rap", "wrapper", "iterate"];
    let strs = strs.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let result = Solution::longest_common_prefix(strs);
    println!("result = {}", result);

    /*
     * iterate with nullish care
     */
    let strs = ["flower","flow","flight"];
    let strs = strs.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let result = Solution::longest_common_prefix2(strs);
    println!("result = {}", result);

    let strs = ["dog","racecar","car"];
    let strs = strs.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let result = Solution::longest_common_prefix2(strs);
    println!("result = {}", result);

    let strs = ["rapid","rabbit","rap", "wrapper", "iterate"];
    let strs = strs.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let result = Solution::longest_common_prefix2(strs);
    println!("result = {}", result);

    /*
     * iterate with nullish care
     */
    let strs = ["flower","flow","flight"];
    let strs = strs.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let result = Solution::longest_common_prefix3(strs);
    println!("result = {}", result);

    let strs = ["dog","racecar","car"];
    let strs = strs.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let result = Solution::longest_common_prefix3(strs);
    println!("result = {}", result);

    let strs = ["rapid","rabbit","rap", "wrapper", "iterate"];
    let strs = strs.iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    let result = Solution::longest_common_prefix3(strs);
    println!("result = {}", result);
}
```

```bash
result = fl
result =
result =

result = fl
result =
result =

result = fl
result =
result =
```
