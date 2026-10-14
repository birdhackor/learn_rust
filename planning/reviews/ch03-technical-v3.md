# 第 3 章技術初判：ch03-prerequisite-v3

審閱者：`/root/chapter04_traits`。日期：2026-10-14。角色是未參與第 3 章寫作的獨立技術審閱者；本角色先前已作為第 4 章作者，實際讀過同一份第 3 章前文及來源，也熟悉後續知識。因此這是本輪獨立技術初判，**不是新的盲讀／首次閱讀紀錄**，不以第 4 章知識替第 3 章補先備。

本輪初判封存前沒有開第 3 章 first-reader、作者 ledger／source-notes、其他 review 或 root 的具體問題／修稿；沒有修改教材。起始任務只給範圍與一般技術焦點，不給待抓的原句或答案。

## 版本與實際閱讀範圍

目標讀者假設只有基本 Python，按第 1 → 2 → 3 章順序閱讀。採用 clear-tutorial／writing-prompt／Rust 術語指引，本輪再讀 review-protocol；判「實際已教」而非章名已出現。

| 凍結版本 | manifest SHA-256 | 本輪核對的檔案界線 |
| --- | --- | --- |
| `ch03-prerequisite-v3` | `0f2135203a1d97836c10810406ce3635a189d33915d91929439430f7edf491c8` | 全六頁、reading-order 的 32 個單位、37 份 sources，含六組練習答案、全部 fixtures／expected、兩 SVG、library 與 package manifest；全部已讀 |
| 第 1 章 `ch01-complete-source-v8` | `2ffad984aceb0e9fd822e5f0bb01a6958b6f063319a576e229df2da68c09e2cd` | 先前已實讀完整第 1 章及修後答案；本輪重核數值範圍、輸出、array/range、module/crate、tests 的原位置 |
| 第 2 章 `ch02-complete-source-v4` | `9a20a66ce1cd36f9c61e533d1621a43f6e0808fd1e67b639db8218f04297e263` | 先前已實讀全六頁；本輪重讀 2.2 全部六單位及 2.6 的必要契約，核對 ownership/Copy 型別依賴 |

三份 snapshot 的 manifest 所列 chunks 與 archived sources 皆逐檔核 hash：第 1 章 50+32、第 2 章 31+34、第 3 章 32+37，全數一致。第 3 章兩份原位 assets 與 archived SVG bytes 也相同。manifest 所列原始頁面 hash 是凍結時的出處資訊；本 snapshot 用渲染後的 chunk 供閱讀，本輪沒有拿可變動的 current 原頁回填成固定來源。

## 初判結論與驗證邊界

- **必要 N1：**第一份 String 正常 example 已做 String 指派，卻在下一單位才說明 String 沒有 Copy。一般 move 規則已教，但這個新型別採哪種行為尚未前置；一個既有句子的前移即可修正。
- **可選 I1：**Vec 的 heap 描述可以限定到本節資料／已配置的元素空間。實際數值、文字與空清單操作都正確，不需要在此擴寫零大小型別。
- 其餘已查正常程式、答案、測試與刻意失敗來源，沒有發現額外未教 Rust API、錯誤結果或與原任務衝突的 ownership／Unicode 契約。這不代替獨立首次閱讀、銜接與實頁驗收。

本輪只為 Unicode 範圍契約作相稱執行：直接編譯／執行封存 `utf8.rs`，輸出逐字元合正文；以封存 `text.rs` 原函式加 reviewer-only harness，核對 2..2、4..1、0..8、0..0、8..8、8..9。另以 UTF-8 encoding 查本例 byte 值。證據保存在 `/workspace/work/tutorial-implementation/reviewer-ch03/technical-v3/checks.json`，不算原教材已展示了新增材料。

沒有重跑全 package fmt／Clippy／test，也沒有在本輪重新執行全套正常例與 fixtures。root 已給先前基線通過資訊，本輪的主要證據仍是固定來源閱讀、必要 API 官方查核及上述局部實驗；不得把基線算成本審閱者親自重跑。

