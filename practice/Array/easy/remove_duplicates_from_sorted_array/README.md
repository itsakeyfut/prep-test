> Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements
> should be kept the same. Then return the number of unique elements in nums.
>
> Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

- Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
- Return k.

Custom Judge:

The judge will test your solution with the following code:

> int[] nums = [...]; // Input array
> int[] expectedNums = [...]; // The expected answer with correct length
>
> int k = removeDuplicates(nums); // Calls your implementation
>
> assert k == expectedNums.length;
> for (int i = 0; i < k; i++) {
> assert nums[i] == expectedNums[i];
> }

If all assertions pass, then your solution will be accepted.

## 和訳

整数の配列 nums が昇順にソートされているとき、重複を取り除き、各ユニークな要素が 1 回だけ出現するように、配列をその場で変更してください。要素の相対的な順序はそのままでなければなりません。その後、配列 nums に含まれるユニークな要素の個数を返してください。

nums のユニークな要素数を k とした場合、解決策には以下の 2 つの条件が求められます：

1. 配列 nums を変更して、最初の k 要素にユニークな要素を順番通りに配置してください。nums の残りの部分や、配列のサイズは重要ではありません。
2. k を返してください。

カスタムジャッジ：

ジャッジは以下のコードを使ってあなたの解法をテストします：

```rs
int[] nums = [...]; // 入力配列
int[] expectedNums = [...]; // 正しい長さの期待される結果
int k = removeDuplicates(nums); // 実装した関数を呼び出す
assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}
```

すべてのアサーションが通れば、解法は受け入れられます。

## 解説

このコードは「ソート済み配列から重複を削除して、先頭にユニークな要素だけ並べ替え、ユニーク要素の数（k）を返す」という問題を解いています。

### 問題の前提

- 配列は昇順にソートされている（= 重複が隣り合っている）
- その場で（in-place）書き換える必要がある（新しい配列は使わない）
- nums の先頭に k 要素にユニークな値を詰めたい

### 🧠 コアとなるロジック部分

```rs
let mut write_idx = 0;
for read_idx in 1..nums.len() {
    if nums[write_idx] != nums[read_idx] {
        write_idx += 1;
        nums[write_idx] = nums[read_idx];
    }
}
(write_idx + 1) as i32
```

### 🧱 変数の意味

- `write_idx`:
  - ユニークな要素を書き込む位置を指すインデックス
  - 最初の要素 nums[0] は必ずユニークなので、write_idx = 0 からスタート
- `read_idx`:
  - 配列を順に読み進めるインデックス（1 からスタート）

### 🔁 処理の流れ（例：[1, 1, 2]）

初期状態：

```rs
nums = [1, 1, 2]
write_idx = 0
read_idx = 1..2
```

1 回目: read_idx = 1

```rs
nums[write_idx] = 1
nums[read_idx] = 1
=> 同じなので何もしない
```

2 回目: read_idx = 2

```rs
nums[write_idx] = 1
nums[read_idx] = 2
=> 違うので、write_idxを進めて、nums[1] = 2 と上書き
=> nums = [1, 2, 2]
write_idx = 1
```

最終的に return `write_idx + 1 = 2`

### ✅ 結果

- 配列の先頭が `[1, 2, _]` のようになる（`_` の部分はどうでもいい）
- 戻り値 `k = 2`

### Time Complexity vs. Space Complexity

- Time Complexity
  - O(n)
- Space Complexity
  - O(1)
