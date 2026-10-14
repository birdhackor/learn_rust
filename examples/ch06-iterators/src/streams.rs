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

pub fn count_down(start: i32) -> impl Iterator<Item = i32> {
    Countdown { remaining: start }
}
// --8<-- [end:countdown]

// --8<-- [start:into]
pub struct NumberList {
    pub values: Vec<i32>,
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
pub fn sum_source<S: IntoIterator<Item = i32>>(source: S) -> i32 {
    let mut total = 0;
    for value in source {
        total += value;
    }
    total
}
// --8<-- [end:sum]

#[cfg(test)]
mod tests {
    use crate::streams::NumberList;
    use crate::streams::count_down;
    use crate::streams::sum_source;

    #[test]
    fn countdown_advances_and_preserves_the_finished_state() {
        let mut stream = count_down(3);
        assert_eq!(stream.next(), Some(3));
        assert_eq!(stream.next(), Some(2));
        assert_eq!(stream.next(), Some(1));
        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn nonpositive_countdowns_are_empty() {
        let mut zero = count_down(0);
        let mut negative = count_down(-4);
        assert_eq!(zero.next(), None);
        assert_eq!(negative.next(), None);
    }

    #[test]
    fn one_consumer_works_with_distinct_numeric_sources() {
        assert_eq!(sum_source(count_down(3)), 6);
        assert_eq!(sum_source(1..4), 6);
        assert_eq!(
            sum_source(NumberList {
                values: vec![1, 2, 3]
            }),
            6
        );
        assert_eq!(sum_source(NumberList { values: vec![] }), 0);
    }
}