兩 SVG 已核 source 的對象／箭頭／數字與原位段落，但**未看實際桌面或手機頁面，也未以圖片視覺驗證**。以下圖的通過範圍僅為原位 source／正文對應。

## N1：String 的 Copy 契約晚於第一份正常程式的指派

- 版本／位置：v3 `004-003.md`「先認識建立、追加與借用的接口」，`strings.rs` 完整 source 最後兩行；下一單位 `004-004.md` 才介紹具體 move 契約。
- 當下原句／程式：正常程式已含 `let moved = greeting;`、`show(moved.as_str());`。下一單位開頭才說「**String 沒有 Copy 能力。`let moved = greeting;` 交出整個 String 的 ownership**」。
- 已教部分：1.4 let／指派、2.2 ownership 與 move／Copy、3.4 String 的 owned 文字角色、當段 as_str 借用，都已足夠；不是整個 move 概念未教。
- 缺少關係：2.2 明說「值是否會 move，要看型別與操作規則」，而其 Order／i32 的例子只告知那兩個型別採哪種行為。新 String 會管理文字，不等於已告知它沒有 Copy；有 ownership 的 i32 也能產生獨立 Copy 值。第一次程式已把資料從 greeting 指派到 moved，讀者尚不能依當時已教內容確定原名稱是否保留，以及誰接手管理責任。
- 實際影響：本節入口問「誰管理這份可改變的文字」；程式末尾新增這項交接，卻要等下一單位才能判斷管理者的變化。輸出能理解／程式能跑不代替這個使用規則的前置解釋，也不能因本例沒有再讀 greeting，就算已教會交接。
- 類型／分級：先備／首次能力契約的時序；**增加理解負擔，必要修正**。沒有技術執行錯誤，不要求提前教 Clone 或 derive。
- 最小修法：把下一單位既有「String 沒有 Copy；最後的指派會把 ownership 交給 moved，之後改用 moved」移到正常 snippet 之前。其後的 heap 內容不另複製與 E0382 診斷可以保留在原 move 小節，不增加旁支。另一個可行處置是先刪正常例末兩行，待 move 小節再呈現；不需兩種都做。
- 發現歷史：本角色先讀本輪全文，建立逐 API／型別能力首次使用表後，回核 2.2 的原句才列為 N1；未取得其他 reviewer 或 root 具體問題提示。中途回報「目前未發現必要項」是尚在檢查中的狀態，這份完成後初判保留後續新增發現。
- 修後複查：未做；教材未改。

## I1：Vec 的 heap 概括可用本節範圍說得更精確

- 版本／位置：v3 `006-002.md`「Vec 擁有一列同型別資料」開頭。
- 原句：「`Vec<T>`…元素存放在 heap 的連續空間」。同單位教 `Vec::new()` 建立空清單，後面也有空輸入回傳空 Vec 的情況。
- 已教部分：2.2 heap 的配置／清理角色，3.4 空 String 可以先沒有已配置文字空間；本節已充分交代 Vec 管理元素、長度、順序，以及 slice 借用的差別。
- 精確性邊界：std Vec Guarantees 說明 new 的零容量狀態可以不配置，零大小元素也不需要元素配置；配置後的元素空間才對應連續 heap 儲存。本文實際使用的 i32／String 元素在有資料時符合現有圖像，沒有把任何原結果算錯。
- 實際影響：本節目前不要求判斷配置成本或零大小型別；空清單也沒有元素被錯誤地定位。因此這不是阻礙主線的缺口。加一句可讓讀者把 new 的空容器與後來實際元素儲存分清，但不值得為此介紹 size_of、allocator 或 ZST。
- 類型／分級：技術限定／模型範圍；**可選改善**。原來的基本使用範圍可保留。
- 最小修法：把一般 T 的概括限為「對本節的 i32／String，配置的元素空間位於 heap 且連續」，必要時補「Vec::new 先建立空容器，不預先配置元素空間」。不把「長度零」一概說成「沒有配置」，因為清空已有 Vec 仍可能保留空間。
- 發現歷史：本輪自行核對 std Vec Guarantees 時提出，沒有他人問題提示；未改稿／未做修後複查。

