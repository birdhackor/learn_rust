use std::rc::Rc;

// --8<-- [start:shared-chain]
pub enum SharedChain {
    End,
    Link { value: i32, next: Rc<SharedChain> },
}

pub fn total(mut chain: &SharedChain) -> i32 {
    let mut sum = 0;
    loop {
        match chain {
            SharedChain::End => return sum,
            SharedChain::Link { value, next } => {
                sum += *value;
                chain = next;
            }
        }
    }
}
// --8<-- [end:shared-chain]

#[cfg(test)]
mod tests {
    use crate::shared_chain::SharedChain;
    use crate::shared_chain::total;
    use std::mem::drop;
    use std::rc::Rc;

    #[test]
    fn two_heads_keep_the_same_tail_alive_independently() {
        let tail = Rc::new(SharedChain::Link {
            value: 120,
            next: Rc::new(SharedChain::End),
        });
        let first = SharedChain::Link {
            value: 80,
            next: Rc::clone(&tail),
        };
        let second = SharedChain::Link {
            value: 40,
            next: Rc::clone(&tail),
        };
        assert_eq!(Rc::strong_count(&tail), 3);
        assert_eq!(total(&first), 200);
        assert_eq!(total(&second), 160);
        drop(first);
        assert_eq!(Rc::strong_count(&tail), 2);
        assert_eq!(total(&second), 160);
        drop(tail);
        assert_eq!(total(&second), 160);
    }

    #[test]
    fn an_upgraded_weak_is_an_additional_strong_owner() {
        let owner = Rc::new(120);
        let observer = Rc::downgrade(&owner);
        let upgraded = observer.upgrade();
        assert_eq!(Rc::strong_count(&owner), 2);
        drop(owner);
        let read_back = match upgraded {
            Some(last_owner) => {
                assert_eq!(Rc::strong_count(&last_owner), 1);
                *last_owner
            }
            None => 0,
        };
        assert_eq!(read_back, 120);
        let remaining_strong = match observer.upgrade() {
            Some(still_here) => Rc::strong_count(&still_here),
            None => 0,
        };
        assert_eq!(remaining_strong, 0);
    }
}
