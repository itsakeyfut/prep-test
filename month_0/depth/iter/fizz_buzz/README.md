# FizzBuzz

```rs
struct FizzBuzz {
    curr: u32,
    max: u32,
}

impl Iterator for FizzBuzz {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr > self.max {
            return None;
        }
        let result = if self.curr % 15 == 0 {
            "FizzBuzz".to_string()
        } else if self.curr % 3 == 0 {
            "Fizz".to_string()
        } else if self.curr % 5 == 0 {
            "Buzz".to_string()
        } else {
            self.curr.to_string()
        };

        self.curr += 1;
        Some(result)
    }
}

fn main() {
    let fizzbuzz = FizzBuzz { curr: 1, max: 20 };
    for word in fizzbuzz {
        println!("{}", word);
    }
}
```

## ✨ コードポイント解説

- curr が現在地
- max を超えたら None
- Option<String> 型で返してる
- next() は呼び出されるたびに状態を進める
- ちゃんと所有権（String）を渡してるから後腐れなし！
