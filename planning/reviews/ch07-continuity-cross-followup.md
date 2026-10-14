# 第 7 章銜接交叉與局部修後覆核

## 本輪身分、來源與界線

日期：2026-10-14；審閱者為 `chapter02_ownership`，第 7 章非作者。這是 initial 封存後、已取得同行發現與 root 修法的交叉／修後覆核，**不是新的 blind 首讀，也不是新的獨立重現**。

原 `ch07-continuity-v10.md`、SHA sidecar 及 sealed 副本保持不變，SHA256 仍為 `65b13249772da61cd8f74cafd46b53952869ed41844293be2238f4dc144a8b06`。先前使用過第 7 章先備的背景仍保留，不以這次 cross 抹去。

本輪已完整讀指定四份紀錄：`ch07-first-reading-v1.md`、`ch07-first-reading-tail-v1.md`、`ch07-technical-v10.md`、`ch07-disposition.md`。第一次大批輸出有截斷，之後分段回讀至全文完整，不把被截斷內容算已讀；未開未授權的第 8 章正文／首讀答案，也未修改教材、shared Cargo／lock／nav。

修後唯一來源為 `/workspace/work/tutorial-implementation/frozen/ch07-review-fixed-v13`：9 頁、62 BODY、66 個 archived sources（v10 的 57 項加 9 份原靜態材料）。manifest SHA256：`87fec0581beb0f2398a81061060c0e47f5d83dfe6cf54b193e9461b38169cfef`。

機器比對全部 manifest 映射、BODY／source 指紋，排除 frozen asset 絕對路徑的版本改名後，實際改動只有六個 BODY：`004-005`、`005-006`、`008-002`、`009-004`、`009-005`、`009-006`。來源改動為三個 main、一張 SVG、九個新增靜態輸入；library／tests／答案／Cargo 均未改。六個 BODY 全文實讀，另回收 `004-006` 的 flush 契約及 `009-003` 的 AppError／status。沒有把本次機器比較全62個單位誤稱全章新首讀。

所有讀入指紋、差異、新材料 bytes、實際執行及畫面存於 `work/ch07-continuity-cross-followup/`；下列通過只涵蓋此次局部改動與明列相鄰關係。

## 各角色初判交叉

| 原問題／角色 | 獨立性、分級差異及本輪處理 |
| --- | --- |
| Stdout buffer／成功狀態：prefix first O1、technical N1、continuity N1 | 三者在互看前均提出相同的缺少關係：7.4 已教 flush，首次 Stdout 沒說 buffer 和完成位置。first 原判可選，沒有實測；technical 與我各自用 frozen 原程式、啟動前已關閉讀端的 pipe 重現短無 LF 漏報，屬兩份獨立技術重現。root 在 first 封存後用 /dev/full 確認必要性，不回算成 first 已自行證明技術錯誤。7.8 複用 main 的同缺口，technical 與我亦在 initial 各自明列，不計成另一種算法問題。 |
| Rust literal 雙引號：continuity N2 | 我的 initial 獨立新發現。tail first `009-004` 明記可按基本 Python escape 和已讀 JSON 作理解，故非單純漏看，而是先備判準／分級差異；technical 未報此語法橋接。能自行讀懂保留，但 JSON 格式的 escape 不能證明 Rust source 已教；依使用者嚴格「tests 也先教後用」採納最小一句合適，不必宣稱主線無法理解。root 本輪任務明確採納，v13 實稿已有該句。 |
| generic writer 完成責任：continuity O1 | 我的 initial 明分「library 已交給 W」和「caller 處理 flush」。tail first 在 `009-005` 仍保留其舊標準出口 O1；technical N1 也限制 7.9 的實證範圍、建議整理 main 完成契約。屬同方向的支持，不能把它們自動算成獨立提出了完整的 library／caller 分工。root 採納 library 只接受 bytes、caller flush，並補 main checked flush；這是可選契約清楚後的一致性改進，沒有把原 LF render 改判為已重現 no-LF 故障。 |
| ToOwned 名稱來源：technical I1 | 同行獨立可選新發現；我 initial 未列為問題，只確認 ToOwned 的 owned／borrowed 意義。這是我的可選名稱來源漏查，不是偷偷使用新 conversion。first 亦未報。v13 在 7.4 補 prelude／explicit use 關係，回應後文為何可不重複 use，收益明確且只一短句。 |
| 靜態材料缺封存：technical E1、continuity U1 | 我的 initial 已記不可從 v10 核全部 raw fixtures，unknown，未因此指控 live 缺檔。technical 另完整列九份，並實跑 archive 單項 test，顯示實際 NotFound 與預期 InvalidData 不符；這是同行較強的 bundle 證據，不是我的獨立執行。root 補新的 v12／v13 bundle，original v10 不變。本輪現在實查九份 rawbytes 與相應測試，只解除新版本的這項 unknown，不能回寫 v10 全材料已驗。 |

