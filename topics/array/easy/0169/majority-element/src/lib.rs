struct Solution;
impl Solution {
    fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut elm = 0;

        for num in nums {
            if cnt == 0 {
                elm = num;
            }
            if elm == num {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }
        elm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![3,2,3];
        let expected = 3;
        assert_eq!(Solution::majority_element(nums), expected);
    }

    #[test]
    fn example2() {
        let nums = vec![2,2,1,1,1,2,2];
        let expected = 2;
        assert_eq!(Solution::majority_element(nums), expected);
    }
}