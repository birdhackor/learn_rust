# 第 6 章技術審閱：ORIGINAL ch06-complete-source-v6

本輪來源初判結論：有 **1 項必要的局部介紹時序修正、1 項可選的 ownership 措辭改善**。核心實作、可觀察答案與選定 API 契約沒有另外查到技術錯誤。必要項只涉及 collect 的失敗 fixture 先用 Vec Debug、稍後才教該能力；不把其餘已教內容判成未教。

## 角色、來源與隔離邊界

審閱者是第 12 章作者，沒有參與第 6 章撰寫；先前為第 12 章先備曾讀過 6.1–6.5 的早期 frozen 版本，因此本輪是獨立**技術來源審閱**，不是新的 blind 首讀。

本輪只讀 ORIGINAL `/workspace/work/tutorial-implementation/frozen/ch06-complete-source-v6` 的 50 個閱讀單位、全部 80 個 archived source（包含 39 個 examples／答案、7 個 library source、21 個 tests、15 個 negative source 及其 expectation、Cargo.toml、3 個 SVG）。manifest SHA-256 是 `dacb0f09813d368bcdb9c7bf94b23a618c29cb6b29da1d5f55fb37aa39fdfb0b`。50 單位與 80 sources 的個別 SHA-256 全部核對相符。

可用的必要前文只取 root 指定 actual frozen：ch01-complete-source-v8、ch02-review-fixed-v5、ch03-review-fixed-v6、ch04-review-fixed-v7、ch05-review-fixed-v7。按疑點回讀下面列出的實際介紹，不從未教大綱推定先備。

沒有閱讀第 6 章任何首讀紀錄、既有 technical／continuity 結果、作者 ledger／source notes、root 處置、live 第 6 章改稿或其他角色提示。沒有修改教材、Cargo、nav 或凍結稿。此初判及 SHA-256 在其他角色結果可見之前封存。

## 必要項 N1：失敗 fixture 先用 Vec Debug，正文稍後才教

- 版本／位置：ORIGINAL `006-004.md` 第 32–41 行；source `examples/ch06-iterators/fixtures/compile-fail/collect_reference_items.rs`。
- 首次需要：fixture 的 `println!("{owned:?}");` 需要 `Vec<i32>: Debug`。
- 當下已教：4.2 已說 Debug 是型別提供的顯示能力；6.5 已交代 Option 的相應能力；這裡已充分教 collect／FromIterator 與 `&i32`→i32 的差別。
- 尚缺的當下依據：frozen 1–5 與 6.1–6.5 沒有教 Vec 的 Debug；同一單位第 **61 行** 才說「標準 Vec<T> 當元素有 Debug 時也能 Debug」。這不是閱讀者忽略已有介紹，而是必要介紹實際在首次使用之後。
- 影響：這份 fixture 預期因 collect 的元素契約得到 E0277，卻同時要求讀者接受尚未教的整份 Vec 顯示能力。雖然主要 collect 診斷正確，仍違反使用者對 **fixture 也不能用未教內容** 的要求。
- 分級：必要修正；類型為先備依賴／時序，局部增加理解負擔。沒有把整個 collect 介面或 Debug 診斷判成錯誤。
- 最小修正選擇：將第 61 行的 Vec Debug／PartialEq 說明移到 fixture 前；或把 fixture 額外的整份 Debug 輸出改成已教的 len 輸出。後者只需改一行 actual source，不擴大講解，也不讓後文倒補首次使用。
- 技術驗證：只針對這份 ORIGINAL fixture 編譯，確實得到 E0277；沒有執行它。這證明預期 collect 失敗契約成立，不能代替前置介紹通過。
- 發現者／提示狀態：本技術審閱自行從 ORIGINAL 順序重建發現，未取得首讀或舊問題提示。後續處置由 root 裁定；本輪未改寫。

## 可選項 I1：將「不是複製原 iterator」限定成按值接手、不主動 clone

