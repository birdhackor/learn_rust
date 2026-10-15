# 第4章作者交接

狀態：第4章七節正文、29個完整example與10個預期編譯失敗fixture已完成作者局部驗證並凍結，可由root判定第5章的實際前綴gate。作者自查不是獨立審閱，也不是網站整合驗收。

## 實際先備與隔離方式

Root 正式交接章3後，作者親讀新1–3必要正文：1.1–1.7在前一任務實讀，另讀1.8、2.1–2.8、3.1–3.8全文，以及2.1–2.5與3.1–3.8全部snippet的實際source、manifests、helper、答案與fixtures。字面值與變數、函式、block／scope、if／return、for／while／loop／break／continue、數字assertion、局部allow、Order、owned／Copy／&／&mut、slice、String::new／push_str／as_str均有真實前文依據。

沒有把3.8後選讀當必要先備；沒有使用Option／Result／Vec／泛型、Debug／derive、String::from／clone。前綴實讀檔案指紋在 `/workspace/work/tutorial-implementation-20261015/ch04-author/prefix-read-hashes.json`。

範例容器是 `examples/intro-workspace/patterns/`，package `intro-patterns`。最小manifest明寫name/version/edition，沒有lib；src/main.rs只提示選example，各個單檔example獨立保存當時全部型別和helper。新增workspace member與lock由root處理，作者沒有改共享設定。

## 已凍結的三節

| 小節與學習問題 | 新概念的實際介紹與首用 | 真實完整source／答案 |
| --- | --- | --- |
| 4.1：缺少、金額0、拒絕如何分開保存？ | 先說enum／variant的互斥形態，以表解釋無資料／位置資料／命名欄位，再看定義；建立值前解釋Quote::與_name。首次snippet前說清對enum局部allow(dead_code)的用途與限制。 | enums.rs／enums_answer.rs，只建構，不藏match或Debug |
| 4.2：依資料選工作，且不漏情況 | 先定義pattern／arm／=>、literal與_，再看數值match；Quote pattern的最小binding先解釋後用。結果型別與exhaustive分別教；Amount(0)仍顯示報價0。 | match_numbers.rs、match_quotes.rs／match_answer.rs；missing_variant.rs E0004、arm_type.rs E0308 |
| 4.3：替資料內部各部分命名 | pattern的形狀與_／..、range用途差別在首次snippet前；struct欄位:新binding與shorthand先解釋。參數tuple pattern／for tuple array／巢狀enum內Order各有當下說明。普通let不能配對失敗最後明教。 | destructure.rs、pattern_places.rs、nested_patterns.rs／destructure_answer.rs；refutable_let.rs E0005 |

各頁同頁內嵌完整manifest、每個成功例與答案；fixtures也完整內嵌並標明預期失敗。未把後節型別或helper藏在library。來源及研究安排見 `sources-ch04.md` 與 scratch `research-and-snapshot-plan.md`。

Root在作者階段提示移動既有說明，以完成先解釋再首次snippet的窄順序修正；這不是獨立review發現或首讀通過證據。只移正文、Rust未變，因此未重跑已成功的Rust檢查。

## 4.1–4.3 局部驗證

已成功執行9個example；結果與正文一致。3份compile-fail在edition2024各得到預期E0004／E0308／E0005。`cargo fmt ... -p intro-patterns -- --check` 與 `cargo clippy ... -p intro-patterns --all-targets --locked -- -D warnings` 通過。執行時按root設定 CARGO_INCREMENTAL=0、DEV/TEST DEBUG=0、JOBS=2；每個shell先載入scripts/env.sh。

原始逐target stdout／診斷保存在 `/workspace/work/tutorial-implementation-20261015/ch04-author/validation-4.1-4.3.json`。同目錄 `freeze-4.1-4.3.json` 保存17個實際檔案的SHA256；其中正文為：

- 04-01：`9487c85aed9cc3682591a88d62aaf046dd06abed9b6927624a606bfd3a3a697f`
- 04-02：`262f9c33eb424515abb9be65146314d24d08650610a54674ce7977f43cfcf1da`
- 04-03：`42d2fb6ad95eeacaf4d01ed3a84155f7f065763ded9bbb1538212671849b5352`

未執行完整site build、未做網站桌面／手機原位檢視；root負責導覽及整合檢查。本章目前不需要新圖才能辨認資料對應，不用裝飾圖代替source解釋。

## 後四節的首次需要與真實材料

