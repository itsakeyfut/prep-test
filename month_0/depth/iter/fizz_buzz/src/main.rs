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

/*
 * 1
 * 2
 * Fizz
 * 4
 * 5
 * Fizz
 * 7
 * 8
 * Fizz
 * 10
 * 11
 * Fizz
 * 13
 * 14
 * FizzBuzz
 * 16
 * 17
 * Fizz
 * 19
 * 20
 */
