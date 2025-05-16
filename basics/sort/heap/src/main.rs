use rand::Rng;

fn heap_sort(nums: &mut [i32]) {
    let len = nums.len();

    // 配列をヒープに変換（最大ヒープを構築）
    for start in (0..len / 2).rev() {
        heapify(nums, len, start);
    }

    // ヒープから最大値を取り出し、後ろに移動
    for end in (1..len).rev() {
        nums.swap(0, end); // 最大値を最後に移動
        heapify(nums, end, 0); // ヒープを再構築（最大値を除いた範囲で）
    }
}

fn heapify(nums: &mut [i32], heap_size: usize, root_idx: usize) {
    let mut largest = root_idx;
    let left = 2 * root_idx + 1;
    let right = 2 * root_idx + 2;

    // 左の子と比較
    if left < heap_size && nums[left] > nums[largest] {
        largest = left;
    }

    // 右の子と比較
    if right < heap_size && nums[right] > nums[largest] {
        largest = right;
    }

    // 最大の子と交換が必要な場合
    if largest != root_idx {
        nums.swap(root_idx, largest);
        heapify(nums, heap_size, largest); // 再帰的にヒープを保つ
    }
}

fn main() {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (0..10)
        .into_iter()
        .map(|_| rng.random_range(0..1000))
        .collect();

    println!("BEFORE: {:?}", nums);
    heap_sort(&mut nums);
    println!("AFTER:  {:?}", nums);
}
