# B3：有限 task 的完成快照

本節只改排程。B2 的原行索引／original／空白規則與 B1 的單筆能力沿用；沒有 CLI、JSON 或使用者取消功能。jobs 必須為正，default_jobs 為 8；空有效清單是 ConfigError::Empty。Input 請從 prepare_inputs 取得，保留唯一且遞增的原行索引。

在教材 repo 根目錄：

```bash
cargo check -p url-b3 --all-targets --locked
cargo test -p url-b3 --lib --locked
cargo run -p url-b3 --example b3_bounded_demo --locked
```

七項 test 驗零上限／空清單拒絕、全 invalid、混合與重複、收取後才補工／完成與輸入順序、task panic／abort 對應、完成未收取仍占名額。合成 task panic 是受控材料，不是正常 HTTP 失敗；其 panic hook 文字不是 test 失敗。

Demo 建真本機服務，動態 port；原 index0 空行，index1／3 重複 /ok、index2 /notes、index4 /missing。先收妥 Fixture 再印四份結果，順序為 1／2／3／4；HTTP200／Input／HTTP200／HTTP404。Opaque error 的 Debug 與 port 不作固定輸出契約。

每筆仍 GET，最多跟五次 redirect，final2xx 通過；只等 final headers，不主動讀 body。Tokio 每筆 deadline 預設十秒，沒有整批共用倒數。jobs 限制已提交尚未收取的 task 數，包括已完成尚未收取者；pending 與結果仍可能隨輸入總數增加。

Id 立即登記完整 Input；task 拿 original 的複本，panic／abort 沒有 Check 時仍保存原文。run_bounded 最後按原索引排序，task 完成順序可不同。測試故障收尾 abort_all 後還要 join 至 None；Drop JoinSet 不代表 children 已等完。

此快照 repo 外複製時，dev-dependency Fixture path 先換實際教材絕對路徑，再依 15.1 選 Rust 1.99.0。直接依賴版本要求與 Cargo.lock 的關係沿 7.1；不要把 version 字串誤當精確鎖版。
