# 第10章 ORIGINAL 銜接初判：ch10-complete-prerequisite-v12

本稿是非第10章作者的獨立 continuity 初判。只審唯一原凍結 `ch10-complete-prerequisite-v12`，未讀 FIRST10、TECH10、root 第10章處置、作者 ledger 或任何新 snapshot／live 第10章。先保存本判斷，才可依 root 後續授權交叉；本輪沒有改教材。

我本人是第12章作者，先前為12.4讀過 actual10.1／10.3／10.4／10.8 的先備及部分正式 API。因此本輪有既有背景，不能稱無背景 FIRST、blind 或實際 Python 初學者測試。這些背景並不代替此次原 v12 的全範圍核對。

## 初判結果與範圍

一項必要的先教後用缺口 N1，兩項可選的銜接改善 O1／O2。主要方法、例子轉換與九張必要圖的關係足夠；沒有發現須提前教 Box、unsafe、Pin、未教第三方 API 或一般 atomic 演算法的缺口。N1 未修前，不能判本章所有 code 都符合使用者的嚴格順序要求。

實際完成：8頁68個 BODY units，完整 source archive 121份材料的範圍／指紋核實。121份包含47個 normal examples／答案、6份 library modules（含一個 test function）、26份 compile-fail 與26份 expected、3份 runtime-fail 與3份 expected、Cargo manifest、9份 SVG。46個 normal 全檔與已讀 BODY 完整程式逐行等價（只排除展示縮排、空行與 snippet marker）；未內嵌全文的 send_sync_solution 另全文讀取。六份 library、全部 fixtures／expected、Cargo 與九 SVG source 均實讀，不把檔名或目錄當作覆蓋。

指紋依 actual manifest，68 BODY＋121 archived files 共189項全相符。原位 assets 中九 SVG 另與 manifest archive SHA 比對全相符。每個頁面原始 SHA 保存於 manifest，preview 為相同 BODY 按頁串接：去重複頁標題、把原位 asset URL 改成隔離站點相對路徑，沒有改原凍結字節或將圖移前。

本輪未重跑正常 Rust、全套 compiler fixtures 或全章 tests；沒有執行死鎖、未關閉 channel 或無限 bad protocol。疑點是介紹順序／名稱銜接，查 actual 前文與正式 literal／prelude 規則已能判斷，不需要重跑全 suite。下文的正常數字與失敗規則是 source／契約核對，並非宣稱本輪自行取得 runtime PASS，也沒有引用 root 舊 QA 作執行證據。

## 實際前文依據

依 canonical1–9 必要正文回讀，不靠章名、作者安排或選讀補洞。以下是此次真正讀取的主要位置；前九章也只在這些必要範圍查覈，不宣稱重新審過全九章。

