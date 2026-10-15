# 第 14 章 TECH14 交叉與最小修法覆核：v10

本輪五項處置皆能回應原問題：兩項本人原列必要問題、CONT 新提示的 assert 訊息橋接，以及兩項各自原列可選補充。四處正文均在首次使用前，兩份相容性 lock 已完整交付。沒有新增未解的必要項；原有 Rust／SVG 全部相同 bytes，沒有重跑未改動的 suite、Miri、bench 或 DHAT。

## 角色、先保存再交叉的順序

本人延續非作者 TECH14 角色，不是新 independent initial；曾撰寫2、4.4–4.5、5、9.2–9.4，既有其他審閱及原14背景保留，不稱 blind。原報告 `ch14-technical-v8.md` 的 SHA **`93cfeddf35415a2e942cf0b6b99de7429244f1fc5c485d13d5959416b0313423`** 與必要 N1／N2、可選 O1 的原分級保持不變。未回寫其他人的原判。

授權後完整閱讀四份 originals：本人 TECH117行、CONT120行、FIRST prefix641行、FIRST tail267行；同行完整 SHA 保存於 `work/ch14-technical-cross-v10/peer-read-proof.json`，分別以 `4939fe3132b7…`、`f75674e841d3…`、`2f9073282082…` 對上指定封存。FIRST prefix 的41 BODY是獨立 prefix 路線；FIRST tail 的41 BODY入口只是該讀者已有背景，尾23 BODY才是其原判範圍，不能算第二份獨立 prefix。

先保存 `pre-root-cross.md` 的分級／理由，再自行 diff 與核唯一修後材料、有限 metadata、實頁；保存本人判斷及 hash 後，才完整讀 `planning/reviews/ch14-root-disposition-v10.md` 作最後比較。不是照 root 處置反填自己的發現。正文擷取超出輸出預算的批次有回讀缺失範圍；沒有以截斷輸出算完整閱讀。

本輪不開第12章同行、root12處置／新版／作者材料，也不把任何12判斷傳給其他角色；只用原14已讀前文。本輪沒有 student／shared Cargo／lock／nav 修改。

## 各原判的相同、差異與新增提示

| 原問題 | 各角色的原判／證據 | 本人交叉判斷與原分級保留 |
| --- | --- | --- |
| expr 首次匹配 | TECH N1、CONT N1各自行列必要。FIRST prefix 在14.2/001-004保存一般 expr 規則未知，只由固定 &distance 和預期 E0603理解 private 主線；001-005補清後仍必要0。 | private 的來源／可見性主線原已成立，但 selector 的匹配契約仍是新能力，嚴格先教後用需首次前一句。本人維持 original 必要；保留 FIRST 的未知→後文釐清，不把它升成當時已教或另一個獨立必要發現。 |
| assert 自訂訊息 | CONT N2自行列必要；TECH original 未列，FIRST prefix 也未報該 argument 的未知。條件 false／Default 0違約的主線皆已懂。 | 這是同行提示後本人回查並採納的必要最低橋接，**不是本人 original 獨立重現**。assert 的第二參數不是另一個 expected 值；已懂 panic 格式與 assert 條件不等於已教此新輸入形式。原 TECH 分級不改。 |
| 兩個相容快照缺 lock | TECH N2、CONT N3各保存原材料 locked101、各自 generate lock後 run0；FIRST prefix只讀 BODY、未查附件或執行，認 same caller概念可理解。 | 維持必要的交付／操作問題。FIRST未驗與兩份獨立 probe 不矛盾；不能拿同caller source相容概念正確來蓋掉命令無法執行，也不把工具準備失敗判成 API相容性錯。 |
| test 選 example 與 main | TECH O1可選；CONT及FIRST prefix未列。已教 example selector、cargo test／測試執行器可推其角色。 | 維持原可選，補句讓責任更直接，不升成新 Cargo／測試機制缺口；收益是讀者能分 run main與test harness的檢查目的。 |
| Just／RngSeed精確入口 | FIRST prefix O1可選；能懂型別／設定用途，但從可見片段不能定位新imports。TECH／CONT完整archive曾看fixture中的use，原未報正文入口。 | 本人交叉後採納的可選改善，不稱 original自行發現。完整source有use不能倒算首讀當段已知道module路徑；補入口讓自行寫檔不需另找名稱，原生成／縮減機制不被重判錯誤。 |
| 尾段未知、工具／測量界線 | FIRST tail23 BODY必要0／可選0，保留CI公式及極大repetitions擴大域未知；TECH／CONT另查source、原資料與有限probe。 | 不將未知升級必要，也不記為所有API通過。讀CI欄位不需自己推估計式，100次固定材料不需任意巨大參數證明；原scope仍有Loom模型、Miri路徑、同工作benchmark與profile資源限制。 |

