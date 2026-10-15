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

// --8<-- [start:into]
struct NumberList {
    values: Vec<i32>,
}

impl IntoIterator for NumberList {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
// --8<-- [end:into]

// --8<-- [start:sum]
fn sum_source<S: IntoIterator<Item = i32>>(source: S) -> i32 {
    let mut total = 0;
    for value in source {
        total += value;
    }
    total
}
// --8<-- [end:sum]

fn main() {
    let values = NumberList {
        values: vec![1, 2, 3],
    };
    println!("自訂清單合計={}", sum_source(values));
    println!("range 合計={}", sum_source(1..4));
    println!("倒數工作合計={}", sum_source(count_down(3)));
    assert_eq!(sum_source(NumberList { values: vec![] }), 0);
}