| 小節／學習問題 | 首用前已教的關係 | 完整source與代表結果 |
| --- | --- | --- |
| 4.4：多個形態共用、範圍、執行時比較、保留符合值 | 各段先介紹or／range／guard／@才給獨立完整target；新variant Estimate的角色先交代；名稱shadowing先觀察，guard再用不同名稱比較外部門檻。guard限制整個or，且不取代fallback。 | or_patterns、range_patterns、shadow_patterns、guards、at_bindings；combinations_answer以(4,false)→0辨認整個or都受guard限制。fixtures or_missing_binding E0408、guard_not_exhaustive E0004。 |
| 4.5：取得owned文字，或只借讀／借改？ | 先以String明示非Copy欄位move，再partial move，接著match &／&mut的binding型別；ref/ref mut與& pattern各自角色先於完整source；2024自動／明寫形式規則在fixture前。 | owned_patterns、partial_move、borrow_patterns、explicit_bindings、reference_pattern、ignore_patterns及borrow_patterns_answer；文字可兩次借讀、再借改。fixtures驗證partial move E0382、不能從&String move E0507、_text會move E0382，以及2024冗餘ref遭拒。 |
| 4.6：一次成功才做，以及反覆取得直到不符 | if let與else、binding scope先說；Batch型別與next_batch如何扣量／回Done在helper首用前；while let每次重算右式、狀態更新與終止關係再看loop。 | if_let、while_let／while_let_answer；7→4→1→0共3批，起始0一次body也不做；答案上限正值以assert檢查，三組結果3／2／0。 |
| 4.7：成功名稱留後文，失敗先離開 | let else、分號與成功scope先說；同任務三寫法；else不能落回後文；迴圈continue／break先備已在2.5，換成&[Quote]先說逐項&Quote。章末Quote::Offered(LabelledOrder)與各helper角色先交代。 | let_else、let_else_loops、quote_report／let_else_answer；三寫法同得140；跳過缺值加總200、遇缺值停止120；報價計價180且保留Missing／Rejected區別。let_else_must_leave E0308。 |

reference_pattern的show刻意完整寫出& pattern／ref binding，第一次Clippy實查回報match_ref_pats與needless_borrowed_reference。只在該函式以兩項具名allow保留比較材料，正文先說清建議與保留理由；其他target沒有擴大關掉lint。

作者另逐節檢查重要新寫法的說明在首次snippet前，並以靜態檢查確認所有29份example、10份fixture都整檔同頁內嵌，每頁都有完整manifest，named snippet的起終marker均存在。所有Rust來源不含後章Option／Result／Vec／Iterator、derive、String::from或clone，也沒有lib。

## 全章局部驗證與凍結

全部29個example已真正執行成功，所有數字assertions通過；10份compile-fail全部拒絕，與各自語言規則及正文相符。新增後四節後，完整package的fmt與Clippy `--all-targets -- -D warnings` 已通過。4.1–4.3的17檔指紋重新比對完全不變。

2024 fixture初次驗證時，腳本預期另一版本常見的diagnostic文字；實際1.99工具鏈用 `cannot explicitly borrow within an implicitly-borrowing pattern`，fixture本身確實遭拒。已將正文與比對條件改為本次實際文字並重驗，沒有把舊文字冒充本輪輸出。

全部原始stdout與diagnostics：`/workspace/work/tutorial-implementation-20261015/ch04-author/validation-all.json`。同目錄 `snippet-audit.json` 保存完整source／manifest／marker／後章API與初期freeze不變檢查。`freeze-ch04.json` 保存7頁正文與41份package檔案，共48檔SHA256。

後四頁正文SHA256：

- 04-04：`a8de69a37fbb52203ed5e88d485cb107d60bb5931320220b4290d2325888a49c`
- 04-05：`8284e571e9214f593c57dbb24870b1915d59cd910aba3268d927e58334804765`
- 04-06：`29a7e763a4b5ae1452a3422a1041d8ae934d2799f15212ee37d5e89fee0671d6`
- 04-07：`4df391d5e3c031a53d2bbb596434e6ba86a6c7c7662fca07e60c474f8e5dcd20`

以上是作者局部驗證；獨立技術／銜接／逐段首讀及網站原位桌面手機檢查仍由root安排，不宣稱已完成。

## 維護fixture harness整合補檔

Root整合時發現作者原本只做獨立rustc診斷比對，遺漏scripts/check-fixtures.py要求的同名非空.expected。已補齊10份.expected：一般fixture用已實測E碼，edition2024用本次實際的穩定診斷文字。逐一與validation-all.json保存的真實stderr比對，全部包含期待字串；未改Rust或正文，未重跑29個成功程式。這些是維護harness期待，不是新增學生先備。

原freeze-ch04.json及原始驗證紀錄保留；新58檔freeze在/workspace/work/tutorial-implementation-20261015/ch04-author/freeze-ch04-with-expectations.json，新增檔與比對記錄在fixture-expected-integration.json。原48檔指紋逐一確認未變。原初判的通過範圍是作者局部執行／獨立編譯診斷，不冒稱當時已通過維護harness。

## Continuity C4a 窄修

Root採納已封存獨立continuity初判的C4a-01與C4a-02：4.2把既有match expression基本規則前移第一次shipping snippet前，具體i32與函式尾端分析留在code後；首次粗體只包英文pattern，中文括註置於外側，避免實頁出現literal **。只改4.2正文，未改Rust、未重跑已成功targets。

原freeze與initial判讀保留；修後58檔指紋保存於/workspace/work/tutorial-implementation-20261015/ch04-author/freeze-ch04-continuity-c4a.json，窄diff為continuity-c4a.diff。逐檔核對原58檔，僅04-02-lesson.md改變，新SHA256為 `5bbb8a9a5406962e325218feb6f35ab4adbaeccea828d6f512bc7ba07562e066`。Root另建preview交獨立followup，作者不冒稱已通過該followup。
