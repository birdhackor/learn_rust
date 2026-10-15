use ch13_futures::pinning::PinnedNote;
use std::mem;

fn main() {
    let mut left = PinnedNote::new(80);
    let mut right = PinnedNote::new(120);
    println!(
        "交換前：left={}，right={}",
        left.as_ref().value(),
        right.as_ref().value()
    );

    mem::swap(&mut left, &mut right);
    assert_eq!(left.as_ref().value(), 120);
    assert_eq!(right.as_ref().value(), 80);
    assert!(left.as_ref().address_matches());
    assert!(right.as_ref().address_matches());
    println!(
        "交換 Pin<Box> owner 後：left={}，right={}，兩份地址仍一致",
        left.as_ref().value(),
        right.as_ref().value()
    );

    left.as_mut().set_value(140);
    assert_eq!(left.as_ref().value(), 140);
    assert!(left.as_ref().address_matches());
    println!("只改 left 的 value：{}，地址仍一致", left.as_ref().value());

    mem::drop(left);
    println!(
        "left 的 owner 已清理，right 仍是={}",
        right.as_ref().value()
    );
    mem::drop(right);
    println!("兩份資料已正常收尾");
}
