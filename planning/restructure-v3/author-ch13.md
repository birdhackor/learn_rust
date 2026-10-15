# 第13章作者交接：thread、同步與 atomic

目前 **13.1～13.8 全章作者稿、source與限定檢查完成並依序凍結**。依AGENTS、clear-tutorial、寫作prompt與術語指引撰寫。本文是作者自查紀錄，不是獨立技術審閱、首次閱讀或網站版面驗收。

## Actual prefix

收到root正式放行後，本人親讀現行9.1～9.7、10.1～10.6、11.1～11.11、12.1完整正文，另讀實際需要的closure／Fn／Iterator／Box／Drop／Deref／static／dyn／Rc／Cell／Any完整selected source與manifest。共65檔的路徑、完整SHA-256及讀取range保存於`/workspace/work/tutorial-implementation-20261015/ch13-authoring/actual-prefix-read.json`。沒有用outline、root摘要、舊章或作者ledger代替實稿。10.7 Serde／10.8 TempDir不是本章std-only source的依賴，未借用這兩節作先備。

本次實讀的12.1是202行，hash為`5f7ab3792ff83635cc74b2147311ae448cd914c74f18e86c96aa54c0e3cf61f0`；已向root回報與放行訊息中的早期hash不同；root確認是TC12P-O1單句精確化，Rust/設定未變，本文依現行完整契約使用workspace inheritance。原第8章freeze不動。

## 概念與首次需要的位置

| 必要契約 | 真實先備／本節責任 | 讀完可判斷 |
| --- | --- | --- |
| 13.1 Send、Sync、marker／auto trait | 11.1 Sized／?Sized、11.5 static、11.8 Rc、11.9 Cell／RefCell；本節先解釋移交／shared reference與等價關係，再helper與source | Cell可Send但不Sync；move不替Rc增加Send；型別能力與來源有效性分開 |
| 13.2 spawn／JoinHandle／join／thread::Result | 9.1 capture、9.2 FnOnce、11.6 where／dyn；本節先教F/T兩邊Send+static、等待及消耗handle，再完整程式 | 正常業務Err仍是join外層Ok；join位置決定何時需要完整成果；兩個工作都等待後再分支 |
| 13.2 panic payload與策略 | 8.1 panic、8.2 alias、10.5 stdout/stderr、11.2 Box move-out、11.11 owned downcast；13.1已正式教Send | dyn Any額外加Send，不假定Any自帶Send；unwind才能由join接Err；String/static str/其他型別分別處理 |
| 13.3 HRTB | 7.1～7.4 reference來源、9.2 callback/Fn/function pointer、11.3 drop；本節先對比外層單一'a與for binder，再局部借用例 | 輸入lifetime通用性不等於capture static／必須owned輸出／Fn重複呼叫能力 |
| 13.3 projection | 同節先解輸入/輸出同次'a與shared Fn呼叫，再helper | bound不選欄位；caller的second選第二欄；reference在各自來源有效時讀成i32 |

13.4～13.8依核准source方案在前節freeze後才逐節新增。先前ch13-preparation/gates.json保留當時pending的歷史；本輪actual handoff與本人實讀證據另外記錄，不回寫研究時點。

## Permanent source isolation

`examples/concurrency-stages`是無library、無第三方依賴的根workspace member。先建立合法member manifest與println-only main，root接入members與lock並通知後，才執行限定Cargo。完整manifest在各頁呈現；workspace共用欄位承接實際12.1。作者沒有修改root Cargo、Cargo.lock、nav、CI、舊ch10 source或他章freeze。

每個example用`examples/<name>/main.rs`與自己的`module/mod.rs`完整快照，沒有chapter共用lib、include!、#[path]、feature隔離或寬泛allow。47個targets／29fixture pairs的核准永久方案在`ch13-preparation/snapshot-plan.json`，實際落地只按已完成節追加。

13.1五targets各自只載需要的require_send／require_sync；require_send_static到static維度才加入。13.2四targets將sum、金額驗證、panic_message分開，首個加總不載panic或HRTB；panic_worker的cfg(test)原三分支測試在literal型別後綴7_u32先解釋後才全文出現。13.3六targets分開on_caller、with_local、sum_selected，沒有在第一個caller source中帶入for或projection。所有根、helper、答案、失敗fixture及.expected都在當頁以真實snippet完整列出。

