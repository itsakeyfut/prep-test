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
                // iter()は、ベクタ内の各要素に対して不変参照を取得できるイテレータを返す。
                // 逆に、into_iter()は所有権を消費するイテレータです。
                // skip(1)は、イテレータの最初の1要素をスキップし、それ以降の要素を操作対象とする。
                // fold()は畳み込み操作で、strs[0].clone()を初期値とし、イテレータ内の各要素を順にaccに適用して結果を計算する。
                // ※畳み込み操作とは、コレクションに内の要素を1つずつ処理しながら、結果を累積して1つの値にまとめる操作です。
                // e.g. fold(初期値, |累積値, 現在の要素|)
                // |acc, x| { ... } はクロージャで、accは現在の最長共通接頭辞、xは現在の処理中の文字列を指す。
                strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
                    acc
                        // chars()は、文字列(String, &str)を文字単位に分割するイテレータを返す。
                        .chars()
                        // zipは、2つのイテレータをペア化して、タプル(a, b)を生成する新しいイテレータを作成する。
                        // この場合、accの文字イテレータと、現在の x の文字イテレータをペアにする。
                        .zip(x.chars())
                        // ペア (x, y)の条件 x == y が成立する間だけイテレータを進める。
                        // つまり、累積値(acc)と現在の文字列(x)の共通部分を抽出する。
                        .take_while(|(x, y)| x == y)
                        // take_whileで得られたタプル(x, _)の最初の値 x を取り出す。
                        // x は共通文字部分を指す。
                        .map(|(x, _)| x)
                        // イテレータをString型に収集する。
                        // この場合、共通接頭辞の文字列が生成される。
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
