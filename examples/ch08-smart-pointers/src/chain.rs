// --8<-- [start:chain]
pub enum Chain {
    End,
    Link { value: i32, next: Box<Chain> },
}

pub fn into_total(mut chain: Chain) -> i32 {
    let mut total = 0;
    loop {
        match chain {
            Chain::End => return total,
            Chain::Link { value, next } => {
                total += value;
                chain = *next;
            }
        }
    }
}
// --8<-- [end:chain]

#[cfg(test)]
mod tests {
    use crate::chain::Chain;
    use crate::chain::into_total;

    #[test]
    fn empty_single_and_two_links_have_their_own_totals() {
        assert_eq!(into_total(Chain::End), 0);
        assert_eq!(
            into_total(Chain::Link {
                value: 80,
                next: Box::new(Chain::End),
            }),
            80
        );
        assert_eq!(
            into_total(Chain::Link {
                value: 80,
                next: Box::new(Chain::Link {
                    value: 120,
                    next: Box::new(Chain::End),
                }),
            }),
            200
        );
    }

    #[test]
    fn zero_and_negative_values_remain_data() {
        let chain = Chain::Link {
            value: 0,
            next: Box::new(Chain::Link {
                value: -5,
                next: Box::new(Chain::Link {
                    value: 5,
                    next: Box::new(Chain::End),
                }),
            }),
        };
        assert_eq!(into_total(chain), 0);
    }
}
