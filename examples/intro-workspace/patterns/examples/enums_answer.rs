// 這一階段只建立形態，尚未讀取 count 欄位。
#[allow(dead_code)]
enum Shipment {
    Waiting,
    Ready { count: i32 },
}

fn main() {
    let _waiting = Shipment::Waiting;
    let _ready = Shipment::Ready { count: 3 };
    println!("已建立等待與可出貨3件的兩筆資料");
}
