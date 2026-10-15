# 第6章v3作者交接ledger

狀態：6.1–6.6全部正文/source完成作者自查並凍結，交root與獨立技術/銜接審閱；不把作者自查當整章最終驗收。沒有改根Cargo/intro workspace members/lock/nav/check/shared設定或其他作者source。root負責新增member與更新鎖定檔。

## 真正的first-use位置

| 能力 | 首次需要／解釋與可觀察材料 | 已成立前提 |
| --- | --- | --- |
| Charge trait、方法簽名、各型別impl | 6.1:19「trait描述」→31「各型別實作」，短片段後解釋不同body | 3.1公開struct/fields、3.6&self/Self/method |
| bound T:Charge、&T、trait import | 6.1:41「bound」→62「module外」；Order360/Bundle160與門檻300 | 5.1泛型/impl；2.6mod/use/pub/crate::；3章借讀 |
| cfg/test/assertions與完整support | 6.1:97「完整來源與數值測試」，只比較數字/bool | 2.7工具/attributes/tests；2.6mod.rs/Cargo target規則 |
| Debug/:?/derive | 6.2:5，使用前解釋格式、生成與欄位要求；debug.rs | 6.1trait；2.7attribute；1.2println |
| PartialEq/逐欄/enum/借讀比較/assert_eq要求 | 6.2:33；equality.rs比較同欄位、不同count、不同variant/variant內值 | 3.1struct、4.1enum；6.2前段Debug |
| String與&str的Debug/內容相等 | 6.2:60；standard_equality:text，先教文字內容與引號再用 | 3.8String new/push_str/as_str；6.2Debug/PartialEq |
| usize/reference比較借到的值 | 6.2:72；standard_equality:references，兩個不同usize變數的refs相等 | 1.8usize；3.3reference；6.2能力 |
| Option的Debug/PartialEq與內部能力 | 6.2:82；None/Some、Some值不同、Option<&usize>具體例 | 5.2Option；6.2前段reference/usize |
| Clone/derive、String獨立文字、Vec<&str>仍借原文字 | 6.3:5與13；cloning.rs保留原值再改另一份 | 3.8String/move、5.4Vec/as_slice、6.2整體比較 |
| Copy/Clone關係、數值struct可用、String不能 | 6.3:37與59；copying.rs/string_not_copy | 3.2具體Copy/move；6.3Clone |
| type Output/Self::Output/泛型trait impl | 6.4:15→29；associated_support各短片段 | 5.1泛型inherent impl；6.1trait |
| S::Output與Output=i32 equality bound | 6.4:53→69；先直接call再通用call再numeric work | 6.4前半；不要求Iterator/type alias |
| Eq/Hash/collision/一致性 | 6.5:7，從鍵身分開始；只用標準String能力 | 6.2PartialEq；6.3String ownership |
| HashMap/use/new/insert/Option old值 | 6.5:15→33；分段new-insert/replace可觀察狀態 | 2.6use、5.2Option、6.5鍵規則 |
| owned_text helper | 6.5:15相鄰完整map_support；沒有counts | 3.8完整已教操作；1.6回傳owned值 |
| get/Borrow<str>/query與map值不同借用來源 | 6.5:43；map-query.svg與query片段，查找當下/之後分開 | 3.5最後使用、3.8as_str、6.2Option<&usize> |
| get_mut與map borrowing衝突 | 6.5:73→85，E0502與map_update完整例 | 3.4mutable dereference；6.5get |
| for &[String]得到&String、缺鍵才clone、None後insert | 6.6:5；count_labels三步table | 3.7slice for、5.4/5.5Vec、6.3clone、6.5query/update |
| owned輸出保留到輸入scope之外 | 6.6:56，完整tests以已教block expression交回map | 1.6block尾端值；3.5scope/owned；沒有新延後初始化 |
| 自訂鍵derive Eq/Hash | 6.6:74，欄位要求先教，再只改一欄查詢 | 6.2derive；6.5Eq/Hash一致性 |
| StockItem完整分組答案 | 6.6:106，&StockItem元素/quantity Copy/name clone先說 | 本章已教操作；不借Entry/closure |

沒有format!、Iterator、Entry、closure/map/filter/collect、自訂lifetime annotation/Display/Drop或未介紹的traithelper。註解只用2.7已教//；沒有新doc comment語法、隱藏allow或manifest繼承。6.2的String/&str/reference/Option規則不是只列名稱，均有先於使用的具體正文與完整可執行source。

## 每個target真正載入什麼