| actual版本／位置 | 已教能力與此次使用 |
| --- | --- |
| ch01-complete-source-v8：1.3 003-002／004，1.4 004-002／003，1.5 005-002，1.8 008-002，1.9 009-002／003，1.10 010-002／003 | 數字型別／推導／數字中底線、有限 overflow；unit／函式回傳；array／range 走訪；package／crate／workspace 欄位繼承、pub mod／use／crate；assertion、cfg(test)／test。數字中底線的教法沒有介紹型別後綴。 |
| ch02-review-fixed-v5：2.4 004-002／003；2.6 006-002／003／004／005 | 普通 reference 的最後使用、mutable 獨佔與重新借用；slice 長度／索引／for 得到 &i32，解引用讀出 Copy 數字。不是看到資料被包起來就可跨 thread。 |
| ch03-review-fixed-v6：3.1 001-002／003／006；3.3 003-002；3.6 006-002／003 | enum／variant／pattern、match／_、泛型 T；Vec::new／vec!／push／as_slice／as_mut_slice 的 receiver、ownership、借用範圍。 |
| ch04-publication-target-fixed-v8：4.4 004-002／003／004／008；4.5 005-003／004 | 單一來源 elision、外層 lifetime annotation、outlives；模型借來源與借模型自身分開，method 返回來源契約、路徑中的 '_。 |
| ch05-review-fixed-v7：5.1 001-003／004；5.2 002-002；5.6 006-002／003 | panic／expect 的 Debug 需要、type alias、? 的成功取值／同型錯誤 From／整個函式早退。 |
| ch06-review-fixed-v10：6.1 001-002／004／006；6.3 003-002／003／004／005；6.5 005-003／004／005 | closure 參數／明寫回傳、無參數、move capture 與 Copy reference；Fn 系列 receiver 能力、prelude、function pointer；owned Vec into_iter／for 逐筆 move 與剩餘工作的角色。 |
| ch07-review-fixed-v13：7.5 005-002／006 | stdout／stderr、process 出口／正常返回與清理分工。第10章沒有依賴 CLI、JSON 或第三方 library 的操作。 |
| ch08-review-fixed-v9：8.1 001-002／003；8.2 002-003；8.3 003-002／004；8.4 004-002／003／004；8.5 005-002／003／004／007；8.7 007-002；8.8 008-001／002／003／004；8.10 010-002／004／005 | Sized／DST／?Sized；std::mem::drop；Deref／mutable 方法查找、Rc 的欄位查找；T:'a／'static 與值清理分開；dyn／Box coercion／where；Rc strong handle 不是 clone T；Cell、RefCell 與 guard 清理；Any owned downcast 失敗交回原 Box。 |
| 第9章 | 此章範例沒有實際使用 feature、build script、macro、include、filesystem新介面，因此沒有為滿足「經過9章」把無關內容算入先備。Cargo manifest 沿 actual1.3 已教欄位即可讀。 |

前文查找有三次較大的批次輸出被截斷；已對缺失中段另分小批完整讀取，沒有把被截斷未見的內容記成讀完。literal 後綴的專門搜尋只在上述 canonical1–9 BODY，沒有搜尋 forbidden 第10章 review／live。

## 必要／可選項

### N1 必要：測試與最後答案先用了未教數字型別後綴

