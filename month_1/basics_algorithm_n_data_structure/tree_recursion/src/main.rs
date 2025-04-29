fn subsets(s: &str, curr: String) {
    if s.is_empty() {
        println!("{}", curr);
    } else {
        let (first, rest) = s.split_at(1);
        subsets(rest, curr.clone()); // 含まないパターン
        subsets(rest, curr + first); // 含むパターン
    }
}

fn main() {
    subsets("abc", "".to_string());
}

/*
 * c
 * b
 * bc
 * a
 * ac
 * ab
 * abc
 */