# B1 起始版：一份網址、一份結果

對應教材18.7。`src/lib.rs`提供已教的資料型別、client建立與URL解析工具；`check`只回固定尚未完成標記。它能編譯，但沒有執行HTTP檢查。普通target沒有tests；零項通過不算功能驗收。

先讀正文的模型、預算與測試helper契約，再把完整`checks/single-tests.rs`追加到工作副本`src/lib.rs`末尾（它已自帶`#[cfg(test)] mod tests`）。不要修改這份快照來保存自己的答案。

在教材repo根目錄：

```bash
cargo check -p url-b1-starter --all-targets --locked
cargo test -p url-b1-starter --lib --locked
```

自己練習時，把完整`Cargo.toml`、`src/lib.rs`與`checks/single-tests.rs`保存到repo外目錄。manifest的`[dev-dependencies]`重用14.9服務；**先將`ch11-local-service`的path換成你電腦上教材`examples/ch11-async/local-service`的實際絕對路徑**。例如教材在`/work/learn_rust`，填`/work/learn_rust/examples/ch11-async/local-service`。其他直接依賴與features沿literal manifest；在工作目錄：

```bash
cargo +1.99.0 check --all-targets
cargo +1.99.0 test --lib
```

未追加checks前0項；追加後6項失敗。依正文逐步改`check`：輸入分類先使1項通過，HTTP分類與gate放行使5項通過，完整Tokio工作期限使6項通過。

工作10秒、真handler通知觀察5秒與pause後結果觀察20秒各有不同角色。測試guard失敗先resume（若已pause），abort/await client，再shutdown服務；不靠sleep或公共站的固定結果。`std::time::Instant`的真耗時不當虛擬十秒判準。
