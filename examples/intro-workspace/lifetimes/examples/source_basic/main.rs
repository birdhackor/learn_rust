mod source;

use crate::source::longer;
use crate::source::same;

fn main() {
    let mut outside = String::new();
    outside.push_str("Rust tutorial");
    println!("單一來源={}", same(outside.as_str()));

    // --8<-- [start:inside]
    {
        let mut inside = String::new();
        inside.push_str("hi");
        let selected = longer(outside.as_str(), inside.as_str());
        println!("第一段較長={selected}");
        assert!(selected.len() == 13);

        let reversed = longer(inside.as_str(), outside.as_str());
        println!("第二段較長={reversed}");
        assert!(reversed.len() == 13);
    }
    // --8<-- [end:inside]

    let tied = longer("ab", "CD");
    println!("等長選第一段={tied}");
}
