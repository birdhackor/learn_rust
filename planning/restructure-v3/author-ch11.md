# 新第 11 章作者交接：11.1–11.2 freeze

日期：2026-10-15。本次只完成 root 正式放行的 11.1、11.2；11.3–11.11 尚未寫新正文或 Rust。角色是作者及作者自查，獨立審閱與網站驗證另由 root 安排。

## 所有權、實讀與 gate

只寫 `11-01-lesson.md`、`11-02-lesson.md`、`examples/intro-workspace/type-stages/`、`docs/assets/types-v3/boxed-chain.svg`、本檔與 sources-ch11.md。root 加入 intro-type-stages member 並更新 lock；作者未改 shared Cargo/nav/checks、其他章或原始審閱紀錄，未 commit/push。

root 交接 actual 1–7 後，本作者承接已實讀的 1–5 與自寫完整7；另親讀 actual 6.1–6.6 全部正文及必要 clone/copy 標準契約 source。11.1/11.2 使用的能力只需 array/slice/ref/usize/泛型／bound／module／test／enum／match／loop/move，沒有 From/ToOwned/closure/Iterator/Display 先備。即使 root 隨後交接 8.3、8.4、9.1–9.4，也未將它們寫進這兩節。

已重讀舊 8.1 全文、sizes/fixed_arrays/chain 真實 helper、caller/答案／四個 fixtures 與 expected；前次全部舊 8.1–8.10 與 152 份 source 讀取仍在 preparation/bookmark 保存。已全文讀 inventory-late/report.md 與 work/audit-packets/ch11.json；A0610-C02 的遠指問題改為直接點名 examples/size_support/mod.rs，再相鄰完整顯示該檔。

## 首次使用 ledger

| 新概念 | 先備 | 首次解釋與使用位置 | 讀者能判斷什麼 |
| --- | --- | --- | --- |
| Sized | 1.7 byte、1.8 array、6.1 trait/bound | 11.1 開頭固定 i32/array 大小後 | 固定的是型別表示，不是內容值或儲存區域。 |
| size_of／預設 T:Sized | 5.1 泛型與turbofish | 11.1 fixed_bytes snippet之前 | 不接收值，查型別bytes；泛型預設bound足以呼叫。 |
| DST／reference自身大小 | 3.7/3.8 slice/string | 11.1 第二段與五列表 | str/[i32] 的資料大小與 &str/&[i32] 表示分開。 |
| size_of_val／?Sized | shared reference、先前Sized | 11.1 borrowed_bytes snippet之前 | 接收 &T、查資料bytes；放寬而非要求unsized。 |
| &T不會自動解除Sized | 泛型型別對應 | 11.1 sized_default完整fixture | word:&str對應T=str；缺少?Sized導致E0277。 |
| const N:usize | 5.1泛型、固定長度array | 11.1 fixed_len snippet之前 | N是編譯時選定的值，不是型別或執行期變長。 |
| `[T; N]`／turbofish值argument | 上一列 | 11.1 fixed_len與完整caller | 由型別推導或明選N；長度不符E0308。 |
| array_bytes答案 | size_of、const泛型 | 11.1練習前已說n×elementbytes | 改內容不改大小；空array大小0。 |
| 遞迴型別／E0072 | 4章enum、11.1固定表示 | 11.2第一段及完整負例 | 有限End資料不替直接內嵌Chain取得固定表示。 |
| Box／Box::new | 1.7heap/ownership、5.2prelude | 11.2尺寸問題後，number-box snippet之前 | owned間接保存；Sized T的Box是固定單pointer，不等於Box總在stack。 |
| Box dereference／借讀借改 | 3.3–3.5 | 11.2 number-box snippet前後 | `&*`與`&mut *`借同一值；最後使用後才能交接。 |
| Box move／owned move-out | 3.2move、3.6不能從shared借用move | 11.2 take-owned snippet之前 | Box不因T可Copy而Copy；持有Box可以取出Amount，不能推成所有pointer通則。 |
| next:Box<Chain> | Box與E0072 | 11.2 chain-type前後及圖 | 目前節點保存handle，下一份另處保存，尺寸循環因此中斷。 |
| 消耗鏈的into_total | match/partial move/loop | 11.2 into_total前後及三列追蹤表 | 每輪接手下一份owned Chain，End結束；0與負數仍是資料。 |

## 逐 target source closure