## 13.1～13.3驗證與freeze

15個examples各run及strict Clippy（`-D warnings`）通過，所有main內原有assertions實際執行；panic_worker的一項helper test直接核對String、static str、u32三分支，避免一個實際panic的payload型別代答全部契約。12份compile-fail以pinned rustc／edition2024實測非零結束、符合各自同名非空.expected。這些共43項限定檢查的完整command/stdout/stderr在`ch13-authoring/stage03-validation.json`。

另完成println-only default binary run、panic_worker測試建置的strict Clippy（連同此no-lib package的default binary測試target）與package fmt --check。15份正常stdout逐一保存；正文literal輸出區塊與實跑相符。真正pymdownx.snippets／details展開三頁，核對所有完整來源閉包、無未解析snippet、無泛型誤認HTML tag或未解析strong。這不是全站建置，也未看實際桌面／手機網站版面。

`stage03-freeze.json`固定目前三頁及package所有已落地source，並快照本階段兩份planning ledger；後續只追加新節source，不改早期freeze。root另安排獨立technical／continuity／reader與視覺驗證，作者未宣稱其通過。

## Audit與範圍

A0610-C04在13.3明說：「這份 bound 說明輸出的有效性；具體選哪個 reference，仍由選擇器的 body 決定。」下文second函式才選第二欄。A0610-O06在13.2用完整句子交代測試意圖，並在完整測試前先教`7_u32`型別後綴，保留三型別真實source。作者不改shared issue-dispositions。

核准範圍只有原八節。Root在研究階段撤回誤列的Condvar、Barrier及thread-local新API；沒有新增這些內容。原scoped body／TLS cleanup界線仍保全，屬13.4待實作。四份先前歷史fetch如實保存且未讀、不算實讀來源。

## 13.4接續交接

13.1～13.3凍結後，才新增五個scoped targets。每個只帶sum或panic_message的自身快照，不帶較晚的channel／shared-state／atomic module。scoped管理callback在caller thread執行、worker才有Send要求；env/source有效性、scope自動等body與手動join完整thread/TLS清理的差異均在首次API簽名前。mutable reference跨thread的T:Send條件先於auto_wait，答案在首次for mutable slice前補Item=&mut i32契約。未新增thread-local API。

五targets run/strictClippy、一項panic helper三分支test、四份compile-fail及一份runtime-fail的編譯/非零執行共17項通過，完整stdout/stderr在stage04-validation.json；另cfg(test) Clippy與fmt在stage04-additional-checks.json。正常panic_worker由caller處理而成功，scoped_auto_panic則實際失敗且符合.expected，不混淆兩種政策。各完整helper/tests/answers/fixtures/expected均在13.4同頁，五份run輸出保存。前綴source/page hash不變；stage04-freeze.json另封存累計前綴。未做網站視覺或獨立審閱。

## 13.5接續交接

13.4凍結後，才新增七個channel targets與兩份compile-fail/.expected。messages只由逐項producer target載入；bounded_messages到SyncSender契約後才在bounded/stop/answer各自永久保存。沒有跨target共用library。Sender/Receiver所有權、send/recv結果與等待、T:Send條件、SendError的tuple struct .0、所有sender消失再耗盡buffer、每個producer保序與跨producer交錯均在首次推理前說明。while let與tuple struct明確回用實際4.6／11.10，而非重稱首次語法。

SyncSender的Send/Sync/Clone條件與滿時等待先於完整helper/target；容量0成功只到接收、不等於處理完成。先recv後join解開等待循環，不執行會掛住的反例；關閉Receiver是協作停止，SendError只還第一個未送值120，沒有保留後續30或撤回80。七examples的run/assertions/strictClippy與兩compile-fail共16項通過，另fmt通過；stdout與正文核對，完整sources/answers/fixture及.expected同頁。stage05-freeze.json保存累計前綴，原stage03/04頁與source hash不變。仍未做網站或獨立審閱。

## 13.6接續交接

13.5凍結後才新增六個Arc/Mutex examples及四份compile-fail/.expected。先教MutexGuard的Deref/Drop/借用來源，再LockResult/PoisonError：poison分支也已取得guard，不照搬Book的「再也拿不到鎖」。單thread guard例先於Arc；Arc clone只增加同配置owner、不要求T Clone，Send/Sync條件與Mutex<T>只需T:Send分開推。每個get/計算/set留在同份guard，join前先釋放需要的鎖；不執行deadlock反例。

