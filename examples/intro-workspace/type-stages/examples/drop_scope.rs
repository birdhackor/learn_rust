// --8<-- [start:tracked]
struct Tracked {
    id: i32,
}

impl Drop for Tracked {
    fn drop(&mut self) {
        println!("清理 id={}", self.id);
    }
}
// --8<-- [end:tracked]

fn main() {
    let first = Tracked { id: 1 };
    println!("建立外層={}", first.id);
    {
        let second = Tracked { id: 2 };
        let third = Tracked { id: 3 };
        println!("建立內層={}、{}", second.id, third.id);
    }
    let view = &first;
    println!("內層已收尾，外層仍是={}", view.id);
    println!("main準備結束");
}
