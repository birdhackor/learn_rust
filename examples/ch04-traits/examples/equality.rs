#[derive(Debug, PartialEq)]
struct OrderSummary {
    amount: i32,
    count: i32,
}

#[derive(Debug, PartialEq)]
enum Shipment {
    Waiting,
    Ready { count: i32 },
}

fn main() {
    let first = OrderSummary {
        amount: 360,
        count: 3,
    };
    let same = OrderSummary {
        amount: 360,
        count: 3,
    };
    let different_count = OrderSummary {
        amount: 360,
        count: 4,
    };
    println!("相同欄位={}", first == same);
    println!("同金額、不同數量={}", first == different_count);
    assert_eq!(first, same);
    assert!(first != different_count);

    let waiting = Shipment::Waiting;
    let ready = Shipment::Ready { count: 3 };
    let another_ready = Shipment::Ready { count: 4 };
    println!("等待與可出貨相等={}", waiting == ready);
    println!("相同形態、不同數量={}", ready == another_ready);
    assert!(waiting != ready);
    assert!(ready != another_ready);
}