## 首次使用、實際依賴與主要通過項

以下位置均指 v3 凍結單位。表中「已足夠」只涵蓋該行明列的關係；N1 仍獨立待修，不能被 String 其餘契約通過蓋過。

| 焦點／首次使用位置 | 當時可用前提與原句 | 技術／用途核對與邊界 |
| --- | --- | --- |
| enum／variant／位置與命名資料，001-002 → 001-003 | 「一個 enum 值在某個時刻只屬於其中一種 variant」；表列 Missing、Amount(i32)、Rejected{code}；2.1已教struct欄位 | Quote定義與三次建構一致，與struct同時有欄位的角色分清；沒有用0代替缺值。已足夠 |
| pattern、arm、match第一符合／exhaustive，001-003 | 先教建立值與拆已有值的外框差別、`=>`、`_`與涵蓋所有情況，再正常程式及遺漏fixture | 120只進Amount arm，37只進Rejected；enum未derive Copy，完整Quote參數取得owner。missing_variant確實漏Missing，E0004有根據。未把一般match都說成會消耗輸入 |
| match expression／各arm結果型別，001-003 → 001-004 | 「整個結果是選中arm的值」接1.6 if expression；本段先教bool結果，各arm後文均bool | is_available以非負Amount判true、Missing/Rejected判false；Rejected顯示代碼但最後回false。工作只是目前可用判斷，不宣稱查外部拒絕原因 |
| shared enum解構，001-004 | 「amount是&i32」與code同理；2.3*與2.2i32 Copy | 兩次借同Quote都合法，讀出*i32不move整個enum。此規則在第一次borrowed pattern前已教，沒有偷用未教ref-binding語法；Reference patterns的自動binding契約一致 |
| if let／let else／return，001-005 | 先教只關心一種variant、成功名稱的scope、else須離開流程，明說本例return unit；1.6已教return | 簡化Quote的理由已交代；Missing時不加20。let-else的return滿足diverge要求，不需要為此教never type／panic macro。小整數運費材料沿用1.4範圍 |
| Shipment答案，001-006／enum_solution | 先給Waiting貢獻0、Ready回count與借用簽名，只對回傳數字assert | 完整答案是同一borrowed match規則，0/3/再次3正確；未提前whole-enum比較或derive |
| Option／Some／None，002-002 → 002-003 | 先教variant、None與0不同、自動在名稱範圍；if兩arm結果形狀相同，Some數字提供推導 | offered/missing兩段保留相同Option數字型別，先match才用金額；沒有把Option當直接數字。prelude名稱與標準型別角色正確 |
| Result／Ok／Err與DivideError，002-003 | 「Err…不會自動顯示錯誤或停止程式」；先給成功商與零除數原因，再if與match | 12/2=6，零分支不執行除法；只有Err arm顯示原因。結果與顯示分工、成功零與失敗資訊的差別均成立 |
| 數值合法範圍，002-003／003-005 | 1v8的004-003已說整數須留在型別範圍；3.2「本章的除法材料…小整數…不涵蓋所有i32邊界」，3.3再保留同範圍 | divide只檢查零，不是能完整處理所有i32輸入的數值函式；例如最小i32除−1的商超範圍，不能推「所有非零除數都安全」。本文已有明確排除且實際材料合法，未報必要增加邊界錯誤政策，也沒有假稱本輪測完整邊界 |
| Option/Result選型與負數金額答案，002-004／002-005 | 原需求分「沒有額外失敗原因」與「原因要交給下一段工作」；題目先給AmountError::Negative、0和−5 | optional_solution只用enum／if／match，0是Ok，−5是Err。這項金額檢查的負數材料不是把前文非負計價約定任意放寬；題目正在檢查非法金額 |
| 泛型T的角色，003-001／003-002 → 003-003 | 「值參數…資料，型別參數…型別」；Holder<T>與hold<T>輸入/欄位/返回同T；只保存不對未知T算術/列印 | generic_holder正常source只在具體i32/bool讀欄位並用既有輸出。泛型不取消owner，定義重用與每個使用位置型別固定兩項理由均成立 |
| 推導與turbofish，003-003 | `hold(120)`從輸入及其他型別要求推導；接收型別或`::<i32>`可指定，先教型別括號與值括號不同 | 120與80推為i32、true為bool；三份Holder各有完整型別。沒有說所有泛型可在執行時換任意內容，也不依unknown T的Display能力 |
| 同一T／獨立T,U，003-004／003-006 | 「兩個欄位都寫T…相同型別」，後教Pair<T,U>可不同 | 故意讓單一T的Pair一邊放i32、一邊放bool，E0308有根據；generic_solution實際Pair<i32,bool>兩型別對到兩欄，未聲稱T與U必定要不同 |
| Option<T>／Result<T,E>完整簽名，003-005 | 原先的具體variant已教，現在先給T與E的角色／None型別註記，再完整函式 | divide是具體i32函式，沒有對未知泛型T做算術；None的原建立式沒提供T，Option<i32>註記足夠。正文先前已承認其他使用要求也能推導，不把推薦註記讀成唯一推導規則 |
| String／UTF-8／str／&str，004-002 | 先教owned型別、byte8bits、&str借用位置與長度、literal由程式保存，再進入API表 | String和&str不是同型別；借用不負責清理owned文字。沒有提前static annotation、unsized/Box、reference struct layout等細節，selected資料關係足夠 |
| new／push_str／as_str／{}，004-003 | 表在source前給receiver、&str參數、unit／borrowed返回；2.5receiver與2.4lastuse | show只讀；先追加Hello與Rust，再借用顯示，呼叫結束後才能追加ending。String與&str的{}支持也明說。非空heap/空String可先未配置、重分配可能性都有適當限定；僅最後指派的型別契約時序另列N1 |
| String具體move／&strCopy／衝突，004-004 | 本段先給String無Copy、heap內容不要求另複製；shared reference可Copy在2.3/2.6已教 | E0382舊名稱失效、E0502借用在追加後又讀成立。沒有把每次追加都說成必換存放區；管理責任與borrowed資料的最後使用分清 |
| append_label完整答案，004-005 | 題目先給&mut String與&str、unit、修改呼叫者原資料；前段全部methods已教 | 只呼叫output.push_str(label)，沒有建立另一個output或取得label owner；最後Rust 教學符合題意。完整答案未帶String::from／ToOwned／Clone |
| Unicode scalar／char／可見字形，005-002 → 005-003 | 先給char單scalar、單引號literal，說e加結合符號可有兩scalar一字形；1.4已有變數格式輸出 | marker為單scalar「中」，用已知println形式顯示；UTF-8 scalar不是任意Unicode surrogate，更不是保證完整字形。操作只以本例scalar分界判range，不需要提前grapheme API |
| str.len／bytes表與圖，005-002 | 「len回傳byte數，usize」；A/中/🦀資料表明說不是所有中文／符號通用長度 | 編碼值65／228184173／240159166128和1/3/4長一致，總8；boundary0/1/4/8，終點不包含。沒有用len推scalar或字形數 |
| str.get(Range<usize>)→Option<&str>，005-003 | source前給start≤end、兩端≤len且scalarboundary、有效空段8..8與無效2..2 | as_str再get，不靠String Deref；Some(part)借原內容，不產生ownedString；1..2和4..9均None但原因不同，Option沒保留細分原因。官方API與本輪原例／小變化執行相符 |
| 直接range索引panic，005-004 | 先說相同合法條件、單整數不是取char的API，再明示故意失敗source | &text.as_str()[1..2]違反boundary，預期not a char boundary正確。已在1.7/2.6教執行時panic，沒有在這裡偷偷要求panic!、catch_unwind或unwrap |
| selected_byte_len完整答案／四tests，005-005 | 題目先給Option<usize> owned長度、Some長度/None缺值，pubmodule沿用1.9；3.1已教_不綁定、3.2已教Some(...)拆出內容 | frozen text.rs只用get/len/match與數值/布林assert，四testscover長度3、合法empty0、partial/empty-insideNone、outboundsNone。沒有提前Option PartialEq、is_none、Iterator或Vec。該答案含tests也不超前 |
| Vec<T>／new／vec!／push／len，006-002 | 類型角色與整張操作表在source前；3.3泛型、1.7array、3.4String owned | allitems sameT、push取得元素，i32Copy/Stringmove，len數元素而非byte。new推導由i32註記或push值提供。heap概括邊界另列I1；本例操作無錯誤 |
| slice.get(position)／as_slice與as_mut_slice，006-002 | table先給&[T]/&mut[T]，再單獨教Option<&T>不取得owner；2.6sliceindex及mut操作已教 | get1成功、get9None；readonly與writeable視圖顯式區分。`writable[0] += 10`最後使用之後，回到values.as_slice讀合法，不認為mutref名稱scope都不可再用owner |
| Vec remove/pop與材料追蹤，006-002 | remove須position<len、後面左移；pop空時None，在code之前已區分 | 80/120/200→移除80→首120→pop200→一項120→加10得130。沒有把原索引0與新索引0當同對象，remove(len)確實失敗且是panic，不是None |
| Vec<String>與borrowed-element限制，006-003 | Stringmove已教，pop owned返回與get借用在table先教；例前再次明確交接 | message→Vec→taken，舊變數未重讀。Option包著&元素仍需borrow；push可能移元素，不聲稱每次必重配置。E0502 source後續仍讀first，衝突成立；全Vec也不因數值元素而自動Copy |
| keep_nonnegative／listdemo／兩tests，006-004 | 先給只保留非負、原順序/duplicates、輸入slice與新結果Vec；說明每次先position<len再索引 | 每項只能按讀取順序push到另一清單，0保留、負值跳過，原材料不改。empty/all-negative結果空；tests只比較len/i32，不提前Vec Debug/PartialEq、closure或Iterator |
| capped_nonnegative答案，006-005 | ceiling非負約定在題目前置，keep、條件替換、保序/重複/原輸入都已教 | source只在value≥0分支對超過ceiling值改本地Copy值，push新結果；樣本100/0/80、原第一項120一致。負ceiling在約定外，沒聲稱函式自動檢查或回Result |