- 位置：ORIGINAL `006-002.md` 的第一段：「新工作保存原工作與需要的 callback；不是複製原 iterator」。
- 已成立部分：本節實際使用的 slice iterator／owned iterator 沒有 Copy；map／filter 不主動 clone 來源，也不先建立完整中間清單。lazy 的輸出交錯與 visits 例子確實支持當下教法。
- 可改善的關係：map 的 receiver 是 self，按值傳入仍沿前文 Copy／move 規則。自訂 iterator 可以實作 Copy；此時呼叫 map 會保留原 binding，新的工作有自己的進度。不要由「不主動 clone」誤讀成任何 iterator 都不可能被 Copy。
- 相稱 probe：在 scratch 中對 6.5 的有限 Countdown 形狀加上前文已教的 Clone／Copy，呼叫 `original.map(...)` 後，mapped 與 original 的 next 分別是 Some(6)、Some(3)，後續也各自推進。probe 保存於 technical-ch06-v6，不放進學生正文作新的強制範例。
- 分級理由：可選的技術措辭改善。原句可理解成 adaptor 不主動建立 clone，而且本節沒斷言所有 caller 的原 binding 都必定失效；目前具體主線並未因此失效。仍建議把概括句改為「新工作按 self 接收原 iterator，不主動 clone；按值交接仍依來源型別的 Copy／move 規則」。
- 理解收益／新增負擔：連回已教的按值規則即可防止 ownership 誤解；不必新增整套 Copy iterator 教學或把 probe 搬入正文。由 root 決定是否採納。
- 正式契約依據：[Iterator::map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) 的 receiver 是 self，callback 收到 Item、交回新 Item。查覈符合本文既有 map 的主要契約。

## 實際前文核對

| 本章需要的能力 | 回讀的 actual frozen 位置與依據 | 判定 |
| --- | --- | --- |
| f64 與浮點 literal | ch01 v8 `004-002` 型別表與 `004-003` 浮點除法 | 6.7 的 f64／3.0 沒有提高入口。 |
| 整數 array／range 的 for | ch01 v8 `008-002` 明說 array 依序交 i32；`008-004` 實例 | 6.7 的固定名稱 array 另由 4.3 已教。 |
| shared／mutable slice 的 for 與 Item | ch02 v5 `006-003`、`006-005` 分別教 &i32／&mut i32 | 6.5 是解釋已用操作的介面，不是首次偷用 Iterator。 |
| Vec as_slice／as_mut_slice | ch03 v6 `006-002` 操作表與完整例子 | 6.5 的 mutable 文字走訪及後面的 slice sort 有實際先備。 |
| Unicode scalar／char／UTF-8 byte | ch03 v6 `005-002` | chars 的單位與 combining mark 比較有前置。 |
| Debug／PartialEq／Clone／Copy | ch04 v7 `002-002`–`002-006` | 不能由「型別有欄位」自動推出其 Copy；本文也沒有這樣做。 |
| HashMap get／get_mut、Borrow、鍵身分 | ch04 v7 `003-003`、`003-005` | Entry 是新的更新安排，沒有把原 get／match 描述成無法完成。 |
| Option<&usize> 比較與 Debug | ch04 v7 `003-005` | entry_callbacks／map_iteration 的 Some(&2) 等沒有偷渡比較能力。 |
| str Debug／整數解析、ParseIntError | ch05 v7 `005-002` | 原文已交代 ParseIntError 的 Debug／Display／PartialEq；章末 parsing tests 不需重教。 |
| Result 比較與 assertion 顯示條件 | ch05 v7 `001-005`、`004-006` | 本章 Result<Vec<i32>, ParseIntError> 在 Vec 能力介紹後可比較。 |
| ToOwned、? 與同型別錯誤傳遞 | ch05 v7 `005-006`、`006-003` | adapters 與答案沒有為第一個 Err 偷套用 From。 |
| 工作區欄位與 example 指令 | ch01 v8 的 1.3／1.9、後續章 package 操作 | 本章沒有依賴第三方 crate、Box、Deref、thread 或未教 macro 實作。 |

## 50 個閱讀單位的契約／用途核對

下列「通過」是本技術角色核對原文、實作與來源後的局部判定，不能代替 blind 首讀或完整頁面驗收。

