struct Solution;
impl Solution {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res = Vec::new();
        let mut i = 0;

        while i < nums.len() {
            let start = nums[i];
            let mut end = start;

            while i + 1 < nums.len() && nums[i + 1] == nums[i] + 1 {
                i += 1;
                end = nums[i];
            }

            if start == end {
                res.push(start.to_string());
            } else {
                res.push(format!("{}->{}", start, end));
            }

            i += 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![0,1,2,4,5,7];
        let expected = vec!["0->2","4->5","7"];
        assert_eq!(Solution::summary_ranges(nums), expected);
    }

    #[test]
    fn example2() {
        let nums = vec![0,2,3,4,6,8,9];
        let expected = vec!["0","2->4","6","8->9"];
        assert_eq!(Solution::summary_ranges(nums), expected);
    }
}