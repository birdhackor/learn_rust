use ch04_traits::counts::owned_text;
use std::collections::HashMap;

struct StockItem {
    name: String,
    quantity: usize,
}

fn quantity_by_name(items: &[StockItem]) -> HashMap<String, usize> {
    let mut totals: HashMap<String, usize> = HashMap::new();
    for item in items {
        match totals.get_mut(item.name.as_str()) {
            Some(total) => *total += item.quantity,
            None => {
                totals.insert(item.name.clone(), item.quantity);
            }
        }
    }
    totals
}

fn main() {
    let items = vec![
        StockItem {
            name: owned_text("Rust"),
            quantity: 2,
        },
        StockItem {
            name: owned_text("Python"),
            quantity: 3,
        },
        StockItem {
            name: owned_text("Rust"),
            quantity: 6,
        },
    ];
    let totals = quantity_by_name(items.as_slice());
    assert_eq!(totals.len(), 2);
    assert_eq!(totals.get("Rust"), Some(&8));
    assert_eq!(totals.get("Python"), Some(&3));
    assert_eq!(totals.get("JavaScript"), None);
    println!("Rust 總量={:?}", totals.get("Rust"));
    println!("Python 總量={:?}", totals.get("Python"));
    println!("原清單仍有{}筆資料", items.len());
}
