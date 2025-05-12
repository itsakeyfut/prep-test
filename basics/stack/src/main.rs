fn validate_format(chars: &str) -> bool {
    let lookup: std::collections::HashMap<char, char> = [
        ('{', '}'),
        ('[', ']'),
        ('(', ')')
    ]
    .into_iter()
    .collect();

    let mut stack = Vec::new();

    for char in chars.chars() {
        if lookup.contains_key(&char) {
            // 今後出てくるべき閉じ括弧を push e.g. ( が来たら ) を push して、 ) が来るのを期待する、という設計
            stack.push(lookup[&char]);
        // 今見ている char が、閉じ括弧のどれか（ }, ], ) ）であるかどうかを判定
        } else if lookup.values().any(|&v| v == char) {
            // stack.pop() は、最後にスタックに追加された「期待される閉じ括弧」を取り出す。
            // それが今見ている char と一致するかどうかをチェックする。
            // 一致しなければ、それは「対応関係が崩れている」ということ。
            if stack.is_empty() || stack.pop() != Some(char) {
                return false;
            }
        }
    }

    stack.is_empty()
}

fn validate_fmt_btree(chars: &str) -> bool {
    let lookup: std::collections::BTreeMap<char, char> = [
        ('(', ')'),
        ('{', '}'),
        ('[', ']')
    ]
    .into_iter()
    .collect();

    let mut stack = Vec::new();

    for char in chars.chars() {
        if lookup.contains_key(&char) {
            stack.push(lookup[&char]);
        } else if lookup.values().any(|&v| v == char) {
            if stack.is_empty() || stack.pop() != Some(char) {
                return false
            }
        }
    }

    stack.is_empty()
}

fn main() {
    let json1 = "{'key1': 'value1', 'key2': [1, 2, 3], 'key3': (1, 2, 3)}";
    let json2 = "{'key1': ['value1', 'key2': [1, 2, 3], 'key3': (1, 2, 3)}";

    println!("{:?}", validate_fmt_btree(json1));
    println!("{:?}", validate_fmt_btree(json2));
}
