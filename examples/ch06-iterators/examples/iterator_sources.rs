use ch06_iterators::streams::NumberList;
use ch06_iterators::streams::count_down;
use ch06_iterators::streams::sum_source;

fn main() {
    let values = NumberList {
        values: vec![1, 2, 3],
    };
    println!("自訂清單合計={}", sum_source(values));
    println!("range 合計={}", sum_source(1..4));
    println!("倒數工作合計={}", sum_source(count_down(3)));
}
