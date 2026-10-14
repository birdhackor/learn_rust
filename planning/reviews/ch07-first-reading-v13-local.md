# 第 7 章 v13 指定局部重讀

這是 AI 模擬讀者的修後局部核對，不是新的盲讀，也不是第 7 章原初判。既有背景為基本 Python、實際已讀第 1–7 章正文及已記錄局部補讀，現在另包含完整第 8 章 v8 的 66 個正文單位。不能用後來的第 8 章知識回填原第 7 章判斷；若當下理解需要這項新增背景，另明記。第 1–6 章已讀版本及局部範圍沿用既有 entry 記錄，不宣稱完整讀過各章最新稿。

唯一新正文來源：`frozen/ch07-review-fixed-v13`；依指定次序只讀 `004-005`、`008-002`、`009-004`、`009-005`、`009-006`。第 7 章原讀來源為 prefix v9、tail v10，7.5 的 `005-006` 曾於 v11 作局部核對，本輪不自動重開。本輪不看整章 source、archive、作者說明、peer／技術／銜接報告、live 或後章。此次通知提供了修改位置與核對範圍，故不是無提示首讀；未提供技術答案或第 8 章 peer 結果。

各單位由 `read_visible_unit.py ch07v13` 在輸出前過濾 closed 答案；新獨立 state／ledger 僅含五個指定單位，上一單位四題與 hash 已存後才開下一單位。Q2 分機制與既存需要。只在原位圖出現後核對同 v13 SVG，未讀完整頁面、未執行 Rust 範例或探測。原初判、舊 followup、entry 與所有既有封存 hash 不變。

### 004-005 — 可控制的短讀取來源與 owned bytes

局部重讀對照：原 prefix v9 此單位 SHA 為 `612fc559277101ef64940d971cf7055c12e4cd5c3a1579ef000271f521e8bdef`；本次核對 v13 可見正文，未重開其他 7.4 單位。

Q1：ShortReader 以 owned `Vec<u8>`、下一個未交出的 position 與每次上限保存來源狀態；`impl Read` 讓已讀的 read_all 使用相同契約。引文：「data 是全部 owned bytes」「每交一個 byte，就更新 copied 與 position」。
Q2a（機制）：`as_bytes()` 借出 UTF-8 `&[u8]`，`to_owned()` 以已讀 5.5 ToOwned 把這份 slice 複製成 `Vec<u8>`；只複製 reference 不取得資料所有權。引文：「建立 Vec<u8>，複製 byte 值供 ShortReader 擁有」。這不需要第 8 章的新增背景。
Q2b（需要）：真實 File 可能碰巧填滿 buffer，難以檢驗短讀取；固定來源限制每批最多 3 bytes，使四格 buffer 必然經歷短批次，且來源真正擁有測試資料。引文：「就不能由此確認我們有處理短讀取」。
Q3：最短已教推論：ABCDE 共 5 bytes，先交 3、再交剩下 2、最後無資料回 0；read_all 保留每批有效前段，所得文字仍為 ABCDE。中斷來源先空 buffer 檢查，再按 step 給 Interrupted、R、PermissionDenied；step=3 表示三次事件，不是三個 bytes。
Q4：兩份範例分別以長度／文字 assert 與 error.kind／reader.step assert 對應成功與錯誤保留；「先前取得的 R 不能把後續失敗變成完整成功」明確界定部分資料不是完整成功。現有正文足以讀出驗收目的，命令與預期結果僅作文字核對，未實跑。

局部關係／未知：原資料借用→複製為 owned bytes→ShortReader 持有→Read 分批輸出→read_all 成功或保留非 Interrupted 錯誤，鏈條完整。ToOwned 名稱來源與 prelude 可用性都有句子，沒有必要新增猜測；本段沒有原位圖。未新增 necessary 或 optional 問題，不改原初判。

原始SHA-256：ab2d0e5e40d8c02a9c649fdb2da7f4ff5afee2a6a69bc2fdab6511a048c56eae；visible SHA-256：ab2d0e5e40d8c02a9c649fdb2da7f4ff5afee2a6a69bc2fdab6511a048c56eae。