所有教學命令使用 intro manifest、`-p intro-type-stages`、`--example`、`--locked`。package 明寫 name/version/edition 2024，無lib、依賴、root inheritance或隱藏helper。每頁同頁完整提供 manifest、所有當下source、fixtures+.expected與答案。

| 當下位置 | 實際編譯source（相對type-stages/） |
| --- | --- |
| 11.1 run/test sizes | examples/sizes.rs＋size_support/mod.rs，test時加同檔一test；無Box/Chain。 |
| 11.1 run const_generics | examples/const_generics.rs＋array_support/mod.rs，只有fixed_len。 |
| 11.1 run array_bytes_answer | examples/array_bytes_answer.rs，自足。 |
| 11.2 run boxes | examples/boxes.rs，自足Amount。 |
| 11.2 run/test chain | examples/chain.rs＋chain_support/mod.rs，test時加同檔兩tests。 |
| 11.2 run chain_answer | examples/chain_answer.rs＋同頁完整chain_support；不載入後續節。 |

每條命令之前，當下的完整source及manifest都已列出。作者Clippy明選六個targets，不採--all-targets作早期學生指令；後續新增source仍不應改動這些快照或共享helper的已凍結內容。

## 實測與freeze

工作證據：`/workspace/work/tutorial-implementation-20261015/ch11-authoring/`。每個Cargo/rustc shell先source scripts/env.sh，compact env：JOBS=2、INCREMENTAL=0、DEV/TEST_DEBUG=0、COLOR=never。

- stage-11-1-checks.json：11.1三run、一test、指定Clippy成功；兩個初期負例實測在stage-11-1-fixtures.json。
- stage-11-1-11-2-checks.json：10條命令成功，包含6run、sizes一test、chain兩tests、指定六examples strict Clippy、僅own package fmt --check。未跑root全套。
- stage-11-1-11-2-fixtures.json：四個預期非零編譯結果，分別E0277/E0308/E0072/E0382；沒有依靠額外未知錯誤湊失敗。
- source-closure-11-1-11-2.json：兩頁snippet/region有效，以及每條正文命令之前完整提供的檔案清單。
- actual-6-7-fingerprints.json：已放行前綴指紋；實讀範圍以上文為準，不以列hash代替閱讀。
- boxed-chain.png/svg-bounds.json：作者親看Chromium渲染的390×570圖，原owner/兩個heap位置、next Box與owned箭頭可追蹤。Book兩張必要原圖也已實際看過，研究cache與manifest另存preparation。
- freeze-11-1-11-2/、freeze-11-1-11-2-hashes.json：本次owned檔案與SHA-256保存。後續新增章節與必要修正另append版本，不覆寫這份原freeze。

## 後續與限制

完整第11章snapshot隔離與保全計畫在preparation/snapshot-plan.md。Sized/DST/constgenerics、Box兩節已完成；Drop/Deref/'static/dyn compatibility/Error/Rc/Weak/Cell/RefCell/newtype/Cow/Any均保留未來分段，未刪除，也未提早寫source。

8.3 Display、8.4 From與9.1–9.4已由root報ready，作者尚需親讀後續節實際必要正文/source；8.5/8.6、其餘9與10尚未因本任務變成可用前綴。正式後續寫作仍等root逐節放行。Error尤其等Display＋dyn/'static都實際成立後才寫；不借未完成12工程、13threads、15unsafe作先備。

本次僅作者自查與單張圖實際渲染；尚未建置／觀察Zensical桌面手機頁面，未做獨立逐段首讀、技術或continuity審閱，不宣稱accepted。

## 後續授權與 Cow 更正（保留原 freeze）

root 實讀 11.1/11.2 後正式授權依序完成 11.3–11.11，前一節作者完成／凍結後可作後節先備。作者已親讀 actual 8.1–8.6、9.1–9.4 全文；必要 source 在使用前另核對。9.5–9.7 當時未交接，故不採 map/filter/collect/sort。

root 隨後核對 Astra 原核准 outline，確認 11.10 是 newtype/Add/orphan/coherence，**沒有 Cow**；正式撤回先前新增 Cow 的要求。保留作者原先發現舊 8.1–8.10 與 source 無 Cow 的紀錄；preparation/snapshot-plan 的 Cow 提案及上文待辦由這項更正取代，不新增 Cow 教材、快照或研究。ToOwned 仍可依 actual 8.5 用於其他實際需要。

