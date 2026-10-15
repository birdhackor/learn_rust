enum Shipment {
    Waiting,
    Ready { count: i32 },
}

fn ready_count(shipment: Shipment) -> i32 {
    match shipment {
        Shipment::Waiting => 0,
        Shipment::Ready { count } => count,
    }
}

fn main() {
    let waiting = ready_count(Shipment::Waiting);
    let ready = ready_count(Shipment::Ready { count: 3 });
    assert_eq!(waiting, 0);
    assert_eq!(ready, 3);
    println!("等待的統計貢獻={waiting}，可出貨={ready}");
}
