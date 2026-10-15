struct Order {
    unit_price: i32,
    count: i32,
}

enum Request {
    Order(Order),
    Cancel,
}

fn show_request(request: Request) {
    match request {
        Request::Order(Order { unit_price, count }) => {
            println!("內層訂單小計={}", unit_price * count);
        }
        Request::Cancel => println!("取消這次工作"),
    }
}

fn main() {
    show_request(Request::Order(Order {
        unit_price: 120,
        count: 3,
    }));
    show_request(Request::Cancel);
}
