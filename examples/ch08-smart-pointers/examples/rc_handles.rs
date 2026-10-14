use std::mem::drop;
use std::rc::Rc;

struct Tracked {
    id: i32,
}

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("清理原資料id={}", self.id);
    }
}

fn main() {
    let first = Rc::new(Tracked { id: 1 });
    println!("建立後strong={}", Rc::strong_count(&first));
    let second = Rc::clone(&first);
    println!("clone後strong={}", Rc::strong_count(&first));
    println!("兩個handle指同一配置={}", Rc::ptr_eq(&first, &second));
    drop(first);
    println!("第一個handle結束，仍讀到id={}", second.id);
    println!("剩餘strong={}", Rc::strong_count(&second));
    drop(second);
    println!("最後一個handle已結束");
}