## 11.3–11.5 中間 freeze

三頁已完成；11.1/11.2 的正文、source、manifest、expected、圖均未改，原24檔freeze保留。此ledger只追加版本，不回寫原記錄。

| 新概念／契約 | 首次位置與先備 | source 隔離 |
| --- | --- | --- |
| Drop/destructor、drop方法 | 11.3第一段→tracked短snippet；6.1trait、3.2ownership | 四個Drop examples各自自足，不載其他helper。 |
| 局部反序／欄位順序 | 11.3 drop_scope後／drop_fields前 | 各完整main+型別在命令前，輸出分辨scope與field。 |
| mem::drop接手／不是直接method | 11.3 drop_early之前／direct_drop之前 | 四個完整負例+expected；Copy限制及Drop partial move另分清。 |
| Deref Target／receiver output | 11.4 deref snippet之前；6.4associated type、7.4receiver | deref_basic只含Wrapper/Deref/DerefMut，沒有into_inner或其tests。 |
| DerefMut／借用式coercion／method lookup | 11.4分段首次使用前 | coercions自己的basic helper；String/Vec標準Target在呼叫前介紹。 |
| owned取出與Target trait不繼承 | 11.4負例／into_inner／完整答案 | deref_complete此時才提供into_inner與兩tests，三階段真正不同source。 |
| T:'a／bound組合／implied bound | 11.5 Borrowed短例之前及之後；7.2outlives、11.1?Sized | type_bounds只含Borrowed/main；不載static helper。 |
| static reference／較短借用契約 | 11.5 static_reference之前 | 自足choose_label/main，不載require_static。 |
| T:static／ordinary Drop／owned內含borrow | 11.5 static_bound之前與Box反例 | 自足require_static/Message/main；static_complete到節末才載完整bounds/Wrapper及五tests。 |

11.3：4 run＋指定strict Clippy通過，四fixture實測E0040/E0505/E0184/E0509。11.4：3 run、兩tests與指定strict Clippy通過，兩fixture E0507/E0277。11.5：4 run、五tests與指定strict Clippy通過，兩fixture均E0597。每次僅rustfmt新加入的自有檔案。

work的stage-11-3/4/5-checks.json保存逐命令stdout/diagnostic；freeze-11-3/4/5-hashes.json與對應副本保留版本。三頁首次使用、manifest/helper/main/test/fixture/答案均同頁；只使用已放行前綴。root交接8/9後，作者已親讀actual Display/parsing/conversions/propagation/owned_text/AsRef與RPIT/callback/Iterator-next必要source，不把外部教材程式當已教先備。

## 11.6–11.8 中間 freeze

| 首次契約 | 正文位置與 source 隔離 |
| --- | --- |
| dyn／動態呼叫／共同借用介面 | 11.6 dyn_borrowed，只載 models/Charge 及 generic/dyn amount；不載 factories、借用 payload 或測試。 |
| heterogeneous list／enum 比較 | dyn_list 自有 helper；兩種表示保留同一筆 order，沒有宣稱 enum 做不到。 |
| Box dyn／RPIT／Fn／unsizing | dyn_factories 當下完整 helper；依 actual 9.3 opaque 單一具體型別，區分借用式 Deref 與 owned unsizing。 |
| object lifetime defaults／dyn compatibility | dyn_lifetime 明寫借用 bound；dyn_compatibility 展示 Sized、generic method、Self 輸出、associated Output 各自條件；完整 helper 只在節末。 |
| Error supertraits／source／錯誤鏈 | 11.7 leaf→chain→borrowed outer→boxed conversion 四份不同 helper；沒有先編整份 errors。source 方法明確指名。 |
| outer lifetime vs cause static type | borrowed helper 先教 source 的 receiver 借用和 static bound；report loop 每層 Display 一次；boxed 階段才用標準 From 與 ?。 |
| Rc owner／weak observer／cycle | 11.8 各自足 handles/observer，shared_tail helper 不載 Weak 測試；完整 shared_chain 到答案才載。 |

11.6 六 run、四 tests、strict Clippy、九個預期失敗 fixture 全通過；11.7 五 run、四 tests、strict Clippy、三 fixture 全通過；11.8 四 run、兩 tests、strict Clippy、兩 fixture 全通過。逐命令證據 stage-11-6/7/8-checks.json；原 freeze-11-6/7/8 副本和 hashes 保存。132 snippets、39 條正文命令的逐 target source closure 通過（source-closure-through-11-8.json），沒有 src/lib.rs。

