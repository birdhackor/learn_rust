struct Order {
    unit_price: i32,
    count: i32,
}

enum Shipment {
    Waiting,
    Ready(Order),
}

fn total_if_ready(shipment: &Shipment) -> i32 {
    let Shipment::Ready(order) = shipment else {
        return 0;
    };
    if order.count == 0 {
        return 0;
    }
    let goods = order.unit_price * order.count;
    let shipping = match goods {
        0..=299 => 20,
        _ => 0,
    };
    goods + shipping
}

fn main() {
    let waiting = Shipment::Waiting;
    let small = Shipment::Ready(Order {
        unit_price: 80,
        count: 2,
    });
    let boundary = Shipment::Ready(Order {
        unit_price: 300,
        count: 1,
    });
    let empty = Shipment::Ready(Order {
        unit_price: 80,
        count: 0,
    });
    assert_eq!(total_if_ready(&waiting), 0);
    assert_eq!(total_if_ready(&small), 180);
    assert_eq!(total_if_ready(&boundary), 300);
    assert_eq!(total_if_ready(&empty), 0);
    assert_eq!(total_if_ready(&small), 180);
    println!("等待=0，小訂單=180，門檻訂單=300，空訂單=0");
}
