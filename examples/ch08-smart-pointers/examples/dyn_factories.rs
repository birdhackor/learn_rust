use ch08_smart_pointers::dyn_charges::Charge;
use ch08_smart_pointers::dyn_charges::make_adjuster;
use ch08_smart_pointers::dyn_charges::make_bundle;
use ch08_smart_pointers::dyn_charges::make_charge;
use ch08_smart_pointers::dyn_charges::make_opaque_bundle;

fn main() {
    let concrete = make_bundle(120);
    println!("具體回傳可讀price欄位={}", concrete.price);
    let opaque = make_opaque_bundle(120);
    println!("opaque介面提供amount={}", opaque.amount());
    println!("dyn工廠選訂單={}", make_charge(true).amount());
    println!("dyn工廠選組合={}", make_charge(false).amount());

    let subtract = make_adjuster(true, 10);
    let increase = make_adjuster(false, 10);
    println!("兩種closure經dyn呼叫={}、{}", subtract(30), increase(30));
}