11.6 freeze 後自查只調整一句診斷介紹：不再把省略識別字反引號的句子稱為逐字診斷；完整 expected 和全部 Rust 不變。舊／新 SHA 與理由另存 11-6-diagnostic-wording-followup.json，原 freeze 不覆寫。

root 交接9.5–9.7後 gate 已開；本批作者仍只使用本人已讀 actual9.1–9.4 與迴圈，不因放行訊息自行當作讀過新source。圖檔已保存，deref圖 root 已原位看過；作者其餘 SVG 的實際渲染核對與完整頁面獨立審閱另記，不以編譯通過代替。
11.6 診斷句修訂 SHA：`80c7a4523d91861e7b2a1e50ee5f5726376ce79c37d2e787b96951c05756c4b1` → `2a3988acf0abe4ab51b6d820397cbaed814b5665893f2002ad9af876ae35cf24`。

11.6 root窄修：開頭「非負小數」改「非負小整數」，與i32模型一致；舊hash `2a3988acf0abe4ab51b6d820397cbaed814b5665893f2002ad9af876ae35cf24`、新hash `ec492344b90ad5c776fd17550cde8540c2241161b42d13a70ce72632d6dbb76f`。原freeze保留；Rust/expected不變，未重跑Cargo。

11.9 原freeze後作者核圖窄修：guard圖實際是單一view的最後讀取／try_borrow_mut Err／drop後重試，故圖說依真實對象改寫，並移至try API介紹之後。Rust/expected/圖檔不變，原freeze保留；舊SHA `4213b66b9641c0ec04f89264b261a99a33d4da5d15a225444976758f7de5fe8f`、新SHA `8f935f8c0f04181dd4321c8505c8ac8aac8c27813e4a1a1d7a8dec865d795925`，詳11-9-figure-position-followup.json。

## 11.9–11.11 完稿與首次使用

| 必要新概念／API | 首次解釋與可觀察責任 | 實際 source 隔離 |
| --- | --- | --- |
| interior mutability／Cell | 11.9先以shared Record與普通Vec的E0596建立需要；Cell表教new/get Copy/set/replace/into_inner，再讀Counter。 | cell_values只有Counter；String get負例及expected完整同頁。 |
| RefCell／Ref／RefMut／guard Drop | 11.9借用狀態表在程式前；Deref/DerefMut連11.4，guard最後使用與Drop分開。 | refcell_guards自足；runtime fixture能編譯、執行才panic。 |
| try_borrow／try_borrow_mut／錯誤型別 | 11.9兩種Result先介紹；圖位於介紹後，再以同一view/writable看衝突與drop重試。 | try_borrows自足，不載Recorder；正文用writable，不重現舊writer錯名。 |
| Recorder view／receiver來源 | 11.9回原Vec的Ref guard，不稱clone快照；drop view後才record；E0515分清來源壽命與runtime檢查。 | recorder只有Record/Recorder；complete到答案才合Counter及三tests。 |
| Rc<RefCell<T>>／Weak parent | 11.9明分owner數與guard數；parent RefCell是為建構後更新Weak；children只需普通Vec。 | shared_cells／weak_parent各自足，短block限制guard與暫時upgrade owner。 |
| tuple struct／newtype／alias | 11.10先比較alias同型別，才教建構/.0/雙層pub與沒有Copy；沒有偷驗證非負。 | unit_aliases／unit_identity兩份不同helper，identity無Add。 |
| Add／Rhs預設型別參數／Output | 11.10表先教三個角色，再完整unit_add；same output bound與owned move、方向換算分開。 | unit_add無報告/From/tests；三個相關fixture完整。 |
| orphan／local／uncovered／fundamental／coherence | 11.10從Vec Display責任→本地wrapper→From正反→blanket overlap；Box特殊規則與Vec一般cover分開。 | unit_interface與unit_from各自helper；units_complete最後才合所有介面及三tests。 |
| Any／downcast_ref／downcast_mut | 11.11先教static型別bound與本次借用不同，確切型別不轉換；泛型helper與trait object method區分。 | any_borrowed自足；any_events只有Event與借讀借改，不載TypeId或tests。 |
| owned downcast／Result回原Box | 11.11簽名及兩分支先教，再看圖與完整程式；失敗原資料仍可再試，原名稱move。 | any_owned自足，對應E0382與E0597完整expected。 |
| TypeId／Box receiver | 11.11先教of/PartialEq/Debug與opaque身分，再分內容和Box；runtime assert負例。 | any_type_id只有concrete_id；any_complete最後才合Event、TypeId、兩tests及完整答案。 |

