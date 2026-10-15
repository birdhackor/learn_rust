pub mod workers;

use crate::workers::sum;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;

fn main() {
    let values = RwLock::new(vec![80, 120]);
    let mut writer = values.write().expect("新建RwLock尚未poison");
    writer.push(30);
    let view = RwLockWriteGuard::downgrade(writer);
    assert_eq!(view.len(), 3);
    assert_eq!(sum(view.as_slice()), 230);
    let second = values.try_read().expect("沒有其他writer或等待者");
    assert_eq!(sum(second.as_slice()), 230);
    println!("write轉read後仍讀剛寫的230，另一份reader也可取得");
    drop(second);
    drop(view);

    let mut writer = values.write().expect("所有reader已釋放");
    writer.push(40);
    assert_eq!(sum(writer.as_slice()), 270);
    println!("兩份read guard釋放後，下一次write得到總和270");
}
