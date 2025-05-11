## merge_sort

```rs
fn merge_sort(nums: &mut [i32], buffer: &mut [i32])
```

- `nums`
  - ソート対象の可変スライス
- `buffer`
  - ソートのための作業バッファ

```rs
let (left, right) = nums.split_at_mut(mid);
```

- 配列を左右に分割して、再帰敵に `merge_sort`

```rs
merge_sort(left, left_buf);
merge_sort(right, right_buf);
```

- 分割した左右をそれぞれ再帰的にソート。

```rs
merge(left, right, buffer);
```

- 左右のソート済み部分を `merge` 関数で統合 → `buffer` に格納。

```rs
nums.copy_from_slice(buffer);
```

- 統合された結果を元の `nums` にコピー。

## merge

```rs
fn merge(left: &[i32], right: &[i32], out: &mut [i32])
```

- ソート済みの `left` と `right` を 1 つの `out` にマージ。

```rs
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
```

- 2 つの配列を比較しながら小さい順に `out` に入れる。

その後、残っている要素を全部追加：

```rs
if l < left.len() {
    out[o..].copy_from_slice(&left[l..]);
}
if r < right.len() {
    out[o..].copy_from_slice(&right[r..]);
}
```