peer 原問題與分級皆保留；未報的新補句不事後塞入他人的 initial。初讀者只 native 看圖、technical 只核 SVG source，我 initial 的 frozen preview 視覺與本輪 actual 頁面亦各記自己的實看範圍，不彼此代答。

## 六處修正與相鄰先備

### 7.4／004-005：ToOwned 名稱從哪裡來

新句「ToOwned 也由 prelude 提供名稱，本例仍明寫 `use std::borrow::ToOwned;` 方便對照來源，後面可直接使用這個已教的 method」位於建立 ShortReader 資料之前。

**機制／性質：已足夠。** method 的 byte slice → owned Vec<u8> 契約保留；新句只交代名稱 scope。**需要／用途：已足夠。** 後面的 7.9 search 沒有 explicit use，學生現在可追到名字來源，不會誤以為不同 method 或漏掉必要 import。此處不需展開 prelude 的完整清單，名稱與 method 角色已足夠；沒有新增 trait 介紹／Rust source／tests，無需為名稱句重跑 short_reader。

### 7.5／005-006 與 7.8／008-002：成功包含 checked flush

7.5 新正文在第一份 main 之前說 Stdout 可能共用 buffer、原資料未必最後有 LF、兩步成功才選成功、cleanup 不能代 main 回報錯誤；保留 terminal 的換行推送限定及「非永久儲存」界線。新 Stderr 不帶這份緩衝的說法，另實讀官方 `std::io::stderr` 文件確認「This handle is not buffered」，不由 Stdout 類比猜得。

三項關係各有用法依據：7.4 `004-006` 已教 `flush() -> io::Result<()>`；7.5 的 mutable output／Write 仍在 scope；write_all Err 立即返回而不再 flush，Ok 才續第二個 match。flush 的 Err 保留原因、diagnostic／FAILURE，沒有替原資料加 LF。7.8 source 前也明說沿此完成條件，其受測 main 與 7.5 同樣順序。

**機制：已足夠；原需要／用途：已足夠。** 成功依然交原筆記，現在 caller 可從 status 察知緩衝送出失敗。原「空文字合法」政策沒有被補 flush 破壞。本輪六個真正 binary probes 支持下面的具體 Linux 結論，不擴成所有 OS／出口已實測：

| package，各三項 | bytes／出口 | 修後結果 |
| --- | --- | --- |
| ch07-path-cli、ch07-fixtures | `R`，正常擷取 | 0、stdout 精確 `[82]`、stderr 空；沒有額外 LF。 |
| 同上 | `R`，啟動前 pipe 已無 reader | 1，stderr 是「無法完成筆記輸出：Broken pipe…」；指出失敗來自新 checked flush。 |
| 同上 | 空內容，啟動前 pipe 已無 reader | 0、stderr 空；沒有應送的 bytes，不把空成功改成輸出錯誤。 |

這些 probes 是我的修後確認，受 N1／root 修法提示，不計為第三次 original 獨立發現。兩個 package 的 library／參數政策沒改，沒有為已不受影響的全部範例再做一輪裝飾性測試。

### 7.9／009-004：Rust source 表示先於引用資料的 tests

