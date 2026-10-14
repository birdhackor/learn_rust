# 第 7 章技術交叉與修後局部覆核

## 版本、原始紀錄與本輪範圍

本輪在我的 TECH original 初判及同行初判均封存後才開始；已讀 `ch07-first-reading-v1.md`、`ch07-first-reading-tail-v1.md`、`ch07-continuity-v10.md`、`ch07-disposition.md`。不是新的 blind 或獨立初判，不回寫原始發現／分級。沒有讀第 8 章首讀結果，也沒有開始第 12.4 節。

| 輸入／紀錄 | SHA-256／狀態 |
| --- | --- |
| original v10 manifest | `98c33ca8e367f9c46c8ac2d5c60e201f7fb2f2db35db1c1664798f241a0b6472`，未變 |
| `planning/reviews/ch07-technical-v10.md` 與 work initial 副本 | 兩者都是 `a55a6e33ffb9ec818ebff33beeb95afe6d6394e11e0ea5f6ca2db7c895373b32`，未變 |
| 唯一修後 actual：`frozen/ch07-review-fixed-v13/manifest.json` | `87fec0581beb0f2398a81061060c0e47f5d83dfe6cf54b193e9461b38169cfef` |

frozen 與 work 皆位於 `/workspace/work/tutorial-implementation/`。我重新核對 v13 的 62 個 chunk、66 份 source 與 4 個 asset 副本，實際 bytes 均吻合 manifest。相較 v10，新增 9 份靜態材料；只改 3 份 main 與 search-boundary.svg，其餘 archived source bytes 不變。

語意變更的 actual 單位是 `004-005`、`005-006`、`008-002`、`009-004`、`009-005`、`009-006`，全部完整親讀；另親讀 `004-006`、`007-006`、`009-007` 的相鄰關係、3 份改動 main、新版 SVG source，以及全部 12 份靜態材料。其餘圖的 frozen 路徑換版不算新內容。這輪沒有重新讀全 62 單位來宣稱新首讀通過。

依 clear-tutorial 的同一讀者、先備和證據標準覆核；新的 code 只使用原先已教的 flush、match、unit、AppError／Display／ToString／status 與 ExitCode，沒有增加未知 helper 或 library。

## 同行交叉：獨立重現、漏查與分級

FIRST prefix 的原路線為 v9 的 34 單位，tail 為 original v10 的 28 單位；TECH／CONT 核 original v10 全章。critical `005-006` 的原 SHA 與實作一致，足以比較出口問題；不把不同閱讀路線默默改寫成同一輪。

| 關係 | 原始來源與我的發現狀態 | 交叉裁定 |
| --- | --- | --- |
| Stdout write_all／flush 與成功狀態 | FIRST prefix O1 可選，tail 保留；TECH N1、CONT N1 在互看前各自以 original closed-pipe 實驗提出必要，涵蓋 7.5 與重用的 7.8。root 的 /dev/full 證據是另一次後續查覈 | 同意必要。FIRST 已指出完成界線的最小關係，原可選分級仍保留；實際輸出失敗卻回成功的技術證據來自後續 root 及獨立 TECH／CONT，不能回算成 FIRST 已自行實證。 |
| ToOwned 的名稱來源一致性 | 我的 original I1 可選；已有方法機制，7.4 明寫 use、後面省略。其他原初判沒有報成同一項 | 同意可選採納。prelude 一句回答省 use 的來源，沒有重新教 owned conversion；也不把前文同一方法已用過說成 7.9 新 API 未教。 |
| Rust literal 的雙引號跳脫與 JSON 表示的橋接 | CONT N2 必要，低嚴重度；**我的 original TECH 未抓到**。FIRST tail 在 `009-004` 以基本 Python 與 JSON 熟悉度判可短推，沒有報缺口 | 本輪受同行提示後同意必要。相同可見符號不自動證明 Rust source 的選用規則已教；專案要求 tests 亦先教後用。只需把這個最小跨表示關係說清楚，不否定原來程式合法、搜尋機制可懂或 Python 經驗可能有幫助。原 TECH 與 FIRST 紀錄原樣保留，這次不補算我的 initial 獨立發現。 |
| generic writer 的成功與 caller flush 責任 | CONT O1 可選；FIRST tail 的舊 O1 延續出口完成未知。我的 original 未提出此 library 契約補充 | 同意可選。write_report 只借 W，原句並未承諾替任意 W flush；明訂接受 bytes 與 caller 推送的分工有助理解。7.9 main 增加 checked flush 是配合明確責任的一致性改進，**不是把 7.5 無 LF 反例轉算成已實證的 7.9 故障**。 |
| 缺 9 份靜態材料與未核 root lock | TECH E1、CONT U1 分別在 original 提出；CONT U2 與我的隔離 lock 範圍限制相容 | original 限制保留。新的 12 份 actual 材料可以解除 v13 的材料未知，不能回寫成 v10 已看／已跑；root candidate lock 和完整 release 檢查由對應紀錄支持。 |

