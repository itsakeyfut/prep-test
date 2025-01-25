# Is Subsequence

```rust
struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut i = 0;
        let mut j = 0;
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        // 両方の文字列を走査
        while i < s.len() && j < t.len() {
            // sの現在の文字とtの現在の文字が一致した場合、sの次の文字に進む
            if s_chars[i] == t_chars[j] {
                i += 1;
            }

            // 常にtの次の文字に進む
            j += 1;
        }

        // sのすべての文字が一致していれば、`i`はsの長さと等しいはず
        i == s.len()
    }

    pub fn is_subsequence2(s: String, t: String) -> bool {
        let mut s_iter = s.chars();
        let mut curr = s_iter.next(); // sの最初の文字を取得

        for c in t.chars() {
            if let Some(sc) = curr {
                if sc == c {
                    curr = s_iter.next(); // 次の文字に進む
                }
            } else {
                break;
            }
        }
        // currがNoneになっていれば、すべての文字が一致
        curr.is_none()
    }

    pub fn is_subsequence3(s: String, t: String) -> bool {
        let mut iter = t.chars();
        for c in s.chars() {
            match iter.find(|&p| p == c) {
                Some(_) => (),
                None => return false,
            }
        }
        true
    }

    pub fn is_subsequence4(s: String, t: String) -> bool {
        let mut map: std::collections::HashMap<char, Vec<usize>> = std::collections::HashMap::new();
        for (i, c) in t.chars().enumerate() {
            match map.get_mut(&c) {
                Some(li) => { li.push(i) },
                None => { map.insert(c, vec![i]); }
            }
        }
        let mut prev = 0;
        for c in s.chars() {
            match map.get(&c) {
                None => return false,
                Some(li) => {
                    match li.binary_search(&prev) {
                        Ok(idx) => prev = li[idx] + 1,
                        Err(ins_idex) => {
                            if ins_idex == li.len() {
                                return false;
                            }
                            prev = li[ins_idex] + 1;
                        }
                    }
                }
            }
        }
        true
    }
}

fn main() {
    /*
     * two pointers
     */
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence(s, t);
    println!("result = {}", result);

    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence(s, t);
    println!("result = {}", result);

    /*
     * iterator 1
     */
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence2(s, t);
    println!("result = {}", result);

    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence2(s, t);
    println!("result = {}", result);

    /*
     * iterator 2
     */
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence3(s, t);
    println!("result = {}", result);

    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence3(s, t);
    println!("result = {}", result);

    /*
     * binary search
     */
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence4(s, t);
    println!("result = {}", result);

    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    let result = Solution::is_subsequence4(s, t);
    println!("result = {}", result);
}
```

```bash
result = true
result = false

result = true
result = false

result = true
result = false

result = true
result = false
```