member `examples/intro-workspace/trait-stages/Cargo.toml`只有name/version/edition，沒有lib、dependencies、features或lint設定。src/main.rs只是已教的固定文字入口，以下selectedexample不編譯它。

helper使用已教的 `mod name;` → `examples/name/mod.rs`；資料夾沒有main.rs，因此不會自動成另一個example target。編譯閉包如下，皆另加本member manifest及root管理的入門workspace manifest/lock：

- `associated`：`examples/intro-workspace/trait-stages/examples/associated.rs`, `examples/intro-workspace/trait-stages/examples/associated_support/mod.rs`。
- `associated_answer`：`examples/intro-workspace/trait-stages/examples/associated_answer.rs`, `examples/intro-workspace/trait-stages/examples/associated_support/mod.rs`。
- `charges`：`examples/intro-workspace/trait-stages/examples/charges.rs`, `examples/intro-workspace/trait-stages/examples/charge_support/mod.rs`。
- `charges_answer`：`examples/intro-workspace/trait-stages/examples/charges_answer.rs`, `examples/intro-workspace/trait-stages/examples/charge_support/mod.rs`。
- `cloning`：`examples/intro-workspace/trait-stages/examples/cloning.rs`。
- `cloning_answer`：`examples/intro-workspace/trait-stages/examples/cloning_answer.rs`。
- `copying`：`examples/intro-workspace/trait-stages/examples/copying.rs`。
- `custom_keys`：`examples/intro-workspace/trait-stages/examples/custom_keys.rs`。
- `debug`：`examples/intro-workspace/trait-stages/examples/debug.rs`。
- `equality`：`examples/intro-workspace/trait-stages/examples/equality.rs`。
- `equality_answer`：`examples/intro-workspace/trait-stages/examples/equality_answer.rs`。
- `grouping`：`examples/intro-workspace/trait-stages/examples/grouping.rs`, `examples/intro-workspace/trait-stages/examples/grouping_support/mod.rs`, `examples/intro-workspace/trait-stages/examples/map_support/mod.rs`。
- `grouping_answer`：`examples/intro-workspace/trait-stages/examples/grouping_answer.rs`, `examples/intro-workspace/trait-stages/examples/map_support/mod.rs`。
- `map_lookup`：`examples/intro-workspace/trait-stages/examples/map_lookup.rs`, `examples/intro-workspace/trait-stages/examples/map_support/mod.rs`。
- `map_lookup_answer`：`examples/intro-workspace/trait-stages/examples/map_lookup_answer.rs`, `examples/intro-workspace/trait-stages/examples/map_support/mod.rs`。
- `map_update`：`examples/intro-workspace/trait-stages/examples/map_update.rs`, `examples/intro-workspace/trait-stages/examples/map_support/mod.rs`。
- `standard_equality`：`examples/intro-workspace/trait-stages/examples/standard_equality.rs`。

6.1只載charge_support，6.4只載associated_support，6.5只載只有owned_text的map_support；6.6才載grouping_support。沒有舊ch04-traits完整lib或ch06-iterators完整lib的穿透。每頁都有完整manifest、完整root/helper與完整答案，snippet短片段與全檔來自同一real source。原有根workspace完整library/tests保留，不為前期複製全部後續模組。

## 實際驗證

所有Cargo/Rust新shell先source scripts/env.sh，使用CARGO_INCREMENTAL=0、CARGO_PROFILE_DEV_DEBUG=0、CARGO_PROFILE_TEST_DEBUG=0、CARGO_BUILD_JOBS=2。只檢查自己的nested member，不跑根workspace/check.sh。

- root接好最小member並生成lock後才開始Cargo。
- prefix6targets check/Clippy -Dwarnings、fmt通過；後續11selectedtargets Clippy通過；全6節完成後對本member --all-targets Clippy -Dwarnings與fmt --check通過。没有用allow壓掉未教建議。
- 17個正常example實際cargo run，stdout逐字比對。6.1/6.2詳細命令與結果保存 `/workspace/work/ch06-v3/prefix-verification.json`；6.3–6.6在rest-verification.json。
- selected testtargets為charges、charges_answer（各2，重用同一support tests）、associated（3）、grouping（3），共10次測試執行、8項不同測試規則，全部通過。
- 6份compilefail皆實際rustc捕捉非零stderr，再建立同名非空.expected：missing_bound E0599、not_debug E0277、string_not_copy E0204、output_bound E0271、map_borrow_conflict E0502、key_without_hash E0599。沒有猜碼、沒有缺sidecar；未單獨重跑全書harness。
- 6頁使用repo uv locked環境、Markdown+pymdownx.snippets(check_paths=True)實際展開成功；HTML留work/ch06-v3/06-NN.html。泛型型別以code span/程式區塊處理。
- 6.5新增map-query.svg，420寬直式、字級20/22，title/desc及箭頭角色相鄰說明。嘗試本地Chromium CLI渲染時未取得截圖，已停止該次進程；**未親看圖、未驗證整站實際桌面/手機頁面，不能算視覺通過**。須由root實際頁面驗證。