新句在第一組 tests 之前，明說 Rust literal `\"` 產生內容雙引號、`\n` 產生已教 LF，並以實際 `"Rust \"筆記\"\n"` 對應 Rust、空白、帶引號的筆記與 LF。接著指明這是 Rust source 表示，Serde 之後另編 JSON。

**機制／使用：已足夠；用途：已足夠。** 補的是源碼字面值與資料的關係，和 7.7 JSON file escape 各有對象；原正確測試不需修改。對 Python 讀者只是小橋接，未擴充整套 escape／raw string／Unicode literal 語法。actual 手機頁也能看見一個 backslash 與 quote，沒有被 Markdown 消掉；code 前首次位置正確。

### 7.9／009-005：library 只接收，不奪走 caller 的完成政策

正文現在說「本函式完成把 bytes 交給 writer，沒有替它 flush；若 writer 有 buffer，由仍持有它的 caller 在需要確認送出時處理 flush」。`write_report` source 未改，仍 shared 借 report、mutable 借 W、render 完整 String 後 write_all；成功不等於 caller 已確認任何 backend 的 buffer。

**機制：已足夠。** W owner 仍在呼叫者，故可以在 helper 返回後接著 flush，沒有轉移 output。**原需要／用途：已足夠。** 純搜尋／排版與實際出口責任仍分層，可接受記憶體 writer 而不硬規定它何時送出。沒有以變成 generic 為由增加實際工作承諾。

我另外以 reviewer-only 可控制 W 做小探針：write 接受全部資料，flush 會失敗。write_report 回 Ok、保存精確 `1:Rust\n`、沒有呼叫 flush；caller 接著 flush 時取得 io 原因，可明放 AppError::Output／status 1。這證明新 library 說明與程式一致，不是假稱已讓真 Stdout 的所有 flush 失敗在 main 重現。

### 7.9／009-006：main 補足自己的完成條件及圖說

新正文先說明「main 持有 Stdout，沿 7.5 在 run 成功後明確 flush」，source 才執行新分支。AppError::Output／status／Display 的實際 `009-003` 與 error.rs 已教；新增 use 只是 library public 路徑，並無新 trait 或型別。run Err 不被當成可成功 flush 的輸出，run Ok 後 flush Ok 才 SUCCESS；flush Err 用 Output 保存 io 原因、寫 stderr／1。

**機制：已足夠；需要／用途：已足夠。** 7.9 的真 main 是 O1 裡仍持有 W 的 caller，現在把只接受 bytes 與真正資料出口的完成責任接起來。help/version 仍走已教 parser 顯示請求，未進 read/search；EmptyQuery／Read／Encode／Output 的既有政策不改。五項真 child tests 仍驗 stdout、stderr、status 與整份 JSON；這些正常測試不能單獨證明新 flush Err branch 已動態觸發，該 branch 本輪以 source／既有錯誤契約覆核，不作超額實測宣稱。

SVG 的第四框改成「write_all → main flush」，desc／alt 也明說 main 檢查 flush；同框接 output 失敗虛線，與 code 保留 AppError::Output 相符。沒有讓 library 框暗示自行 flush，沒有新增不明資料形狀或錯誤碼。

## 九份原靜態材料與證據解除

本輪逐份讀 rawbytes、核 manifest SHA；九個新 source 均有實體內容，intentional missing.txt／missing.json 仍沒有被錯補。Python JSON 檢視只是讀原材料，不冒充 Rust／Serde 模型驗證：

| 新材料 | 實際內容／當下作用 |
| --- | --- |
| io-text empty.txt | 長度 0，合法 UTF-8 成功空文字。 |
| io-text、json 的兩份 not-utf8.bin | 各長度 1、byte255；失敗在文字讀取階段，不能當 JSON 格式失敗。 |
| broken-syntax.json | UTF-8 合法，values 的 `[80, 120,]` 尾逗號使 JSON 語法不合法。 |
| missing-title.json | JSON 合法，但缺 Notebook 必要 title。 |
| unknown-field.json | 正常三欄之外多 `archvied`，deny_unknown_fields 拒錯字名。 |
| wrong-value-type.json | values 第二項為文字 `"120"`，不是 i32。 |
| number-out-of-range.json | JSON 數字2147483648，超過 i32 最大值；不是 malformed JSON。 |
| quoted-title.json | 原 JSON file 含 LF／quote escape，讀出的標題是 `第一行\n"筆記"`，清單仍80／120、false。 |