N2 與 generic writer O1 的交叉同意皆受同行提示，不列為我 original 的新發現。必要／可選各項的最小修正與新增負擔，和 root disposition 的處置一致；沒有未回應的已報項目。

## v13 的局部修後結果

- **`004-005`：I1 完成。** 在 byte slice 的 ToOwned 說明處補 prelude 來源與 explicit use 的用途，位置早於後續省略 use 的 search。原 reader／ownership／3、2、0 的操作完全不變；不需要重跑這個未改 source。
- **`005-006`、`008-002`：N1 關閉。** Stdout 緩衝與不保證末尾 LF 的說明位於第一份出口 code 前，7.8 明確沿用此完成條件。兩份 main 都先檢查 write_all；Err 立即回 FAILURE，Ok 再檢查 flush，兩步都成功纔回 SUCCESS。回報內容沒有擅加 LF，flush 失敗也不是永久儲存／撤回已寫 bytes 的保證。Stderr 不緩衝的補充和官方 `std::io::stderr` 契約相符。
- **`009-004`：同行 N2 關閉。** 第一份 search tests 前，明說 Rust quoted literal 中的反斜線接雙引號產生內容雙引號，原 LF 規則仍沿用；接著分開 source 表示與 Serde 的 JSON 編碼。沒有把 JSON 規則或 Python 熟悉度當成省略教學的證據，也沒有為一句橋接增加整套跳脫章節。Rust source 本身未改。
- **`009-005`、`009-006`：同行 O1 完成。** library 的成功只表示將 bytes 交給 W，caller 仍持有 writer，需確認送出時自行 flush。main 是 Stdout 的 caller，run 成功後 checked flush；失敗由已教 AppError::Output 保存原 io 原因，既有 Display／status 給診斷及 1。run 的 Err 保留原原因與狀態，沒有被 flush 覆蓋；help／version 仍停在 parser 顯示路徑。
- **新版 search-boundary source：相應關係已更新。** `write_all → main flush` 與正文／main 相符，AppError 虛線仍接回診斷／ExitCode；沒有把 library helper 說成自行 flush。圖 source SHA 為 `4acde7637d99c851a64afbe5ac53309fd71a6da4d8a18939e7d78371013517e7`。本輪沒有親看其實際頁面，視覺結論另記未驗。

以上局部沒有發現新問題。沒有改教材或 shared 設定，也沒有把 source 正確當作首次閱讀者重新通過的證據。

## 補材料：v13 的 E1 覆核

12 份材料這次才全部親讀 rawbytes 並核 SHA；9 份補入材料不是我在 original 已見過的內容。

| actual 材料 | 本輪核對的資料／限定 |
| --- | --- |
| io-text note、empty、not-utf8 | note 為9 bytes且末尾LF，empty 真正0 bytes，非法檔真為單一 `ff`。原 missing.txt 仍刻意不存在，沒有為消除 NotFound 而建立它。 |
| notebook 基準與 broken-syntax | 基準三欄位符合模型；語法材料只在 values 的最後元素後多一個不合法逗號，確實針對 JSON 語法。 |
| missing-title／unknown-field | 前者缺必要 title；後者保留正確三欄位，另加拼錯的 archvied，針對 deny_unknown_fields。沒有把名稱同義與模型欄位自動匹配。 |
| wrong-value-type／number-out-of-range | 前者第二個值是文字 "120"；後者是2147483648，超出 i32。兩者與該欄位契約不同的失敗原因相符。 |
| quoted-title／JSON not-utf8 | title 真有 JSON 表示的 LF 與內容雙引號；另一檔真為單一 `ff`。前者可往返欄位資料，後者在 fs 文字讀取已成 Io／InvalidData，尚未進 JSON decode。 |
| search note | 原四行52 bytes與指定 Rust／rust／中文內容、行號及末尾LF相符；未改原規則。 |

v13 下原受限的 I/O／JSON tests 已能實際執行，結果列下；E1 對**這版材料完整性**已關閉，v10 的原失敗與未驗記錄不改。

## 自己執行的範圍與未報通過項覆核

