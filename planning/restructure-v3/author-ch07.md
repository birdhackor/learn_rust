# 新第 7 章作者交接

日期：2026-10-15。角色：本章作者與作者自查；不是獨立首讀、技術或 continuity 審閱者。四節初稿完成，等待 root 凍結後的獨立審閱，不宣稱 accepted。

## 所有權與實際 gate

本次只寫 `docs/chapters/07-01-lesson.md`～`07-04-lesson.md`、`docs/assets/lifetimes-v3/`、`examples/intro-workspace/lifetimes/`、本檔與 `sources-ch07.md`。root 加入 `lifetimes`、`lifetimes/complete` members 並更新 intro lock；作者沒有改 shared manifest、lock、nav、checks 或其他章，也沒有 commit/push。

- 收到 chapter7 放行後，先在 work 保存／暫停 chapter11 prep；沒有把準備筆記當正式先備交接。
- actual 1–4 的必要前綴包含 scope/block、module/use/pub、test/assert、struct/public field、move/Copy 行為、shared reference/dereference、最後使用、`&self`、String/str、pattern 與 match。寫作時重讀實際 3.1–3.8 正文，並核對 strings/methods/last_use/reborrow/slices、pattern 借用及 let else 的真實來源；本章不依賴 4.4 的範圍 pattern、guard 或 `@`。
- 親讀 actual 5.1–5.5 全部正文及 `data-steps/`、`data/` 全部 manifest/Rust/fixtures/expected。可用的是泛型宣告與泛型 impl、Option、UTF-8 byte 長度、String 建立／追加／as_str、borrowed slice 與 Vec 基本操作；本章沒有因為 generic 的名稱就自行引入其他 API。
- 7.1–7.2 先完成不依賴 6.2 的 source；只用數值／bool assertion，等長來源透過不同文字的輸出辨認。
- root 交接 actual 6.2 後，再親讀 6.1、6.2 全文及 `standard_equality.rs`、debug/equality source。7.4 才使用已教的 `&str` 內容 assertion 與 usize assertion；沒有新增 derive，沒有依賴尚未交接的 6.3–6.5。
- 來源研究已親讀 Book lifetime EN/TW 全文與必要 Reference 選段，實讀範圍及原始指紋見 `sources-ch07.md` 與 work 的 preparation 證據。

## 逐節首次使用與讀者可判斷的關係