### 008-002 — 受測 main 延續輸出完成條件

局部重讀對照：原 tail v10 此單位 SHA 為 `df2d18c554b445c427c315fe8ac8ae25e44f337484b8f3f64ad5d597312d2a28`；本輪只核本段受測介面／main，不重開 7.8 後續測試或 package source。

Q1：這個 package 用 path dependency 重用 parse_filename，自己的 main 負責讀檔、輸出與狀態；測試目標是本 package binary。引文：「不是拿另一個 package 的執行檔路徑猜測」。tempfile 僅 dev-dependencies，依已讀 7.1 的測試依賴規則理解。
Q2a（機制）：main 先判 write_all，再判同一 stdout 的 flush；只有 flush 的 Ok 分支回 SUCCESS，兩種寫出錯誤都經 stderr 診斷回 FAILURE。引文：「write_all 後還要檢查 flush，再選成功狀態」。所需輸出完成機制已在本人實讀的 7.5 v11 `005-006` 明教，無須新增 API 猜測或第 8 章背景。
Q2b（需要）：CLI 的狀態要代表輸出完成，因此不能在 write_all 通過時就回成功；受測 main 加入這個最後判斷，讓讀檔成功但完成輸出失敗也有失敗狀態。四種輸入表仍區分正常內容、讀取錯誤及呼叫方式錯誤。
Q3：parse_filename 失敗走狀態 2；read_to_string 失敗走 1；讀取成功後才 write_all、flush，兩者皆成功才 0。report_error 自身寫 stderr 失敗也回 FAILURE。引用：「無法完成筆記輸出」與 `Ok(()) => ExitCode::SUCCESS` 可逐分支核對。
Q4：正文表的 `Rust\n中\n`、未建檔、byte 255、無檔名給出可對照的 stdout／stderr／status；可見 main 的分支與這些預期相合。此局部未呈現測試碼，也未實測，不能據此聲稱 fixture 已涵蓋 flush 故障。

局部關係／未知：已教的 stdout 完成條件已落到本 package 的受測 main，從來源與結果分支可讀出責任。此輪沒有重讀 v13 的 7.5；只沿用實際已讀 v11 的局部說明，未宣稱整章 v13 均已核對。未新增 necessary 或 optional 問題；原 O1 分級保持原樣。

原始SHA-256：b546d0c28c4f43529d5316e02a058427857b7518b08685858e12b199cdcc5db0；visible SHA-256：b546d0c28c4f43529d5316e02a058427857b7518b08685858e12b199cdcc5db0。

### 009-004 — 搜尋保留原文與 Rust 字串表示

局部重讀對照：原 tail v10 此單位 SHA 為 `a8bcebe9092c5cbb12cf3e4a25c123da26493bf84436a39024f5d18bf8072e27`。本輪只核可見搜尋正文及測試，不取完整 source 或實際測試結果。

Q1：search 將驗證、比較與 owned Report 的建立串接；比較用的新 String 與保存原 line 的 String 角色不同。引文：「這份新 String 用於判斷，符合時卻保存原 line 的 to_owned」。contains、ASCII 轉換與 enumerate 的接收／結果都在本段表格，沿用原先實讀即可，未借第 8 章補前置。
Q2a（機制）：contains 查連續片段，ASCII 大寫轉小寫而其餘保持；enumerate 接手 iterator，交出從 0 計數的 `(usize, &str)`，index+1 才是原行號。Rust literal 的 `\"` 代表內容的一個雙引號，`\n` 是 LF；引文：「這是 Rust source 的表示；稍後 Serde 會另將這份資料編成 JSON 表示」。
Q2b（需要）：為忽略 ASCII 大小寫而改比較表示，同時保留輸出原拼寫；為讓測試中的引號／換行與後續 JSON 清楚對接，先分清 source 寫法和實際資料。只改 query 無法消除行內容的大小寫差別，片段規則也解釋 Trust 為何可命中 rust。
Q3：先 reject 空 query，再建立一次比較 query，逐行建立 candidate；命中時存原 line 及全部原行的 index+1，最後 owned Report 包含原 query。最短已教推論：複製內容成 String 後，局部輸入 String 結束不使 Report 失效；此依據是已讀所有權與 ToOwned，不是第 8 章額外規則。
Q4：五個測試逐一對應大小寫差異、Ä 不轉成 ä、空行／CRLF 行號、輸入結束後保留帶引號原文、空 query 即使 contents 空也失敗。`Rust \"筆記\"\n` 的解說足以讀出 assert 中的真正內容；Cargo 的名稱選測指令僅核可讀目的，未執行。