## 方法理由、例子用途與通過範圍

| 原問題與位置 | 機制／性質的支持 | 需要／用途的獨立來源與連接 |
| --- | --- | --- |
| 3.1把「缺報價」與「報價零」區分，001-001/002 | enum互斥variant與payload提供不同形態；match先辨形態再取資料 | 原需要在選擇enum前已給，不是從enum的產物倒推理由；Missing與Amount(0)仍可保留區別。全部三情況正常例支撐其分支工作 |
| 3.2需要保留失敗原因，002-001/004 | Option與Result的variant內容不同；Err資料自身不負責顯示 | 呼叫者需要知道原因時選擇Err，沒原因的缺值可選None。12/2與12/0顯示真實分工；0與−5練習把成功零/失敗區分，未誇大為全數值政策 |
| 3.3相同結果形狀服務不同資料，003-001/003 | T替換內容型別，定義一次Holder/hold；實際i32與bool各自固定 | 原有可選bool設定已給用途；共享shape保留而內容型別改變。不是只有一個實例卻宣稱泛型優勢，也沒說原先分別定義完全不能完成 |
| 3.4只顯示時不接手、顯示後再追加，004-001/003 | show(&str)借既有內容，as_str不複製文字；lastuse後允許push_str | 任務在API前已定：兩段問候、函式顯示、繼續追加。原例與String圖把同一份資料上的owner/讀者分工連起；N1只影響末尾額外交接的前置契約，不能否定此前正常借用的成立 |
| 3.5要取出「中」，005-001/002/003 | 1..4完整scalar，1..2切斷；get給Option保留失敗；empty合法與不存在分清 | 原需要是取目標文字，不是機械練get。確定的素材／bytes／range/圖/輸出能逐一追蹤；其改善是讓不確認的位置得到可處理結果，不宣稱做完語言字形切分 |
| 3.6保留非負、順序與重複、長度不定，006-001/004 | 新Vec按原索引順序push，slice輸入保持借用；while guard維持每次索引合法 | 輸出數量變化在引入容器前給定，Vec改變結果表示而非篩選規則。空/all-negative、重複/零檢查不同需求，未把可變容器講成替代控制流程的新算法 |

