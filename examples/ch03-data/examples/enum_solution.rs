enum Shipment {
    Waiting,
    Ready { count: i32 },
}

fn ready_count(shipment: &Shipment) -> i32 {
    match shipment {
        Shipment::Waiting => 0,
        Shipment::Ready { count } => *count,
    }
}

fn main() {
    let waiting = Shipment::Waiting;
    let ready = Shipment::Ready { count: 3 };
    assert_eq!(ready_count(&waiting), 0);
    assert_eq!(ready_count(&ready), 3);
    assert_eq!(ready_count(&ready), 3);
    println!("等待出貨=0，可出貨=3，原資料仍可借用");
}