隔離 copy 只用 v13 archived source／材料，共同 metadata 沿 actual1 已教欄位；自己 generate-lockfile --offline，自己指定 Rust 1.99與 target 路徑。全部已複製 source 再核 SHA，沒有混入 live source。獨立 lock 為 clap 4.6.7、serde 1.0.229、serde_json 1.0.154、tempfile 3.27.0；以下 --locked 指**自己的隔離 lock**，不是 root candidate。

| 本輪實際執行 | 結果／支持範圍 |
| --- | --- |
| `cargo test -p ch07-io-text --all-targets --locked --offline` | 3 tests 通過，包括 original 曾因材料缺失失敗的 NotFound／InvalidData 分類與9／19新增量。 |
| `cargo test -p ch07-json --locked --offline` | 4 tests 通過；包含5種拒絕材料、quoted-title 往返、Io／Json 階段分類，以及原先已選的 owned／round-trip test。 |
| clap 的 `help_and_version_are_successful_display_requests` selected test | 1 test 通過；仍支持正常顯示請求的 stdout／0，而不是全4 parser tests的新執行。 |
| search 的 writer target、CLI 的 `json_stdout_is_only_json_and_preserves_quote_data` selected test | 各1 test 通過；前者驗 write 的 Output 原因與 called，**不是新增 flush 分支的直接注入測試**；後者真 child 的整份stdout可讀為合法Report。 |
| `cargo run -p ch07-io-text --example text_io_solution --locked --offline` | 原答案真輸出總長9，第二份非法文字沒有交回部分成功長度；使用 actual 的 empty／not-utf8。 |
| build 修後 path-cli 與 fixtures，兩個 binary 各用無 LF 的 Rust 材料正常捕捉／closed pipe | 正常都是0、stdout exact `52757374`、stderr空；啟動前已關read端的pipe都是1，診斷為「無法完成筆記輸出：Broken pipe」。這是自己的4個執行結果，直接支持兩份 checked flush 修正。 |

合計 **10 個 Rust tests 通過**，前輪 selected 4 全涵蓋，其中 JSON owned test 已包含在4項JSON執行內，沒有重複再跑或計為11項。另1個答案執行、4個修後 binary 對照；沒有跑全 41 tests／23 executable targets／全站check或127 個全開發fixtures。root 的全開發 fixture 與 release 證據不轉算為我的執行。

除了裁定缺口，也覆核同行未報問題的方法支持：

- **完整文字與新增量：** 7.3 要追加前綴、保存原因；9 bytes不是最後19 bytes的長度。原 helper／actual材料與3項tests同時支持該角色和計量界線，不從輸出倒定需求。
- **owned JSON 與往返：** 7.7 的需求是保全部欄位、回收模型及拒絕不合政策的輸入，不恢復原空白。Raw fixture 的變化有針對性，decode／encode／wrapper及4tests支持這個限定；模型離開原String仍有效也保留。
- **輸出與讀取的不同錯誤：** RefusingWriter 的非空內容使 write_all 真呼叫，沒有借真權限偶然失敗；新caller責任沒有把這個既有test說成已測flush。AppError::Output 與 Read 的 source／觀察仍一致。
- **資料與診斷分流：** 7.9 selected child 在模型／字串literal的新橋接後，整份stdout解為Report，核query／選擇／原行號／引號文字與空stderr。不是隻看main短或某段JSON子字串；新flush不產生額外stdout標題或換行。

上述是本輪直接的材料／source／局部執行覆核，未用同行的「已足夠」四字代替判斷，也沒有用新設情境替正文未教的關係補理由。

## 呈現與保存界線

本輪未做桌面／手機實頁、DOM、PNG或新版SVG視覺檢查。CONT 的原稿獨立 preview 與 FIRST 的局部SVG可以作為同行記錄理解，不能記成我親看的結果；新版SVG已改，也不能由舊圖的可讀性直接通過。

正式最小疑點另查 `std::io::stderr` 不緩衝，以及 Rust Reference 的 string literal／quote escape；沒有把其他 escape、raw string或標準出口的跨平臺特殊情境塞入學生程式。

本輪 source fingerprints、隔離命令／stdout／stderr／code、修後closed-pipe結果與lock版本保存在 `work/tutorial-implementation/technical-ch07-v13/`。original report、initial副本與SHA均保持原樣；本文件另存 cross 時點的 prompted N2／O1與局部 closure。完成後停止，等待後續授權與 actual 10.8；沒有先讀first 8或寫新unsafe正文。
