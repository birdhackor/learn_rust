pub mod workers;

use crate::workers::panic_message;
use crate::workers::sum;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

fn main() {
    let values = Arc::new(RwLock::new(vec![80, 120]));
    let reader_values = Arc::clone(&values);
    let reader = thread::spawn(move || -> () {
        let view = reader_values.read().expect("新建RwLock尚未poison");
        assert_eq!(view.len(), 2);
        panic!("持read guard的工作故意panic");
    });
    match reader.join() {
        Ok(_) => panic!("本例的reader應發生panic"),
        Err(payload) => {
            let message = panic_message(payload);
            assert_eq!(message.as_str(), "持read guard的工作故意panic");
            println!("已等待reader，收到panic：{message}");
        }
    }
    let view = values.read().expect("reader的panic不會poison這份RwLock");
    assert_eq!(sum(view.as_slice()), 200);
    println!("後續read得到Ok，原總和仍是200");
}