| 原始單位 | 當下主要工作／首次需要 | 機制與獨立需要的核對 |
| --- | --- | --- |
| 001-001 | 保存運費工作與環境 | 需要是之後多筆訂單共用門檻；普通函式也可傳門檻，公平保留。 |
| 001-002 | closure／capture／參數直線／呼叫時點 | 先教資料與呼叫角色才給 code；建立不先計算某張訂單。 |
| 001-003 | 型別推導不等於泛型 | 固定型別與 i32→bool 的 E0308 對照成立。 |
| 001-004 | shared capture | String owner 仍在外部，兩次讀借用內容，沒有 clone。 |
| 001-005 | mutable capture／兩個 mut | 原數字與 closure receiver 分開；最後使用點與 E0502 overlap 符合。 |
| 001-006 | move capture／Copy reference | 建立時接手 String，但 body 只讀可重用；Copy reference 仍依賴原文字。 |
| 001-007 | body 移出 non-Copy String | 建立／呼叫是兩個時點，E0382 表示 work 被消耗，圖 source 同一文字。 |
| 001-008 | 門檻與 visits 合併練習 | 答案只增加已教 mutable capture；運費 60／0、visits2 符合規則。 |
| 002-001 | 不同內容保留型別 | 需求是共同取得工作，不強制 bool 改成整數。 |
| 002-002 | Holder／SwitchState models | 泛型、pub、欄位皆有前文；無 derive 偷加能力。 |
| 002-003 | associated type／Self::Output | 說明實作選型別及 self 接手 owned 內容，才給 trait。 |
| 002-004 | impl<T> 與具體輸出 | i32／String／bool 關係準確；method 需要引入 trait 有提醒。 |
| 002-005 | S::Output 泛型轉交 | helper 不使用未承諾算術；來源如何取值交給實作。 |
| 002-006 | equality bound | 限制輸出不是重新改選 impl；E0271 與 bool fixture 相符。 |
| 002-007 | source 與輸出測試 | 3項tests驗值與 owned String 可用性；不是 trait 自動證明邏輯。 |
| 002-008 | FirstPair partial move 答案 | first交出、second未交出；沒有自訂Drop或需要後章才能理解的型別。 |
| 003-001 | 描述 callback 呼叫契約 | 由只接收共同能力的需要引入，沒有要求作者環境型別可命名。 |
| 003-002 | FnOnce／Output／task按值 | 只承諾一次，不把所有符合 FnOnce 的工作都說成只能一次。 |
| 003-003 | FnMut／重複計數 | 同一狀態兩次得到1／2；E0525不能由加mut修復移出String。 |
| 003-004 | Fn／shared receiver | shared access 是獨立需要；Fn能力包含與move不決定呼叫能力符合正式契約。 |
| 003-005 | fn pointer／non-capturing coercion | pointer不保存任意環境；捕捉base的E0308是對應拒絕。 |
| 003-006 | sum_with 答案 | FnMut/while/slice既有；60與visits3、空輸入0都依規則。 |
| 004-001 | 工廠交回 capture 工作 | 需要保存設定並繼續呼叫，而非宣稱 function pointer 普遍不能回傳。 |
| 004-002 | APIT／RPIT 誰選型別 | 呼叫者提供輸入、實作選輸出，匿名參數與turbofish限定準確。 |
| 004-003 | 單一 opaque 具體型別 | 兩個closure的E0308成立；將分支移入同一body滿足原工作需要。 |
| 004-004 | RPIT implicit lifetime capture | 明確採edition2024，capture包含省略輸入lifetime；區分型別依賴與closure保存值。 |
| 004-005 | use<'a>／use<>／overcapture | 先教precise capture角色、+用途；沒有延長來源；source probe與官方確認E0597/合法精確版。 |
| 004-006 | owned counter 答案 | FnMut＋move保存數字；2／3、另一份11顯示兩份獨立狀態。 |
| 005-001 | 來源負責逐筆推進 | 不需要索引來源的共同工作；仍公平承認既有索引迴圈。 |
| 005-002 | Item／next／Some／None | mutable更新、None非數字0；可non-fused例外已正確限定。 |
| 005-003 | Countdown／lazy／opaque Item | start≤0空、正數減1；沒有暗示建構本身零成本；SVG狀態與source相符。 |
| 005-004 | Option Debug／PartialEq | 能力在first next顯示前給；五次Some3/2/1/None/None正確。 |
| 005-005 | IntoIterator／IntoIter／for／NumberList | 關聯輸出能力先教，Vec IntoIter 名稱與標準型別role先給；3來源總和6。 |
| 005-006 | iter／iter_mut／into_iter與borrowed Vec | 三條ownership路徑與Item先列；as_mut_slice前文已教；E0499/E0382分別對應borrow/move。 |
| 005-007 | StepDown 答案 | 先取5，再同一進度3/1，合計4；沒有重建工作。 |
| 006-001 | 門檻／倍率／保序重複結果 | 需要本來就指定[-2,0,3,3]→[0,30,30]，不是由新方法反推用途。 |
| 006-002 | adaptor／map／FnMut Item | lazy交錯與visits2支持逐筆；I1另記一般Copy措辭改善。 |
| 006-003 | filter的&Item與保留型別 | &&i32/owned String的&String對照充分；E0631只說參數不符。 |
| 006-004 | collect／FromIterator／自訂目的 | 輸出決定目的、Item須相符；主線通過，N1另記fixture的Vec Debug首用晚教。 |
| 006-005 | fold／accumulator | 初值0、兩項角色與空輸入0先教；不建立結果Vec、不聲稱較快。 |
| 006-006 | Result FromIterator短路 | 首個Err原型別、成功清單不部分回傳；visits2/3支持第三筆未解析。 |
| 006-007 | parse_total 答案 | 沿?取得成功Vec後owned fold，Ok1/Ok0/Err保留原因；未新增溢位政策。 |
| 007-001 | 報表身分與雙排序規則 | 保留空白/大小寫、空行略過、次數降序名稱升序在方法前決定。 |
| 007-002 | chars／lines／escape／is_empty | scalar不等字形，3bytes/2scalars；LF/CRLF/loneCR與借用Item具體，Unicode輸出probe相符。 |
| 007-003 | Entry／FnOnce／mutable map借用 | method契約表與callback2驗新增；key.to_owned仍先執行的限定避免錯誤成本主張。 |
| 007-004 | map三種iterator／任意順序 | (&K,&V)/(&K,&mutV)/(K,V)符合；不給可修改key；輸出順序不保證。 |
| 007-005 | Ordering／PartialOrd／Ord／NaN | Less方向、None≠Equal、Eq一致性與NANassociated constant先教。 |
| 007-006 | sort／sort_by_key／sort_by | mutable借用、unit結果、stable意味、多次callback；tuple能力在code前說。 |
| 007-007 | 降序次數＋名稱tie break | 比較反左右的例子接回固定報表需求；stable不自動固定HashMap任意源順序。 |
| 007-008 | rare_first 答案 | 只改次數比較方向，其他身分/資料ownership不變；空輸入0無新API。 |

