struct Solution {}

impl Solution {
    /// 後ろから要素を比較して配置
    // Time Complexity: O(n)
    // Space Complexity: O(1)
    pub fn merge1(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // インデックスを指定するためにusizeに変換
        let (mut m, mut n) = (m as usize, n as usize);

        // nums1とnums2の要素を後ろから比較して配置
        while n > 0 { // num2の要素を全て処理するまで繰り返す
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                // nums1の要素がnums2の要素より大きい場合はnums1の末尾に配置
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            } else {
                // nums2の要素をnums1の末尾に配置
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
        }
    }

    /// num2の要素を追加後、ソート
    // Time Complexity: O(n log n)
    // Space Complexity: O(1)
    pub fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // nums2の要素をnums1の末尾に追加
        for (j, i) in (m..m+n).enumerate() {
            nums1[i as usize] = nums2[j];
        }
        nums1.sort();
    }

    /// Two Pointers
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // 結合後のインデックスを管理するポインタ
        let mut merge_pos = (m + n - 1) as usize;
        // nums1 と nums2 のそれぞれの末尾を指すポインタ
        let mut nums1_pos = (m - 1) as i32;
        let mut nums2_pos = (n - 1) as i32;

        // nums2 の要素がすべて処理されるまでループ
        while nums2_pos >= 0 {
            if nums1_pos >= 0 && nums1[nums1_pos as usize] > nums2[nums2_pos as usize] {
                // nums1 の現在位置の要素を末尾に移動
                nums1[merge_pos] = nums1[nums1_pos as usize];
                nums1_pos -= 1;
            } else {
                // nums2 の現在位置の要素を末尾に移動
                nums1[merge_pos] = nums2[nums2_pos as usize];
                nums2_pos -= 1;
            }
            merge_pos -= 1;
        }
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0,];
    let mut nums2 = vec![2,5, 6];
    let m = 3;
    let n = 3;

    Solution::merge1(&mut nums1, m, &mut nums2, n);
    println!("{:?}", nums1);

    let mut nums1 = vec![1, 2, 3, 0, 0, 0,];
    let mut nums2 = vec![2,5, 6];
    let m = 3;
    let n = 3;

    Solution::merge2(&mut nums1, m, &mut nums2, n);
    println!("{:?}", nums1);
}
