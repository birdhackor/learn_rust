# 新第 2 章作者依賴與交接紀錄

角色：作者自查；不是獨立首讀、技術或銜接審閱。日期：2026-10-15。2.1–2.8 已完成作者稿；root 已先閱讀前 6 頁並指出兩處錯回指，已逐筆修正。本紀錄不表示全章已被 root accepted 或完成獨立審閱。外部來源實讀範圍見 sources-ch02.md。

## 實際前綴與入口

動筆前全文讀新 `docs/chapters/01-{01,02,03,04,05,06,08}-lesson.md` 及 `intro-workspace/{hello,greeting,steps}` 對應完整設定與 source。沒有使用尚未完成的 1.7 heap 知識。前綴必要能力是真正的數字/bool/函式/block/scope/tuple/array/index/len/usize/panic；不是舊章號或歷史 ledger。

- 選 package／target：`01-03-lesson.md` 的「一個設定檔」「獨立範例」「Cargo.lock」各段，含 --manifest-path/-p/--example/--locked。
- 型別與顯示：`01-04-lesson.md:5` 起的 i32/運算/named placeholders，`:37` bool；本章沒有用 Debug 格式或未教的 positional placeholders。
- 更新狀態：`01-05-lesson.md:15` mut/+=，`:66` -=；本章 while 的減法不是偷用未教縮寫。
- 函式與scope：`01-06-lesson.md:5` 參數／回傳／尾端expression，`:19` unit，`:68` block與scope。
- 複合值與邊界：`01-08-lesson.md:5` tuple、`:23` array、`:37` usize/len、`:77` panic；完整 array 函式仍固定三項，沒有先引 slice/reference。

## 首次需要與先教位置

| 概念／操作 | 第一次需要 | 先備 | 新主文實際解釋位置與可預測結果 |
| --- | --- | --- | --- |
| 六種整數比較、=與== | comparisons target | 1.4 i32/bool | 2.1「比較把數字變成bool」；恰好300時 >=true、>false。 |
| &&/||/!、非負與有商品不同 | comparisons logic | bool/比較 | 2.1「組合幾個條件」；price120/count0 非負true、有商品false。 |
| 短路與普通函式回傳bool | comparisons helper | 1.6 fn/尾端值 +2.1比較 | 2.1「短路」；四次呼叫只有B、D執行右側並顯示數量。 |
| if/else、if值、條件只收bool | conditions | 2.1 +1.6 block | 2.2「if先檢查」；運費0/20的型別與分支、if_number編譯被拒。 |
| return與省略else | conditions early-return | fn回傳+if | 2.2「return提早結束」；count0不算小計或運費。 |
| 用條件阻止不合法索引 | show_selected_price | 1.8 usize/len/index +if | 2.2「先檢查索引」；傳3只顯示無效，傳1才選80。 |
| else if先後順序 | conditions_answer | 比較/if值 | 2.2「多個條件」；299/300/499/500運費20/10/10/0。 |
| for拿當次整數、外部累積 | for_range | 1.8 array +1.5 mut/+= | 2.3「for每次取得」；120→200→400與名稱scope。 |
| ..與..=、空範圍 | range_sum/inclusive | i32/for | 2.3「range」；0..3總和3、0..=3總和6、3..3零次。 |
| array型別作函式參數 | for_range_answer | 1.6參數 +1.8固定array | 2.3練習明寫[i32;3]；正負兩組得400/0。 |
| while前測與跨次狀態 | while_work | 比較/if/mut/-= | 2.4「每次進body前」與狀態表；7→4→1→0共3批。 |
| zero iterations與終止前提 | while_work_answer | while狀態更新 | 2.4「零次」及練習；work0為0批，capacity必須正。 |
| loop/break、break值 | loop_control | if/block expression | 2.5前兩段；break結束迴圈、main仍繼續，found3。 |
| continue與for/while去向 | loop_control | for/while/break | 2.5「continue」；跳過2總和4；while若跳過必要更新可能不終止。 |
| 巢狀loop label | loop_labels | range/if/累積 | 2.5「巢狀」；break 'search檢查5格後結束，continue 'rows只處理每列1格。 |
| library/bin兩root、crate名稱 | modules-basic | 1.3 package/crate | 2.6「兩個crate」；lib/main分開、hyphen→underscore。 |
| mod/pub/public vs private | modules-basic lib/pricing | fn與檔案 | 2.6「module」；pub mod不自動公開shipping。 |
| ::/crate::/外部crate路徑/use | modules-basic main | module/pub/scope | 2.6「路徑」；use只縮短本範圍名稱、不宣告/下載/執行module。 |
| folder example +mod.rs +子module解析 | order_answer/helper | 1.3 example +2.6mod | 2.6練習在完整答案前先教；order_answer/main.rs真正載display/mod.rs。 |
| assert!/assert_eq!、left/right | control tests/answer | bool/i32/1.8panic | 2.7「assertion」；280與300各自檢查，不把參數順序說成macro強制規定。 |
| #[test]/attribute/runner | control lib tests | fn/unit | 2.7「屬性」先教再展示tests；四項library tests由runner呼叫。 |
| inline mod/#[cfg(test)] | control lib tests | 2.6module | 2.7同段先教同檔形式、configuration與test條件，再展示完整region。 |
| --lib/--all-targets | test/Clippy命令 | 1.3選target | 2.7在首次命令前解釋--lib；TC02T-01修後將--all-targets移到2.8完整答案之後才解釋/使用。 |
| fmt/-- --check、Clippy/lint | 工具命令 | 編譯與測試分工 | 2.7「fmt與Clippy」；格式、診斷、政策正確性分開。 |
| 精確allow與//註解 | lint_return | attribute/return/block | 2.7「先讀訊息」；只對刻意語法比較函式allow needless_return，不關掉warnings/all。 |
| 組合已知module與測試 | integration_answer | 2.1–2.7 | 2.8規格→分工→完整source→實作題與完整答案；沒有新API。 |

