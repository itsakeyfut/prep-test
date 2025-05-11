use rand::Rng;

fn shell_sort(nums: &mut Vec<i32>) {
    let len = nums.len();
    let mut gap = len / 2;

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
        .into_iter()
        .map(|_| rng.random_range(0..1000))
        .collect();

    println!("BEFORE: {:?}", nums);
    shell_sort(&mut nums);
    println!("AFTER: {:?}", nums);
}