## 圖中必要對象與來源核對

1. `string-view.svg`，原位 `004-003.md`：上框 `greeting: String` 對上方正文的owner，管理箭頭下指中框 `heap: Hello, Rust`；下框 `show 的 text: &str` 的借用箭頭上指**同一中框**。正文先說明兩箭頭角色並限定關係模型不是完整String佈局。沒有多畫一份文字讓人誤認borrow產生副本。圖描繪第一次show，後文再說明呼叫完成後追加；沒有把兩個時點重疊成同時擁有mutable與shared借用。source對應已足夠，N1的末尾move不在此圖表示。
2. `utf8-boundaries.svg`，原位 `005-002.md`：前三卡片依次是A/中/crab；正文明確crab就是第三項🦀。每張range、byte值、位置與下一邊界對到素材表；1..4完整中與1..2切斷中的兩卡對到下一段合法條件。無需由數值相同猜對象，0/1/4/8不是四個字元索引。source對應已足夠；400px/24px規格不能代替真實頁面讀圖。

沒有發現當前主操作必需卻隱藏在選讀裡的圖。Vec的移除前後資料、while索引與新結果以固定材料、逐步文字和數值輸出足夠追蹤，不因沒有第三張SVG就判缺圖。

## 官方查核與證據來源

本輪工具實際取得的std頁面標示1.99.0，與工具鏈一致。以下只將相關段落用作事實校核，沒有把其含未教語法的完整版定義搬入學生先備。

