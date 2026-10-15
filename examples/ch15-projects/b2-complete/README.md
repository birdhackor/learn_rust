# B2 完成快照：依序檢查清單

對應18.8。input_index保存原始行0-based索引；先編號再略空白，trim只作判空，原內容與重複網址仍各保留。Entry包住原索引與B1 owned Check。

零有效行／直接交給runner的零項清單回InputError::Empty，是設定問題。所有非空無效URL仍依序得到Input結果；HTTP未通過／Request／Timeout也不抹掉後續。借同一client，圈內await每份check後才保存與取下一份；每筆各用相同Duration跨度建立工作期限。

教材repo根目錄：

```bash
cargo check -p url-b2 --all-targets --locked
cargo test -p url-b2 --lib --locked
cargo run --quiet -p url-b2 --example b2_batch_demo --locked
```

六項checks驗owned文字、LF／CRLF／單獨CR／空白、原索引與duplicates、兩種Empty入口、全invalid、混合結果繼續、held兩筆gate與雙方收尾。testguards只守觀察，不加正式整批期限；檢查結構仍為每圈await後才取下一份。

demo得到index1／2／3／4四份結果：兩份200、Input、404。真Fixture處理三份工作請求並shutdown。port與錯誤Debug可變，不依賴公共網站狀態。

repo外：複製完整manifest、src與examples，先把dev-dependency ch11-local-service的path換成教材examples/ch11-async/local-service的實際絕對路徑。在快照目錄：

```bash
cargo +1.99.0 check --all-targets
cargo +1.99.0 test --lib
cargo +1.99.0 run --quiet --example b2_batch_demo
```

single.rs沿actual B1 core，只移除與舊test module之間的尾端分隔空白，程式內容相同；本步只接輸入與依序結果，沒有CLIJSON／JoinSet／取消。最終工具另整合，stages不累加成練習負擔。
