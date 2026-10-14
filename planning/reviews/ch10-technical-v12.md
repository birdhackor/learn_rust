# 第 10 章 TECH ORIGINAL initial：v12

本輪確認兩個需要補最小介紹的首次用法：10.2 測試的數字 literal 型別後綴，以及同段的 `cargo test --lib`。另列兩個可選前文／名稱來源改善。沒有確認核心 thread、鎖或 atomic 的技術錯誤；其充分性仍按正文條件與官方模型判斷，不由程式跑通放行。這是第 10 章非作者的獨立技術初判，不是 blind 首讀或完整發布驗收。

## 角色、版本與封存前隔離

- 審閱者：`chapter02_ownership`，不是第 10 章作者。先前曾撰寫 ownership、lifetime、error 與 9.2–9.4，也做過第 3／7／8 章銜接審閱；使用實際前文的背景如實保留，不把專家常識列成學生先備。
- 唯一第 10 章來源：`/workspace/work/tutorial-implementation/frozen/ch10-complete-prerequisite-v12`。manifest SHA-256：`b22762f35195ce6fc287cca720b5fa6812b519aaa0f8ef4e9a76c9e2e291632e`。
- 先按順序讀完整 8 頁、68 BODY，保存自己的分節四題／疑點；之後全文讀 manifest 全部 121 個 archived sources、回查 actual 先備、selected 官方契約、有限探查與原位圖。沒有把作者大綱當依賴證據。
- 121 份來源為 6 個 library Rust（含 root lib）、47 個正常 example Rust（含八份完整答案）、26 個 compile-fail Rust、3 個 runtime-fail Rust、29 份 expected、1 個 package Cargo.toml、9 個 SVG。library 只有一項 test，在 workers.rs；完整 helper／test 也讀，不重複增加計數。
- 本輪未讀 live10、新版10、FIRST10、任何 peer10、root notes／處置或第 10 章作者 ledger／source notes。全章原初判封存前沒有交叉提示，沒有更改學生 source、正文或根 Cargo／lock／nav。
- 案例的 stdout、失敗條件、ownership、等待、業務政策與型別能力分開核對。無 Tokio 或本章外 async API；第 11 章只看本稿末尾的過渡目的，未開正文或同行紀錄。

`work/ch10-technical-v12/` 保存 input／prerequisite 指紋、原 BODY 筆記、source-batches、官方原 HTML／選段、own-lock probe、原位 screenshot 與封存副本。無 required 以外的修稿，本文件只提出待處理項。

## 兩個必要的首次介紹缺口

### N1：測試首次使用 `7_u32`，尚未教 literal 後綴

- 位置：10.2／002-005，workers.rs 的 `accepts_both_text_types_and_reports_other_types`；後續 10.8／008-008 的完整答案另使用 `80_usize`。
- 最小原句／程式：先說「完整測試直接建立 String、static 文字與 u32 的 payload」，接著直接寫 `panic_message(Box::new(7_u32))`；沒有說 `7_u32` 的尾端是在指定 literal 型別。008-008 則直接寫 `let amounts = [80_usize, 120, 30];`。
- 已教部分：actual 1v8 的 004-002 已教 u32／usize、`let price: i32 = 120`、型別推導，也教數字**中間**的底線分組如 `1_000`。但這沒有交代數字尾端的型別後綴。查 canonical 1–8 全 BODY，沒有該後綴的介紹或使用可回收；本章 002-005 前也沒有。
- 缺少關係／影響：讀者要理解測試明確建立哪種 payload，並自己寫對應範例，需知道 `7_u32` 是 u32 literal、不是名稱或型別轉換；Any 檢查確切型別，這個選擇與材料身分有直接關係。後面的 usize array 同樣需要此語法。編譯成功不能補成讀者已教。
- 分級：**必要、增加理解負擔；語法／首次使用**。不是 panic payload 契約或並行演算法錯誤，不把它升為整章主概念無法理解。兩處是同一未教語法，計一項。
- 最小修正：在第一次測試前補「數字後的 `_u32` 指定這個 literal 是 u32」及其與已教型別註記的關係；後面的 `_usize` 可回指。也可直接沿用已教寫法：先 `let other: u32 = 7;` 再 Box::new(other)，答案的 array 寫 `let amounts: [usize; 3] = [80, 120, 30];`。兩種都不需擴寫數值章或新增測試。
- 證據與時點：自己在 BODY 留待查後，回到 actual 1.4、全 canonical BODY 搜尋及完整 sources 確認；另實讀 Rust 1.99 Reference 的 **integer literal expressions**，核後綴指定 primitive integer type。未受同行／root 問題提示，未修改程式。

