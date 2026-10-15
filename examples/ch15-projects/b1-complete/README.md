# B1 完成快照

對應教材18.7，只完成單筆owned `Check`，沒有輸入清單、並行、CLI或JSON。GET最多跟隨5次redirect，final2xx通過；HTTP非2xx保留狀態，無效輸入、請求失敗與逾時各自分類。

client不設定reqwest請求期限，`check`以Tokio timeout包整份GET至finalheaders工作，預設10秒。只保存最後URL與狀態後drop response，不主動讀body。原文字不被Url解析或redirect改寫。輸入拒絕是None耗時；已嘗試是Some真毫秒數，成功量到headers、失敗量到失敗結束；client建立與解析在量測外。

教材repo根目錄：

```bash
cargo check -p url-b1 --all-targets --locked
cargo test -p url-b1 --lib --locked
cargo run --quiet -p url-b1 --example b1_single_demo --locked
```

六項測試重用14.9的真本機Fixture：輸入拒絕零工作請求、200/404/500、redirect對應、redirect上限、gate等待與放行、handler ack後的Tokio期限。timer測試不以真毫秒值判虛擬時間。五秒entered觀察期限與二十秒結果guard讓缺少請求或缺少工作期限的錯碼也能有限失敗並收尾；不是額外設定正式client期限。checks整份module與starter材料相同。

demo本機port和elapsed可變，固定答案為原URL `/redirect`、最後URL `/ok`、HTTP200通過，server處理兩份工作請求並收尾。不靠公共網站狀態或sleep猜測。

repo外參考：複製完整manifest、src與examples；**先把dev-dependency `ch11-local-service`的path改為你電腦上教材`examples/ch11-async/local-service`的實際絕對路徑**。在快照目錄執行：

```bash
cargo +1.99.0 check --all-targets
cargo +1.99.0 test --lib
cargo +1.99.0 run --quiet --example b1_single_demo
```

固定API核對reqwest0.13.5、Tokio1.53.3、url2.5.8。這是引導階段快照，最終B工具另整合；練習工作量不累加所有stage副本。