11.9：7 run、3 tests、指定strict Clippy、3 compile-fail＋1 runtime-fail通過。11.10：6 run、3 tests、strict Clippy、7 compile-fail通過。11.11：5 run、2 tests、strict Clippy、2 compile-fail＋1 runtime-fail通過。stage-11-9/10/11-checks.json保存逐命令結果；各節原freeze保留，不覆寫。

本章沒有Any+Send/Sync/Arc版本，沒有提前教跨執行緒契約；沒有使用尚未本人讀取的9.5–9.7方法。正式gate已開不等於作者自動讀過；目前需要均以已讀前綴與迴圈完成。沒有以後章12設定、13threads或15unsafe當先備。

## 全章 closure、保全與最後驗證

- source-closure-through-11-11.json：11頁、193 snippets、60正文命令（50 run／10 test）；每條命令前同頁完整提供manifest及遞迴mod閉包。package沒有src/lib.rs，也沒有workspace欄位繼承或dependency默載後文。完整topic helper只在對應節末載入。
- 10個test命令共28個test cases；全章50個教學run均有逐階段實測輸出。作者各階段Clippy明選當下targets。
- fixture-preservation.json：舊ch08的40份expected名稱與新版40份一一對上，沒有缺項或額外項；38 compile-fail、2 runtime-fail均實測（前四項另在stage-11-1-11-2-fixtures.json，其餘在各stage）。所有fixture與expected完整同頁，不用失敗編譯代替runtime契約。
- 最終僅own package執行fmt --check與Clippy --all-targets -- -D warnings，兩項通過；後者在全章完成後補核cfg(test)內容，沒有在早期正文默載未教例子。環境與命令保存final-package-checks.json。未跑root全套。
- freeze-deltas-before-final.json核對所有原freeze：只有已記錄的11.6兩句措辭與11.9圖說/位置改動；11.1/11.2正文/source/manifest/圖及所有已凍結Rust/expected完全不變。規劃ledger依要求append，不拿它改動冒充原freeze。
- boxed-chain、deref-coercion、dyn-dispatch、error-chain、shared-tail、strong-cycle、guard-lifetime、owned-downcast八張SVG均用Chromium實際渲染並親看，工作png與all-svg-render-bounds.json保留。語意分別核對owned next、兩段Deref、資料/方法表、source原因、同一尾段、假設循環、guard Drop、Result ownership交接。
- 回指逐一核對：11.4的「上一節」明指11.3 Drop；其餘外章回指均具體節號，使用actual3/5/6/7/8/9已教內容。含角括號的教學型別/簽名以code span或code block表示。

作者自查與原圖親看不代替root安排的獨立逐段首讀、technical或continuity；仍待獨立審閱，不宣稱accepted。最終整章檔案副本與hash在work/ch11-authoring/freeze-final/及freeze-final-hashes.json，既有freeze歷史另存。

### 作者實際頁面核對追加

使用work內隔離author-preview.toml／author-docs／author-site，連到目前十一頁實稿與當前stylesheet，不修改root config/nav或他人文件。Zensical build成功；隔離站只納章11，因此11.11連12.1產生一則範圍外頁面警告，repo實際12.1存在，不以此改寫教材連結。

Chromium於390×844、1440×1000共量測22頁：沒有整頁水平溢出，193 snippets均已展開，八圖在兩種尺寸都載入成功。作者親看八張手機原位圖及相鄰正文、手機Cell／RefCell／Add表，另看桌面dyn圖及RefCell／Add表；文字與箭頭關係可辨，表格必要欄可同時讀。截圖、page-metrics.json與author-page-observations.json保存完整量測範圍及實際看過的樣本，不將產生截圖數量當成逐頁視覺驗收。初次隔離站用目錄symlink未複製靜態檔，已在work改成實際拷貝並重建；最後八圖均成功載入。

這是作者版面自查；獨立首讀／技術／continuity和全書nav驗證仍由root另安排。所有stage原freeze保留，最後當前稿另freeze，不回寫原始審閱紀錄。
