struct Solution {}

impl Solution {
    pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let size = nums.len();

        for gap in 1..size {
            for j in gap..size {
                let i = j - gap;
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![]
    }
    
    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 各数値とその元のインデックスをタプル化 (値、インデックス) の形で格納
        let mut sorted_nums: Vec<(i32, usize)> = nums
            .iter()
            .copied()
            .enumerate()
            .map(|(i, num)| (num, i))
            .collect();

        // 数値を基準に昇順ソート (インデックスはそのまま保持)
        sorted_nums.sort_by_key(|&(value, _)| value);

        // 左右のポインタ
        let mut left = 0;
        let mut right = sorted_nums.len() - 1;

        while left < right {
            let sum = sorted_nums[left].0 + sorted_nums[right].0;

            if sum == target {
                // 目標の和が見つかった場合、元のインデックスを返す
                return vec![
                    sorted_nums[left].1 as i32,
                    sorted_nums[right].1 as i32,
                ];
            } else if sum < target {
                // 合計が小さすぎる場合、左側のポインタを右に移動
                left += 1;
            } else {
                // 合計が大きすぎる場合、右側のポインタを左に移動
                right -= 1;
            }
        }

        vec![]
    }

    pub fn two_sum3(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        // 全ての数値とそのインデックスをマップに格納
        for (i, &num) in nums.iter().enumerate() {
            map.insert(num, i);
        }

        // 再度リストを走査し、target - value がマップ内にあるかを確認
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&complement_index) = map.get(&complement) {
                if complement_index != i {
                    return vec![i as i32, complement_index as i32];
                }
            }
        }

        vec![]
    }

    pub fn two_sum4(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&complement_index) = map.get(&complement) {
                return vec![complement_index as i32, i as i32];
            }
            map.insert(num, i);
        }

        vec![]
    }
}

fn main() {
    /*
     * Brute Force
     */
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    let nums3 = vec![3, 3];
    let target3 = 6;
    println!("nums1 = {:?} | nums2 = {:?} | nums3 = {:?}",
        Solution::two_sum1(nums1, target1),
        Solution::two_sum1(nums2, target2),
        Solution::two_sum1(nums3, target3)
    );

    /*
     * Two Pointer
     */
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    let nums3 = vec![3, 3];
    let target3 = 6;
    println!("nums1 = {:?} | nums2 = {:?} | nums3 = {:?}",
        Solution::two_sum2(nums1, target1),
        Solution::two_sum2(nums2, target2),
        Solution::two_sum2(nums3, target3)
    );

    /*
     * Two Pass Hash Table
     */
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    let nums3 = vec![3, 3];
    let target3 = 6;
    println!("nums1 = {:?} | nums2 = {:?} | nums3 = {:?}",
        Solution::two_sum3(nums1, target1),
        Solution::two_sum3(nums2, target2),
        Solution::two_sum3(nums3, target3)
    );

    /*
     * One Pass Hash Table
     */
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    let nums3 = vec![3, 3];
    let target3 = 6;
    println!("nums1 = {:?} | nums2 = {:?} | nums3 = {:?}",
        Solution::two_sum4(nums1, target1),
        Solution::two_sum4(nums2, target2),
        Solution::two_sum4(nums3, target3)
    );
}
