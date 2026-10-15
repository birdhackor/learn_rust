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
