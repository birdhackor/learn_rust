use ch08_smart_pointers::shared_chain::SharedChain;
use ch08_smart_pointers::shared_chain::total;
use std::mem::drop;
use std::rc::Rc;

fn main() {
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
    println!("共同尾段的strong={}", Rc::strong_count(&tail));
    println!("第一條總額={}", total(&first));
    println!("第二條總額={}", total(&second));
    drop(first);
    println!("第一條結束後strong={}", Rc::strong_count(&tail));
    drop(tail);
    println!("原tail handle結束，第二條仍是={}", total(&second));
}
