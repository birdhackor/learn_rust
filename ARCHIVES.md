# 歷史審閱與發布紀錄

目前教材、可執行範例、實測效能資料及作者來源索引保留在主線。歷史規劃、審閱輪次、重複盤點、來源指紋及完整驗證輸出移出目前工作樹；網站建置與正常測試不讀取這些封存檔案。

## 固定版本

重整前的完整版本為 [`9be13cb88a033e77194aaf2d2fd701fd0f88345d`](https://github.com/birdhackor/learn_rust/tree/9be13cb88a033e77194aaf2d2fd701fd0f88345d)。其 [`planning/`](https://github.com/birdhackor/learn_rust/tree/9be13cb88a033e77194aaf2d2fd701fd0f88345d/planning) 保存原來的1,227個檔案，包括舊版與重排後的審閱、作者紀錄、問題處置、實跑及視覺抽查限制。原始檔案不因本次整理而改寫，也沒有改寫 Git 歷史。

- [當時的全書重排進度](https://github.com/birdhackor/learn_rust/blob/9be13cb88a033e77194aaf2d2fd701fd0f88345d/planning/restructure-v3/status.md)
- [當時的審閱範圍與限制](https://github.com/birdhackor/learn_rust/blob/9be13cb88a033e77194aaf2d2fd701fd0f88345d/planning/restructure-v3/reviews/README.md)
- [當時的實際執行與視覺驗證](https://github.com/birdhackor/learn_rust/blob/9be13cb88a033e77194aaf2d2fd701fd0f88345d/planning/restructure-v3/verification/README.md)

這些是當時版本的紀錄，不表示後續版本已通過同一輪審閱。AI審閱、程式執行、視覺抽查與實際讀者學習成效仍分開判斷。

## 按需取得與離線核對

需要離線查閱時，在專案根目錄執行（Python 3.11以上，只使用標準函式庫）：

```bash
python scripts/archive-assets.py fetch --group review-history
python scripts/archive-assets.py verify --group review-history
```

`fetch`取得固定提交的公開來源壓縮包，先核對壓縮包大小與SHA-256，再逐檔核對清單，最後將原始 `planning/` 放到 `outputs/archives/review-history/`。壓縮包快取在 `outputs/archive-cache/`；這些輸出都不提交。來源、完整commit、每檔大小與SHA-256記在 [catalog.json](assets/archives/catalog.json)。來源索引另保存在 [curriculum/](curriculum/README.md)。

已核對的檔案或壓縮包會離線重用。`verify`不連網；不同內容的本機檔案會報錯，不覆寫修改。工具也拒絕清單以外的審閱檔案、路徑越界、連結及不完整壓縮包。GitHub的來源包同時含其他原始碼，但工具只還原清單指定的歷史審閱樹。

來源壓縮包若改變壓縮方式而不再符合固定雜湊，工具會停止；維護者須重新比對固定提交的原始檔案並明確更新清單，不會自動接受新內容。瀏覽固定commit不需要下載包；一般完整clone仍包含舊歷史。

## 日後的保存方式

目前有效的教材結構、來源、授權與支持公開主張的實測資料保留在 Git。日常審閱的完整log、逐段紀錄、快照與截圖寫入被忽略的 `work/`，需要協作時使用有保存期限的CI artifact；PR保存範圍、結論、必要修正及未驗證限制。

需要長期重現的大型新資料應使用固定Release或適當的資產儲存空間，並將版本、大小、逐檔SHA-256、來源與授權清單保存在Git。不能把這類必要資料只放在會過期的artifact，也不把下載藏進一般測試或網站建置。
