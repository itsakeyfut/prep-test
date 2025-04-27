# Find the Index of the First Occurrence in a String

```rust
struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle_len = needle.len();
        let haystack_len = haystack.len();

        if needle_len > haystack_len {
            return -1;
        }

        for i in 0..(haystack_len - needle_len + 1) {
            if haystack[i..i + needle_len] == needle {
                return i as i32;
            }
        }

        return -1;
    }

    pub fn str_str2(haystack: String, needle: String) -> i32 {
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
```

```bash
result = 0
result = -1

result = 0
result = -1
```
