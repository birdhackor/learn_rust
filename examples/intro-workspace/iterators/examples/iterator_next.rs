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
    let mut stream = count_down(3);
    println!("第一次 next={:?}", stream.next());
    println!("第二次 next={:?}", stream.next());
    println!("第三次 next={:?}", stream.next());
    println!("第四次 next={:?}", stream.next());
    println!("第五次仍是={:?}", stream.next());
    let mut zero = count_down(0);
    let mut negative = count_down(-4);
    assert_eq!(zero.next(), None);
    assert_eq!(negative.next(), None);
}
