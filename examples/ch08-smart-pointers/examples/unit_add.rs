use ch08_smart_pointers::newtypes::Meters;
use ch08_smart_pointers::newtypes::Millimeters;
use ch08_smart_pointers::newtypes::add_same;

fn main() {
    let joined = Meters(80) + Meters(120);
    println!("兩段路程相接={}公尺", joined.0);
    let generic = add_same(Meters(40), Meters(60));
    println!("要求同型別輸出的函式={}公尺", generic.0);
    let converted = Millimeters(500) + Meters(2);
    println!("先換右邊的單位，結果={}毫米", converted.0);
}