| 必要概念 | 首次需要與解釋位置 | 已教先備 | 本節能判斷的事 |
| --- | --- | --- | --- |
| 回傳 reference | 7.1 same 短例前後 | 3.3、3.5、3.8、5.3 | 回傳借用不取得 String，不會留下新文字。 |
| lifetime elision 的唯一輸入規則 | 7.1 same 的第一段解釋，首個 run 之前 | 3.5 有效性、函式參數／輸出 | 沒寫 annotation 仍存在輸入與輸出來源關係。 |
| 兩輸入省略歧義 | 7.1 ambiguous_output 完整 fixture 與 E0106 | same 的唯一輸入規則 | 為何同樣的省略形式不能表達兩候選來源。 |
| lifetime parameter 與 `&'a str` | 7.1 longer snippet 前 | 5.1 泛型宣告位置、3.5 lifetime | 尖括號先宣告；描述借用關係，不是型別或秒數。 |
| 兩來源共同要求 | 7.1 longer 後、內層呼叫前 | 上一列 | 結果的後續使用要由兩候選來源支持，不要求 owner 同時開始／結束。 |
| 標註不延長值 | 7.1 longer 解釋與 source_too_short | 3.5 dangling block、3.8 String | 即使這次內容選外層，簽名仍約束兩來源。 |
| 只接第一來源的較窄契約 | 7.2 first_text 前後 | 7.1、4 章 `_` pattern | 第二來源可先結束，但這已改變任選來源的規格。 |
| 區域 String 無法借出 | 7.2 local_output 前後 | String ownership、7.1 annotation | 任意 `'a` 不會創造有效的來源，E0515 是區域值問題。 |
| owned 文字替代 | 7.2 copy_text 前後 | 3.8 new/push_str/move、7.1 借用 | 新 String 可與原來源分開結束及修改，需保存另一份內容。 |
| placeholder `'_` | 7.2 same_placeholder 前後 | 7.1 elision | 推導而非重用具名參數；不能消除兩來源歧義。 |
| `Option<&str>` | 7.2 fallback 動機段 | 5.1、5.2 | 有一份借用、沒有借用、借來空文字是不同情況。 |
| `'b: 'a` outlives | 7.2 fallback snippet 之前 | 7.1 parameter、前述 Option | 第二來源至少支持輸出所需區間，允許相等，不會延長資料。 |
| struct 的 lifetime 參數 | 7.3 TextView 定義前後 | 3.1、5.1、7.1 | 模型持有 reference，外部來源仍有 owner。 |
| `TextView<'_>` 回傳型別 | 7.3 make_view snippet 之前 | 7.2 placeholder、7.1 elision | 唯一輸入連到回傳模型欄位，不是任意存活。 |
| `impl<'a> TextView<'a>` | 7.3 byte_len snippet 之前 | 3.6、5.1 generic impl、7.3 struct | impl 宣告參數，再用於型別；usize 輸出可獨立保存。 |
| receiver elision | 7.4 reference-methods snippet 之前 | 3.6 receiver、7.1 elision | 省略輸出連到借讀模型的 `&self`，可能為後續輸出使用繼續保留借用。 |
| 原欄位來源輸出 | 7.4 original 與 receiver 對照 | 7.3 欄位 `'a`、3.8 shared ref Copy 行為 | 模型可結束但 source 必須有效；不是令 receiver 與欄位 `'a` 相等。 |
| 兩欄位的獨立來源 | 7.4 TextPair snippet 之前及之後 | 7.4 original、7.1 parameter | 第一來源輸出不要求模型或第二來源繼續存在。 |
| library 與 source/tests 組裝 | 7.4 完整 library 段 | 2.6–2.8、6.2 內容比較、7.1–7.4 | 每個實際被編譯的檔案、完整 helper 與 test 都已在本頁提供。 |

## source 與命令閉包

`lifetimes/` 是無 lib 的 `intro-lifetimes`。`complete/` 另外是 `intro-lifetimes-complete`，有完整 library；它不會因早期選用父 package 的 example 而編入。兩份 manifest 都明寫 name/version/edition 2024，無 workspace inheritance、外部依賴、路徑 attribute 或隱藏 common root。

所有教學命令從 repo 根目錄執行，使用 `--manifest-path examples/intro-workspace/Cargo.toml`、`--locked`；下表省略這兩個共用參數。每一頁都提供當下完整 manifest、每個必要 helper、main、fixture 與非空 `.expected`。成功例與練習答案不靠 GitHub 外連補原碼。

| 當下位置／命令選擇 | 實際教學 source 閉包（相對 lifetimes/） |
| --- | --- |
| 7.1 `run -p intro-lifetimes --example single_source` | examples/single_source.rs；只有 same/main。 |
| 7.1 `run -p intro-lifetimes --example source_basic` | examples/source_basic/main.rs＋source/mod.rs；只有 same/longer，無 7.2 函式、tests 或 views。 |
| 7.1 `run -p intro-lifetimes --example shorter_answer` | examples/shorter_answer.rs；自足答案。 |
| 7.2 `--example source_scope` | examples/source_scope.rs；longer/first_text/main。 |
| 7.2 `--example owned_output` | examples/owned_output.rs；copy_text/main。 |
| 7.2 `--example placeholder` | examples/placeholder.rs；same_placeholder/main。 |
| 7.2 `--example outlives` | examples/outlives.rs；fallback/main。 |
| 7.2 `run/test --example source_complete` | examples/source_complete/main.rs＋source/mod.rs；已教完的七函式，test 時加同檔四 tests。沒有 views。 |
| 7.3 `--example view_basic` | examples/view_basic/main.rs＋views/mod.rs；只有 TextView/make_view/byte_len。沒有 text/original/TextPair。 |
| 7.3 `--example view_answer` | examples/view_answer.rs；只有 Label/byte_len/main。 |
| 7.4 receiver 段 `--example receiver_only` | examples/receiver_only/main.rs＋views/mod.rs；TextView/byte_len/text/original。沒有 TextPair 或完整 library。 |
| 7.4 完整來源後 `run -p intro-lifetimes-complete` | complete/Cargo.toml、src/main.rs、src/lib.rs、src/source.rs、src/views.rs。全部完整 source 在命令之前提供。 |
| 同段 `test -p intro-lifetimes-complete --lib` | 完整 library 三檔＋六個同檔 tests；不先編後面的 example。 |
| 7.4 `--example two_models` | 完整 library＋complete/examples/two_models.rs；完整 caller 先於命令。 |
| 7.4 `--example label_answer` | 完整 library＋complete/examples/label_answer.rs；完整答案先於命令。 |