### N2：`--lib` 首次進入測試命令，未說選取作用

- 位置：10.2／002-005，命令 `cargo test -p ch10-concurrency --lib --locked`。
- 已教部分：actual 1v8 的 010-003 教 `cargo test -p ch01-basics --locked`，也說可能列出 binary／文件測試；010-004 明教 `--all-targets` 與 fmt 選項。前文已有 library／binary／example 分工與 `-p`／`--locked`。
- 缺少關係／影響：canonical 1–8 全 BODY 沒有 `--lib`。當段要求用新選項執行，卻沒有交代它讓 Cargo 只選 package 的 library 測試 target。讀者能複製命令，不代表能說明為何此次不會同樣選取其他 test targets；也不能從上方 `mod tests` 反推出 Cargo 選項語意。
- 分級：**必要、增加理解負擔；操作／首次使用**。這是很小的工具契約缺口，不是測試本身或 thread 結果有錯。本人完成有限探查後再查漏發現；探查時由審閱者知道該旗標，不回算學生先備。
- 最小修正：命令之前加一句「`--lib` 只執行這個 package 的 library 單元測試」。若不需要這項 target 限定，可沿用已教的無 `--lib` 命令。無需重跑未改 Rust，也不要求重講 Cargo。
- 證據：實回讀 actual 1.10 兩段，搜尋 canonical 1–8，並實讀 Rust 1.99 Cargo Test 的 Target Selection：「`--lib` — Test the package’s library」。自己封存前提出，沒有同行提示。

## 兩個可選改善

### O1：as_mut_slice 的回指應是 3.6

10.4／004-007 練習寫「使用2.6已教的 as_mut_slice 與 for value」。actual 2v5 的 2.6 是 array slice／借用與索引；Vec 的 `as_mut_slice()` 在 actual 3v6 的 **006-002** 操作表及完整程式才教。學生在第 10 章前確實已學會，所以不是新 API 超前，也不阻礙此處操作；只是讓回讀者找錯前文。最小改為「使用3.6已教的 as_mut_slice」，for 走訪亦可沿已教 6.5 契約理解。**可選：引用定位**。BODY 提出候選、實回讀 3.6 後確認。

### O2：第一次省掉 drop 的 use，可補 prelude 名稱來源

10.3／003-006 的完整例子直接 `drop(text)`，沒有 `use std::mem::drop;`；之後多份完整例子亦如此。actual 8v9 的 002-003 曾明教「可以用 use 引入它，之後只寫 drop」並在完整程式寫 use。清理契約已充分，本章這些 calls 也合法：標準 prelude 已提供該函式。可在第一個省 use 的例子前補「drop 也由 prelude 提供，可以直接使用」，或沿早先示範保留 use；不加新機制／清理規則。**可選：名稱來源的一致性**，不把它當漏 import 或未教 Drop。actual 5.4 的 prelude 背景來自先前 actual 已讀與第 8 章覆核；本輪另實讀 1.99 prelude re-export 確認。

## 實際依賴與主要方法的通過範圍

以下保持原需要，分開身分／操作、機制與用途、例子證據；必要項 N1／N2 沒有因全節主線清楚而被蓋掉。

