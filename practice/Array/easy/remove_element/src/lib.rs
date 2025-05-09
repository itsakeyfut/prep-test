struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        match nums.is_empty() {
            true => return 0,
            false => {
                let mut write_idx = 0;

                for read_idx in 0..nums.len() {
                    if nums[read_idx] != val {
                        nums[write_idx] = nums[read_idx];
                        write_idx += 1;
                    }
                }
                write_idx as i32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut v = vec![3, 2, 2, 3];
        let expected = 2;
        assert_eq!(Solution::remove_element(&mut v, 3), expected);
    }

    #[test]
    fn example2() {
        let mut v = vec![0,1,2,2,3,0,4,2];
        let expected = 5;
        assert_eq!(Solution::remove_element(&mut v, 2), expected);
    }
}
