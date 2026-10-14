# 11.7 單一 async HTTP

依序等最後的response，再按--body選擇讀完整文字。Client與runtime分工；同一次async request從連線到body完成共用10秒，文字解碼沒有大小上限。僅用已教API；不含受控server或並行範例。

從教材repo根目錄：

```bash
cargo test -p ch11-http-single --locked
cargo run -p ch11-http-single --locked -- --help
cargo run -p ch11-http-single --locked -- https://www.rust-lang.org/
cargo run -p ch11-http-single --locked -- https://www.rust-lang.org/ --body
cargo run -p ch11-http-single --example page_reply_solution --locked
```

公共URL兩項是手動練習，不是自動測試；status、最後URL、body與elapsed依當次網路。正常模式保留環境proxy；--direct只在允許直接連線時明選。不要用direct繞過環境要求的proxy。

HTTP2xx報告完成回0；HTTP未通過回1仍有stdout報告；client/request/body/output錯誤回1且嘗試stderr診斷；參數／URL錯誤回2。report寫出後明確flush。量測從GET準備前到網路工作／owned報告資料準備後，不含URL解析／clientbuild／報告輸出，不作效能優勢證據。

4 library＋2 CLI tests只驗離線輸入／人工報告／輸出flush／owned JSON模型／真binary參數；不能證明網路、redirect或timeout。page_reply_solution同樣人工材料，只驗HTTP成功後模型仍需解碼，印筆記：revision=3。11.8先教服務才加入受控網路測試。