## 獨立prefix初判與窄修

root已通知獨立技術prefix初判必要0、選讀TC06P-O1。按root裁定，6.1練習的「非負小數」窄修為「非負小整數」，與i32欄位相符，未改Rust或其他內容、不重跑Rust。

原prefix freeze保留於 `/workspace/work/ch06-v3/prefix-freeze.json`：06-01原hash 2ae67ddfedea1c5feafc589171a8277879502eb56bcab4b5b9773bd9d94624b5；06-02原/現hash a4c6d3934cae52521b282cd9fb52132a62d2f43a9a8ee669f79f74452ffa40c9。這筆窄修之外prefix source/hashes核對未變；不把原judge改寫為整章完成稿已驗收。

## 最終作者freeze SHA-256

```text
5df389994db35de562be09374b10686ef2b11774eb70893f2200d4d159141d0a  docs/chapters/06-01-lesson.md
a4c6d3934cae52521b282cd9fb52132a62d2f43a9a8ee669f79f74452ffa40c9  docs/chapters/06-02-lesson.md
5a2de0ca59151643d07bfed212b014e36f275975cdbd9074d283d5aede346807  docs/chapters/06-03-lesson.md
aa304cf004199a6d7fe5925854539614d1781f80072f40f77e3fff4dfea53cba  docs/chapters/06-04-lesson.md
2d846dd3acda928c4c736d54db14c80bd22dbeae831b4d8d32489e07f6a1fc26  docs/chapters/06-05-lesson.md
7e30cb746015e9d5a76fa7e57d866869893717fae34cd09b895cd60bab2a2850  docs/chapters/06-06-lesson.md
bf22ea0e24e449c70861282ff9b2a9e4777fe97dd1764b55b4e16264a897f371  examples/intro-workspace/trait-stages/Cargo.toml
1105dacc52be55109dbd9f2b797778b3e6597ef04a0df54e6efeb7cf18bec0f5  examples/intro-workspace/trait-stages/examples/associated.rs
999cfa843c2ab28b26f123525cd33597787594dff951a6453998b5a1862a0304  examples/intro-workspace/trait-stages/examples/associated_answer.rs
8116faa4739db768fdabd25015c468879d42e9621743633def82579ee9449ba1  examples/intro-workspace/trait-stages/examples/associated_support/mod.rs
29b686ddbd39006071e407063d8e2207b39c21e6ed0863f31f1f0e2d4586fce7  examples/intro-workspace/trait-stages/examples/charge_support/mod.rs
e549a9efda165828e133c81d0d4a65cbed40df215b4727bf6716f18b2bbe1540  examples/intro-workspace/trait-stages/examples/charges.rs
7ecb507c30a3d3afaa6c01a8badd1d1a6af717f1045e38e43001bda4cf1fb38e  examples/intro-workspace/trait-stages/examples/charges_answer.rs
f88172a04c70f704fb0aefefabf7f670219ba9badba2e2415a1b31d05c068f53  examples/intro-workspace/trait-stages/examples/cloning.rs
89a4e6e0b47fffd0524d41a0df3b9d1f3d49d9a64a9ad6b9814255306f64423c  examples/intro-workspace/trait-stages/examples/cloning_answer.rs
8467e40d475365deb3af8633e39314fa3a3e1e49e8ec5c20cb06c6653274734c  examples/intro-workspace/trait-stages/examples/copying.rs
3b8ff510cbeee0f954659ea3bb98b8b33c3eb907a905d99c95fc1880008dd416  examples/intro-workspace/trait-stages/examples/custom_keys.rs
e77daf767da69e7348c871ecc70b91fb2043727fba61df9fb483704a498755fb  examples/intro-workspace/trait-stages/examples/debug.rs
c3a730043d5774e5462a5cfd54fedc8332f0fcdb44a5e63ef3ec80cae101ce0b  examples/intro-workspace/trait-stages/examples/equality.rs
5bd909d17e425094a33529f75546e1b5a837d624f8d41a8b6df7c43f35deeb6b  examples/intro-workspace/trait-stages/examples/equality_answer.rs
25a3f006640450d57e61272513c52fe9fa32bfafa77157aea83db1f2236df152  examples/intro-workspace/trait-stages/examples/grouping.rs
0785f255f09a505c58da74c1f76b88aba22b5fdb711d54afdaaaa1e9eda49499  examples/intro-workspace/trait-stages/examples/grouping_answer.rs
ea8a57e573e122aacf57972ecd74f0774495d31b63d56b150a6a1e3a409fbffc  examples/intro-workspace/trait-stages/examples/grouping_support/mod.rs
71cfb0ba0659f76c15e3346522ef8e9b98b066bc436e6df416ddc1c03e547271  examples/intro-workspace/trait-stages/examples/map_lookup.rs
23319161dd7f4830085b89a4f226b91e3ffc620a042f132e5d1758c091f48489  examples/intro-workspace/trait-stages/examples/map_lookup_answer.rs
c8b46307d1e61c6c9211cc252bb5acd277c5db5b42a2e839384ca6109e3d2ee6  examples/intro-workspace/trait-stages/examples/map_support/mod.rs
13cd0c65e377dbb880a087bc741ee69e59ac374097b8a14dd90082e88d46bff9  examples/intro-workspace/trait-stages/examples/map_update.rs
e442e57e01a4c7f9bccf7a4e96076a5a106f1a6da75376c87511c0d80e98b33d  examples/intro-workspace/trait-stages/examples/standard_equality.rs
e03b27f19f221f0ea2a3809e0da4983870354d9db49f275f5692df404d9c0312  examples/intro-workspace/trait-stages/fixtures/compile-fail/key_without_hash.expected
3c3591bc011548ec70846763eb92085e2a35c69e89c02ca8ed4446552ebb0d99  examples/intro-workspace/trait-stages/fixtures/compile-fail/key_without_hash.rs
f1f10512aef688783e72228eac90052c4477e5fc328ff82296757e6fb41f09af  examples/intro-workspace/trait-stages/fixtures/compile-fail/map_borrow_conflict.expected
751545c6c6e80065bf407cdafa3152a2327260b34bc781752c7cda159f9cd225  examples/intro-workspace/trait-stages/fixtures/compile-fail/map_borrow_conflict.rs
e03b27f19f221f0ea2a3809e0da4983870354d9db49f275f5692df404d9c0312  examples/intro-workspace/trait-stages/fixtures/compile-fail/missing_bound.expected
efade76c33ff192ef1b0460cb45199a138a8b5d24646f766b985adceb5f76810  examples/intro-workspace/trait-stages/fixtures/compile-fail/missing_bound.rs
7dd22f1dc9232d673c5d6f1354af49043359702f187da0bab6c4eaca7c358c36  examples/intro-workspace/trait-stages/fixtures/compile-fail/not_debug.expected
e8c8eeb215523bd8a67d4af2cbfe1ba1c35b27ad0c13c2e045ce99e62cbb037a  examples/intro-workspace/trait-stages/fixtures/compile-fail/not_debug.rs
43322ecdbfcad1364df631b6713d4c750abbf95dd7dbe533851c6a229a195efa  examples/intro-workspace/trait-stages/fixtures/compile-fail/output_bound.expected
9e7be66747981927a947cd4f0efb797572e8aba9f7435950e74c72fda7103536  examples/intro-workspace/trait-stages/fixtures/compile-fail/output_bound.rs
cb4611f64327a3a3e8c8733ec7967c163ae478479c01703e2816e6466fc68341  examples/intro-workspace/trait-stages/fixtures/compile-fail/string_not_copy.expected
c193600f7afbeb09298fc1ad2100d6af6c8e6638ee5e96d650440a82b16c1467  examples/intro-workspace/trait-stages/fixtures/compile-fail/string_not_copy.rs
018c7d106ed7e348d943c40674e26c48ee5965208ace26ca9127a294908b0b30  examples/intro-workspace/trait-stages/src/main.rs
a8322647d83cb8b527cae782ebdd9b9e8202e5c59a44856bbf42083a4745d14d  docs/assets/traits-v3/map-query.svg
```


## root交接精確回指窄修

root完整讀稿後指出6.5的PartialEq來源應直接回指6.2。只把「上一節已用 PartialEq 比較內容。」改為「第 6.2 節已用 PartialEq 比較內容。」；沒有修改Rust或執行路徑，不重跑Rust。上方freeze保留為此窄修前版本；6.5目前SHA-256如下，其餘檔案不變：

```text
2b15a18fcedcfe1d602caaddbb570b6e921f776a67ea932f49f965a7d3a947f5  docs/chapters/06-05-lesson.md
```
