# v3 結構檢查器

`scripts/check-tutorial-structure.py` 檢查重排資料、真實 Markdown／snippet 來源及建置後的內部連結。它不執行 Cargo、站點建置、網路請求或發布，也不修改受檢資料。教學充分性、完整 API 載入閉包、逐段首讀與實際桌面／手機畫面，仍由獨立審閱及既有驗收流程確認。

## 執行

在儲存庫根目錄使用已安裝依賴的 Python：

```bash
.venv/bin/python scripts/check-tutorial-structure.py --draft
.venv/bin/python scripts/check-tutorial-structure.py --self-test
```

完成既有嚴格站點建置後，可指定產物位置檢查：

```bash
.venv/bin/python scripts/check-tutorial-structure.py --draft --site-dir site
.venv/bin/python scripts/check-tutorial-structure.py --final --site-dir site
```

`--final` 未指定 `--site-dir` 時使用 `zensical.toml` 的 `site_dir`。`--root` 可指定另一份儲存庫；`--site-dir` 的相對路徑以執行時工作目錄為準。`--max-messages 500` 可增加每類訊息的顯示上限。一般 `python3` 可跑 stdlib 自測；正式 Markdown／snippet 檢查使用儲存庫鎖定的 `markdown`、`pymdownx` 依賴，缺少依賴會列為待完成。

退出碼：

- `0`：所選模式的結構檢查完成。
- `1`：結構錯誤；或 final 模式仍有待完成項目。
- `2`：draft 尚有缺頁、未接受或尚未執行的檢查。draft 不會將未完成遷移算成通過。

`--self-test` 只使用可自動清除的暫存目錄，涵蓋有效連結、缺失圖片／連結／anchor、站點部署前綴、重複 HTML id、凍結來源漂移與作者兼任審閱者的拒絕。使用鎖定環境時另以真實 snippets 引擎檢查有效具名區段、缺失區段及缺失檔案；無引擎時明記略過。

## 實際檢查範圍

- 18 章、131 個唯一 canonical id／path；章與節的順序、標題、`NN-MM-lesson.md` 路徑一致，未列入清單的 canonical 檔案會報錯。
- 109 個唯一舊 id／path 與獨立保存的 `legacy-anchors.json` 來源集合完全相同；每個舊來源都有 canonical 去向，`old_ids` 與遷移表雙向一致，歷史來源範圍有有效行號。原來源行號是搬移前的證據，不拿相容入口的新行號冒充原文。
- 原始 27 個掃描問題 id 全數存在且唯一，保留原始引文、位置、處置及收益理由。最終狀態與實際證據位置分開查核。
- canonical Markdown 經設定中的 Markdown extensions 與真實 snippets 引擎展開；檢查唯一 H1 的顯示文字為 manifest 的 `id title`，檢查來源檔／具名區段／行數切片及遞迴 include。snippets 的檔案讀取基準固定為儲存庫根目錄，禁止網路下載；實際切片及區段語義沿用已鎖定引擎，未另寫一套解析器。
- canonical 連結與圖片的檔案目標、Markdown／HTML／SVG anchor；建置後所有 HTML 的內部 `href`、`src`、`poster`、一般 `srcset`、重複 id 與 anchor。百分比編碼及部署子路徑按 `site_url` 解析，外部網站不發請求。
- 全部 canonical 產物及 H1 存在；109 個原 URL 保留總計 890 個已捕捉的 heading anchors，且相容入口列出其全部新去向。
- 導覽的 18 章順序、章名、各節標籤與路徑完全依照 manifest。

## 接受狀態與證據格式

整合時使用既有欄位，不另增加 `sections.json` 的欄位：

- 每節 `status` 的最終值為 `accepted`；`owner` 與證據中的 `author` 相同。
- `readiness_evidence` 是儲存庫相對 JSON 檔路徑，可由同一波多節共用。
- `loaded_teaching_sources` 列出實際檔案路徑，包含所有 snippet 展開直接或遞迴讀取的檔案，以及審閱核定的其他載入來源。
- `manifest` 可為 `null`；有值時須指向實際檔案，`command` 須為具體非空指令。檢查器不執行該指令。
- `first_available_at` 使用已存在且不晚於本節的 canonical id。

一份波次證據的最小形狀如下。SHA256 必須換成真實當前值；人名／代理識別須對應實際作者與獨立審閱者：

```json
{
  "reviewed_status": "accepted",
  "section_ids": ["1.1"],
  "author": "chapter01-author",
  "technical_reviewer": "chapter01-technical-reviewer",
  "continuity_reviewer": "chapter01-continuity-reviewer",
  "review_reports": [
    "planning/restructure-v3/reviews/ch01-technical.md",
    "planning/restructure-v3/reviews/ch01-continuity.md"
  ],
  "file_sha256": {
    "docs/chapters/01-01-lesson.md": "實際的64字元SHA256"
  },
  "teaching_positions": [
    {
      "section_id": "1.1",
      "path": "docs/chapters/01-01-lesson.md",
      "line": 12,
      "quote": "該行實際存在的教學原文"
    }
  ]
}
```

`author`、`technical_reviewer`、`continuity_reviewer` 三者須不同。每節至少有一筆定位在該節 canonical 正文的 `teaching_positions`，`quote` 必須出現在指定單行。`file_sha256` 覆蓋正文、`manifest`、`loaded_teaching_sources` 及實際 snippets 檔案；額外列出的閉包檔案也逐一比對。正文或來源修改使雜湊失效後，必須重新凍結及完成受影響審閱。

27 筆處置的 `implementation_status` 最終同樣使用 `accepted`。`new_locations` 為非空 list，元素可為 canonical id、儲存庫相對檔路徑，或 `{ "path": "…", "line": 12, "quote": "…" }`。`verification` 及 `review_reports` 使用實際檔路徑／位置物件，也可使用其 list。位置物件有 `line` 時必須同時有該行的真實 `quote`。

## 邊界與目前結果

欄位齊全、名稱出現及引文存在都不證明先教後用或教學充分。檢查器只核對實際定位、凍結檔案與獨立接受紀錄的結構；它不能辨認假冒的審閱身份，不能由 `loaded_teaching_sources` 自動推出 Rust 所有 API／module／helper／fixture 的完整載入閉包。這些仍須技術與銜接審閱按真實內容裁定，重大新寫章的逐段首讀另核對原紀錄。

建置後檢查只讀現成站點，不能證明站點剛由當前 source 建出，應接在成功的嚴格建置後執行。它不解析 CSS 背景圖片、JavaScript 動態插入連結或含 `data:` 的混合 `srcset`，不做外部連結健康檢查，也不以圖檔／anchor 存在代替視覺檢查。相容入口是否清除過時教學內容仍由正文覆核確認。

實作初次驗證時，syntax、暫存 fixture 自測，以及舊 14.3 真實 snippets 展開（8 個來源檔案）成功；當時尚未落地 canonical，真實 v3 draft 回報 `0 errors / 291 pending`、退出碼 `2`。隨後章 1 的七頁初稿落地，重查實際發現七個 H1／manifest 標題差異及兩個尚缺目標的連結，為 `9 errors / 284 pending`、退出碼 `1`；這些待整合項已交回 root。兩次都沒有宣稱完成。把既有舊站交給 HTML checker 時，內部連結與原 anchor 檢查未另報錯；相容入口尚未產生，因此仍正確回報缺少 canonical 去向，不能當成 final 通過。