| 原稿範圍／首次需要 | 身分與實際先備 | 機制／性質的支持範圍 | 原需要、例子作用與界線 |
| --- | --- | --- | --- |
| 10.1／001-001～008 | thread／stack、concurrency／parallelism 在001-002先教；Send／Sync marker、auto、std::marker／prelude在 helper 前；8.4 require_static、8.8 Cell／RefCell、6.1 capture 由實際已讀背景支持。 | Send 是值交接；Sync 精確等價 &T:Send；普通 struct 按欄位組成。Fn 系列、跨 thread 能力與 static 借用依賴三個維度分開。Rc 的普通計數亦是資料，move closure 不改其共享保護。 | 原需交出整份資料或保留 owner 只借。helpers 明說只編譯期核契約、不建立 thread；Batch200／Cell4／RefCell200不是跨 thread 實測或速度。Cell／RefCell有Send無Sync，Rc連Send也無；失敗型別／來源各定位。 |
| 10.2／002-001～008 | spawn 的 FnOnce／F和T Send+static，JoinHandle 的 owned 等待權限，join blocking、泛型 type alias、BoxAnySend payload、unwind／abort在使用前教。6.3呼叫能力、8.10 owned downcast、5.1 Debug/expect與既有測試分工均有實際前文。 | join 消耗權限，正常結果與 panic payload不同；handle Drop detach不等不取消。工作自己的 Result是內層，正常回傳業務Err仍是外Ok。String／staticstr downcast失敗保原SendBox，未知型別只通報。 | 原需80/120算200、主thread算6、等待後206；例子安排等待位置，未量測加速。兩份練習先各join存結果再match，任何分支均等過兩份；兩種文字加數值測試不靠當次panic型別代答。N1／N2仍待修。 |
| 10.3／003-001～009 | 6.3 function pointer／Fn hierarchy、4.4回傳reference與8.5 where已教。新for量化在003-004先定角色、作用範圍、T在量化之外，再用於局部callback；明寫後才教elision。 | caller外'a只能保那次能力，不能借函式局部value到任意外'a；for給當次選合法短借的通用能力，不是延壽、迴圈、static capture或重複呼叫能力。輸入／輸出同'a投影與固定T分開。 | 原需共用函式自己建立80、只借給加120規則、交200；外text view合法、局部number逃出失敗，支持來源判斷。第二欄投影120/6得126、FnMut兩局部值100/140和visits2，確有各次來源差異；不用後文thread解釋普通callback。 |
| 10.4／004-001～008 | Scope context／兩lifetime env:scope、HRTB管理callback、worker Send+scope與ScopedJoinHandle在使用前教。thread-local只補必要角色，不偷用API；普通mutborrow與T:Send在004-004先說。 | Rust1.99自動等body與手動join等TLS清理的範圍精確區分；callback在caller thread不需Send。丟handle不立即等，但scope仍追蹤。callback內才建來源仍不滿scope bound；結果可借外env，handle不可逃scope。 | 原需worker只借80/120後owner還能push；borrow200後len3、外total寫200後scope讀、兩Vec各mut+1得211，分開保活／borrow／業務成功。手動panicErr可處理、未joinpanic自動向scope傳播；完成等待不回復部分修改。 |
| 10.5／005-001～008 | mpsc全名、端點／buffer、SendErrortuple／RecvError、send／recv receivers與結果，在例前。Sender clone、Send／Sync與Receiver非Sync／Clone、owned迭代／?、while let在首次code前說清。 | unbounded send只accept不等空位，不保已收／處理；allSenders消失後仍drain才Err。bounded限制待收筆數，非全程記憶體；容量0成功保已收到但不保處理。協作關閉不強制cancel，餘未走訪值清理與未送值不同。 | 原需worker逐項交出五項206，main集中處理，drain後另join核工作完成。省掉原sender Drop造成永久等待的理由可推，不執行壞例。先join bounded會與等空位互等；容量0只recv80確定120失敗，不靠sleep／排程；小數值所有交錯不overflow。 |
| 10.6／006-001～008 | Mutex全名、guard Deref／DerefMut／Drop與lifetime沿8.8；LockResult alias、PoisonError持成果／Debug、Arc全名／atomic計數最低角色，皆使用前教。 | Arc只保活及保護計數；Mutex<T> Send／Sync需T:Send，不需TSync；Mutex<Cell> guard護整段get/set與Arc<Cell>不同。guard!Send、重鎖未指定返回／deadlock／panic、join前先drop guard與owner生命周期分開。 | 原需兩路共享總額0+80+120=200，與channel集中處理是分工差別。Totals80/0的panic揭露invariant破壞；error持guard、reset已知0/0再clear，政策明選丟棄未完成狀態。poison advisory、不poison不等業務正确。 |
| 10.7／007-001～010 | RwLock兩guard／能力、read/write與try結果、WouldBlock無guard／Poisoned已有guard、Debug、平台優先限制，在操作前教；downgrade先說穩定版本1.92、ownership與回傳，再code。 | RwSync需TSend+Sync，與Mutex不同；read guard並存而writer互斥、最後使用不Drop。downgrade不中斷write→read、無reverse upgrade。read→release→write需重查；readerpanic不poison、writerpanic可能poison。 | 原需兩reader共有同Vec200、全drop後push30成230。重查例主動插另一更新由len2→3，非用sleep賭交錯。完整追加policy保留30前先核每項，不以sum/len代內容；一致兩鎖順序只排特定循環，沒有宣稱排全部deadlock／starvation或比較速度。 |
| 10.8／008-001～009 | AtomicUsize／Bool own非Copy、sharedAPI、load/store/fetch_add舊值／wrapping、Ordering五值與可用方法先教；RMW、memory model、happens-before、yield／busywait亦在需要前。後面80_usize併入N1。 | 同位置修改順序、RMW接前一值，五次＋joinHB後Relaxed最後讀5；唯一一次發布的Acquire真的讀自Release，DATA寫HB讀，沒有後寫所以230。所有四步SeqCst的共同順序排除0/0；沒有把強序說成合併split更新，沒有把Relaxed未見反例當保證。 | 原需單一計數各增加一次，另需READYtrue含230發布。單threadlostupdate示合法交錯非頻率；SeqCst循環是假設矛盾非軌跡；twoPayload3/230靠一次發布前提，重複更新不保批次一致。發布先load後join、不能由後join代解先讀；poll不保公平／固定時間收尾，失敗未發布沒協定明說。 |

