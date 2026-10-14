use ch05_errors::policy::checked_count;

fn main() {
    let known = checked_count(3).unwrap();
    let empty_batch = checked_count(0).expect("固定的空批次件數應符合0到100的規則");
    println!("固定件數={known}，空批次件數={empty_batch}");

    let values = [2, 4];
    let view = &values[..];
    let first = view.get(1).unwrap();
    let second = view.get(1).expect("已建立兩筆資料，索引1應存在");
    println!("Some中的數字={}、{}", *first, *second);
}
