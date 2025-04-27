# 🦀 Data Structure and Algorithm x Rust

This repository tracks my journey to master algorithms and data structures using Rust,  
with the goal of solving LeetCode Hard problems consistently within 6 months.

---

## 🚀 Goal

- Master Rust fundamentals and standard libraries
- Solve 50–100 LeetCode Easy/Medium problems
- Solve LeetCode Hard problems using Rust without major struggles
- Build pattern recognition across typical algorithm types

---

## 🗺️ Roadmap

### 📚 Month 0 (Week 1)

**Review Rust basics and standard libraries**

- Vec, HashMap, BinaryHeap (Priority Queue), BTreeMap
- Ownership, References (&, &mut, clone)
- Rc, RefCell, Box (for tree structures)

👉 Goal: **Be able to write array, map, queue, and stack operations smoothly in Rust**

---

### 📚 Month 1–2

**Install basic algorithm patterns**

| Topic                    | Examples                              | Rust-specific tips                  |
| ------------------------ | ------------------------------------- | ----------------------------------- |
| Array, String processing | Two Pointers, Sliding Window          | Slice handling, iterator usage      |
| Tree, Graph traversal    | DFS, BFS                              | Vec<Vec<>> for graph representation |
| Recursion, Backtracking  | Subsets, Permutations                 | Manage clones for borrow checker    |
| Basic DP                 | 1D/2D DP, Memory optimization         | Vec initialization, state tracking  |
| Union-Find               | Connected components, cycle detection | Struct practice                     |

👉 Goal:

- Solve 50–100 Easy/Medium LeetCode problems using Rust
- Start recognizing patterns intuitively

---

### 📚 Month 3–4

**Dive into intermediate patterns and advanced techniques**

| Topic                     | Examples                                   | Rust-specific tips                     |
| ------------------------- | ------------------------------------------ | -------------------------------------- |
| Advanced DP               | State DP, Interval DP, Memory optimization | Memory reuse, efficient Vec management |
| Segment Tree / BIT        | Range queries, dynamic array updates       | Struct implementation practice         |
| Advanced graph algorithms | Topological Sort, Dijkstra, MST            | Custom BinaryHeap usage                |
| Scheduling problems       | Greedy + Heap combination                  | Priority queue handling                |
| String algorithms         | KMP, Rolling Hash, Trie                    | Rc<RefCell> tree structures            |

👉 Goal:

- Attempt LeetCode Medium–Hard in contest-like conditions
- Focus on recognizing underlying patterns even if you cannot solve fully
- Create review notes for failed problems

---

### 📚 Month 5–6

**Full focus on Hard problems**

- Target 30–50 LeetCode Hard problems
- Solve with explanations at first, but **re-implement from scratch**
- During second pass, solve without hints

👉 Goal:

- Solve more Hard problems independently
- Quickly recognize patterns even if you can't solve right away
- Handle Rust-specific errors (ownership, borrow checker) confidently

---

## 📈 Weekly Progress

| Week | Focus                               |   Status    |
| :--: | :---------------------------------- | :---------: |
|  1   | Rust basics + simple array problems | In Progress |
|  2   | HashMap, Two Pointers problems      |             |
|  3   | DFS, BFS basics                     |             |

---

## ✅ Solved Problems (LeetCode)

- [x] 2025-04-28: 167. Two Sum II (Medium)
- [x] 2025-04-28: 15. 3Sum (Medium)
- [ ] 2025-04-29: 200. Number of Islands (Medium)

(keep updating...)

---

## 🛠️ How to Use

1. Clone this repository:

```bash
git clone https://github.com/your-username/rust-algo-practice.git
```

2. Navigate into the project folder.
3. Each problem solution is categorized by topic under `/src/`.
4. Each file includes:

- Problem description
- Thought process
- Final Rust code
