use ch10_concurrency::workers::panic_message;
use ch10_concurrency::workers::sum;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

fn main() {
    let values = Arc::new(RwLock::new(vec![80, 120]));
    let writer_values = Arc::clone(&values);
    let writer = thread::spawn(move || -> () {
        let mut guard = writer_values.write().expect("新建RwLock尚未poison");
        guard.push(30);
        panic!("追加完整一筆後持write guard故意panic");
    });
    match writer.join() {
        Ok(_) => panic!("本例的writer應發生panic"),
        Err(payload) => {
            let message = panic_message(payload);
            assert_eq!(message.as_str(), "追加完整一筆後持write guard故意panic");
            println!("已等待writer，收到panic：{message}");
        }
    }

    match values.read() {
        Ok(_) => panic!("本例writer持鎖panic，後續read應帶poison警告"),
        Err(error) => {
            let view = error.into_inner();
            assert_eq!(view.len(), 3);
            assert_eq!(view.as_slice()[0], 80);
            assert_eq!(view.as_slice()[1], 120);
            assert_eq!(view.as_slice()[2], 30);
            assert_eq!(sum(view.as_slice()), 230);
            println!("poison Err中的read guard仍看到長度3、總和230");
            values.clear_poison();
        }
    }
    let view = values.read().expect("已檢查已知內容並清除poison");
    assert_eq!(view.len(), 3);
    assert_eq!(sum(view.as_slice()), 230);
    println!("依本例政策保留完整追加，清標記後read得到Ok");
}
