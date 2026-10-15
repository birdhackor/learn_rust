# 18B 採納修正窄複查（封存 2026-10-15T11:02:19.176000+00:00）

本輪只複查已採納 C18B-01／02 的三份正文與實際頁面，不重審全章，不把修後結果倒寫成獨立初判。原初判於 2026-10-15 10:52:14 UTC 封存；其六份封存證據 SHA-256 全部維持原值。

## 結果

- **C18B-01：CLOSED。** 18.10:67 在 `record` 說明前新增「Tokio 的 **Id 具備 Copy**」，明說 `match &result` 借讀、成功分支取得 `&Id`、`*id` 複製 Id，然後借讀結束才接手 `result`。兩尺寸實際頁面均依此先教再用，文字與 inline code 可讀。
- **C18B-02：CLOSED。** 18.11:199–205、18.12:103–109 在原 TXT snippet 前先提供完整 JSON 字串，逐一說明 LF、單一一般空白、五個原始行號、最後 LF 與外層雙引號不屬於 TXT；原 snippet 明標為「逐行預覽」，並警告直接複製可能漏空白。兩頁、兩尺寸的實際 DOM JSON 均解碼成原 fixture 的精確 bytes，SHA-256 為 `e89bcb35b58442ca0ff82705c99cc9c344dd8175f4bf78cdac888e54a58f94ee`，保留輸入索引 **1、3、4**。桌面可見完整字串；手機可於 code 區獨立橫捲，已實看首尾與完整重建段落。原預覽仍省略空白，現在其用途與精確資料來源已有清楚區分，足以修復原 copy mismatch。

## 版本與實頁證據

原始 86 個來源重新逐筆比對，只有 `18-10-lesson.md`、`18-11-lesson.md`、`18-12-lesson.md` 三份正文變動；其餘 **83 份**（包含 Rust、fixtures、manifests、locks、SVG 與設定）與初判相同。修後來源 hash／before-after／diff 見 `round18b-ch18b-followup-source.json` 及同前綴各頁檔案。

使用正式 current strict site `http://127.0.0.1:8110/`，1280×800、390×844；實際擷取 2026-10-15T11:00:48.075Z 至 2026-10-15T11:00:57.189Z，HTML／來源 hash 與 DOM 字串存於 `round18b-ch18b-followup-visual/metadata.json`。已親自檢視全部 10 張原圖，透過以下五張桌面／手機並排圖查看：

- `pair-10-left.png`（`desktop-10-adopted-left.png`、`mobile-10-adopted-left.png`）
- `pair-11-left.png`、`pair-11-right.png`（各自對應兩尺寸 JSON 首尾視圖）
- `pair-12-left.png`、`pair-12-right.png`（各自對應兩尺寸 JSON 首尾視圖）

第一次重用同一 browser page 導航的擷取曾找不到 details，未據此形成結論；成功這次逐 URL 新開頁、取得 HTTP 200 並核對 current DOM，僅計成功後以上 10 圖。

未改 repo，未執行 Cargo／tests／build，未發布。兩項窄複查完成，依交接 **STOP**。
