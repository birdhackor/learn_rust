use lesson_engineering::order_subtotal;

fn main() {
    let subtotal = order_subtotal(80, 2);
    let batch_enabled = cfg!(feature = "batch");
    println!("單筆商品小計={subtotal}，batch={batch_enabled}");

    #[cfg(feature = "batch")]
    {
        use lesson_engineering::two_orders;
        let total = two_orders([80, 2], [120, 1]);
        println!("兩筆商品小計={total}");
    }

    let unix_target = cfg!(unix);
    let windows_target = cfg!(target_os = "windows");
    println!("編譯目標：unix={unix_target}，windows={windows_target}");
}
