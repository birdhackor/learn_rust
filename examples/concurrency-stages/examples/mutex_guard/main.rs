use std::sync::Mutex;

fn main() {
    let total = Mutex::new(0);
    {
        let mut guard = total.lock().expect("新建Mutex尚未poison");
        *guard += 80;
    }
    let mut guard = total.lock().expect("沒有持鎖panic");
    *guard += 120;
    assert_eq!(*guard, 200);
    println!("兩次持鎖更新，總和={}", *guard);
    drop(guard);

    let guard = total.lock().expect("前一份guard已解鎖");
    assert_eq!(*guard, 200);
    println!("再次取得guard，讀到={}", *guard);
}