所有負例都是獨立完整 `.rs`，以 rustc 2024 固定 edition 實測，不載其他章。source_too_short 與 model_source_too_short 的過短來源是原 String；method_receiver_too_short 的過短來源則是模型 view；正文指出這個差別。

## 作者驗證與 freeze 證據

工作目錄：`/workspace/work/tutorial-implementation-20261015/ch07-authoring/`。所有 Cargo/rustc shell 先 source scripts/env.sh，設定 JOBS=2、INCREMENTAL=0、DEV_DEBUG=0、TEST_DEBUG=0、COLOR=never。沒有 run root 全套，也沒有格式化其他 package。

- `selected-checks.json`：19 條命令成功，包含父 package 11 個 examples、完整 package 主要 binary 與 2 個 examples 的 14 次 run；source_complete 四項 test、完整 library 六項 test；兩個作者 package 分別 strict Clippy --all-targets，以及這兩個 package 的 fmt --check。所有 target 都在本章完整提供後才做作者整包檢查；正文沒有提早使用整包命令。
- `fixtures-final.json`：六個負例實際非零結束且符合 `.expected`；E0106、E0515、兩個原來源 E0597、receiver E0597、missing_outlives 的穩定字串。未修改腳本或跑其他作者的 fixture。
- `source-closure.json`：正文 snippet 引用／區域存在性、每頁完整引用、target 對應及 no-lib 邊界的靜態紀錄。
- `prefix-fingerprints.json` 與 `gate-6-2-fingerprints.json`：實際交接的前綴指紋。指紋清單是版本證據，不把清單內所有檔案都說成此次重讀範圍；實讀範圍以上文為準。
- `borrowed-field.png`、`svg-text-bounds.json`：用實際 Chromium 內嵌渲染本章 SVG，作者親看 340×360 圖，框、來源／欄位名稱、箭頭與字體均可讀；所有文字都在畫布內。系統 ImageMagick 缺 rsvg delegate，改用已安裝 Playwright/Chromium，沒有改 repo 依賴。
- `final-author-hashes.json`：本章全部 owned 正文、source、manifest、expected、圖及兩份 author/source 紀錄 SHA-256；`freeze/` 保存同版副本。後續必要修正另 append ledger／另存 hash，不覆寫原 freeze。

初次 outlives 的 `assert!(empty.len() == 0)` 觸發 len_zero 建議使用尚未在本章採用的 is_empty。已改成先保存長度數字再作既有數值 assertion；沒有為了排除 lint 引入新 API 或寬泛 allow。修後 strict Clippy 通過。

## 保全與待驗證

原 lifetime 章的 same/longer、first_text/copy_text/placeholder/outlives/first_nonempty、TextView/make_view/byte_len/text/original、TextPair、來源／receiver 三種過短關係、Label 原來源答案均有具體新位置，詳 `sources-ch07.md`。7.3 的新答案只交出數字，舊 Label::original 答案留在 7.4。

沒有 String::from/to_owned/clone/format!、closure、Iterator API、`'static`、T:'a、trait object 或後續章節工程設定。使用 Copy 的具體行為以 actual 3.7/3.8 已教的 shared reference 為依據，不依賴尚未交接的 6.3。

作者尚未建置或觀察 Zensical 的完整桌面／手機頁面，未驗證頁面折疊、站點 nav 與整體首讀路徑；僅已看單張 SVG。這些及獨立逐段首讀／技術／continuity 驗證由 root 另行安排。第 8 章前綴仍由 root 決定何時交接，不由作者自行 accepted。
