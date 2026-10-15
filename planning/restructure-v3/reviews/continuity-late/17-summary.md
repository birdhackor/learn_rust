# 第17章獨立銜接初判

八節全文、全部直接snippet完整來源與本地依賴、生成器/oracle、manifest/fixtures/expected、測試、答案和正文嵌入的量測原始輸出均已讀。17.4分1–210及211–375行，未以工具截斷代替完整閱讀。逐節四題、實源SHA256與rendered全檔嵌入比對見 independent-17-initial.json；另補兩空lib與derived_default.expected的hash於17-extra-owned-sources.json。

必要問題：無。可選問題：無。

macro主線從已懂宣告樣板走到名稱來源，再利用已有AST資訊生成impl；scope、hygiene、求值次數與生成後型別失敗各有契約。公開API用有效default、私有欄位、外部分類、同一consumer與features分開來源/行為相容。property tests先固定oracle，再教生成/縮減與反例保存。Loom/Miri回收13/15章機制，保留探索/解譯範圍。Criterion與dhat分別固定計時/配置邊界、同一工作及證據限制，未把少做工作當改善。

九張原位圖均實看桌面1280×800及手機390×844；宏來源、語法轉換、反例、交錯/失效與量測邊界可對應且手機可讀。另開17.4及17.6多檔長折疊。所有完整source已在frozen preview找到同頁嵌入。

未重跑Rust、Loom、Miri或量測，也未重開原profile JSON viewer；此次不代替技術/執行驗證。非全書首讀盲測，未讀作者ledger或其他17章finding。
