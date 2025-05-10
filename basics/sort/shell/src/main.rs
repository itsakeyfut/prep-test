use rand::Rng;

fn shell_sort(nums: &mut Vec<i32>) {
    let len = nums.len();
    // 最初は大きなギャップで並べ替え、徐々にギャップを小さくしていきます。
    let mut gap = len / 2;

    // ギャップが0になると、最終的な挿入ソートが完了します。
    while gap > 0 {
        for i in gap..len {
            let tmp = nums[i];
            let mut j = i;

            while j >= gap && nums[j - gap] > tmp {
                nums[j] = nums[j - gap];
                j -= gap;
            }
            nums[j] = tmp;
        }
        gap /= 2;
    }
}

fn main() {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (0..10)
        .map(|_| rng.random_range(0..1000))
        .collect();

    println!("Before sorting: {:?}", nums);
    shell_sort(&mut nums);
    println!("After sorting:  {:?}", nums);
}