- 原位：10.2 `002-005.md:83`，test code `panic_message(Box::new(7_u32))`；第一次需要該語法。後續10.8 `008-008.md:29` 的答案又用 `[80_usize, 120, 30]`。相應原 source 為 `src/workers.rs` panic-tests 與 `examples/atomic_solution.rs`。
- 已教：actual1.4 004-002 介紹 u32、usize、`let price: i32`、型別推導與數字中底線 `1_000`；actual8.10 純數字以已知具體型別交進 Box／Any；當下10.2說測試直接建立「String、static文字與u32」payload，卻沒有先解釋 `7_u32` 如何指定 u32。語意名稱出現不等於新語法已教。
- 缺的最小關係：literal 後面接 `_u32`／`_usize` 是為這個數字指定型別，底線不使它變成變數名稱，也不是1.4數字中純閱讀分隔。前九章 canonical BODY 沒有任何 numeric type suffix 或該規則；逐項搜尋 `後綴／suffix` 及所有數字＋整數／浮點型別的 token 皆沒有結果。10.2此前正文同樣沒教。
- 影響：Python程度的讀者要猜新 token；在 Any 的範例，實際具體型別是技術關係的一部分；在最後答案，array 及加總要依 AtomicUsize 使用 usize。程式可編譯不解決讀者的順序缺口。
- 分級：必要／增加理解負擔／首次語法依賴。不是主 concurrency 方法全未教，也不是 Rust 技術錯誤。
- 最小修正選項：測試先寫已教的 `let other: u32 = 7;` 再 Box::new(other)，答案採 `let amounts: [usize; 3] = [80, 120, 30];`，即可完全使用前文形式；或在第一次 test 前教一句型別後綴並指出後面 usize 形式相同。不需要提前擴整章 numeric literal。
- 正式依據：[Reference integer literals](https://doc.rust-lang.org/reference/expressions/literal-expr.html#integer-literal-expressions) expr.literal.int.suffix／infer，實讀該段及123_u32／型別註記對照；suffix指定primitive type，未指定者依context推導，欠約束通常i32。這是查規則，未為prose問題重跑Rust。
- 發現：本輪來源自行檢出，未取得 peer10／root 已知問題提示。未修改原稿，N1仍待處理。

### O1 可選：省掉 drop 的 use 時，補明它也在 prelude

- 原位：10.3 `003-006.md:14` 的完整 `callback_once_hrtb` 首次直接 `drop(text)`，imports只有with_local；後續scope／channel／guard例子也採同樣短名稱。
- 已教：8.2 002-003已完整教std::mem::drop的ownership／清理用途，並說「可以用前文的use引入它，之後只寫drop(value)」，範例明寫use。Fn系列及Send／Sync的prelude也已分別解釋。
- 改善的最小關係：標準drop也由prelude提供短名稱，因此現在省略import仍呼叫同一個已教函式。可補一短句或明寫use；不用重教清理機制。
- 收益／分級：名稱來源對齊，避免讀者以為library import附帶匯入drop；API角色、owned按值消耗及FnOnce理由已有明說，不以import省略把已教方法升成未教主要概念，因此列可選。
- 正式核對：[std prelude v1](https://doc.rust-lang.org/std/prelude/v1/index.html)列 `pub use crate::mem::drop`。本輪實讀；不要求first reader事先查看該清單。

### O2 可選：as_mut_slice 回讀章節定位應是3.6

- 原位：10.4 `004-007.md:7`，「使用2.6已教的as_mut_slice與for value」。
- 實際已教：2.6教slice／借用走訪；Vec與其as_mut_slice在actual3.6 `006-002` API表與完整例子明教（mutable借整份Vec，回&mut[T]、不衝突存取原Vec）。本輪真回讀該單位及相鄰ownership段。
- 最小改善：寫「使用3.6已教的as_mut_slice與2.6的mutable slice走訪」，或只把API定位改3.6。
- 收益／分級：回讀者可找到正確介面；所有順序讀者到10.4前已學3.6，答案沒有新API，此為可選的定位修正，不能算未教API的必要缺口。沒有要求重新寫Vec章。

## 68個閱讀單位的順序與來源覆核

下表每列保存當下必要關係，不把頁名當已教證據；「足夠」針對本列關係，不替N1放行。延伸來源單位只提供查證入口，沒有用其連結補主要理解。

| UNIT | 當下依賴、轉換與覆核判定 |
| --- | --- |
| 001-001 | 資料／借用交接是獨立原需求；當下只契約檢查、下一節才建立thread，導覽足夠。 |
| 001-002 | thread／stack／排程、concurrency vs parallelism先定義；普通函式呼叫不建thread、無速度／固定行序保證，足夠。 |
| 001-003 | Send身分、marker／auto、來源prelude／欄位組成、原樣交回helper；bound check不冒稱實際移交，足夠。 |
| 001-004 | Sync等價& T:Send、owned vs borrowed同String圖；?Sized實際8.1、elision4.4、Batch組成與import先說，足夠。 |
| 001-005 | 同樣資料換Cell／RefCell、Send與Sync分開；data race先說衝突與同步角色；fixtures只測bound、不猜這次只有get就有Sync，足夠。 |
| 001-006 | Rc管理計數也是資料；move捕捉不改Rc能力；ownedString closure與含Rc closure公平對照，足夠。 |
| 001-007 | Send／Sync與static不同維度；reference與owned型別T分清；fixture缺來源有效性，足夠。 |
| 001-008 | Report的Cell變化只用前面能力；source完整答案另讀，沒有新語法，足夠。 |
| 002-001 | 原需求兩份小計206與業務Err／panic分層，不從API產物反推需要，足夠。 |
| 002-002 | spawn F與T各有FnOnce／Send／static角色；handle不是T，join耗權／blocking／panicpayload Box dyn Any與alias／unwind-abort先說，足夠。 |
| 002-003 | sum先給完整body與有限數字；main6／worker200／join後206；三compile-fail依次局部借用、move、結果Rc Send，足夠。 |
| 002-004 | 改任務為金額單值並明說不再加總；外Ok／內Err代表正常拒絕，不是panic；每次join才下一筆，足夠。 |
| 002-005 | Any downcast保留原Box Send、兩種文字和unknown分支、自訂helper完整；stdout／stderr與main成功分開。test有N1後綴，其餘能力足夠。 |
| 002-006 | 同一handle二次join move、drop handle只detach與process結束邊界，沒有把清理權限當工作完成，足夠。 |
| 002-007 | 兩次join都先取得結果再tuple-match，fail分支不提前丟第二權限；答案足夠。 |
| 002-008 | 延伸查證，不拿來源連結補先備；合理待教別的thread builder。 |
| 003-001 | 已有callback與reference來源，現在要函式內提供借用；HRTB位置服務scope理由，足夠。 |
| 003-002 | caller提供'a與FnOnce參數不同角色先說；已教where／capture，足夠。 |
| 003-003 | 源碼將value移進helper；外層'a可長於局部來源，failure不是計算錯，足夠。 |
| 003-004 | HRTB全名、for量化scope、固定T、僅呼叫一次先說；局部80→owned200圖與static-only失敗對照足夠。 |
| 003-005 | 輸入量化與capture／外部輸出T分開；同一with_local可回外部str卻不能回局部number，足夠。 |
| 003-006 | HRTB與FnOnce／Mut／Fn維度分開；drop-ownedString造成一次性，不是move自行決定，足夠；O1名稱改善。 |
| 003-007 | 輸出借用連當次輸入；Fn是因只借&F，不拿重複次數代替；第二tuple結束前Copy值；function pointer elision當下另說，足夠。 |
| 003-008 | 函式內兩不同來源＋同一mutable capture，FnMut mut參數與for先教後用；答案足夠。 |
| 003-009 | 名稱／語言規則外部查證，不以Nomicon作必要未教先備。 |
| 004-001 | 保留owner且回來可改是原需求；普通static不因立刻join放寬，需要介面收尾邊界，足夠。 |
| 004-002 | scope callback在caller、Scope::spawn才新worker；env／scope／outlives／HRTB／F,T各區分；TLS僅角色解釋、不使用API，auto body與manual完整清理不同，足夠。 |
| 004-003 | Vec外源shared borrow→manualjoin→push；callback內局部source失敗先解釋scope包含callback後等待，足夠。 |
| 004-004 | 丟scoped handle vs普通detach不同；獨佔total＋readvalues、scope返回後assert；E0499／Cell能力仍保留，足夠。 |
| 004-005 | 返回env借用vs逃scope handle兩來源不同；最後borrow後push與HRTB固定T，足夠。 |
| 004-006 | manualjoin處理Err、未manualjoin由scopepanic、callbackpanic仍等body；部分修改非rollback，足夠。 |
| 004-007 | 兩Vec不同來源mutable更新、211與各len；API actual3.6已教，O2定位不準，其餘答案足夠。 |
| 004-008 | 對應API可查，沒有把更高階context內部實作當先備。 |
| 005-001 | join最後單成果→逐項工作進度交接的需求；channel不是thread，足夠。 |
| 005-002 | mpsc展開、兩端同channel／buffer／send owned／recv blocking、SendError tuple struct與.0先教，兩層Result不偷換worker收尾，足夠。 |
| 005-003 | 空但尚有Sender vs全部消失且排空；same-thread示意非並行紀錄；expect Debug及Err取回ownedVec，足夠。 |
| 005-004 | clone端點不cloneT；Sender／Receiver traits、Rc message負例；helper?／owned迭代／whilelet當下先教；三Sender關閉→drain→兩join，足夠。 |
| 005-005 | bounded固定buffer容量原需求；backpressure與SyncSender contract；先recv避免互等循環不是sleep經驗法，足夠。 |
| 005-006 | 關Receiver不強製取消、rendezvous容量0保證第二值120未送；第三值清理不誤稱全部工作可恢復，足夠。 |
| 005-007 | 容量1兩producer變化保留資料／解收尾循環；各Result、received5和sum206一起確認，答案足夠。 |
| 005-008 | 延伸標準channel契約，沒有借外部thread pool補操作。 |
| 006-001 | channel集中處理與shared model不同分工；先分保活／存取，無速度比較承諾，足夠。 |
| 006-002 | Mutex全名、guard Deref/Mut/Drop／lifetime、lock兩branch已有guard與poison提醒；兩短guard得到200後再lock，足夠。 |
| 006-003 | Arc全名／atomic基本角色當下足夠，不調後面的Ordering；new／clone／Drop／Deref／Send+Sync先教；只讀文字再回數值，足夠。 |
| 006-004 | Arc保活Mutex、Mutex保護T的兩層；T:Send而非Sync，Rc外計數和Arc<Cell>公平失敗；同一guard保護get算set，足夠。 |
| 006-005 | guard最後使用不解鎖，join相依互等；同鎖重lock不保證返回／不執行；guard!Send成果界線，足夠。 |
| 006-006 | Totals total/count不變條件先定義；panic留部分80,0；into_inner／clear_poison先教，具體重設政策不冒稱通用自動修復，足夠。 |
| 006-007 | Arc同Vec而非各自副本；每push短guard、兩join後read；與channel更新分工對照、非效能，答案足夠。 |
| 006-008 | 來源查證，內部鎖實作合理不教，本節操作不依它。 |
| 007-001 | 多位reader view可並存的需要固定，writer等釋放；不從230反推效能，足夠。 |
| 007-002 | 兩guard權限、receiver／poison／try Result兩Err、OS策略與第二tryread受控條件全在第一code前，足夠。 |
| 007-003 | first／second同Vec與drop one不夠的狀態可見；圖對象明示read1/read2；readguard缺DerefMut fixture，足夠。 |
| 007-004 | RwLock Sync比Mutex多T:Sync理由是多份reader；Arc<Cell>及guard移交不同失敗，不讓同鎖衝突污染能力fixture，足夠。 |
| 007-005 | 上限3原需求；old觀察值不是guard，窗口另writer手工插入可重現；持write重查到更新同區，足夠。 |
| 007-006 | 新需求寫完狀態要保留且供reader；downgrade接ownedwriter、write→read／atomic無窗口；沒有競爭的230不代契約證明，足夠。 |
| 007-007 | readerpanic vswriterpanic；poison read取得readguard而非write、三項確切值支持本例保留政策；clear不修復，足夠。 |
| 007-008 | channel／Mutex／RwLock分工不貶Mutex；fairness／starvation當下角色、雙鎖同序只排除相反循環，限定有用，足夠。 |
| 007-009 | writer正常join後才建立依賴其更新的reader；兩reader都等待、可並存非當次同時持有／非速度，答案足夠。 |
| 007-010 | 查證鎖契約，未知底層演算法不當必要先備。 |
| 008-001 | 單counter五次與旗標發布230兩獨立原需求，不由新方法自定目的，足夠。 |
| 008-002 | atomic值／receiver／!Copy／Arc同位置、load/store/fetch_add舊值／RMW／wrapping、Ordering／Relaxed限制先教，足夠。 |
| 008-003 | 手工安排兩路load0/store1，不稱兩thread race頻率；atomic無data race仍lost-update，足夠。 |
| 008-004 | 同位置RMW每步不丟加1，兩join建立完成後HB，Relaxed final5依據非「最新」通則；_索引不用已說，足夠。 |
| 008-005 | READY唯一Release→Acquire讀自該store、P1..R2與HB明說；兩列共享同兩位置；yield/busywait及無固定停止保證先教；後join不代先前read，足夠。 |
| 008-006 | AcqRel各側與SeqCst新增總序；合法方法表與runtimefail Ordering引數形式各自先教，足夠。 |
| 008-007 | 全四步SeqCst才用最後preceding寫入理由；四步循環是反證而非執行；Relaxed00可允許≠必出現；SeqCst split stilllost，足夠。 |
| 008-008 | 兩payload一次發布，固定三欄role、唯一旗標、不再更新是必要限定；答案有N1後綴，其餘shared API／計算／join皆已教；async只基本過渡非操作先備。 |
| 008-009 | API來源及platform延伸，未把atomic底層全算法當當下必要。 |

## 未報問題的主要方法：理由與證據分開

| 方法及原需求位置 | 機制／性質依據 | 需要／用途的最短已教連結 | 例子支持與界線 |
| --- | --- | --- | --- |
| Send／Sync，001-001–007 | T移交vs&T移交、欄位／capture組成；helper保留值，只編譯bound | 交接整份數列或借用原資料，兩者分開才能選型別；static另看來源依賴 | Batch、Cell、Rc、borrowedstr變化呈現能力差異；stdout沒冒稱真thread或加速。 |
| spawn／join，002-001–007 | 啟動不等、owned等待權、T正常返回與payload兩層；耗handle／detach | main自己的6不依賴已move資料；需要worker200時join才合併206；兩工作都要收尾 | sum明給，兩錯誤維度明給；等待位置變化206不變但獨立工作時序改；無速度測量。 |
| HRTB，003-001–008 | 外部'a單次輸入vsfor內輸入量化；固定T、capture、Fnreceiver分開 | helper自身提供局部value，callback須先承諾接受該有效借用；不是延長value | caller同200與local同200表示來源角色變了；外部borrow／局部逃逸／static-only足夠共同規則；Fn選擇另以&F access交代。 |
| scoped工作，004-001–007 | callback／spawn兩角色，env覆蓋scope、自動body與manual完整清理，結果來源不同 | 想保留owner且scope後push，介面收尾保證支援非static借用；已join不取消bound | 借讀／獨佔寫／外部返回／逃handle／panic分支都對照，同樣保留普通borrow能力；不用TLSAPI黑箱。 |
| channel，005-001–007 | send接ownedT，recv取T，Sender關閉＋drain後Err，join另收尾；有界backpressure | 工作進行中逐筆交出，main集中加總；容量滿先recv使producer可前進；協作停止以send結果返回 | 手工same-thread保留buffer、兩producer206、容量1、容量0確定未送120，各自支持不同關係；send成功不假稱處理完成。 |
| Arc＋Mutex，006-001–007 | Arc handle計數保活、Mutex短guard互斥與Drop解鎖；poison並非撤銷 | 要更新仍留sharedmodel的同總額／清單，兩層各負責不同工作；判條件到更新保留guard | 只讀Arc與lock先分開，Cell / Rc混合差異、兩合法200順序及80,0重設政策讓性質與用途都可見；不說channel無法完成。 |
| RwLock／downgrade，007-001–009 | 多readervswriter、try無guard／poison有guard、重取窗口與write→read無窗口 | 多位reader各持view仍允writer等待；上限3須重查；寫完狀態需要保留而又允許readers | 兩readerdrop狀態、手工otherwriter、downgrade契約與本例無競爭區分，writer正常後才reader；不宣稱更快／固定公平。 |
| atomic／Ordering，008-001–008 | RMW同位置整體；joinHB；唯一Release被Acquire讀自再串HB；SeqCst共同順序不合成RMW | 五次完成後讀5與旗標讀230兩不同工作，各用恰當保証；多欄反覆狀態未被此協定保證 | 手工lostupdate非實測race、P1→P2→R1→R2、全四SeqCst循環支持契約；只看本機230／00未現不能代替全部排程。 |

以上各方法的身分／角色、介面輸入輸出與下一步用途、理由兩項和例子邊界都有當下依據。核了通過項，不只找N1；沒有要求所有traits、所有payload型別或整個memory model一口氣完整推導。

## 實際圖文檢查

自建隔離8頁 strict preview，讀的是同一v12，沒有開新修稿或全站第10章。Chromium在桌面1280×1000、手機390×844各實際view九張圖全貌及圖前原位說明，共36張；另看兩種viewport的10.2／10.3／10.8展開答案首屏、10.7存取分工表、10.8lost update與Ordering表，共12張，合計48張全部親看。手機長答案正常需要繼續垂直捲動，末段未各另截view；不將首屏抽查稱手機每行完整視覺驗收。Rust code長行只在各自code block內橫向捲動，整頁document_width都不超過viewport。

九圖對應逐一追蹤：

- send-sync：上方ownership A→B，下面reference從B指回A的同String，底部&T Send等價T Sync；兩操作不混。
- thread-join：main6和worker200來源不同，worker結束包括清理→join取得200→206，縱距不是測時。
- hrtb-local：F全借用能力與此次&i32藍箭頭、owned200綠箭頭，局部來源結束與輸出不同。
- scoped-wait：外部values／total置於managed frame外；callback返回後還要等workerbody，之後讀total，沒有替TLS清理追加保證。
- channel-drain：三份Sender都納入、clone不clone訊息；兩worker匯合到同buffer，Sender消失後仍可收，Err與join分開。
- arc-mutex-guard：兩Arc同一Mutex、示意main120→worker200輪流持guard，join另收尾；另一順序正文明許。
- rwlock-readers：read1/first與read2/second名稱已對應，同一資料；first灰而second仍綠，兩次WouldBlock沒有writerguard，最後兩drop才230。
- atomic-publication：P1 DATA、P2 READY在producer列，R1 READY、R2 DATA在consumer列；藍線必須R1讀自P2，灰線同thread順序；不是四份各自數值。
- seqcst-cycle：A1／A2與B1／B2顏色不同，紅回線來自兩讀0假設，灰線thread內先後；共同順序循環不是真實執行流程。

全圖必要標字、線端和狀態在兩viewport皆可讀，完整圖不裁切、不靠放大；圖前說明保持相鄰。RwLock頂部fork左支是連線、右支有箭頭，正文與框已明說同一鎖的兩reader，因此沒有把樣式差異升成必要辨認缺口。本輪沒有發現需要新增必要圖的關係；deadlock兩方等待／雙鎖循環已用短步驟文字交代。

## 證據、命令與限制

work證據根：`/workspace/work/tutorial-implementation/continuity-ch10-v12/`。

- `source-fingerprints.json`：manifest原SHA、8頁原SHA、68BODY與121archive逐項指紋。
- `archive-body-equivalence.json`：46 normal與BODY全文等價、單獨讀Report答案、九原位SVG與archive等價。
- `preview/zensical.toml`：隔離8頁nav與相同站點renderer；來源snippets只指原凍結archive，不指live10。
- `screenshots/render.json`、48個PNG：實際viewport、page、region、原位asset、document_width與截圖來源。
- 實際命令：先source repo scripts/env.sh，再 `uv run --locked --python '3.14+gil' zensical build --config-file /workspace/work/tutorial-implementation/continuity-ch10-v12/preview/zensical.toml`，EXIT0／No issues found／0.51s；`python /workspace/work/tutorial-implementation/continuity-ch10-v12/render.py`，EXIT0、48screenshots、document overflow False。全部截圖確由view_image親看，建置成功不是visual代用品。
- 本輪沒有Rust runtime／compiler probe commands；語法順序疑點以actual正文和Reference／prelude核對。未重跑47正常examples／26compile-fail／3runtime-fail或library test，故不宣稱這些本輪實跑通過。
- 沒有其他平臺、target／作業系統lock策略、abort條件、Miri／Loom或效能測量；沒有以normal輸出證明排程、atomic所有模型結果或整章soundness。
- 只抽看答案／表格指定區域，不宣稱全8頁的每段desktop／mobile滾動首讀都測過；九必要圖和其當下解釋完整覆核。

封存此original初判及SHA／work immutable副本後停止，等root授權cross；不互看peer、不改本稿分級、不修改任何教材／shared配置，亦不開始第13章。
