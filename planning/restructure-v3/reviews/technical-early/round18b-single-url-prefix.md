# 第18.7節獨立技術初判

必要問題：0。可選修正：0。

完整實讀current正文、B1 starter／checks／complete／demo及全部normal path dependency closure，未看作者驗證／peer／歷史問題，也未讀18.8後稿補洞。

- 原文String、解析URL與final URL分開；HTTP2xx／非2xx／輸入／Request／Timeout分類與headers-only邊界正確。輸入拒絕None、已嘗試Some真毫秒（可為0），沒有把std時間當虛擬10秒。
- check在parse後開始計時，async work內建立GET／send，再以單一Tokio期限包住；client沒有另一份reqwest期限。兩層Result逐支保留一份Check，成功先複製欄位再drop response，沒有主動讀body。
- starter固定未完成標記與手動追加checks規則準確。完整六測試與starter checks逐字相同；5秒entered觀察、真網路後pause、20秒借用JoinHandle結果guard、resume／abort／await／server shutdown皆吻合，取得Output不重取。Check的Send及借用handle等待符合實際型別／Tokio API。
- 四份manifest的角色與repo外絕對path調整先於命令；Fixture→http-single→config/error/reply的七份normal來源全列出且與本人14章已seal source hash一致。dependency自己的binary／examples／integration／cfg(test)不會變成本次隱藏載入。
- demo先server收尾再印redirect→ok／200通過／可變Some耗時；SVG的原文保留與parse早退／headers期限路線正確。

15owned檔（13loaded source/config）、18snippets、6fenced／inline commands、24hash含2cached API，六test functions每份完整。清單與閉包證據在同名JSON。

未跑Cargo、Clippy、tests、HTTP、thread或timer；沒有browser／SVG視覺／讀者實驗。