本輪按需要實回讀的 canonical BODY 最初13段及封存前的 actual1測試兩段保存指紋；不是新讀完整1–8。包括 actual1的 workspace欄位／數值literal、actual2 reborrow／最後使用、actual3 Vec契約、actual4 outlives、actual5 expect Debug、actual6 FnOnce／Fn／function pointer／owned iterator、actual8 std::mem::drop／static型別bound／owneddowncast。第7／8章的已有實讀背景來自本人已完成的原／修後銜接路線；這些未當作新blind或大綱證據。回查新CLI選項／suffix時更搜尋全部 canonical BODY，搜尋結果與真正已教原文一起判，不只憑沒有關鍵字作結論。

## 官方契約與 memory ordering 的查證

標準函式庫／Reference／Cargo來源固定 `https://doc.rust-lang.org/1.99.0/`；std原HTML也核版號1.99.0。真正讀到的 selected 範圍：

- Send／Sync：auto能力、`&T`／`&mut T`的跨thread條件、Rc／Cell／RefCell限制；沒有把官方unsafe手寫實作當學生先備。
- spawn／JoinHandle／scope／Scope::spawn／ScopedJoinHandle：F與T各bound、丟棄與panic、普通join及scoped手動join的TLS清理與完成同步，scope自動只等body；HRTB Reference支持for能力與scope簽名。
- Arc／Mutex／MutexGuard／RwLock／WriteGuard：TSendSync／MutexTSend／RwSyncTSendSync、持guardDrop／noSend、poison advisory、Err帶已取得guard、clear責任、重鎖未指定、Rw平台priority／readerpanic與writerpanic，1.92穩定downgrade的atomic權限轉換。
- Sender／Receiver／SyncSender／SendError：recv的alive／drain條件、Senderclone無TClone、SenderTSend才能Sync、Receiver!Sync、sendaccept不保最後收到、容量0會合、錯誤含原值、SendError無需TDebug也有Debug。
- Ordering／AtomicUsize及其 aliased class 的 **usize methods**：合法load／store／RMW order、fetch_add舊值與wrapping、讀寫兩側ordering的差別；panic文字payload型別未固定、Cargounwind／abort及prelude drop。
- Rust atomic module 明說採C++20模型。先查當前eel draft後，再補精確C++20最終工作稿 **N4861** 的 intro.races write-read coherence，以及 atomics.order RMW讀修改順序前一值／SeqCst共同順序；不把後期draft新增API或延伸規則代入原稿。

