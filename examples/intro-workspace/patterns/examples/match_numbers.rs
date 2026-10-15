// --8<-- [start:function]
fn shipping(tier: i32) -> i32 {
    match tier {
        0 => 20,
        1 => 10,
        _ => 0,
    }
}
// --8<-- [end:function]

fn main() {
    for tier in [0, 1, 2] {
        println!("等級{tier}的運費={}", shipping(tier));
    }
    assert_eq!(shipping(0), 20);
    assert_eq!(shipping(1), 10);
    assert_eq!(shipping(2), 0);
}