## 全部 source／tests／配置核對的界線

39個examples/答案均逐一實讀。它們的macro與API都能對到上表及actual前文：String/Vec/HashMap基本操作、closure語法、trait呼叫、Iterator與各個新method、parse、to_owned、比較和排序。除了N1的fixture顯示能力，不需要Box、dyn、Deref、lifetime bound T:'a、thread、unsafe或任何第三方library才能讀本章實作。

21項library tests均實讀；associated3、callbacks3、opaque3、streams3、adapters5、text_ops4。Result與ParseIntError比較有5章actual依據；Vec整份比較只在6.6之後的相關module使用；tuple／名稱結果比較在6.7的能力介紹之後。沒有把泛型測試輸出相同當成trait自動證明所有輸入。

15份negative及expected均實讀：closure推導E0308、borrow overlap E0502、take twice E0382、callback E0525、pointer capture E0308、opaque兩分支E0308／短來源E0597／implicit capture E0597、iterator overlap E0499／move E0382、output equality E0271、filter callback E0631、collect E0277、sort bool E0308。每份都用當下已教的語法表達預期問題；N1單獨記錄其中不必要的超前Vec顯示。只針對疑點重新編譯兩份fixture，沒有重跑全部15份後冒稱本輪全部執行。

Cargo.toml使用前文的workspace欄位；新`[lints.clippy] unnecessary_fold = "allow"`在actual manifest以註解先交代設定範圍、提醒的目的與只允許一項lint，沒有用未知dependency／feature來改變Rust語意。保留fold而不改寫成尚未教sum的教學目的成立。主文未展示manifest設定，本技術角色核對的是其現有配置契約，不把它當作學生已學完一般lint設定。

