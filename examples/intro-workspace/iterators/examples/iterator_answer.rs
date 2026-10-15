struct StepDown {
    remaining: i32,
}

impl Iterator for StepDown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let current = self.remaining;
            self.remaining -= 2;
            Some(current)
        } else {
            None
        }
    }
}

fn main() {
    let mut steps = StepDown { remaining: 5 };
    assert_eq!(steps.next(), Some(5));
    let mut total = 0;
    for value in steps {
        println!("先取一筆後，for 繼續取得={value}");
        total += value;
    }
    assert_eq!(total, 4);
    let mut empty = StepDown { remaining: 0 };
    assert_eq!(empty.next(), None);
}
