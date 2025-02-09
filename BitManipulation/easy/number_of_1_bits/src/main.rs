struct Solution;
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        (0..32)
            .fold(0, |bit_count, bit_position| {
                // (1 << bit_position) は、1 を bit_position 回だけ左にシフトして、ビットマスクを作成
                // n & (1 << bit_position) で、n の bit_position 番目のビットを抽出
                // そのビットを右シフトして 0 または 1 を得る
                bit_count + ((n & (1 << bit_position)) >> bit_position)
            }) as i32
    }
}

fn main() {
    let n = 11;
    println!("{}", Solution::hamming_weight(n));

    let n = 128;
    println!("{}", Solution::hamming_weight(n));

    let n = 2147483645;
    println!("{}", Solution::hamming_weight(n));
}