因此五次fetch_add的每次舊值接上一修改，全部join建立HB後load不能回較早值；發布例由P1→P2（thread內）、P2→R1（讀自唯一Release）、R1→R2（thread內）串接，沒有後續DATA寫所以R2必230。SeqCst四步例只有指定兩位置、兩份寫與四步皆SeqCst：假定兩0要求A1<A2<B1<B2<A1，與共同順序矛盾。splitSeqCst的load0/load0/store1/store1則是合法順序，仍lostupdate。這些是官方模型與原條件的推理，有限執行不能證不存在的排程。

官方原 HTML／URL／SHA與選段在 official-selected.json、official-usize-methods.json、official-cpp20-selected.json 等。首次把方法anchor查在AtomicUsize alias頁失敗，沿alias移到Atomic並再定位usize的load-11／store-11／fetch_add-9；沒有把先出現的bool／i8方法當usize證據。literal初次搜尋落到character段亦未採用，重選integer段。輸出曾截斷的條文另外取所需段落；不是頁面下載存在就算讀過完整API。

## 本輪有限探查

隔離工作區在 `work/ch10-technical-v12/own-workspace/`：從原archive複製全部112份package材料（不含9SVG），逐byte核同原稿。成員Cargo保持原bytes；隔離根只提供actual1已教的version0.1.0／edition2024／rust-version1.99／publishfalse共同欄位與resolver3。自生成std-only Cargo.lock，只有ch10-concurrency一package，SHA-256：`937ce7203f57cbba3bc5388bacbb09f3d7b1bdd04739f2dee615e7d7871b9064`；未用根lock或跑rootworkspace。

新shell先source scripts/env.sh，Rust1.99.0、host x86_64-unknown-linux-gnu，自己的target目錄。完整argv／cwd／stdout／stderr／exit在run-probes.py與probe-results.json，目的先固定：

- 唯一library test：`accepts_both_text_types_and_reports_other_types`，1 passed／0 failed。直接建立兩種文字與數值payload，獨立驗fallback而不由一次panic的實際型別代答；N1教學缺口仍成立。
- 六個normal：scoped_panic、channel_stop、mutex_poison、rwlock_downgrade、atomic_counter、atomic_seqcst_split。全部exit0、實際stdout逐byte符合原稿文字／空白／換行；有workerpanic的main仍0，stderr保留預設panic診斷。
- 六個compile-fail：callback_fixed_lifetime E0597、callback_only_static FnOnce不夠通用、callback_local_escape lifetime診斷、scoped_local_source E0373、scoped_handle_escape lifetime診斷、rwlock_guards_not_send。均編譯exit1、實讀相應錯誤位置與原因，含原expected；最後一份有兩個E0277，分別指reader／writer guard，不混成同鎖存取衝突。
- 三個有限runtime-fail：scoped_auto_panic、atomic_load_release、atomic_store_acqrel。先編譯0，執行101，stderr各含相鄰expected；scope fixture沒有印「scope正常返回」。不把runtime失敗當compile錯。

