use rand::Rng;

fn insertion_sort(nums: &mut Vec<i32>) {
    let len = nums.len();

    // 1番目の要素を保持したいため、1から走査
    for i in 1..len {
        // 1番目の要素を保持
        let tmp = nums[i];
        // 挿入位置を決めるために j を i　に設定
        let mut j = i;

        // 挿入位置を探す
        while j > 0 && nums[j - 1] > tmp {
            // 左の要素が保持した値より大きければ、右にシフト
            nums[j] = nums[j - 1];
            // 右に移動させた位置を1つ左にずらして、次の要素を比較
            j -= 1;
        }

        // 保持した値を左に小さい値として挿入
        nums[j] = tmp;
    }
}

fn main() {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (0..10)
        .map(|_| rng.random_range(0..1000))
        .collect();

    println!("BEFORE: {:?}", nums);
    insertion_sort(&mut nums);
    println!("AFTER:  {:?}", nums);
}