四角色的通過理由有可追溯的需要／機制，而不只一句「皆通過」。宣告樣板要減少重複，普通函式已處理runtime借讀；derive要保使用者原宣告／可見性並補impl，AST形狀觀察與Rust型別檢查分工；公開API要守原輸入、成功、錯誤、owner及default，不由private或版本號自動擔保相容。property的逐鍵直接掃描回答分錯鍵，總量回答遺失，兩者不以實作當自己oracle；composition也不能代替這兩項。

尾段的 original需同樣有具體材料：每份worker完整增加一次，Loom每輪新建共享狀態、RMW與兩join，不能以native一次2判全部交錯；Miri追所選存取資格，safe wrapper全部前提仍靠source契約。benchmark先固定同答案／材料／計時範圍，profile再問heap配置與來源鏈，兩者不互換CPU比例或CLI倍率；保owned結果有用時也不能悄換成失效borrow。FIRST的孤立SVG支持當段對應，TECH／CONT的實頁與source／raw資料證據各保自己的範圍，不借同行驗證填成本人新實驗。

## 自行核對唯一修後稿與最小收益

唯一 fixed：`/workspace/work/tutorial-implementation/frozen/ch14-review-fixed-v10`；manifest SHA **`67b328ca646947734332a557cdf8c78d46594644b57836597f4f70946dbe74f6`**，八頁64 BODY／136 sources。本人逐檔對原manifest核 hash：**原134 sources全部 exact**，包含全部Rust、九SVG、設定、expected、raw reports／profiles；新增只有 v1與v1_1兩Cargo.lock。64 BODY 有12個raw hash改變，正規化兩snapshot資產根路徑後僅四個material，其他八只是資產URL；不拿root proof代本人。

各 material 與前後各一unit已完整回讀，共12 BODY；affected source也實讀，不只看新增一行：reading_many、hygiene、derived_default、overwritten_count，以及兩版manifest／library與同實體consumer。