- [Reference Patterns](https://doc.rust-lang.org/reference/patterns.html)：reference與non-reference pattern相遇時的automatic binding；核對3.1 &Quote中的&i32。另讀 [Match expressions](https://doc.rust-lang.org/reference/expressions/match-expr.html) 的first matching arm及place/value差別，避免把3.1的具體接收owner推廣成所有match都move。
- [Reference Statements](https://doc.rust-lang.org/reference/statements.html)：let-else必須diverge，3.1選擇return滿足。沒有要求教材窮舉所有diverge方式。
- [std String](https://doc.rust-lang.org/std/string/struct.String.html)：push_str的&mut self/&str/unit及new初始空配置說明；as_str得到借用視圖。對容量極限／allocation政策不擴寫，本章是短字串材料。
- [std char](https://doc.rust-lang.org/std/primitive.char.html)：Unicode scalar角色；[std str](https://doc.rust-lang.org/std/primitive.str.html) 的len按bytes與get非panic取得契約。char邊界／長度的完整關聯由正文加本輪小實驗支持，不把char等同顯示字形。
- [std slice](https://doc.rust-lang.org/std/primitive.slice.html)：position get回借用元素或None；[std Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) 的new、as_slice/as_mut_slice、pop、remove、len與Guarantees相關段。所有成功／失敗條件與ownership契約與正文一致；heap概括精度另列I1。
- 本輪執行證據：`reviewer-ch03/technical-v3/checks.json`保存凍結sourcehash、Unicode正常輸出、reviewer新增range小變化結果、byte values、SVG XML及原位asset相同核對。reviewer harness不是教學答案，不補算為學生原例已教。

## 待後續階段

N1需由root裁定並在獲授權的修稿中最小處理，I1由root依據收益與篇幅選擇；本角色沒有自行修改正文/source。原始初判保存後才能互看其他角色紀錄；若交叉或修後複查，另存followup，標出提示來源／時點與新版本，不覆寫本文件。

本報告沒有宣稱第3章整章已通過全部QA，也沒有擴到第4章本角色作者稿。實際頁面仍未驗證。
