# 第 14 章作者追加研究：macro、公開 API 與驗證工具

研究者：`/root/chapter04_traits`。日期：2026-10-14。這是作者親自重新閱讀網路來源的紀錄，與既有 `ch14.md` 分開保存。既有檔案本輪前的 SHA-256 為 `99838bf0079e3a36fc884dc3736ed9db3e35541c17036f2cec5a51603a04a8f8`，沒有覆寫。

目前只研究，沒有開始第 14 章正文、學生範例、測試、SVG、依賴選定、Cargo／導覽修改或工具安裝。以下來源的讀取不表示它們的 code 可直接收入教材；實際前文仍須由 root 開閘後逐份重讀。

## 來源讀取與證據界線

英文／繁中文 Book 與 Rust Reference 透過網路工具實讀；固定版本 docs.rs 與部分原始 Markdown 另以 HTTP 取得、抽出本文後實讀。網路工具曾對固定 docs.rs URL 回報不可開啟，但直接 HTTP 成功，不能把工具錯誤記成該版本不存在。

HTTP 原始來源與抽出的文字保存在 `/workspace/work/tutorial-implementation/research-ch14-author04/`；`http-sources.json` 記錄 URL、實際回應 URL、內容 hash 與文字檔。下載全文與實讀範圍分開記錄；未讀尾段不算已研究。後續開稿仍應固定所選版本，不混用 Book 的 syn 2 manifest 與 syn 3 API。

## 14.1–14.3：macro 的教法與實際契約

