use std::mem::drop;
use std::rc::Rc;
use std::rc::Weak;

struct Tracked {
    id: i32,
}

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("清理原資料id={}", self.id);
    }
}

fn main() {
    let empty: Weak<Tracked> = Weak::new();
    match empty.upgrade() {
        Some(value) => println!("空Weak意外取得id={}", value.id),
        None => println!("空Weak沒有資料"),
    }

    let owner = Rc::new(Tracked { id: 1 });
    let observer = Rc::downgrade(&owner);
    println!("建立Weak後strong={}", Rc::strong_count(&owner));
    match observer.upgrade() {
        Some(temporary) => {
            println!("upgrade讀到id={}", temporary.id);
            println!("暫時取得strong={}", Rc::strong_count(&temporary));
        }
        None => println!("資料已結束"),
    }
    println!("暫時handle結束後strong={}", Rc::strong_count(&owner));
    drop(owner);
    match observer.upgrade() {
        Some(value) => println!("仍有owner，id={}", value.id),
        None => println!("最後strong結束，Weak不能再取得資料"),
    }
    drop(observer);
    println!("觀察handle也已結束");
}
