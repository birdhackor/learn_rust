use std::sync::RwLock;

fn main() {
    let values = RwLock::new(vec![80, 120]);
    let observed = {
        let view = values.read().expect("新建RwLock尚未poison");
        view.len()
    };
    assert_eq!(observed, 2);
    {
        let mut other_writer = values.write().expect("舊reader已釋放");
        other_writer.push(30);
    }
    let mut writer = values.write().expect("另一路徑已釋放writer");
    let before = writer.len();
    if writer.len() < 3 {
        writer.push(30);
    }
    assert_eq!(before, 3);
    assert_eq!(writer.len(), 3);
    println!("舊讀取長度={observed}，持write guard重查={before}，沒有重複追加");
}
