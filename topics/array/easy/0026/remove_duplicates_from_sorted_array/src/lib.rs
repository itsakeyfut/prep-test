#[allow(unused)]
struct Solution {}

#[allow(unused)]
impl Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        match nums.is_empty() {
            true => return 0,
            false => {
                let mut write_idx = 0;

                for read_idx in 0..nums.len() {
                    if nums[read_idx] != nums[write_idx] {
                        write_idx += 1;
                        nums[write_idx] = nums[read_idx];
                    }
                }
                (write_idx + 1) as i32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut nums = vec![1,1,2];
        let expected = 2;
        assert_eq!(Solution::remove_duplicates(&mut nums), expected);
    }

    #[test]
    fn example2() {
        let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
        let expected = 5;
        assert_eq!(Solution::remove_duplicates(&mut nums), expected);
    }
}