局部關係／未知：原文字→比較複本→片段判斷→owned 原文結果→另一階段 JSON 表示，界線已明寫；不必猜 literal 跳脫是否直接就是 JSON。這一單位沒有圖，未新增 necessary／optional 問題，不更改原 initial。

原始SHA-256：e33feba62fdf41cab62e57626b26d057379de4d0309aeda7ff57bed94fcd23de；visible SHA-256：e33feba62fdf41cab62e57626b26d057379de4d0309aeda7ff57bed94fcd23de。

### 009-005 — library 寫出與 caller flush 的分工

局部重讀對照：原 tail v10 此單位 SHA 為 `32e4b76529a24b921060893875f03176bd69b0e8f590a71d0f0cde84e210c950`。本輪不取 render 的完整 tests 或任何 archive。

Q1：render 借 Report 產生文字／JSON String，read_report 先驗 query 再讀檔並回 owned Report，write_report 借 caller 的 `W: Write`，run 串接兩者；main 的政策仍在外層。引文：「排版不需啟動 process，讀寫入口則保存各自的失敗」。
Q2a（機制）：String::push 接 char 追加 UTF-8 bytes，故 `'\n'` 追加一個 LF；write_all 完成將 bytes 交給 writer，write_report 不替 caller flush。引文：「若 writer 有 buffer，由仍持有它的 caller 在需要確認送出時處理 flush」。這條界線由已讀 7.5 v11 的 Write／Stdout 說明即可理解，不需第 8 章背景。
Q2b（需要）：同一 library 函式要能用 Stdout 或可控制 writer，保持讀寫失敗及輸出表示分離；明定 caller 的 flush 責任，避免把 `run` 的 Ok 誤讀為有 buffer 的出口已完成送出。空 query 先驗證則保留「用法錯誤優先於讀檔失敗」的既有工具需求。
Q3：JSON 先序列化再加 LF，文字逐筆 format!／push_str；空 matches 的文字為空而 JSON 仍有欄位。read_report 驗 query→read_to_string→search；write_report render→write_all，對讀取、編碼、寫出各回相應 AppError，run 不自行輸出診斷。
Q4：可見函式簽名與逐分支程式直接支持資料／錯誤的分工；「先檢查 query，再讀取」也與程式順序相符。正文提到兩項 render tests 的目標，未在本單位展開，我只核說明與本段程式，不聲稱看過那些測試。

局部關係／未知：run 成功只完成本段 library 契約，caller 仍需完成自己的 buffer 出口；本段尚未展開 main，我先保留下一個指定單位核實 caller 行為，並非猜其已執行。此分工本身足夠，未新增 necessary／optional，也不回填原 O1。

原始SHA-256：01c272b76857ebe6e65336d7855fd40575a3f968340261ec81b075c0252d32e7；visible SHA-256：01c272b76857ebe6e65336d7855fd40575a3f968340261ec81b075c0252d32e7。

### 009-006 — main 完成 flush 與原位邊界圖

局部重讀對照：原 tail v10 此單位 SHA 為 `f75dc8c8f145eb9f4029486a603ac144b259a8accd747e3e0893d7449fa16116`。已先存 009-005 的責任界線，才開本段與原位 v13 圖。

