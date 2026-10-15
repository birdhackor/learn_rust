use intro_modules_basic::pricing::sum_three;
use intro_modules_basic::pricing::total_with_shipping;

fn main() {
    let total = total_with_shipping(80, 2);
    let sum = sum_three([120, 80, 200]);
    println!("訂單總額={total}，價目總和={sum}");
}
