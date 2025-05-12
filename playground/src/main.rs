use rand::Rng;

fn merge_sort(nums: &mut [i32], buffer: &mut [i32]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let (left, right) = nums.split_at_mut(mid);
    let (left_buf, right_buf) = buffer.split_at_mut(mid);

    merge_sort(left, left_buf);
    merge_sort(right, right_buf);

    merge(left, right, buffer);
    nums.copy_from_slice(buffer);
}

fn merge(left: &[i32], right: &[i32], out: &mut [i32]) {
    let mut l = 0;
    let mut r = 0;
    let mut o = 0;

    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            out[o] = left[l];
            l += 1;
        } else {
            out[o] = right[r];
            r += 1;
        }
        o += 1;
    }

    if l < left.len() {
        out[o..].copy_from_slice(&left[l..]);
    }
    if r < right.len() {
        out[o..].copy_from_slice(&right[r..]);
    }
}

fn main() {
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (0..10)
        .into_iter()
        .map(|_| rng.random_range(0..1000))
        .collect();
    let mut buffer = nums.clone();

    println!("BEFORE: {:?}", nums);
    merge_sort(&mut nums, &mut buffer);
    println!("AFTER: {:?}", nums);
}