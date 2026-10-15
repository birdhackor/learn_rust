pub mod workers;

use crate::workers::sum;
use std::sync::RwLock;
use std::sync::TryLockError;

fn main() {
    let values = RwLock::new(vec![80, 120]);
    let first = values.read().expect("新建RwLock尚未poison");
    let second = values.try_read().expect("沒有writer或等待中的writer");
    assert_eq!(sum(first.as_slice()), 200);
    assert_eq!(sum(second.as_slice()), 200);
    println!("兩份read guard同時存在，小計各為200");
    match values.try_write() {
        Err(TryLockError::WouldBlock) => println!("兩份read guard仍在，try_write回WouldBlock"),
        _ => panic!("本例應因活躍reader而無法取得writer"),
    }
    drop(first);
    match values.try_write() {
        Err(TryLockError::WouldBlock) => println!("只drop第一份，第二份仍阻止writer"),
        _ => panic!("第二份reader還在，本例不能取得writer"),
    }
    drop(second);
    {
        let mut writer = values.try_write().expect("已無活躍guard或poison");
        writer.push(30);
    }
    let view = values.read().expect("writer已釋放，更新正常完成");
    assert_eq!(view.len(), 3);
    assert_eq!(sum(view.as_slice()), 230);
    println!(
        "兩份reader都drop後追加30，長度={}，總和={}",
        view.len(),
        sum(view.as_slice())
    );
}
