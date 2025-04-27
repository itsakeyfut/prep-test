struct Solution {}

impl Solution {
    fn calc_scale(c: Option<char>, a1: char, a2: char) -> i32 {
        match c {
            Some(ch) if ch == a1 || ch == a2 => -1,
            _ => 1,
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut result = 0;

        for n in 0..chars.len() {
            let next_char = chars.get(n + 1).copied();

            match chars[n] {
                'M' => result += 1000,
                'D' => result += 500,
                'C' => result += 100 * Self::calc_scale(next_char, 'M', 'D'),
                'L' => result += 50,
                'X' => result += 10 * Self::calc_scale(next_char, 'C', 'L'),
                'V' => result += 5,
                'I' => result += 1 * Self::calc_scale(next_char, 'X', 'V'),
                _ => (),
            }
        }
        result
    }

    pub fn roman_to_int2(s: String) -> i32 {
        let mut roman = std::collections::HashMap::new();
        roman.insert('I', 1);
        roman.insert('V', 5);
        roman.insert('X', 10);
        roman.insert('L', 50);
        roman.insert('C', 100);
        roman.insert('D', 500);
        roman.insert('M', 1000);

        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut result = roman[&chars[n - 1]];

        for i in (0..n - 1).rev() {
            if roman[&chars[i]] < roman[&chars[i + 1]] {
                result -= roman[&chars[i]];
            } else {
                result += roman[&chars[i]];
            }
        }
        result
    }

    pub fn roman_to_int3(s: String) -> i32 {
        let s_translated = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");

        s_translated.chars().map(|c| {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }).sum()
    }
}

fn main() {
    /*
     * Simple
     */
    let s = "III";
    let res = Solution::roman_to_int(s.to_string());
    println!("res = {}", res);

    let s = "LVIII";
    let res = Solution::roman_to_int(s.to_string());
    println!("res = {}", res);

    let s = "MCMXCIV";
    let res = Solution::roman_to_int(s.to_string());
    println!("res = {}", res);

    /*
     * HashMap
     */
    let s = "III";
    let res2 = Solution::roman_to_int2(s.to_string());
    println!("res2 = {}", res2);

    let s = "LVIII";
    let res2 = Solution::roman_to_int2(s.to_string());
    println!("res2 = {}", res2);

    let s = "MCMXCIV";
    let res2 = Solution::roman_to_int2(s.to_string());
    println!("res2 = {}", res2);

    /*
     * replace
     */
    let s = "III";
    let res3 = Solution::roman_to_int3(s.to_string());
    println!("res3 = {}", res3);

    let s = "LVIII";
    let res3 = Solution::roman_to_int3(s.to_string());
    println!("res3 = {}", res3);

    let s = "MCMXCIV";
    let res3 = Solution::roman_to_int3(s.to_string());
    println!("res3 = {}", res3);
}
