# B3：有限 task 的起始版

這是 18.9 的獨立起點，不是可用的批次檢查器。Input／讀行與 single core 已完成；fill、next、run_bounded 尚未實作。三個 stub 可編譯，不表示功能成立。

將 Cargo.toml、整個 src、checks/bounded-tests.rs 保存到自己的 url-b3-work。repo 外先把 dev-dependency ch11-local-service 的 path 換成教材 examples/ch11-async/local-service 的實際絕對路徑，再用 Rust 1.99.0：

```bash
cargo +1.99.0 check --all-targets
cargo +1.99.0 test --lib
```

尚未加入 checks 時是零項 test。正文介紹所有 helper／收尾契約後，把完整 checks/bounded-tests.rs 追加到 src/lib.rs；它已含 cfg 與 tests module，不再包一層。再跑 test 應有六項失敗、一項通過，不能當作完成。

建議先完成 fill＋next，再連上 run_bounded：前者應有三項通過、四項失敗，後者應七項通過。起始版的 dead_code allow 只避免尚未使用欄位的 lint 提醒；完成快照不需要它。不要用公開 mutable 欄位或虛假的讀取取代實作。

測試用有限觀察期限，失敗時先 abort_all／join 到 None／shutdown，再 panic。它們驗排程狀態與事件，不以公共站點、sleep 或執行速度判定。合成 task 測試故意產生一份 panic，Tokio 的 panic hook 可能印出訊息；test 最後檢查 JoinError，而非把它當 HTTP 錯誤。

參考答案在旁邊 b3-complete。各階段快照不累加成讀者同一次作業。
