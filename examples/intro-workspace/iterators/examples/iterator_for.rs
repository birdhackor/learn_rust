// --8<-- [start:countdown]
struct Countdown {
    remaining: i32,
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let current = self.remaining;
            self.remaining -= 1;
            Some(current)
        } else {
            None
        }
    }
}

fn count_down(start: i32) -> Countdown {
    Countdown { remaining: start }
}
// --8<-- [end:countdown]

fn main() {
    let stream = count_down(3);
    let mut total = 0;
    for value in stream {
        println!("for 取得={value}");
        total += value;
    }
    println!("倒數合計={total}");
    assert_eq!(total, 6);
}