其餘41份normal／答案與20份compile-fail都全文核讀，但未執行。沒有跑完整all-target checks／Clippy／fmt、其他平台、atomic_publish／atomic_solution輪詢、故意deadlock或沒有收尾協定的壞例；沒有用yield／sleep／一個host觀察證明fairness、無反例或所有ordering情況。有限probe只確認選定使用與預期結果，未當作整章全部測試驗收。

## 九圖原位視覺與必要圖檢查

由原68BODY按原順序重組同theme獨立preview，只移除重複H1、改本地asset URL；不開live10／新稿／同行實頁。Chromium在1280×1200及390×1200各拍九張，**18張均view_image親看**，HTTP實載每SVG另核originalarchive bytes。desktop intrinsic400px，mobile約343px；24px字縮後約20.6px，不需點放大。title／desc與明確width／height也核來源。

| 原位圖／BODY | 親看與正文支持的必要對應 |
| --- | --- |
| send-sync／001-004 | 綠箭交String ownership A→B；藍借用箭由B的&String指A仍持文字，兩種能力不混，圖明是允許關係。 |
| thread-join／002-003 | 主thread6／worker200兩路，join等包含清理後200交入6+200=206；縱位置不是排程量測，前後說明相鄰。 |
| hrtb-local／003-004 | F任合法borrow能力與本次80局部來源分開；藍箭借給callback、綠成果200越body框，沒帶局部ref出外。 |
| scoped-wait／004-004 | 外values／total在scope框外，callback返回後另有等待body；worker寫200再scope返回，並未將TLS清理畫成自動全部等待。 |
| channel-drain／005-004 | 原sender加兩clone／workerA80,120及B1,2,3合流同buffer；所有端點消失後仍drain、最後另join，沒有把三份權限畫三個channel。 |
| arc-mutex-guard／006-004 | 兩Arc連同Mutex0、一種main120先／worker80後順序，兩guard輪流、join收尾再讀200，非兩份總額／同時修改。 |
| rwlock-readers／007-003 | read1/first與read2/second同鎖；灰first已解除仍綠second活躍，WouldBlock是沒writer；全drop才writer追加30，230屬同清單。 |
| atomic-publication／008-005 | P1 DATA230Relaxed→P2 READYtrueRelease，藍線明須R1讀自P2才同步，再R2 DATA230；兩欄是同共享位置。該配對與非共同時間軸說明相鄰。 |
| seqcst-cycle／008-007 | 藍A兩步／綠B兩步，灰內先後、紅假設讀0的要求；紅B2回A1矛盾可追，不當實際執行軌跡，X/Y與tuple欄位映射在下方。 |

沒有確認的必要圖缺席：nested Result可以由分層match／文字直接判；poison80/0→0/0的政策與狀態有有限對照；重取len2→3與downgrade230→270亦已有完整順序與契約，沒有必須再想像的未交代資料流。既有九圖必要字、箭頭與相鄰解釋在這些畫面清楚；一般長程式在手機使用水平捲動，未另逐code block操作驗收。

局部preview strict=false，建置無issues只證其可建，不宣稱正式strict站點、所有fold／下載／nav／外部link／發布通過。原始圖與截圖保存，沒有修改字型、SVG或借root親看結論。

## 原初判封存與停止

必要2項N1／N2、可選2項O1／O2，保留其最小引用、理解影響、實際先備及分級。其餘技術／方法理由只在原稿明定條件與本輪實讀範圍通過；未驗證的執行／平台／正式發布與真人讀者回饋另列，不自動報缺陷也不填成pass。

本文件、SHA sidecars與work immutable initial副本封存前再核原v12全部材料未變。先封存原分類，之後才可依root新授權互看；不提前讀peer、回寫原判或自行修source。完成此original任務後停止。
