# BinaryTree を InOrder (中順)でイテレータ化する

## やりたいこと

例えば、こんな二分木があったとする：

```
    2
   / \
  1   3
```

これを中順（InOrder）（左・自分・右）でたどると...

```
1 → 2 → 3
```

って順番になる。
これを Rust イテレータとして作る！！

## ✅ 設計イメージ

まず、基本のツリー構造から。

```rs
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>
    right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode { val, left: None, right: None }))
    }
}
```

これでツリーを作れる。

## 次に、イテレータの骨組み！

```rs
struct InOrderIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
    current: Option<Rc<RefCell<TreeNode>>>,
}

impl InOrderIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        InOrderIterator {
            stack: vec![],
            current: root,
        }
    }
}
```

ポイント：

- `stack` で左ノードを順番に積んでいく
- `current` は、今見ているノード

## そして、肝心の `Iterator` 実装！

```rs
impl Iterator for InOrderIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.current.take() {
            self.stack.push(node.clone());
            self.current = node.borrow().left.clone();
        }

        if let Some(node) = self.stack.pop() {
            let val = node.borrow().val;
            self.current = node.borrow().right.clone();
            Some(val)
        } else {
            None
        }
    }
}
```

## 🎯 まとめると

- current がある間は、左に降り続ける（左探索）
- 振り切ったら、stack.pop() で取り出して「自分」を読む
- その後、右の子に移動して、また左に降り続ける
- これで InOrder（左 → 自分 → 右）が完成する！
