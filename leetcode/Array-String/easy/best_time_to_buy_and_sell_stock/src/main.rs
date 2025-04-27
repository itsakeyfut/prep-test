struct Solution {}

impl Solution {
    /// 貪欲法を使ったアプローチ
    /// 最小価格を追跡しながら最大利益を更新する
    pub fn max_profit1(prices: Vec<i32>) -> i32 {
        let mut buy = prices[0]; // 最小購入価格を初期化
        let mut profit = 0;      // 最大利益を初期化

        for i in 1..prices.len() {
            // より低い購入価格があれば更新
            if prices[i] < buy {
                buy = prices[i];
            // より高い利益が得られる場合は更新

            } else if prices[i] - buy > profit {
                profit = prices[i] - buy;
            }
        }
        // 最大利益を返す
        profit
    }

    /// 動的計画法を使ったアプローチ
    /// 各ステップで最小価格と最大利益を計算する
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX; // 最小購入価格を初期化
        let mut max_profit = 0;       // 最大利益を初期化


        for curr_price in prices {
            // 最小価格を更新
            min_price = min_price.min(curr_price);
            // 最大利益を更新
            max_profit = max_profit.max(curr_price - min_price);
        }

        // 最大利益を返す
        max_profit
    }

    /// 二つのポインタを使ったアプローチ
    /// 左ポインタは最小購入価格、右ポインタは売却価格を指す
    pub fn max_profit3(prices: Vec<i32>) -> i32 {
        // 要素が2つ未満の場合、利益は得られない
        if prices.len() < 2 {
            return 0;
        }

        let mut max_profit = 0;   // 最大利益を初期化
        let mut left_ptr = 0;   // 左ポインタ (購入価格)
        let mut right_ptr = 1;  // 右ポインタ (売却価格)

        while right_ptr < prices.len() {
            let sell_price = prices[right_ptr]; // 現在の売却価格
            let buy_price = prices[left_ptr];   // 現在の購入価格

            // 利益が得られる場合
            if buy_price < sell_price {
                // 現在の利益を計算
                let curr_profit = sell_price - buy_price;
                // 最大利益を更新
                max_profit = max_profit.max(curr_profit);
            // より安い購入価格を見つけた場合は左ポインタを移動
            } else {
                left_ptr = right_ptr;
            }

            // 右ポインタを移動
            right_ptr += 1;
        }

        // 最大利益を返す
        max_profit
    }
}

fn main() {
    let prices = vec![7,1,5,3,6,4];

    let profit1 = Solution::max_profit1(prices);
    println!("{}", profit1);

    let prices = vec![7,1,5,3,6,4];

    let profit2 = Solution::max_profit2(prices);
    println!("{}", profit2);

    let prices = vec![7,1,5,3,6,4];

    let profit3 = Solution::max_profit3(prices);
    println!("{}", profit3);
}
