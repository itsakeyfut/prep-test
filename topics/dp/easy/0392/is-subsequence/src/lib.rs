struct Solution;
impl Solution {
    fn is_subsequence(s: String, t: String) -> bool {
        let mut s_iter = s.chars();
        let mut curr = s_iter.next();

        for ch in t.chars() {
            if let Some(c) = curr {
                if c == ch {
                    curr = s_iter.next();
                }
            } else {
                break;
            }
        }

        curr.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();

        assert_eq!(Solution::is_subsequence(s, t), true);
    }

    #[test]
    fn example2() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();

        assert_eq!(Solution::is_subsequence(s, t), false);
    }
}