poison例先解invariant、into_inner已持鎖guard、修復/clear_poison/解鎖三動作，固定政策將80/0重設為0/0；main接住worker panic後繼續。無poison不證明業務一致，Arc計數安全也不替任意內容同步。panic helper與sum分target保存，完整同頁。六run/assertions/strictClippy、一項三分支test、四compile-fail共17項通過，另cfg(test) Clippy和fmt通過；六份stdout已核對正文。stage06-freeze.json固定累計前綴，原三次freeze頁/source不變；無網站視覺或獨立審閱主張。

## 13.7接續交接

13.6凍結後才新增七個RwLock targets與三份compile-fail/.expected。先教read/write兩種guard、Drop/!Send、LockResult與writer-only poison，再try結果分清WouldBlock沒有guard、Poisoned已有對應guard。跨thread讀者的T:Send+Sync條件先於Arc<RwLock>例；read後放鎖再write須重新檢查條件，受控單thread視窗不用sleep。downgrade先說消耗writer、原子改read無解鎖空窗、不是read升write，再完整程式；不以沒有其他writer的230輸出代替官方契約證明。

reader panic不poison，writer panic後read得到的PoisonError含read guard。保留完整追加30的政策以逐項80/120/30驗證，再clear_poison；不用總和單獨代答內容。公平性/飢餓/等待writer阻新reader/固定取鎖次序皆保留原限制，不承諾所有deadlock已排除或RwLock更快。七run/assertions/strictClippy、兩份原三分支helper test、三compile-fail共19項通過，另兩cfg(test) Clippy與fmt通過。七stdout已核正文、完整source閉包同頁。stage07-freeze.json固定累計前綴；原13.1～6頁/source hash不變。未做網站或獨立審閱。

## 13.8與全章交接

13.7凍結後才新增最後七個std-only atomic targets與一compile-fail/兩runtime-fail及.expected。先教AtomicUsize/AtomicBool、shared receiver、不是Copy、new/load/store/fetch_add舊值與wrapping，再Relaxed單位置修改順序。與9.7大小比較用的std::cmp::Ordering區分名稱來源；Acquire/Release的read-from配對先於發布圖和程式，唯一producer/一次true/發布後不再改payload均明列，P1→P2→R1→R2說明為何DATA的Relaxed load也得到230，較晚join不倒用為先前讀取的理由。

五Ordering全保留：Relaxed/Acquire/Release/AcqRel/SeqCst，load/store/RMW合法組合與RMW兩側效果；兩份非法Ordering fixture先編譯再實際panic。SeqCst用四步共同順序的循環推理排除(0,0)，允許輸出三種；本機一次觀察不當成全排程證明。SeqCst拆開load/store仍失更新，與fetch_add/完整guard的責任分清。yield_now/busy-wait沒有公平或進度保證，失敗不發布可能永等；不新增fence/unsafe/release-sequence/Cow等API。答案先回收80_usize的literal後綴。

七run/assertions/strictClippy、一compile-fail與兩runtime-fail編譯/非零執行共19項通過，另fmt通過。全部七份stdout保存，其中atomic_seqcst記當次允許結果，不寫成固定排程。stage08-freeze.json封存全八頁及package；原stage03～07前綴頁/source hash都未變。

全章保留47個正常examples、26份compile-fail與3份runtime-fail（各有非空.expected）、9張原SVG；println-only預設binary另計。panic helper的唯一測試設計在五份需要的永久快照重複保存，每份分別真測String/static str/u32三條路徑，不把複製成五份說成五種不同測試設計。各階段已記錄共143項通過的限定命令：47 examples run+strictClippy、5份helper test、26 compile-fail、3 runtime編譯+執行，另default run、5個cfg(test) Clippy與6次各期fmt。沒有執行整個workspace all-targets或全書check。

source-closure.json逐target記實際local modules與同頁完整來源，validation.json彙整執行與stdout，freeze.json包含全章owned頁/source/planning最終hash；各期freeze及當時ledger快照保留。八頁真正snippets/details展開並檢查完整來源/expected、泛型HTML/strong；未做全站建置、桌面/手機實頁或獨立reader/technical/continuity驗收。完成本輪作者交接後停止，未開始18B研究。