## 正式技術來源與相稱 probe

- [Reference：Closure call traits](https://doc.rust-lang.org/reference/types/closure.html#call-traits-and-coercions)：FnOnce、FnMut、Fn的能力由body決定；move仍可Fn／FnMut。本文沒有把shared receiver泛稱所有資料永遠不可變。
- [Edition Guide：RPIT capture](https://doc.rust-lang.org/edition-guide/rust-2024/rpit-lifetime-capture.html)與[Reference precise capture](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing)：edition2024的implicit lifetime與use<>／use<'a>限定符合當前規則；全部in-scope型別參數須列入的限制有對應正式文件，沒有copy舊edition說法。
- [Iterator::map/filter/collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html)：map回呼接Item、filter回呼借&Item、collect依FromIterator目的，正文的reference層級與receiver安排相符。
- [Result FromIterator](https://doc.rust-lang.org/std/result/enum.Result.html#impl-FromIterator%3CResult%3CA,+E%3E%3E-for-Result%3CV,+E%3E)：遇Err不再取後續Item；不自動轉換錯誤型別，符合source。
- [str::lines/chars](https://doc.rust-lang.org/std/primitive.str.html#method.lines)：行尾與Unicode單位契約吻合；沒有把combining sequence說成一個char。
- [Entry](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)：and_modify / or_insert / or_insert_with的接收、回傳與執行條件符合。
- [slice sort](https://doc.rust-lang.org/std/primitive.slice.html#method.sort)：3種排序都保留equal相對順序；名稱次序是內容的字典順序，沒有當成locale排序。本文沒有宣稱實測效能優勢。

本輪用固定Rust1.99.0、edition2024，只從ORIGINAL archived source編譯library一次。相稱執行：impl_trait_precise、adapter_results、text_units，以及技術角色另建的Copyiterator probe。另重新編譯implicit capture與collect兩份fixture核對E0597/E0277。命令／返回碼／stdout／stderr保存於 `/workspace/work/tutorial-implementation/technical-ch06-v6/probes.json` 與 collect-fixture.json；沒有使用live source、跑root workspace全套或宣稱本輪重新執行39targets/21tests/15negative。

## SVG與版面證據

3張SVG原始碼全部核對：

1. closure-transfer：兩條箭頭分別對應建立take和呼叫take；同一String owner text→take→saved，沒有clone。
2. iterator-next：同一Countdown的remaining 3→2→1→0，結果Some3／Some2／Some1／None；第四次保持0。不是四份獨立source。
3. adaptor-pipeline：values的slice Item=&i32，filter參數&&i32仍保留&i32，map回傳i32，collect到Vec<i32>；true才往下與收集結果[0,30,30]符合文字／code。此圖畫逐筆型別，不是四份完整中間清單。

必要對象與動作在正文相鄰圖說已有明確定位，3個圖的explicit width/height、24px文字與同一SVG內角色標記存在。**本輪沒有看ORIGINAL實際桌面／手機頁面**，因此原位縮放、折疊、header遮擋、手機字體及實際可讀性均記未驗證；source核對與圖存在不能代答版面。

## 尚未完成與禁止混算

本輪來源初判仍有N1待root處理；I1須由root核實收益及篇幅後裁定。未讀其他角色結果、未作交叉覆核、未查修後稿，不能標成主文已全面驗收。保存初判後停止；即使後續版本已修正，也保留此稿的原始發現、分級與依據。
