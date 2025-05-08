fn validate_format(chars: &str) -> bool {
    let lookup: std::collections::HashMap<char, char> = [
        ('{', '}'),
        ('[', ']'),
        ('(', ')')
    ]
    .iter()
    .cloned()
    .collect();

    let mut stack = Vec::new();

    for char in chars.chars() {
        if lookup.contains_key(&char) {
            stack.push(lookup[&char]);
        } else if lookup.values().any(|&v| v == char) {
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
