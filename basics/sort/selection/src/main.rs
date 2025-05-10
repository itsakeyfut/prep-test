use rand::Rng;

fn selection_sort(nums: &mut Vec<i32>) {
    let len = nums.len();

    for i in 0..len {
        // 仮に今の場所の値を最小とする
        let mut min_idx = i;

        for j in (i + 1)..len {
            // 後ろまで走査し、最小値を探す
            if nums[j] < nums[min_idx] {
                min_idx = j;
            }
        }

        // 最小値と現在位置を入れ替える
        if i != min_idx {
            nums.swap(i, min_idx);
        }
    }
}

fn main() {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (0..10)
        .map(|_| rng.random_range(0..1000))
        .collect();

    println!("BEFORE: {:?}", nums);
    selection_sort(&mut nums);
    println!("AFTER: {:?}", nums);
}