| 實讀來源 | 本次實讀部分 | 可借鑑方式／不可照搬的地方 |
| --- | --- | --- |
| [英文 Rust Book：Macros](https://doc.rust-lang.org/book/ch20-05-macros.html) | 全節：declarative、derive、attribute、function-like；含三個 crate、parse／quote 骨架 | 從已用過的 macro 回到原始碼展開，再比較手寫 trait impl 與自動生成。範例骨架直接 unwrap parser、略過 API 契約，不符合本專案「使用前先教」 |
| [繁中 Rust Book：巨集](https://rust-lang.tw/book-tw/ch19-05-macros.html) | declarative 段與 derive 主要程式／解說；沒有把後段 attribute／function-like 全部算已讀 | 可學同一需求逐步重構。舊例的 `let gen` 在 edition 2024 不能直接使用；教材正文術語依本專案保留 macro／trait 等英文 |
| [Reference：Macros by Example](https://doc.rust-lang.org/reference/macros-by-example.html) | matcher／transcriber、metavariables、重複、片段轉送、edition 差異、scope／hygiene 相關段 | 先用一條規則、有限輸入、明確展開，再教 repetition。匹配語法，不是在編譯時算出任意 runtime 值 |
| [Reference：Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html) | 全文 | proc-macro crate 的位置限制、三種輸入／輸出與錯誤途徑要先清楚；derive 添加 items，attribute 替換 item，不能混成同一種轉換 |
| [2024 Edition Guide：gen keyword](https://doc.rust-lang.org/edition-guide/rust-2024/gen-keyword.html) | 全節 | 以 `generated` 等一般變數名避開保留字；不為沿用舊例額外引入 raw identifier |
| [std::any::type_name](https://doc.rust-lang.org/std/any/fn.type_name.html) | std 1.99.0 全頁 | 它提供診斷用型別名字，格式／唯一性沒有保證。不能用「Rust 無法在 runtime 取得型別名」當 derive 的必要理由 |

### 先教能力再出現骨架

1. macro 的理由應是讀取呼叫處的語法、產生語法或減少固定樣板。普通函式也能透過 slice／Vec 接收不定數目的同型資料，不能把這種能力說成 macro 獨有。
2. `macro_rules!` 最小例需要在 code 前說清楚規則、`$name:expr`、代入展開與呼叫結果；首次 repetition 再說明 `*`、分隔符與空輸入。尾逗號若接受，要明示它是哪條規則支援。
3. 先展示生成的普通 Rust 程式再討論 hygiene。區域變數／labels 的解析與其他名字不同；`$crate` 指向定義 macro 的 crate，沒有繞過 visibility。`#[macro_export]` 不等於所有未 export 的 macro 都不能在本 crate 使用。
4. 展開式若重複某個輸入 expression，可能重複執行副作用；hygiene 不會自動保證只計算一次。若採此例，以已教的整數計數與 block 即可，不需要未教的 RefCell。
5. 第一條成功匹配規則會被採用，生成的 Rust 若失敗不會繼續嘗試下一條規則。`expr` 的接受範圍受定義處 edition 影響；本專案固定 edition 2024，不混用舊教材的語法限制。
6. procedural macro 先有 consumer crate／proc-macro crate 角色與 manifest 設定，再有 compiler TokenStream → syn AST → quote tokens → compiler TokenStream 的完整路徑。每一步的型別、成功結果與失敗輸出必須在使用前出現。
7. derive 的示範需求應實際用到原始碼結構，例如生成一份已理解的 impl；先展示手寫答案。不以「只能靠 derive 印出型別名字」建立假的必要性。
8. proc macro 可用 `compile_error!` tokens 回報錯誤，不必只能 panic。支援的輸入要窄而明確；Reference 允許 derive 輸入 struct／enum／union，若本範例只接受某類，應主動拒絕其他種類並給出訊息。

### 固定 API 的本次核對

下列是研究時讀到的版本，尚未選定專案依賴。

| API／版本 | 本次核對結果 | code 前的必要說明 |
| --- | --- | --- |
| [syn 3.0.9 DeriveInput](https://docs.rs/syn/3.0.9/syn/struct.DeriveInput.html)；另讀 [syn 2.0.117 同頁](https://docs.rs/syn/2.0.117/syn/struct.DeriveInput.html) | 兩版本所核的核心欄位均有 attrs、vis、ident、generics、data；不據此宣稱整個 API 相同 | 選定 AST 型別、哪些欄位會讀、它們代表原輸入什麼部分；Data variant／相關型別用到才逐項教 |
| [syn 3.0.9 parse_macro_input!](https://docs.rs/syn/3.0.9/syn/macro.parse_macro_input.html) | 全頁；成功得到 AST，解析錯誤會早退並傳回 compiler error tokens；需 parsing／proc-macro features | `input as DeriveInput` 是這個 macro 的語法，不是一般 Rust `as` cast；呼叫位置必須能傳回 compiler TokenStream |
| [syn 3.0.9 Error](https://docs.rs/syn/3.0.9/syn/struct.Error.html) | introductory error handling 與方法區，至 trait impl 之前 | Error::new 的 span／訊息、into_compile_error 消耗 error 並得到 proc_macro2 TokenStream；new_spanned 額外需要 printing／ToTokens。先解釋位置資訊用途，不把全套 Span internals 當必修 |
| [syn 3.0.9 Generics](https://docs.rs/syn/3.0.9/syn/struct.Generics.html) | 欄位、方法與 split_for_impl 回傳形狀 | 若支援泛型，必須保留 impl／type 泛型與 where clause；若只教無泛型輸入，應明確檢查／拒絕，不能默默忽略。params 是 Punctuated，不可當已教 Vec；本次尚未逐頁核 Punctuated 的候選檢查 API |
| [quote 1.0.47 quote!](https://docs.rs/quote/1.0.47/quote/macro.quote.html) | interpolation、repetition、hygiene、return type、片段合併、identifier／方法呼叫／字串限制 | `#name` 插入 ToTokens 內容；字串內的 `#name` 不會插值；輸出是 proc_macro2 TokenStream，要先教轉為 compiler TokenStream 的那個轉換 |
| [proc-macro2 TokenStream](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html) | HTTP 取得時版本 1.0.107；未把全文 trait boilerplate 算已讀；角色由 quote 文件核對 | 若 helper 明寫這個型別，要直接宣告依賴。不能因 syn／quote 間接依賴便假設 consumer 可 import；一般單元測試也不應依賴只存在 compiler context 的 proc_macro API |
| [trybuild](https://docs.rs/trybuild/latest/trybuild/) | HTTP 取得時版本 1.0.122，主頁全文 | TestCases、compile_fail／pass、glob 與 stderr 規則都須先教。compile_fail 不只是失敗，還核對相鄰 .stderr；新 wip 訊息要先判讀才採納，不能盲目 overwrite |

本 repo 原 compile-fail 腳本以獨立 rustc 執行，沒有自動替 proc-macro consumer 提供外部 crate。未來不能把找不到依賴造成的失敗算成 macro 正確拒絕輸入；需要已教的 Cargo consumer 或相稱的 trybuild 安排，核對目標診斷。這項只做契約研究，沒有改腳本。

## 14.4：公開 API、features、版本相容

| 來源 | 本次實讀範圍 | 教法與陷阱 |
| --- | --- | --- |
| [Rust API Guidelines：Future proofing](https://rust-lang.github.io/api-guidelines/future-proofing.html) | 全頁 | 以原 representation 與 newtype／private fields 保護不變條件的具體差異說明，不要求所有資料 struct 都寫 getter。sealed 阻止外部 impl，不保證日後任何 trait 更動都不 breaking；`#[doc(hidden)]` 是在文件中隱藏項目，不是語言存取權限 |
| [Cargo SemVer compatibility](https://doc.rust-lang.org/cargo/reference/semver.html) | 導言、分類、0.x；public item、struct、enum／trait／function／bounds／features 相關段 | 同一個未改 consumer 在改版前後的編譯／行為，比抽象分類更容易理解。新增 enum variant 可破壞窮舉 match；後來加 non_exhaustive 本身也可 breaking |
| [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) | default、feature union、互斥 features、resolver 2 隔離上下文相關段；不是全文 | 宣告 feature 不等於啟用；default-features=false 只影響該依賴邊，其他邊仍可能啟用。正式稿須按實際 workspace resolver 說明，不將 reference 的 resolver 2 例直接當本 repo 設定 |
| [Rust 語言聖經：features](https://raw.githubusercontent.com/sunface/rust-course/main/src/cargo/reference/features/intro.md) | 定義／cfg 與 default 兩段；未讀全部後續 | 文中宣告 webp=[] 後「肯定啟用」的說法與官方契約、其本節 default 解說都不一致，不照抄 |
| [cargo publish](https://doc.rust-lang.org/cargo/commands/cargo-publish.html) | 命令用途、dry-run、registry 與 package 選擇相關選項 | dry-run 不上傳但會檢查；no-verify 不能算建置已驗證。GitHub Pages 發佈不等於 crate registry 發佈。以後只在已教 manifest／CLI 契約之後給具體檢查命令 |

## 14.5：property testing

實際讀取 [Proptest getting started](https://proptest-rs.github.io/proptest/proptest/getting-started.html) 的日期 parser 初始失敗／回歸部分，未將整節後段修復算已讀；另全文讀取 [strategy basics](https://proptest-rs.github.io/proptest/proptest/tutorial/strategy-basics.html)、[shrinking basics](https://proptest-rs.github.io/proptest/proptest/tutorial/shrinking-basics.html)、[failure persistence](https://proptest-rs.github.io/proptest/proptest/failure-persistence.html)。固定 API 讀取 [proptest 1.12.0 collection::vec](https://docs.rs/proptest/1.12.0/proptest/collection/fn.vec.html)、[prop_assert!](https://docs.rs/proptest/1.12.0/proptest/macro.prop_assert.html) 全頁。

可學習「已知例子沒找到的失敗 → 自動生成 → 縮小 → 保存回歸」。首例宜用有限整數／已教 Vec，不能讓讀者先忽略 regex 或策略 DSL。strategy 不是實際輸入 Vec；`input in strategy` 是 proptest macro 的生成語法。prop_assert! 傳回 TestCaseError 而非普通 assert 的 panic，使用位置要先說明。

property 應來自獨立契約，不複製被測 implementation 當 oracle。例如 idempotence 單獨使用仍容許「永遠回傳空清單」；應與保留元素／順序／重複數等必要關係合併。具體題目須等實際先備開閘。有限數值範圍需對照函式原契約，不能借縮小輸入偷偷略掉已承諾的合法邊界。

shrinking 找到較簡單且仍失敗的案例，不承諾唯一全域最小值。persistence 保存 seeds，改變 strategy 後未必重現原輸入；重要失敗另存明確 unit case。比較整份 Vec 若用 assert_eq!，須核該具體型別的 Debug／PartialEq 是否已教，不能以已教 Vec::clone 代替。

## 14.6：Loom 與 Miri

已全文讀 [Loom 0.7.2 首頁](https://docs.rs/loom/0.7.2/loom/) 與 [model](https://docs.rs/loom/0.7.2/loom/fn.model.html)，選讀 [Builder](https://docs.rs/loom/0.7.2/loom/model/struct.Builder.html) 的欄位、新建／check 與限制。model callback 是 Fn + Sync + Send + 'static，會執行多次，不是 FnOnce。需要換成 Loom 能觀察的 thread／synchronization 型別；僅包一層 model 但保留普通 std 同步動作，不能宣稱這些動作已被探索。

模型應讓排程成為明確的不確定來源；現實隨機／syscall 需要相應模型，不能假裝它會自動觀察。max branches／permutations／duration／preemption bound 改變覆蓋範圍；工具對弱記憶體行為也有明確限制。「所有排程」必須限定在模型、設定與工具支援之內。Builder 非窮舉結構以 new 建立，不用字面值跨過其契約。若採用 cfg(loom)，須先說明 cfg 與 check-cfg 設定。

讀取 [Miri README](https://raw.githubusercontent.com/rust-lang/miri/master/README.md) 前 148 行：檢測能力／限制、host isolation、安裝／run／test／setup 基本流程；未把後面所有 flags 當已核。Miri 可發現選定執行中的 UB，不證明所有執行正確；aliasing 模型部分仍實驗性，網路等操作受限。跳過 cfg(miri) 測試須記清楚不再覆蓋的內容。

只讀探測 [2026-10-13 nightly manifest](https://static.rust-lang.org/dist/2026-10-13/channel-rust-nightly.toml) 顯示該日 x86_64-unknown-linux-gnu 的 miri-preview available=true，Rust 為 1.101.0-nightly。這個事實只表示元件在 manifest 列出；沒有安裝、驗證本 repo 相容或執行 Miri。原始 manifest 與探測 JSON 已存研究目錄。實際教學須另選／固定 nightly、元件、命令範圍，不修改整個 repo 的預設 stable 工具鏈。

## 14.7–14.8：benchmark 與 profiling

| 來源 | 本次實際讀取 | 可學習／應避開 |
| --- | --- | --- |
| [Rust 語言聖經：benchmark](https://raw.githubusercontent.com/sunface/rust-course/main/src/test/benchmark.md) | 全文 | 從被優化掉的工作解釋 0ns 有助理解。不可照搬舊 Criterion 0.3／nightly override 或 Fibonacci 大輸入溢位例；「black_box 讓編譯器不做任何優化」太絕對 |
| [Criterion getting started](https://criterion-rs.github.io/book/getting_started.html) | 全文 | 參數、測量 callback、獨立 benches target 逐步接起；harness=false 為什麼要設定須在 manifest 前教。guide 的 0.5.1 不等於本次固定 API 0.8.2 |
| [Criterion 0.8.2 Bencher](https://docs.rs/criterion/0.8.2/criterion/struct.Bencher.html) | intro、iter、iter_batched、iter_batched_ref 與其 timing models | iter 包含 routine、輸出 Drop 與迴圈開銷；batched 的 setup 在計時外，ref 形式可改變輸入 Drop 的計時。要先界定測哪一段，不以搬走 clone 即稱比較公平 |
| [std::hint::black_box](https://doc.rust-lang.org/std/hint/fn.black_box.html) | std 1.99.0 全頁 | identity 加 best-effort 的優化阻礙，沒有正確性／安全保證。`black_box(5 * 10)` 不保證內部乘法未先常數摺疊；輸入與實際消費結果要對準測量目的 |
| [Criterion command line output](https://criterion-rs.github.io/book/user_guide/command_line_output.html) | warmup、samples、time interval、change 與 noise 相關段 | sample 不等於一次 iteration；三個時間是信賴區間下限／估計／上限，不是最小／平均／最大三次執行。只教本例需要的比較與不確定程度 |
| [Rust Performance Book：Profiling](https://nnethercote.github.io/perf-book/profiling.html) | CPU／allocation 工具與 release line-table 部分；未讀全節後段 | 先有待解釋的現象，再選 CPU 或 allocation 量測；profile 給線索，真實改進仍須公平 benchmark 複核 |
| [samply README](https://raw.githubusercontent.com/mstange/samply/main/README.md) | 全文；README 當前下載連結 v0.13.1 | 預設 1000Hz 抽樣不是逐次函式計時；Linux 目前 on-CPU，其他平台能力不同。倉庫層級設定優先，不抄全域 home config；不為研究更改 sudo／sysctl 或上傳 profile |

benchmark 前先檢查正確結果、同一輸入／ownership／輸出策略，再界定 setup、執行與 drop 邊界。真實 API 若需要 clone，不能因測量方便把它移出後忽略總成本。callback 使用的 Fn trait、block、參數與回傳也須實際前置。

profiling 的圖形必須按真實選擇的 UI／模式解釋：flame graph 的寬度可代表抽樣數，不能泛稱橫軸都是時間。尚未安裝／執行 samply、親看 profile 或製作任何效能數字。固定實際 binary 版本、完整命令與可用權限要留到開稿後核對。

## 待開稿的實際依賴與研究續點

以下只是已規劃 gates，不能當成已學能力：

- 14.1：actual 7.9 + 8.9；14.2：14.1。
- 14.3：14.2 + 9.2 + 6.2；14.4：8.9 + 8.6 + 9.1 + 7.9。
- 14.5：7.9 + 7.8 + 4.3；14.6：10.8 + 12.4 + 9.1 + 7.1。
- 14.7：9.1 + 11.4 + 7.9 + 6.3；14.8：14.7。

root 開閘後先實讀固定前文與所有相關 source，重建可用 API 清單；選定 syn／quote 等版本後讀取所有實際使用的固定 API。尚待核對所選 AST traversal／Punctuated、proc_macro2／trybuild 固定頁與真實診斷，不能用本次候選研究替代。

本輪研究範圍較寬，但沒有把下載未讀部分、資料中的命令、元件可用性或作者給的 benchmark 數字當作自己執行成果。沒有執行 property test、Loom、Miri、benchmark、profiler，也沒有做真實桌面／手機版面核對。研究先封存，待實際依賴齊備再繼續正文與相稱驗證。