| fixed單位 | 首次使用前的最低修法、source與實際收益 |
| --- | --- |
| 001-004 | 「run執行main；test --example reading_many用測試執行器跑單元測試，不執行原main」在首對命令前。source的main借讀80／30／500，cfg(test)另驗0／30／−5及保wrapper，兩個target責任對上。原重複宣告／借讀需求不改，O1仍可選。 |
| 002-004 | expr一句在private_reading pattern之前，回收1.5 expression與已教shared borrow，明指 &distance輸入。能在知道匹配後才把E0603定位private權限；相鄰002-003裸Reading與002-005mixed-site／once／order分工不變。無新API、無需搬整份fragment表。 |
| 004-003 | assert第二形式／false時panic訊息在負例前，對上source與.expected所選非零規則訊息；Default 0違約／手動100有效的原資料與理由不變。本人另核Std1.99 [assert Custom Messages](https://doc.rust-lang.org/1.99.0/std/macro.assert.html)，保存真正選段：自訂panic訊息與格式參數只在failure時求值。未執行已知未改的panic例。 |
| 005-005 | 明列 `proptest::strategy::Just`／`proptest::test_runner::RngSeed`與兩條use，在Just構造／Fixed設定前；完整fixture亦使用相同兩條。型別Clone／Debug、固定seed／None保存器、隔離負例仍原樣；只補讀者能寫入檔案的名稱來源，FIRST O1仍可選。 |

兩個新增locks各只有本地 `api-contract-demo` 的1.0.0／1.1.0，與各manifest一致，沒有新第三方依賴。兩份獨立空workspace、共用 `../consumer.rs` 路徑與同caller default false assertion均核現有source；v1_1新增private limit100不改已承諾false。外層root lock不能取代這兩份lock；版號仍只是此演示label，SemVer敘述與有限caller的範圍沒有擴大。

在本人七份相容性材料副本上各執行一次 **`cargo metadata --locked --offline --no-deps --format-version 1`**，兩次exit0，package與targets對上1.0.0／1.1.0／same consumer，兩lock before／after SHA相同。**本輪沒有generate-lockfile，也沒有重新run或test**；原TECH自行generate的診斷對照與這次「現成交付lock可解析」是不同證據。紀錄在 compatibility-metadata-results.json，不能把metadata0宣稱新run成功。

## 本人實頁、圖像與source範圍

同 v10 64 BODY依序重組本人的same-theme preview，只去重H1／替換本機資產URL；explicit extra_css沿原TECH14已授權取得的本人public CSS副本，未開live14或借root畫面。Zensical build exit0／No issues found。固定預覽 `127.0.0.1:8791`；HTML新增文字逐項核源自相應fixed unit，unit SHA、config／rendered SHA、實際served CSS與SVG byte證據保存在work。

本人在1280×1200、390×1200各親看四處新段落／相鄰code或命令共8份截圖；都在首次使用前可讀，換行不遮語意。另親看受影響property頁的原位手工shrinking圖兩份：4／2筆仍失敗，1筆通過、保留2筆反例，與前後正文source一致且字可直接讀。十張均實際view_image，不由bbox、抓取text或root證明代視覺。

原TECH14九圖與兩表的22份親看仍保原scope；本輪hash證九SVG同byte、CSS同byte，沒有全重看無變圖，也不把先前22份當本輪新增十份。此次新property圖兼核正文插入後的原位關係；其餘圖沿原已驗物件與原條件保留。未全站驗dark mode、所有code水平捲動、所有fold／link／downloads／發布頁；沒有因少量local畫面宣稱整章正式發布驗收。

## 最後與 Root disposition 比較、未驗與封存

本人cross判斷與material／metadata／實頁已先保存，才讀root-v10處置。其採用的五項及必要／可選差別與上述核對一致；root記「既有locks加入archive／發布allowlist」中的archive已由本人byte及metadata核，**發布allowlist未另開，不算本人驗證**。該文件敘述v9漏profiles後未交付審閱者；本人沒有開v9或用它作任何通過證據，唯一fixed始終v10。

沒有新的Rust變更，故本輪未重跑原16命令、normal／negative集合、Loom／Miri／property／consumer、bench、DHAT或全workspace／Clippy。原工具版本、own-lock、raw reports、精確數字與Miri只解譯UB的原驗證／未驗限制保持，不由root候選檢查補成自己執行。兩metadata只驗這兩lock對現有獨立manifest的離線解析；不涉及跨平台、所有SemVer變更、所有模型／輸入或測量環境重現。

報告另存same byte initial／SHA，保存本人134原source與新增兩lock指紋、四material／八URL-only範圍、同行原SHA、pre-root交叉筆記、兩metadata、正式assert選段與十親看PNG。再次核 original TECH14 93cfeddf及所有同行原SHA未變；第12章original也保持原bytes，未開其同行或處置。完成後STOP，不自行開新角色或修改學生材料。
