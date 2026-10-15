# 第4章後半 4.4–4.7：獨立銜接與實頁初判

封存時間：2026-10-15T07:27:40.166824+00:00。本角色是 continuity / rendered-page reviewer，接續已實讀1–4.3；不是作者、technical reviewer、盲首讀者，也不是全書通過聲明。未讀作者 ledger、其他角色結論、歷史 issues/audits、root dispositions 或第五章稿。Root 指定檢查4.6手機表與4.5字樣／借用對應等 scope，明說未提供尾部已知 finding；此處判斷先獨立封存。

**結論：無新必要缺口；1項可選手機呈現改善。** 四頁43份實際 source/config/fixture快照及hash見 `round04-ch04b-sourcehashes.json`、`round04-ch04b-frozen/`。封存前43份逐一重算，未變動。4prefix的已報窄修通知不改寫原4a初判，也不將修後閱讀當原始首讀。

## C4b-01（可選，低嚴重度／高確定性）：讓4.6批次表在手機同時比較四項資訊

位置 `docs/chapters/04-06-lesson.md:53–58`。原句／表頭：「呼叫前剩餘量｜next_batch 回傳｜呼叫後剩餘量｜body 是否執行」；末列「0｜Done｜0｜否，結束」。390×844實頁中可視scrollwrap375px，內容wrapper434px，table402px。預設左端畫面截去右端「否，結束」尾字；捲至最右59px則左端呼叫前數字消失。桌面可完整比較，手機可捲動取得所有內容，沒有不可取得的資料。

必要規則已在45行教「每次進body前重新計算」，47行說Done不執行body；60行又解釋processed的單次範圍、外層狀態與終止，81行說零項第一次即Done。因此這是減少來回比對負擔的可選改善，不是新先備缺口。可縮短表頭，或保留全部資料改成三欄（例如「剩餘量前→後｜回傳｜body」），保留7→4、4→1、1→0、0→0與More/Done、末列不執行。具體收益是正常手機位置便能同時看清最後Done、剩餘仍0、body未執行三者。

實際證據：`round04-ch04b-visual/mobile-batch-table-left.png`、`mobile-batch-table-right.png`、`table-metadata.json`。最初使用locator自動捲到table會移動水平位置，故另拍明確scrollLeft0/59兩端，沒有將自動偏移畫面誤認成預設。

## 各節正向證據與依賴閉包

### 4.4

3 announces four independent needs;7/9 gives Amount|Estimate shared binding before full or_patterns;23 explains same names/types/borrow before E0408 fixture;33–35 defines one-value range membership and contrasts for traversal;49 explains shadow binding before example;63–67 explains guard selection, false fallthrough and whole-or scope before guards;79 explains nonexhaustiveness before E0004 fixture;89–91 explains @ checks and actual value before source.

299/300 and499/500 test endpoints; Amount80 fails threshold100; count1,false proves ready applies to first or option; exercise combines only earlier mechanisms and for tuple pattern from4.3.

### 4.5

3 identifies String/&String/&mut String question;7/9 explains owned move before consume;27 partial move before source and37 explains remaining count and whole-borrow failure before fixture;47/49 gives automatic borrowed binding types and operations before borrow_patterns;61 explains attempted owned extraction through shared reference before E0507.71/73 distinguishes ref, ref mut, mut and disjoint fields before explicit_bindings;85 distinguishes pattern & from expression & and ref;87 names both lint allows and teaching reason before full reference_pattern.101/103 gives edition2024 restriction before fail;109 introduces binding-mode name after rule already known;115/125 explains _ versus _text behavior before success/failure programs.

String::new/push_str/as_str already taught3.8. consume no later owned reuse; partial move only remaining count reused; repeated show, append, show correspond to borrowed types. Explicit ref uses different label/count fields and last-use rule3.5. Borrow answer mutates String without moving it. No new ownership internals needed.

### 4.6

7 gives right evaluation, pattern match and body binding scope before if_let;9 states &Quote gives &i32 before code and defines else;29 explains deliberate one-variant policy versus match classification.33/35 defines More/Done and mutable remaining helper behavior before named next snippet;41 makes state source explicit.45 defines evaluation before every body;47 Done skip precedes loop;60 states per-iteration processed, persistent remaining/batches and helper-driven termination.85 capacity-positive/nonnegative work precedes answer assertions.

Three More batches3/3/1 then Done update7→4→1→0→0, final body false; zero initial work first yields Done so0 batches. Full enum/helper/main appears inline64–67. Optional mobile table issue is C4b-01.

### 4.7

3 gives need for successful name in later steps and early missing exit;7 defines scope, failure exit and final semicolon;9 defines &i32 and unit return before focused source;19 explains match/if let/let else comparison before complete program.48–50 explains diverge and bad non-exit before E0308;58 defines continue/break direction,60 maps &[Quote]→&Quote→&i32 before loops;76 compares200 vs120 and whole-function return.80–84 defines combined types, pricing policy, missing/rejected distinctness and shared access before complete quote_report;107–109 states answer and repeat-borrow checks.

Uses array/slice and owned String construction already taught; no new container/API. Offered order remains borrowed; full classification show_status retains Missing/Rejected distinction while numeric contribution0 is explicitly local policy. Successful loop path leaves amount for addition; Missing continue still reaches80, break does not. Answer asserts waiting0, small180, boundary300, zero0 and repeat180.

## 完整內嵌、設定與實頁範圍

已實讀4頁全部成功程式、練習答案與失敗fixture：4.4的or/range/shadow/guard/@與組合答案；4.5的owned/partial-move/automatic-borrow/ref/reference-pattern/2024限制/ignore與借用答案；4.6的if-let、while-let完整helper/enum/main與capacity答案；4.7的三式比較、let-else失敗例、continue/break、章末quote_report與Shipment答案。七份相鄰.expected診斷與頁內說法相同。所有full include在實際頁DOM均與來源吻合（只正規化空白、去snippet marker註解）。4.6及4.7先展示named區段，再同頁內嵌全部型別、helper與main，沒有要求從外部連結自行補程式。

讀過patterns四欄manifest、default main、workspace/lock與toolchain。此package沒有lib、build.rs、dependency或隱藏helper；未讀第五章source。沒有執行Cargo/build/Rust tests，沒有改docs、程式或共用設定。此處不重複technical驗證聲明。

桌面1280×800、手機390×844實際開啟4頁，查看range單值與for差別、guard涵蓋整個or、借用binding三型別、ref不同欄位、&pattern/ref對照、while每次重新取得、批次表、let-else scope、章末report用途。已看過的截圖為JSON清單中的20張；不是只存圖未看。所有四頁實際img數皆0，prose／程式註解／表格已建立必要對應，未發現缺少必要圖解。4.5手機換行後&String/&mut String等關鍵字完整；長程式水平捲動是局部code容器，完整來源仍在頁內。其他本輪各焦點strong均正確，沒有可見原樣**。

實頁時點／URL／HTML hash／viewport／full-inline逐項結果：`round04-ch04b-visual/metadata.json`；手機table實際祖先容器及左右位置：`table-metadata.json`。本輪是針對必要關係的實頁觀看與完整source/DOM核對，未聲稱每張長頁每個pixel都看過，partial-nav preview亦非最終strict site驗收。
