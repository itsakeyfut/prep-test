struct Solution {}

impl Solution {
    pub fn climb_stairs1(n: i32) -> i32 {
        let (mut a, mut b, mut ways) = (0, 1, 0);

        for _ in 1..=n {
            ways = a + b;
            a = b;
            b = ways;
        }
        ways
    }
    pub fn climb_stairs2(n: i32) -> i32 {
        let mut dp = vec![-1; (n + 1) as usize];
        Self::solve(n, &mut dp)
    }

    fn solve(n: i32, dp: &mut Vec<i32>) -> i32 {
        if n < 0 {
            return 0;
        }
        if n == 0 {
            return 1;
        }

        if dp[n as usize] != -1 {
            return dp[n as usize];
        }

        dp[n as usize] = Self::solve(n - 1, dp) + Self::solve(n - 2, dp);
        dp[n as usize]
    }

    pub fn climb_stairs3(n: i32) -> i32 {
        (0..n)
            .fold((1, 0), |(res, prev), _| (res + prev, res))
            .0
    }

    pub fn climb_stairs4(mut n: i32) -> i32 {
        let mut mat = [1, 1, 0];
        let mut res = [1, 0, 1];
        while n > 0 {
            if n & 1 == 1 {
                res = Self::cross_product(res, mat);
            }
            mat = Self::cross_product(mat, mat);
            n >>= 1;
        }
        res[0]
    }

    fn cross_product(a: [i32; 3], b: [i32; 3]) -> [i32; 3] {
        [a[0] * b[0] + a[1] * b[1],
         a[0] * b[1] + a[1] * b[2],
         a[1] * b[1] + a[2] * b[2]]
    }
}

fn main() {
    let n = 2;
    println!("result = {}", Solution::climb_stairs1(n));

    let n = 2;
    println!("result = {}", Solution::climb_stairs2(n));

    let n = 2;
    println!("result = {}", Solution::climb_stairs3(n));

    let n = 2;
    println!("result = {}", Solution::climb_stairs4(n));
}
