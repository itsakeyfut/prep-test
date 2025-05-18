#[allow(unused)]
struct Solution {}

#[allow(unused)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            let mut complement = target - num;

            if let Some(&complement_idx) = map.get(&complement) {
                return vec![complement_idx as i32, idx as i32];
            }

            map.insert(num, idx);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2,7,11,15];
        let target = 9;
        let expected = vec![0,1];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn example2() {
        let nums = vec![3,2,4];
        let target = 6;
        let expected = vec![1,2];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn example3() {
        let nums = vec![3,3];
        let target = 6;
        let expected = vec![0,1];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}