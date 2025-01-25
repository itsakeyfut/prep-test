struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut len = 0;
        for b in s.trim_end().bytes().rev() {
            if b == b' ' {
                return len;
            }
            len += 1;
        }
        len
    }

    pub fn length_of_last_word2(s: String) -> i32 {
        let mut len = 0;
        for &c in s.as_bytes().iter().rev() {
            if c != b' ' {
                len += 1;
            } else if len != 0 {
                break;
            }
        }
        len
    }
}

fn main() {
    /*
     * trim
     */
    let s = "Hello World".to_string();
    let result = Solution::length_of_last_word(s);
    println!("result = {}", result);

    let s = "   fly me   to   the moon  ".to_string();
    let result = Solution::length_of_last_word(s);
    println!("result = {}", result);

    let s = "luffy is still joyboy".to_string();
    let result = Solution::length_of_last_word(s);
    println!("result = {}", result);

    /*
     * iter
     */
    let s = "Hello World".to_string();
    let result = Solution::length_of_last_word2(s);
    println!("result = {}", result);

    let s = "   fly me   to   the moon  ".to_string();
    let result = Solution::length_of_last_word2(s);
    println!("result = {}", result);

    let s = "luffy is still joyboy".to_string();
    let result = Solution::length_of_last_word2(s);
    println!("result = {}", result);
}