`ch07-io-text` 失敗分類單項 test 已真正通過，InvalidData 不再因材料未提供錯得 NotFound。`ch07-json --lib` 四項真正通過，含五種拒絕、quote／LF 往返和 Io／Json 原因分流；九份新材料都被相應內容／契約核過。故 E1／我 U1 對 **v13** 的 raw-material 限制解除。original v10 沒有任何重寫，先前「未驗 rawbytes」仍成立。

## 實際執行、畫面與封存

### 執行範圍

只複製 v13 的 path-cli／fixtures／json／search／io-text 五個成員至本輪隔離 workspace；Cargo 配置採 actual 1.3 的共同欄位，Rust 1.99.0、edition2024。生成自己的 offline lock，SHA256 `720d20c629fd91ecdc02bc1a71dde90acb01d45efb4c18bdeef09f02f0355da2`，沒有讀取或改 shared root lock。以下 build/tests 均 `--locked --offline`、使用自己的 target：

- build 三個受改 main：ch07-path-cli、ch07-fixtures、ch07-search，成功。
- io-text `absent_file_and_invalid_text_keep_distinct_kinds`：1 項通過。
- ch07-json `--lib`：4 項通過。
- ch07-search `--test cli`：5 項通過。
- 六個修後 stdout controlled binary probes 及一個 reviewer-only generic writer 契約 probe：結果如上。不是教材新增 tests，也不列作 original 獨立發現。

全部原始 commands／stdout／stderr／statuses 保存在 `cargo-checks.json`、`stdout-fixed-probes.json`、`writer-contract-result.json`。沒有跑全章41tests／全部 targets、fmt／Clippy；它們仍不在我的本輪完成宣稱內。root／同行過往結果亦不當成自己的執行。

### actual 第7.9頁面

使用 root 更新的 `http://127.0.0.1:8765/chapters/07-09-complete-cli/`，確認 served SVG bytes 精確等於 v13 的 hash `4acde7637d99c851a64afbe5ac53309fd71a6da4d8a18939e7d78371013517e7`，不是 initial 的自組 preview。7.4／7.5／7.8 served HTML 亦含對應新句，僅做內容版本確認，沒有假稱其整頁 visual 重驗。

在1280×1000與390×1000各親看 quote bridge／library writer responsibility／main flush 新段落及更新 SVG，共8張 Playwright screenshot，全部經 view_image 實看。三段皆相鄰於首次程式之前；mobile 的 quote escape 仍看得見，必要正文不需水平捲動；code 沿既有方式可水平捲動，不在這次主張完整 code 無捲動。SVG仍340×650，字與「write_all → main flush」未碰撞、桌面未被放大至938寬；實線資料及 Output 虛線關係可辨。

actual 畫面、DOM bbox、HTML／SVG hash 在 `actual-visual-metadata.json`／`actual-preview-input.json`；本輪沒有點第8章或看未指定的答案，左側導航顯示章名不算閱讀其正文。沒有重新驗其他三張未變 SVG／全站導覽或宣稱首次閱讀路徑已重演。

### 本輪完成判斷

N1／N2 的必要缺口在指定 v13 局部已解除；O1、technical I1 的最小補強有實際理解收益且先備負擔低，採納合適；補完整材料後 E1／U1 的新版本限制也解除。本次受影響的八個 BODY（六改、兩 context）、三 main／library／error／writer source 和新材料，未再發現需要修正的銜接或 output-policy 問題。

initial U2 的跨平台、shared lock 與全套檢查仍未由我驗證；AI銜接／局部修後覆核不等於真人讀者測試或全章發布驗收。原 initial／peer分級未改，這份 followup另封存 SHA及work副本；完成後停止等待 root，不自行擴大第8章審閱。
