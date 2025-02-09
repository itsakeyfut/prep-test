# Reverse Bits

```rust
struct Solution;
impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        // 結果を格納する変数 (反転後のビット列)
        let mut reversed = 0;

        // すべてのビットを反転する (32ビット整数)
        for _ in 0..32 {
            // reversed を左シフトし、num の最下位ビットを追加
            // x & 1 は最下位ビット (一番右のビット) を取り出す操作である
            reversed = (reversed << 1) | (x & 1);
            // num を右シフトして次のビットを取得
            x >>= 1;
        }
        reversed
    }
    pub fn reverse_bits_recur(x: u32) -> u32 {
        x.reverse_bits()
    }
}

fn main() {
    let n = 0b00000010100101000001111010011100;
    println!("{}", Solution::reverse_bits(n));

    let n = 0b11111111111111111111111111111101;
    println!("{}", Solution::reverse_bits(n));

    let n = 0b00000010100101000001111010011100;
    println!("{}", Solution::reverse_bits_recur(n));

    let n = 0b11111111111111111111111111111101;
    println!("{}", Solution::reverse_bits_recur(n));
}
```

```bash
964176192
3221225471
964176192
3221225471
```