Q1：parser 的 help／version 正常顯示請求在讀檔前完成；其他成功解析成 Settings 才進 run。library 借 main 的 Stdout 寫資料，main 持有出口並決定診斷／狀態。引文：「library 交付 writer 與真正 CLI 確認出口完成，各自有明確責任」。
Q2a（機制）：run 的 Ok 後還判 output.flush；flush 的 Ok 才 SUCCESS，Err(cause) 包成已有 AppError::Output，再 to_string／report_error。引文：「這一步也成功，才回正常狀態」。flush 與既有錯誤 variant／From／Display 的用法都有本人已读第 7 章背景，未借第 8 章新增前置。
Q2b（需要）：stdout 要只有完整搜尋資料，供下一個程式解析 JSON；若送出失敗，不能回正常搜尋 0。caller 的 flush 判斷填上上一單位明確留下的完成責任，且把失敗原因交 stderr，資料與診斷仍分流。
Q3：解析顯示請求先 print，成功且 code=0 才正常；其他用法為 2，顯示失敗為 FAILURE。run 失敗與 flush 失敗各保留 AppError 的診斷／status；run、flush 皆成功為 0。正文的缺檔 1、缺 positional 2、help／version 0 與分支角色相符，未實際執行命令或重導向。
Q4（原位圖）：我在正文圖 hook 後才讀同 v13 SVG 並以 Inkscape 原生尺寸渲染，逐關係核對：實線 args_os／Parser→owned Settings→read_report 的 UTF-8／owned Report→render 借 Report／String→write_all／main flush；右側三條虛線由讀取、render、write／flush 階段交具體 AppError 回 main，接 stderr／ExitCode。下方正常搜尋 0、作業 1、用法／空 query 2 與 parser help／version 0 分清；圖無新增未解關係，文字與箭頭可辨。這是單圖原生渲染，沒有看正式整頁或手機版。

圖 SHA-256：`4acde7637d99c851a64afbe5ac53309fd71a6da4d8a18939e7d78371013517e7`；原生 PNG SHA-256：`536492150f94f291ac0cc2b66eeb9c21dcfa41a42124bbe2b684d714aa1b7ca8`。原 tail v10 同圖 SHA 為 `a5473c1a270b6fd203a78fd3e6c264de247c3eb192308f1ab163cb9d97230df7`，本輪沒有重開旧圖。

局部關係／未知：009-005 先限定 write_report 不 flush，009-006 main 接下 flush 並判錯誤，正文、程式與更新圖形成同一責任鏈。本次無必要猜測，未新增 necessary／optional。原 O1 與初判不回填；此結果只是有提示、指定範圍的修後核對。

原始SHA-256：8d2e3a2a69767df060ea3a9fc6006b1d6ce3cc5780d8b3657316cd55886da358；visible SHA-256：8d2e3a2a69767df060ea3a9fc6006b1d6ce3cc5780d8b3657316cd55886da358。

## 局部核對封存

五個指定單位已依序保存，再開下一個；各自原始／visible hash 皆與當次 ledger 相合。這五個單位 closed fold 計數皆 0，輸出仍全程先經過濾器；沒有因本輪而開任何完整答案。只看 009-006 的同 v13 原位 SVG 單圖。

本輪沒有新增 necessary 或 optional。可見文字已交代 borrowed slice 取得 owned bytes 的方式、Rust source 跳脫與實際字串／JSON 的區別，以及 write_report 交付 writer、main 在 run 成功後 flush 的責任；受測 main、完整 CLI main 與圖對應同一完成條件。所有理解所需前置皆在本人實讀第 1–7 章正文，沒有依第 8 章新知彌補第 7 章前提。第 7 章原 O1 的初判與分級不變，v11 局部 followup 也不改。

通過範圍限指定五個 BODY 單位及原位單圖的可讀關係，未核其他 v13 正文／source／tests／archive，未執行 CLI、錯誤探測或正式頁面的桌面／手機版檢查。本輪亦未重開 v13 的 7.5 `005-006`，只沿用實際已讀 v11 局部說明。不擴讀第 9 章，封存後停下等新的 actual gate。

封存前 hash 核對：既有 15 個指定 record／sidecar 的 SHA 與已封存值相同；第 8 章既有 sidecar 的 88 個檔案 hash 亦相同。本輪只有新局部紀錄、新獨立 gate／ledger／圖渲染與 helper 的新 alias，沒有修改上述原紀錄或 frozen 教材。