## 真實 source 閉包與同頁完整版本

共同入口是 `examples/intro-workspace/Cargo.toml`（root負責members/lock整合），各member自己的manifest只明寫name/version/edition。所有學生命令從repo根以--manifest-path選入門workspace；沒有根維護workspace --workspace、workspace inheritance或外部依賴。

| 節 | package/targets | 實際載入 source 與隔離 |
| --- | --- | --- |
| 2.1 | intro-control-steps: comparisons / comparisons_answer | 只有各自examples/*.rs＋member manifest；無lib。第一個完整檔不含if。 |
| 2.2 | 同package: conditions / conditions_answer | 各自單檔；if_number獨立fixture與.expected，不是正常target。 |
| 2.3 | 同package: for_range / for_range_answer | 各自單檔；沒有while/loop/continue，無lib。 |
| 2.4 | 同package: while_work / while_work_answer | 各自單檔；沒有loop/break/continue。 |
| 2.5 | 同package: loop_control / loop_control_answer / loop_labels / loop_labels_answer | 各自單檔，不依賴共享helper或library。 |
| 2.6 | intro-modules-basic: bin + order_answer | src/lib.rs只有pub mod pricing；載src/pricing.rs。bin載main；答案載examples/order_answer/main.rs與display/mod.rs，並使用同一lib。此快照永久無cfg/test/assert。 |
| 2.7 | intro-control: bin / lint_return / assertions_answer | package lib為完整src/lib.rs＋pricing.rs；任何example也連帶建lib。lib中的test attributes、cfg、assert在本節先教；lint_return內allow也先教。答案main與其test module全檔同頁。 |
| 2.8 | intro-control: bin + integration_answer | 同樣的完整lib/pricing/main，答案自有main與兩項test；所有source同頁，沒有後章module。 |

每頁內嵌完整member manifest、正常program與完整練習答案；library頁另外完整內嵌lib/main/pricing。2.6 folder target的helper全檔也同頁。2.7/2.8以折疊保存完整檔案，不以GitHub或局部region代替完整版本。全文snippet路徑與region存在性已逐項查驗（每頁7/8/6/4/10/9/8/8項）。

## 實際驗證與證據

工作紀錄位於 `/workspace/work/tutorial-implementation-20261015/ch02-author/`。所有Rust shell先source scripts/env.sh，固定1.99.0，compact Cargo env；未跑root全套、未改shared manifests/lock/nav/checks，未commit/push。

- `stage-2.1-2.5-checks.json`：12個指定example逐一run成功，對同12 targets的Clippy `-D warnings`通過；if_number由rustc實際E0308拒絕。
- `stage-2.6-and-lint-checks.json`：modules-basic主要binary與folder answer執行成功、Clippy --all-targets -D warnings通過；移除精確allow的工作副本由clippy-driver實際報needless_return，並建議尾端expression。
- `stage-2.7-2.8-checks.json`：control主要binary與3個examples run成功；library4項、assertions_answer1項、integration_answer2項測試通過；control全targets Clippy -D warnings通過；僅三個作者package的fmt --check通過。
- `threshold-mutation-check.json`：只在工作副本將>=300改>300；library仍能編譯，四測試中只有門檻測試失敗，left320/right300。repo中的正式pricing始終保留>=，沒有把mutant留進教材。
- `owned-target-metadata.json`：實際Cargo辨認的三個owner package targets；證明folder example被辨認，不以檔名推測代替命令驗證。
- `final-author-hashes.json`：交付時所有owner正文/source/manifests與兩份作者紀錄SHA-256；`prefix-at-delivery-hashes.json`保存交付時前綴指紋。這些是實際檔案指紋，不宣稱未做的首讀或視覺驗證。

初次2.2把固定index3與固定array放同一函式時，cargo run被unconditional_panic拒絕（即使有if檢查）；已依1.8模式改為usize函式參數提供索引，修後真實執行通過。初次失敗保存在stage-2.1-2.2-checks.json，不以後來Clippy成功掩蓋它。

Root指出的兩處錯回指已修：2.5 return首次教在2.2；2.6 sum_three首次完成在2.3。完稿後另rg所有上一節/前節/前一節/下一節，逐一核對；目前保留的上一節只指同manifest、2.3for、2.4while，下一節連結逐頁對應正確canonical頁。

## 尚未教與限制

本章不使用reference/String/Option/Result/derive/closure/Iterator API，也沒有struct、slice、trait、自訂Debug或文字比較。for只教整數array與range的當下行為；不把macro內部借用或Iterator展開當讀者已學過。後章需要新能力時仍須先教。

目前完成作者自查與root前段閱讀回饋修正；獨立技術／銜接／首次閱讀與網站桌面手機視覺驗證由root另行安排。本作者未執行網站build或瀏覽器驗證，不冒稱全章accepted。後章可使用的實際能力需以root核對與凍結稿為準。


## TC02T-01 修正交接（2026-10-15）

獨立技術初判指出首次 `cargo clippy --all-targets` 提早載入尚未解釋的 lint_return allow，以及下一節才完整提供的 integration_answer。採用必要修正，不以原先命令成功宣稱先備時序已成立。

- 2.7 首個 Clippy 命令改用已教的 `--lib`，只選 library/pricing；先教精確 allow 並完整顯示 lint_return 後，才明選 `--example lint_return` 檢查。
- `--all-targets` 的意義與完整 package Clippy 命令移至 2.8 integration_answer 完整來源之後。2.7/2.8所有 target 至此都已教／已提供。
- 同段 `cargo fmt -p` 也會讀完整 package，故兩條整包格式命令一併移到 2.8 同一位置；2.7保留工具角色與 `-- --check` 說明，明指後節在來源完整後執行。沒有新增 --bin 或其他新選項。
- Rust source、member manifests、root設定與測試一律未改，不重跑語意測試。兩條修後限定Clippy命令已實際通過，證據在 `tc02t-01-command-checks.json`。
- 修前兩頁與本ledger原稿保存於 `freeze-before-tc02t-01/`；原 `final-author-hashes.json` 保留不覆寫。`tc02t-01-revised-hashes.json` 另存修後全39檔hash與相對原freeze的差異；本節修正說明取代上文原作者時序判斷。

修後等待 root 安排獨立技術複查，作者不自行宣稱必要項已被審閱者關閉。此修改未觸及第7章準備以外的新教學或source。


## C2-01／C2-03 窄修正交接（2026-10-15）

依 root 對已封存獨立 continuity 初判的裁定，只改 2.2、2.4 與本 ledger：

- C2-01：將既有 `return 0;` 立即結束本次函式呼叫、交回0並略過後續工作的操作段落移至首次 early-return snippet 前，沒有新增機制或更動Rust。
- C2-03（root採納可選改善）：2.4批次追蹤表由五欄改成「檢查前剩餘量／本次判斷與更新／完成批次」三欄。7→4→1→0、每次true/false、處理3/3/1項、更新後4/1/0、完成1/2/3批均保留；最後false仍明示不執行body、剩0、3批。表前相鄰文字逐一對應remaining/processed/batches。
- 修前兩頁與ledger、其hash及所有owner範例檔案hash保存於 `freeze-before-c2-01-c2-03/`；既有原freeze與TC02T-01修後manifest不覆寫。修後差異另存 `c2-01-c2-03-revised-hashes.json`。

全部Rust、member manifests與其他教材不變，未執行Cargo。沒有修改首讀已凍結packets或原始records。手機／桌面表格驗證由root重建後的獨立continuity進行，本作者尚未做版面驗證，不宣稱該項已通過。
