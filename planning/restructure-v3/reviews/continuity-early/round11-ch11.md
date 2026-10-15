# 第11章獨立銜接與實頁初判

封存 2026-10-15T09:54:20.015315+00:00；來源始 2026-10-15T09:33:09.221663+00:00。**必要0，可選0。** 角色為continuity與rendered-page reviewer，非作者/technical/firstreader。依實際1–10已讀前綴順讀11.1→11.11，封存前未讀他角結論/作者validation/knownaudit/rootdisposition；未用12+內容補。不是盲首讀，已知尾部窄核預露與一次過寬rg限制在文末明記。

187實檔hash始末全不變；124 Rust、40 expected、8 SVG、11 canonical、package/workspace/lock/toolchain。174完整include occurrences每viewport逐一與實檔吻合；19具名region另依全檔核首次位置。全章沒有details折疊，完整helper/test/answer皆普通同頁內嵌。

## 發現

沒有需要修正的銜接或實頁finding。下列為實際正面證據，不以篇名推先教覆蓋，也不以技術能編譯替代學習順序。

## 概念四題與首次必要

引文逐行從本輪frozen原始source抽出；判讀與引文分列。Q2機制／性質與Q2獨立需要／用途各自評已足夠。

### Sized 與 size_of

首次必要：11.1 fixed_bytes16前。

docs/chapters/11-01-lesson.md:3

> 第 3.7 節用 slice 借不同長度的資料，第 3.8 節用 `&str` 借文字。現在比較兩件事：三項 i32 array 本身需要 12 bytes；同一個 `&str` 型別卻可以借 5 bytes 的 Rust!，也可以借 13 bytes 的 Rust tutorial。型別大小與這次借到的資料大小，究竟在哪裡分開？

docs/chapters/11-01-lesson.md:7

> `i32` 使用 4 bytes，`[i32; 3]` 使用 12 bytes。array 改成 `[10, 20, 30]` 或 `[80, 120, 200]`，都不改變這個型別需要的空間；把長度改成 2，則換成另一個型別 `[i32; 2]`。

docs/chapters/11-01-lesson.md:9

> **`Sized`** 是標準 trait，表示這個型別的大小在編譯時可確定。編譯器依型別判斷它，不需要我們寫方法或自行實作。一般區域變數與按值傳入的參數，需要這種已知大小的表示。Sized 由第 5.2 節的 prelude 提供，不用另外 use。

docs/chapters/11-01-lesson.md:11

> 先用標準函式 **`std::mem::size_of::<T>()`** 查詢：它不接收值，要求 T 是 Sized，回傳 usize，單位是 bytes。`std::mem` 是標準函式庫處理記憶體相關工作的 module；這裡只查大小。`::<T>` 沿用第 5.1 節的 turbofish，明選要查的型別。

docs/chapters/11-01-lesson.md:13

> 泛型參數 `<T>` 預設包含 `T: Sized`，所以以下 fixed_bytes 能把 T 交給 size_of：

- Q1：Sized是編譯時可確定型別表示大小的標準trait，無需手寫method。
- Q2機制／性質（已足夠）：array內容改變不改型別大小，長度改變才是另一型別；泛型T預設Sized。
- Q2獨立需要／用途（已足夠）：先分清固定型別大小與同型別reference借到的不同資料大小，才處理下節遞迴表示。
- Q3：size_of無輸入，明選T回usize/bytes；turbofish回用5.1。
- Q4：i32四bytes、三項array十二，80換200仍同大小；不推值必在stack。

### DST、reference與實際資料

首次必要：11.1 borrowed_bytes38及sizes60前。

docs/chapters/11-01-lesson.md:23

> **DST** 是 **dynamically sized type**，也就是大小不能只由型別在編譯時確定的型別。第 3 章的 str 與 `[i32]` 都是例子：str 可以有不同的 byte 長度，`[i32]` 可以有不同的項目數。

docs/chapters/11-01-lesson.md:25

> 它們要與 `&str`、`&[i32]` 分開。reference 自身保存定位資料需要的資訊；對這兩種 slice，資訊包含位置與長度。這份表示有固定大小，所借的內容卻可以長短不同：

docs/chapters/11-01-lesson.md:35

> 標準函式 **`std::mem::size_of_val(value)`** 接收一份 `&T`，回傳被借資料的 bytes；它也接受 DST。要把這種能力放進泛型函式，需明寫 **`T: ?Sized`**，解除預設的 Sized 要求：

docs/chapters/11-01-lesson.md:41

> `?Sized` 表示 T 可以是 Sized，也可以不是；並非要求 T 一定是 DST。參數使用 `&T`，所以函式收到的 reference 自身仍有固定大小；它沒有接手原資料，只交回一個獨立的 usize。

docs/chapters/11-01-lesson.md:70

> 在本教材驗證的 64-bit 環境，輸出是：

docs/chapters/11-01-lesson.md:82

> 16 是這個目標平台的 reference 大小，不是每個平台都必須得到的數字。關鍵是：`fixed_bytes::<&str>()` 查 reference 本身；`borrowed_bytes(word)` 的參數對應 `&T`，此時 T 是 str，查到的才是文字的 5 bytes。長文字換成 13 bytes，reference 型別本身不變。測試也用整段、部分、空文字確認資料大小跟著借用範圍改變。

docs/chapters/11-01-lesson.md:84

> String、Vec 的內容也能增長，但它們仍是 Sized：它們本身保存管理另一處資料的資訊，這份管理資訊有固定大小。不能從「內容可變長」推論型別一定是 DST，也不從 Sized 推論值一定放在 stack。

- Q1：DST全名/中文意義、str與slice是例；&str/&slice自身另為Sized。
- Q2機制／性質（已足夠）：位置與長度是reference表示資訊，資料長度不等reference大小；String/Vec固定管理資訊仍Sized。
- Q2獨立需要／用途（已足夠）：同&str可借Rust!5與Rust tutorial13，避免把型別表示與這次內容混算。
- Q3：size_of_val借&T回資料bytes；?Sized解除預設、可Sized也可DST。
- Q4：整段/部分/空文字tests；64bit reference16限當前目標，非跨平台定值。

### 泛型 reference 不自動解除 Sized

首次必要：11.1 missing_question_sized91前。

docs/chapters/11-01-lesson.md:88

> 下面完整失敗例把 borrowed_bytes 寫成 `<T>`，少了 `?Sized`：

docs/chapters/11-01-lesson.md:94

> 放進第 1.2 節的獨立練習專案執行 `cargo check`，得到 **E0277**。word 的型別是 `&str`，對應參數 `&T` 時，T 是 str；str 不符合這個函式預設的 Sized 要求。只接收 reference，沒有自動改掉泛型參數的 bound。

docs/chapters/11-01-lesson.md:96

> 前面成功版本的 `T: ?Sized` 才表達接受 DST 的需要。完整 `.expected` 如下：

- Q1：只有&T不代表泛型bound自動放寬。
- Q2機制／性質（已足夠）：word&str對&T時T=str，預設Sized不成立；?Sized是精確修法。
- Q2獨立需要／用途（已足夠）：將DST查詢能力放泛型，需表達接受這類T。
- Q3：完整負例與E0277同頁，放1.2獨立練習project不誤跑wholeworkspace。
- Q4：成功與失敗只差必要bound，沒有新未教helper。

### 常數泛型與 array N

首次必要：11.1 fixed_len111前。

docs/chapters/11-01-lesson.md:104

> slice 已能讓函式借不同長度的資料。若希望簽名保留**完整 array 型別與固定長度**，又不想為長度 2、3 各寫一份函式，就可以將長度也設成泛型參數。

docs/chapters/11-01-lesson.md:106

> **常數泛型（const generics）** 讓參數代表編譯時選定的值。`<T, const N: usize>` 中，T 是型別參數，N 是 usize 型別的常數值參數。`[T; N]` 在型別位置使用 N，每次選定 T 與 N 後，仍是一個大小固定的 array 型別。

docs/chapters/11-01-lesson.md:108

> 下面 fixed_len 只借 array，交回它的 N。參數位置的 `_` 沿用第 4 章的忽略 pattern：不替輸入取名稱，但它仍須符合 `&[T; N]`。

docs/chapters/11-01-lesson.md:114

> `fixed_len(&counts)` 可由 array 型別推導 T 與 N。也能寫 `fixed_len::<i32, 3>(&counts)`，依宣告順序明選型別 i32 與常數值 3；第二項不是另一個型別。

docs/chapters/11-01-lesson.md:116

> N 不會等到執行時才變成不同長度。本例仍以普通函式呼叫回傳、顯示 N；「參數在編譯時選定」不等於「這個普通函式只在編譯時執行」。

docs/chapters/11-01-lesson.md:159

> 寫 `array_bytes<T, const N: usize>`，借完整 array，以 N 乘上 `size_of::<T>()` 回傳它需要的 bytes。array 的大小等於元素大小乘上項目數，因此這個乘法適用。T 仍有預設的 Sized 要求。

docs/chapters/11-01-lesson.md:161

> 驗證兩項 i32 是 8 bytes、三項 bool 是 3 bytes、空 i32 array 是 0；把 i32 array 的第一項由 80 改成 200，大小仍是 8。這裡改內容，沒有改型別或 N。

- Q1：const N是編譯時選定的usize值參數，T仍是型別參數。
- Q2機制／性質（已足夠）：每次T/N選定為固定array；不是普通函式改成只編譯期執行。
- Q2獨立需要／用途（已足夠）：slice已能借多長，但需保留完整array型別/固定長度並共用定義時才用N。
- Q3：推導或明選<i32,3>；_只忽略輸入名稱；array_bytes N乘size_of。
- Q4：2/3長、i32/bool、空array、改內容同大小；錯N E0308不自動補項。

### Box owned pointer、借用與 move-out

首次必要：11.2 number34/owned46前。

docs/chapters/11-02-lesson.md:23

> 要改的是 next 的存放方式：目前連結只保存一個固定大小的 handle，讓下一份完整 Chain 放在另一處。這裡還希望目前連結管理下一份的 ownership，因此使用 Box。

docs/chapters/11-02-lesson.md:27

> **`Box<T>`** 是標準函式庫提供的 owned pointer 型別，負責它管理的 T 與配置。Box 由 prelude 提供，不用另加 use。對本節這些非零大小的值，**`Box::new(value)`** 取得 value，為它安排 heap 空間，交回 Box。

docs/chapters/11-02-lesson.md:29

> 當 T 是 Sized，`Box<T>` 自身以固定大小的單一 pointer 表示；T 的內容另外存放。因此移轉 Box 不需要複製一份完整的 T。Box 自身也可以放在另一個 struct 或 heap 資料中，不必然在 stack。

docs/chapters/11-02-lesson.md:31

> 先用一個 i32 看清楚如何借讀、借改與交接，再把這些操作用回數字鏈。`*boxed` 找到 Box 管理的值；`&*boxed` 借讀它，`&mut *boxed` 借改它：

docs/chapters/11-02-lesson.md:37

> view 最後讀完 120，才建立 writable 將原數字改成 140。writable 用完後，`let second = first;` 把整份 Box move 給 second；Box 不會因為裡面是可 Copy 的 i32，就自行變成 Copy。

docs/chapters/11-02-lesson.md:39

> 此時 `*second` 讀出的是 i32，可以 Copy 來顯示與比較。只有 second 繼續管理這份配置，原 first 不能再用。

docs/chapters/11-02-lesson.md:43

> 若持有 `Box<Amount>`，而 Amount 沒有 Copy，Box 仍支援用 `*boxed` 把裡面的完整 Amount move 出來：

docs/chapters/11-02-lesson.md:49

> taken 接手 Amount，原 boxed 不再可用。這項 Box 契約不同於第 3.6 節的 shared reference：借讀者沒有權限將非 Copy 的原值移走；這裡本來就持有整份 Box。也不能從這個例子推論每種 pointer 都允許 move 出內部值。

docs/chapters/11-02-lesson.md:71

> Box 的最後 owner 結束時，會依 ownership 規則清理仍由它管理的值與資源。這裡的重點是分清接手、借用與取出，不是為單一 i32 配置 heap 空間來改善計算。

- Q1：Box owned pointer負責T與配置，prelude；本例非零值new安排heap。
- Q2機制／性質（已足夠）：T Sized時固定單pointer，Box本身位置不必stack；moveBox≠CopyBox；借讀≠接手。
- Q2獨立需要／用途（已足夠）：需固定handle且管理下一份ownership；單i32只用來看操作，不宣稱加快運算。
- Q3：&*、&mut*、*複製i32；Box支援owned非CopyAmount move-out，不能泛推所有pointer。
- Q4：先借120→改140→moveowner，另Amount80取出；afterMove E0382完整。

### 遞迴型別的有限 owned 表示

首次必要：11.2 inline_chain12與chain92前。

docs/chapters/11-02-lesson.md:3

> 第 11.1 節分開了型別的固定大小與所借資料的大小。現在要表示一條數字鏈：每個連結保存一個數字，以及下一段鏈；最後用 End 收尾。實際資料只有 80、120 兩項，但同一型別也希望能表示空鏈或更多項。

docs/chapters/11-02-lesson.md:5

> 這份數列仍可放在 Vec。這次改用連結，是要理解一個存放問題：型別的欄位又包含同一型別時，如何取得固定大小的表示？這個問題也會出現在包含子節點的樹狀模型；本節先只處理有限短鏈，總和不超過 i32 範圍。

docs/chapters/11-02-lesson.md:9

> **遞迴型別（recursive type）** 指型別沿著自己的欄位關係，又回到自身。這裡的 Chain 有 End 與 Link；若 Link 的 next 直接保存另一份完整 Chain，就得到以下失敗例：

docs/chapters/11-02-lesson.md:15

> 放進獨立練習專案執行 `cargo check`，得到 **E0072**。要決定一份 Chain 多大，編譯器得考慮 Link；Link 包含 i32 與完整 Chain，而那份 Chain 又要能容納同樣的 Link，於是問題不斷回到自身。

docs/chapters/11-02-lesson.md:17

> End 可以讓某筆**資料**結束，卻沒有替 Chain 的**型別表示**取得固定大小。即使只打算建立兩項，Rust 仍須先決定這個型別的表示。完整 `.expected` 如下：

docs/chapters/11-02-lesson.md:89

> 回到開頭的連結。把 next 改成 `Box<Chain>`：

docs/chapters/11-02-lesson.md:95

> 每一份 Link 現在只直接保存 i32 與 Box handle；handle 管理另一處的 Chain。決定目前連結的表示時，不必再將下一份完整 Chain 不斷內嵌，因此編譯器能決定固定大小。

docs/chapters/11-02-lesson.md:97

> 下圖對應稍後程式的 80、120、End。實線箭頭表示 Box 管理下一份 owned Chain；它不是 shared reference 的借用箭頭。框與距離是關係示意，不是按比例的記憶體布局：

docs/chapters/11-02-lesson.md:101

> 最外層 Chain 由 main 的 chain 持有；第二份 Link 與最後的 End，分別由前一份 next Box 管理。三個位置共同表示有限的兩項資料，型別的直接內容卻不再隨鏈長無止境展開。

- Q1：recursive type是沿欄位回自身，資料只有有限80/120/End。
- Q2機制／性質（已足夠）：End終止資料不解決型別表示；Box隔開直接內嵌，大小能收束。
- Q2獨立需要／用途（已足夠）：Vec能保存這份數列，改鏈為理解自含型別表示問題，未製造Vec無法處理的假需要。
- Q3：每next Box管理下一份owned Chain；圖是關係而非比例layout。
- Q4：三位置mainLink80→heapLink120→heapEnd與完整程式一致；不要求先懂遞迴函式。

### 接手鏈的 loop 與完整 helper

首次必要：11.2 into_total108前。

docs/chapters/11-02-lesson.md:105

> into_total 接受 owned Chain。沿用第 4 章的 match 解構，Link 分支接手數字 value 與 Box next；累加後用 `*next` 取得下一份 owned Chain，重新指派給 chain。End 分支直接回傳總和：

docs/chapters/11-02-lesson.md:117

> 這裡每輪接手、拆開目前連結，再交接下一份，原鏈因此被消耗。遞迴的是**型別關係**；處理函式使用已教的 loop，沒有另外加入遞迴呼叫。空鏈第一次就遇到 End，直接交回初始總和 0。

docs/chapters/11-02-lesson.md:121

> 這一步沒有 library。`examples/chain.rs` 與本頁練習答案都明寫 `mod chain_support;`，只載入同層 `chain_support/mod.rs`。以下是完整 helper，只有 Chain、into_total 與兩項測試：

docs/chapters/11-02-lesson.md:127

> 第一項測試分別驗證空鏈、一項、兩項；第二項確認 0、-5、5 都作為資料參與加總，不把 0 當成結束標記。真正的結束標記是 enum 的 End。

docs/chapters/11-02-lesson.md:129

> 完整呼叫者從 End 向外建立 120，再建立 80；設定沿用本頁 manifest：

docs/chapters/11-02-lesson.md:145

> 兩項 test 也應通過。Box 回答了如何以 owned 間接存放取得固定表示；數字仍按原順序相加。這沒有證明連結表示比 Vec 快，String 與 Vec 也不需要再套 Box 才能自行管理 heap 資料。

docs/chapters/11-02-lesson.md:149

> 建立 40、80、120 三項鏈，以 End 收尾，交給同一份 into_total，確認總和 240。先從最內層的 End 往外組，說出每個 next Box 分別管理哪份 Chain。

- Q1：函式接owned Chain，每輪match解構再接下一份。
- Q2機制／性質（已足夠）：累加value，*next取ownedChain並重新指派；End立即return；原鏈被消耗。
- Q2獨立需要／用途（已足夠）：有固定表示後仍要完成總和200，並保空鏈與任意合法數字。
- Q3：已教loop/match/mut重新指派；chain_support僅當步型別/函式/兩test。
- Q4：empty0、一項、兩項，0/-5/5非sentinel；答案40/80/120→240完整同頁，無速度推論。

### Drop destructor 與清理時點

首次必要：11.3 Tracked12/完整scope24前。

docs/chapters/11-03-lesson.md:3

> 第 3.2 節已知道，仍由局部變數持有的值會在離開 scope 時結束；第 11.2 節的 Box 也負責自己管理的內容。現在讓自訂資料在清理時印出 id，追蹤先後順序。這只是讓收尾工作可見，沒有自行實作記憶體釋放。

docs/chapters/11-03-lesson.md:7

> **destructor** 指值結束時的清理動作。Rust 會處理值已經擁有的欄位；若還需要額外工作，可以實作標準的 **`Drop` trait**。它由 prelude 提供，必要方法是 `drop(&mut self)`：借改即將清理的值，完成自訂工作，回傳 unit。

docs/chapters/11-03-lesson.md:9

> 下面的 Tracked 只保存 i32 id。`impl Drop for Tracked` 沿用第 6.1 節的 trait 實作形式，但這項方法由 Rust 的清理流程呼叫：

docs/chapters/11-03-lesson.md:41

> 正常離開內層 block，局部變數依宣告的反序清理，所以先3、再2。外層 first 仍持有 id1，直到 main 結束才清理。

docs/chapters/11-03-lesson.md:43

> view 最後讀取之後，還印出了「main準備結束」，才清理 first。**reference 的最後使用與 owner 的清理位置不同**：借用完成，讓後續能做其他合法存取，不表示原值立刻被清理。

- Q1：destructor值結束清理，Drop加額外工作；drop&mutself回unit由Rust流程叫。
- Q2機制／性質（已足夠）：局部變數正常離scope反序；reference最後用與owner清理位置不同。
- Q2獨立需要／用途（已足夠）：讓已知ownership收尾可見，追id順序而非自己釋放記憶體。
- Q3：trait impl沿6.1，Tracked只id，內2/3後外1。
- Q4：輸出3→2、還能讀外1、main準備結束後1，吻合模型。

### mem::drop 是 owned 交接

首次必要：11.3 drop_early52/直接method負例77前。

docs/chapters/11-03-lesson.md:47

> 標準函式 **`std::mem::drop(value)`** 接收值、回傳 unit。非 Copy 的值 move 進去，在函式返回前完成正常清理，呼叫者不能再使用原值。用 `use std::mem::drop;` 引入後可直接寫 drop；它是普通函式，沒有 `!`。

docs/chapters/11-03-lesson.md:66

> first 交給 second 時，值只換了 owner，沒有當場清理。`drop(second)` 才交出並清理這份值；main 結束時不會再清理同一份資料。

docs/chapters/11-03-lesson.md:68

> 對 i32 或 shared reference，傳入 drop 的是 Copy 副本。因此 `drop(&value)` 不會清理原 value；它只收到一份 reference 值。

docs/chapters/11-03-lesson.md:72

> Drop 的方法只收到 `&mut self`，不是接手整份值。若讓一般流程直接呼叫收尾方法，原值卻仍由呼叫者持有，後續使用與自動清理的責任就不再清楚。Rust 禁止直接呼叫它；提早收尾使用接收 owned 值的 std::mem::drop。

docs/chapters/11-03-lesson.md:84

> 本節負例都可放進第 1.2 節的獨立專案，執行 `cargo check`。下面另一份得到 **E0505**：view 在 drop 後還要使用，因此原 value 不能先被交走。把最後一次借讀移到 drop 前，才符合原本的 borrowing 規則。

- Q1：stdmemdrop普通函式不是macro，接值回unit。
- Q2機制／性質（已足夠）：非Copy move進去完成正常清理；move換owner不drop；Copyreference副本不清原值。
- Q2獨立需要／用途（已足夠）：需要提早清理時明確移交owner，避免直接呼叫Drop仍保留原owner的責任衝突。
- Q3：use後drop(second)，禁止直接traitmethod E0040；borrow之後還用則E0505。
- Q4：固定id1只清一次，後續仍繼續；負例完整expected。

### owned 欄位清理與 Drop 限制

首次必要：11.3 drop_fields101與copyDrop122前。

docs/chapters/11-03-lesson.md:96

> 再把兩份 Tracked 交進 Pair 的 first、second 欄位。Pair 的 Drop 先讀欄位、顯示自己的收尾工作；這項方法結束後，Rust 繼續清理 owned 欄位。

docs/chapters/11-03-lesson.md:98

> **struct 欄位按型別定義中的欄位順序清理**，與局部變數的反序不同。下面的 first、second 是 Pair 欄位名，不是前面例子的局部變數：

docs/chapters/11-03-lesson.md:115

> 一般持有 String、Vec 或 Box 的 struct，不需要為了它們的正常清理再手動重做一遍；這些欄位本來就有自己的清理責任。Drop 用於所需的額外工作。本節觀察的是正常離開 scope 與顯式 drop 的流程。

docs/chapters/11-03-lesson.md:119

> 第 6.3 節的 Copy 允許隱含複製；Rust 不允許同一型別同時實作 Copy 與 Drop。即使欄位只有 i32，下面完整負例也得到 **E0184**：

docs/chapters/11-03-lesson.md:129

> 有 Drop 的值仍可以**整體 move**，如 first 交給 second。但不能直接 move 出它的非 Copy 欄位，留下不完整的值等待自訂收尾。以下 Labelled 的 Drop 要讀 label；即使 main 擁有整份值，也不能先取走 String 欄位，會得到 **E0509**：

docs/chapters/11-03-lesson.md:139

> 只需要讀文字時，改用 `value.label.as_str()` 借讀；需要交接時，可以交出整份 Labelled。Drop 內讀 i32 id 則能 Copy 數字，沒有把欄位移空。

- Q1：Pair自身Drop後，Rust仍清owned欄位。
- Q2機制／性質（已足夠）：欄位依型別定義正序不同局部反序；Copy+Drop禁止；不能move非Copy欄位留不完整Drop值。
- Q2獨立需要／用途（已足夠）：額外工作要讀仍完整欄位，也不能手動重做String/Vec/Box原責任。
- Q3：i32可Copy讀欄、String可as_str借或整體move。
- Q4：Pair10/20先自身再10再20；E0184/E0509；答案先drop1再讀2/最後drop2完整。

### Deref Target、星號與 DerefMut

首次必要：11.4 Wrapper10/Deref18/DerefMut30前。

docs/chapters/11-04-lesson.md:3

> 第 11.2 節用 `&*boxed` 借出 Box 內的值。現在自己建立 Wrapper，保存 String，卻要呼叫既有的 `text_bytes(text: &str)`。直接讀欄位再用 as_str 已能完成；這節讓外層提供共同的借用介面，省去每個呼叫者重寫欄位路徑。

docs/chapters/11-04-lesson.md:7

> Wrapper 只是一般 struct；value 欄位擁有 T，不會自行把 T 另外配置到 heap：

docs/chapters/11-04-lesson.md:13

> **`Deref`** 是 `std::ops` 的標準 trait，提供 shared 借用內容的契約。associated type **`Target`** 選定借出什麼型別；`deref(&self) -> &Self::Target` 借讀外層，交回指向目標的 reference。這沿用第 6.4 節的 associated type，以及第 7.4 節輸出連到 receiver 的省略規則。

docs/chapters/11-04-lesson.md:15

> 標準契約允許 Target 是 `?Sized`，因此目標也可以是 str 或 slice。這份 Wrapper 實作先選欄位的 T，body 只借出原欄位：

docs/chapters/11-04-lesson.md:21

> `&self.value` 沒有 clone 或 move；欄位若是 String，仍由 Wrapper 持有。

docs/chapters/11-04-lesson.md:25

> 對 Wrapper 使用 `*wrapped`，Rust 可經由 Deref 取得 `&T`，再存取它指向的 T。理解 shared 讀取時，可以連到 `*wrapped.deref()`。若 T 是 i32，可以 Copy 數字；若是 String，得到存取位置不等於能移出 ownership。

docs/chapters/11-04-lesson.md:27

> 要透過外層修改內容，再實作 **`DerefMut`**。它同樣位於 std::ops，要求型別已有 Deref，沿用相同 Target；`deref_mut(&mut self) -> &mut Self::Target` 把 mutable 借用連到原欄位：

- Q1：Deref標準借用契約，Target associatedtype可?Sized；普通WrapperownedT不自動heap。
- Q2機制／性質（已足夠）：deref借外層→目標ref，星號存取不授權move非Copy；DerefMut要求Deref同Target。
- Q2獨立需要／用途（已足夠）：既有text_bytes要&str，讓wrapper提供共同借用介面，免caller重寫欄位路徑。
- Q3：先明叫deref/deref_mut，i32Copy、String仍由Wrapper持有。
- Q4：basic完整helpers/main只有當步操作，不偷用稍後coercion/lookup。

### deref coercion 與 method lookup 的順序

首次必要：11.4 coercions80前；直接Wrapper內層method在finalanswer144。

docs/chapters/11-04-lesson.md:63

> **deref coercion** 指 Rust 在需要某種 reference 型別的位置，透過 Deref 把 reference 轉成借用內容的形式。coercion 在這裡是依型別要求的轉換，沒有複製內容或交出 owned 值。

docs/chapters/11-04-lesson.md:65

> 例如函式要求 `&str`，卻收到 `&Wrapper<String>`：Wrapper 的 Deref 先借到 `&String`；標準 String 有 `Deref<Target = str>`，因此還能接著借到 `&str`。圖中每個向下箭頭都是繼續借讀原內容：

docs/chapters/11-04-lesson.md:69

> 標準 Vec 也有 `Deref<Target = [T]>`，所以 `&Wrapper<Vec<i32>>` 能提供 `&[i32]`。mutable 形式透過 DerefMut 提供 `&mut T`；mutable 也能依需要提供 shared 借用，shared 則不能因此變成 mutable 權限。

docs/chapters/11-04-lesson.md:97

> **method 查找也會沿 Deref 尋找內容的方法，再依 receiver 要求借用。** 因此對 mutable 的 `Wrapper<String>` 可呼叫既有 push_str，借改原 String；as_str 則借讀它。這適合外層確實要提供「像內容一樣借用」的介面；若只希望開放少數業務操作，可以保留具名 method，而不讓所有內容方法都參與查找。

- Q1：coercion於需要reference位置連續借，methodlookup另有自動查找角色。
- Q2機制／性質（已足夠）：Wrapper→String→str不複製文字；Vec→slice，mutable能給shared，不能逆升權限。
- Q2獨立需要／用途（已足夠）：保留wrapper ownership而借給既有str/slice函式。
- Q3：實際80root用wrapped.value.as_str，不是wrapped.as_str；97lookup在144首次直接inner-method之前。
- Q4：兩箭頭圖與text_bytes(&wrapped)同值，同頁三helpers；順序經真source核，無想像finding。

### into_inner 與 borrow/Copy 邊界

首次必要：11.4 into_inner116與完整answer144前。

docs/chapters/11-04-lesson.md:101

> Box 支援以星號取出 owned 非 Copy 值，是第 11.2 節的特定契約。自訂 Deref 只提供 reference，不能因此對 `Wrapper<String>` 寫 `let taken = *wrapped;`。以下完整負例得到 **E0507**：

docs/chapters/11-04-lesson.md:111

> 放進獨立專案執行 `cargo check` 即可觀察。這份 Wrapper 沒有 Drop；失敗來自借用式介面，而不是上一節有 Drop 時不能移出欄位的限制。

docs/chapters/11-04-lesson.md:113

> 需要交出內容時，另外提供 **`into_inner(self) -> T`**。它接手整份 Wrapper，再 move 出 value；這份 Wrapper 沒有自訂 Drop，因此可直接交出欄位：

docs/chapters/11-04-lesson.md:119

> Target 的其他 trait 也不會自動變成外層的能力。以下完整負例要求 owned 輸入有 Copy，傳入 Wrapper 而得到 **E0277**；雖然欄位是 i32，Wrapper 本身並未實作 Copy：

docs/chapters/11-04-lesson.md:129

> 只需要數字時，可以把 `*wrapped` 讀出的 Copy i32 交給函式。Deref 的借用轉換不會替一般 trait bound 改選另一個 owned 型別。

- Q1：普通wrapper的Deref只借；本地inherent into_inner接owner回T。
- Q2機制／性質（已足夠）：非Dropwrapper可移field；沒有因此取得Copy，與Box特殊move-out分清。
- Q2獨立需要／用途（已足夠）：需要先借改文字、最後真的取出ownedString，不能以coercion誤當owned參數。
- Q3：wrapper_move_out E0507、Copybound E0277後完整實作。
- Q4：完整views兩test與答案追加文字再into_inner，沒有藏未知API。

### T:a 與 implied lifetime bound

首次必要：11.5 Borrowed18/fulltypebounds30前。

docs/chapters/11-05-lesson.md:3

> Box 擁有它保存的值，但那個值仍可能是一份 reference。`Box<&str>` 擁有借用資訊，沒有因此取得原文字。第 7 章已能描述 reference 的來源，現在要問：泛型 T 內部的借用，是否能支持指定的有效範圍？

docs/chapters/11-05-lesson.md:7

> 第 7.2 節的 `'b: 'a` 表示 `'b` 至少涵蓋 `'a`。**`T: 'a`** 則要求 T 所包含的 lifetime 關係至少滿足 `'a`；它沒有命令某個 T 值一定存活到哪裡，也不延長來源。

docs/chapters/11-05-lesson.md:9

> | T | 如何符合 `T: 'a` |

docs/chapters/11-05-lesson.md:15

> 把第 7.3 節的 borrowed field 改成借任意 T。**`?Sized + 'a`** 用加號同時列出要求：允許 T 是 DST，且 T 內的 lifetime 要能符合 `'a`。欄位自己則是 `&'a T`：

docs/chapters/11-05-lesson.md:21

> 在這個特定定義中，`&'a T` 本來就要求 `T: 'a` 才合法，Rust 能推導這項 **implied bound**（隱含要求）。明寫 `+ 'a` 是為了看清關係；刪掉它不會使這份 struct 不合法。這種推導針對相應 lifetime 要求，不表示任意 trait bound 都能省略。

- Q1：T:a限制T內含借用有效期，不是讓外層容器永久。
- Q2機制／性質（已足夠）：&aT本身要求T:a，在這種reference位置可推得，並非任意traitbound都能省。
- Q2獨立需要／用途（已足夠）：把7.3 borrowedfield推廣到任意T且容DST，需要分開T內借用與欄位ref。
- Q3：Borrowed<a,T:?Sized+a>value&aT；i32/String、&bstr、Box<&bstr>比較。
- Q4：fullsource含DST/ref-to-ref/Boxref，Box擁有reference仍借原文字。

### static reference 與 static type bound

首次必要：11.5 staticreference57/require_static85前。

docs/chapters/11-05-lesson.md:48

> **`'static`** 是 Rust 預留的 lifetime 名稱，不需在尖括號宣告。寫在 reference 上時，表示被借資料能在程式剩餘的執行期間保持有效。

docs/chapters/11-05-lesson.md:50

> 前面使用的文字 literal 本來就有 `&'static str` 型別。保存這份 reference 的局部變數結束，不會使 literal 文字跟著消失。

docs/chapters/11-05-lesson.md:52

> 較長的有效性也能符合較短的要求。下面 choose_label 回傳 candidate，或固定 literal Rust；簽名依第 7.1 節的唯一輸入省略規則，仍把回傳契約連到 candidate。literal 足以提供這項較短的借用，不必把整個函式改成只能接受 static 來源。

docs/chapters/11-05-lesson.md:69

> 第一行是把 reference 值交到外層，不是延長 inside 變數。反過來，局部 String 的借用不能只靠標註變成 static；以下完整負例執行 `cargo check` 得到 **E0597**：

docs/chapters/11-05-lesson.md:81

> 型別 bound **`T: 'static`** 要求 T 不依賴短命外部借用。String、i32、`&'static str` 都符合；借自局部 String 的 `&str` 則不能符合這項要求。

docs/chapters/11-05-lesson.md:83

> 接收者可以持有符合條件的值，而不會因短命外部來源先結束就失效；也可以立即清理它。這是資料依賴的契約，不是延後 drop 的指令。

docs/chapters/11-05-lesson.md:107

> 下面把局部 String 的 `&str` 放進 Box，再交給 require_static，會得到 **E0597**。這次不是 Box 不能保存 reference，而是裡面的借用不能符合 static：

docs/chapters/11-05-lesson.md:117

> 第 7.3 節的 `TextView<'a>` 也可能由呼叫者 own，內部卻仍借文字。判斷 `T: 'static` 要看型別內的借用，不能只看目前是否擁有外層值。需要不依賴原 String 的內容，就交出 owned String；只需局部借用時，則保留實際來源關係。

- Q1：&staticstr資料borrow有效久；Tstatic是型別內無短命外借限制，兩者不同。
- Q2機制／性質（已足夠）：literal能久借、可縮短；ownedString/Message符合Tstatic仍能局部Drop。
- Q2獨立需要／用途（已足夠）：後續traitobject/Any要表達內部借用限制，先避免誤認值必永遠存在。
- Q3：require_static只接owner，MessageDrop證scope；局部Stringstatic/refbox負例。
- Q4：fullhelpers三函式三test；Wrapper與11.4已讀completeviews bytes相同，所有操作已教。

### dyn trait object 與 unsizing coercion

首次必要：11.6 borrowedhelper48/root54前。

docs/chapters/11-06-lesson.md:3

> 第 6.1 節的 Order 與 Bundle 都能提供 amount，但資料形狀不同。現在要把80元訂單與120元組合放進同一份費用清單，取得200；呼叫端再加入20元運費，既有加總函式也應得到220，不必為運費多寫一個分支。

docs/chapters/11-06-lesson.md:13

> 實作相同 trait，不會把兩個型別變成一個。以下完整負例讓兩者都有 Charge，仍不能直接混進同一份 Vec，會得到 **E0308**：

docs/chapters/11-06-lesson.md:23

> 本節負例都可放入獨立專案，執行 `cargo check`。enum 能包住兩種資料，讓每項都成為 KnownCharge；種類固定時這已能完成相同加總。這次額外需要呼叫端自行新增費用型別，不必回來修改 enum 的 variant 與 match，才引出新的介面。

docs/chapters/11-06-lesson.md:27

> **trait object** 讓使用處保留 trait 的操作介面，而不在該位置寫出內部具體型別。型別寫成 **`dyn Charge`**；dyn 並沒有取消 Rust 的型別檢查。

docs/chapters/11-06-lesson.md:29

> dyn Charge 是第 11.1 節的 DST：只看介面，不知道原資料是多大的 Order 或 Bundle。因此透過 reference 或 owned pointer 使用。**`&dyn Charge`** 借讀原資料，不接手，也不需要先配置 Box。

docs/chapters/11-06-lesson.md:31

> 需要 `&dyn Charge` 的地方，可以把符合 Charge 的 `&Order` 或 `&Bundle` 轉成這份視圖。這稱為 **unsizing coercion**：由具體型別的 pointer 轉成 DST 介面。它與第 11.4 節藉 Deref 逐層借出欄位不同，也沒有 clone 原資料；轉換時仍會檢查具體型別的 trait 實作。

docs/chapters/11-06-lesson.md:36

> --8<-- "examples/intro-workspace/type-stages/examples/dyn_borrowed/dyn_charges/mod.rs:amount-interfaces"

- Q1：traitobject保存共同介面並隱藏具體型別，dyn是DST，需reference或ownedpointer。
- Q2機制／性質（已足夠）：unsizing把具體pointer轉共同DST介面，檢Charge、不clone，與Deref不同。
- Q2獨立需要／用途（已足夠）：enum能算200；新需要caller新增Shipping不用改enum/match，非enum完全不能算。
- Q3：generic_amount/dyn_amount，同80/120資料、borrowed不必Box。
- Q4：混Vec具體型別失敗E0308，borrow成功仍保原owner，完整stagedhelper無後文功能。

### owned dyn 清單與 dispatch

首次必要：11.6 dynlist80/root86前。

docs/chapters/11-06-lesson.md:65

> 清單要接手資料，就使用 `Vec<Box<dyn Charge>>`。Box::new 仍先接手具體值；在需要共同型別的位置，`Box<Order>`、`Box<Bundle>` 可 unsize 成 `Box<dyn Charge>`，每份配置仍保存原具體資料。

docs/chapters/11-06-lesson.md:67

> 本頁 Charge 沒有自身 lifetime bound。在函式簽名或 struct 欄位寫 **`Box<dyn Charge>`**，省略的 object lifetime 預設為 `'static`，相當於 `Box<dyn Charge + 'static>`。依第 11.5 節，這要求內容不依賴短命借用，不是要求 Box 永遠存在；本例只有 owned 數字，符合條件。

docs/chapters/11-06-lesson.md:69

> **dispatch** 是選擇對應 method 實作的過程。泛型在具體 T 已確定的版本中可以用 **static dispatch**，編譯時便知道實作；把泛型帶入具體型別、形成相應程式的過程稱 **monomorphization**。

docs/chapters/11-06-lesson.md:71

> trait object 提供 **dynamic dispatch**：handle 保存找到原資料與相應方法的資訊，方法入口由 **virtual method table（vtable）** 提供。Order 與 Bundle 的 handle 型別相同，但連到不同的資料及 amount 實作：

docs/chapters/11-06-lesson.md:75

> 箭頭表示找到資料或方法，不是 clone、move，也不承諾精確位元組排列。

docs/chapters/11-06-lesson.md:77

> 這一步的完整 helper 加入 KnownCharge enum 與 total。total 只借 slice，以 while 逐項呼叫 amount；method 查找經 Box 的 Deref 連到 dyn 介面：

docs/chapters/11-06-lesson.md:83

> 完整 caller 先用 enum 算同一批80+120，再用 dyn 算一次。Shipping 由 caller 自行定義、實作 Charge，只提供 fee；total 沒有新增分支：

docs/chapters/11-06-lesson.md:99

> Vec 的元素型別始終只有一種 Box trait object。新 Shipping 能參與同一份加總，支持了本節的擴充需要；不是證明 enum 無法計算費用。dyn 有方法間接查找，Box 另有配置；這些輸出也沒有證明效能排名，編譯器在能確認目標時仍可能消除間接呼叫。

- Q1：BoxdynCharge同一元素型別保存不同資料；dispatch選method，static/monomorphization/dynamic/vtable各有定義。
- Q2機制／性質（已足夠）：handle找資料與對應method入口；Box簽名預設static不等值長存；不承諾ABI。
- Q2獨立需要／用途（已足夠）：同80+120與enum比，再callerShipping20得到220，確實支援擴充。
- Q3：total借slice/while，BoxDeref查method沿11.4；i32缺Charge E0277。
- Q4：圖兩handle各兩箭頭Order80×1與Bundle120；無性能排名推論。

### 工廠的具體、opaque、dyn 與 dyn Fn

首次必要：11.6 opaque負例116/factories130前。

docs/chapters/11-06-lesson.md:113

> 第 9.3 節的 RPIT 仍選定一種具體結果。以下完整負例雖然兩分支都有 Charge，Order 與 Bundle 仍不是同一個 opaque 型別，因此得到 **E0308**：

docs/chapters/11-06-lesson.md:123

> 若確實需要兩種資料形狀，可讓兩分支各建立 Box，轉成相同的 `Box<dyn Charge>` 結果。make_bundle 直接交回 Bundle；make_opaque_bundle 固定選 Bundle、只公開 Charge；make_charge 則能在兩種資料間選擇。

docs/chapters/11-06-lesson.md:125

> 同樣地，兩個 closure 表達式有不同型別。**`Box<dyn Fn(i32) -> i32>`** 保存共同的 Fn 呼叫介面，Box 中的工作可用 `work(value)` 呼叫。本例 closure 只按值保存 Copy 的 base，符合回傳 Box 預設的 static 要求。

docs/chapters/11-06-lesson.md:127

> 以下完整 helper 只加入這些工廠，沒有稍後的 borrowed object 或 compatibility 型別：

docs/chapters/11-06-lesson.md:151

> 泛型或參數 impl Trait 由 caller 提供具體型別；RPIT 由實作固定一個結果型別；enum 列出已知種類；dyn 保存不同具體型別的共同能力。選擇依資料種類、ownership 與公開操作，不是由語法長短決定。

- Q1：RPIT仍一具體型別，dyn能保留共同能力而選不同形狀。
- Q2機制／性質（已足夠）：兩closure表達式不同型別，BoxdynFn提供同call介面，Copybase符合static。
- Q2獨立需要／用途（已足夠）：工廠需要依條件選Order/Bundle或不同closure，不能靠相同trait假定同RPIT。
- Q3：Boxnew兩分支→共同Boxdyn，work(value)呼叫。
- Q4：完整helper/root只此階段；20/40算例，後借用/compat未被早命令編進。

### object lifetime 的明寫與省略

首次必要：11.6 borrowedobjecthelper162前。

docs/chapters/11-06-lesson.md:155

> 現在 BorrowedCharge 只借 prices slice，逐項加總，不複製清單。**`dyn Charge + 'a`** 要求被保存的具體型別內借用至少符合 `'a`；**`Box<dyn Charge + '_>`** 則用第 7.2 節的 placeholder，依唯一輸入來源推導回傳 bound。

docs/chapters/11-06-lesson.md:157

> 這與完全省略 object bound 的 Box 預設不同。`impl Charge for BorrowedCharge<'_>` 也保留模型的相應來源，沒有固定成 static。

docs/chapters/11-06-lesson.md:173

> 輸出為模型借原資料200、dyn reference仍讀200、原資料項目數2。owned Box 沒有取消內容對 prices 的依賴。

docs/chapters/11-06-lesson.md:175

> 若把回傳型別改回預設的 Box trait object，就要求 static。以下完整負例得到 `lifetime may not live long enough`：

docs/chapters/11-06-lesson.md:185

> | 本頁 Charge 的型別位置 | 有效性關係 |

docs/chapters/11-06-lesson.md:192

> 括號把 dyn 與它的 bound 收成被借型別。局部 expression 中的 object lifetime 也可能由使用推導；不能把簽名的 Box 規則推成所有省略位置都 static。

- Q1：dynCharge+a限制內部型別借用；Boxdyn+placeholder依唯一輸入推導。
- Q2機制／性質（已足夠）：ownedBox不消除prices依賴；&a dyn以外ref推objecta，Box簽名defaultstatic與局部expression推導分清。
- Q2獨立需要／用途（已足夠）：希望共同handle仍借原slice，不迫使clone或無法回傳短借。
- Q3：BorrowedCharge impl保留placeholder、borrow_charge→Boxdyn+placeholder；defaultstatic負例。
- Q4：表四位置各合實際source，原prices2項保持，結果200，無回頭偷用static永生誤讀。

### dyn compatibility 與 where Self:Sized

首次必要：11.6 三負例201/211/221及合法compat253前。

docs/chapters/11-06-lesson.md:196

> **dyn compatibility** 表示 trait 符合建立 trait object 的規則；舊名是 **object safety**。Charge 的 shared receiver 提供原資料，回傳 i32 已確定，不需 caller 再知道具體 Self。

docs/chapters/11-06-lesson.md:198

> 在 trait 名稱後寫 `: Sized` 是 **supertrait 要求**：實作者也必須符合 Sized。dyn 本身是 DST，因此整份 trait 若要求 Self:Sized，就不能成為 dyn。以下完整負例得到 **E0038**：

docs/chapters/11-06-lesson.md:208

> 可動態呼叫的 method 也不能另有泛型型別參數：一份方法表不提供 caller 任選 T 的所有版本。下面同樣得到 **E0038**：

docs/chapters/11-06-lesson.md:218

> 若 method 按值回傳未知 Self，dyn caller 也不知道具體結果表示，不能使用這份介面。以下完整負例得到 **E0038**：

docs/chapters/11-06-lesson.md:228

> 需要保留這種 method 時，可以個別限定 **`where Self: Sized`**。**where clause** 把 bound 寫在簽名尾端、body 或分號前；這個 method 只供已知 Sized 的 Self 呼叫，不放進可動態呼叫介面。

docs/chapters/11-06-lesson.md:230

> RebuildCharge 用 `: Charge` 要求共同 amount；rebuild 個別排除 dyn 呼叫，trait object 仍能使用繼承的 amount。這與把整份 trait 限制成 Sized 不同。若硬要透過 dyn 呼叫 rebuild，下面負例會指出 rebuild 不能透過 trait object 呼叫：

docs/chapters/11-06-lesson.md:240

> 普通 associated type 則可以使用，但 object 型別要選定結果，例如 **`dyn Measure<Output = i32>`**。這沿用第 6.4 節的 equality bound，讓 measure 的結果型別明確。只寫 dyn Measure 的完整負例得到 **E0191**：

docs/chapters/11-06-lesson.md:264

> 輸出具體型別可rebuild120、dyn仍可amount120、已選Output結果80。直接以未知 Self 當其他參數，或回傳 RPIT 的 method，也不能直接當可動態呼叫的方法；這些限制不表示它們不能用於一般具體型別。本節列的是當前需要的判斷，不是所有進階 trait 規則。

- Q1：dyncompat舊稱objectsafety；supertrait要求實作者也具能力，whereclause簽名尾bound。
- Q2機制／性質（已足夠）：dynDST不可整traitSelfSized；method泛型/未知Self不能直接動態叫；method個別whereSized可排除該method保trait可dyn。
- Q2獨立需要／用途（已足夠）：隱藏具體Self後仍需知道呼叫入口和結果表示，讓同介面有實際可用操作。
- Q3：RebuildCharge:Charge繼承amount、rebuild只concrete；dynMeasure選Output=i32。
- Q4：E0038/E0191/不能dynrebuild完整；合法root沒有額外traitupcast，complete四tests全先教。

### Error supertraits 與預設 source

首次必要：11.7 leafhelper22/root28前。

docs/chapters/11-07-lesson.md:3

> 第 8 章已把整數解析與件數驗證的失敗保存在 enum，caller 可以 match 分類。現在希望報告器不用知道每個 module 的 variant，仍能顯示「哪項工作失敗」，再沿保存的下層原因繼續說明。

docs/chapters/11-07-lesson.md:5

> Result 本來就沒有要求 E 實作某個 trait。這裡新增 **`std::error::Error`**，是為了共同的顯示與原因借用介面，而不是讓 Result 開始能保存錯誤。

docs/chapters/11-07-lesson.md:9

> Error 要求 **`Debug + Display`**；這是第 11.6 節的 supertrait 關係，兩項要求一起成立，因此借用 Error 介面也能顯示錯誤。

docs/chapters/11-07-lesson.md:11

> 它提供 **`source(&self) -> Option<&(dyn Error + 'static)>`**，借出下一層原因：Some 有原因 reference，None 表示沒有下一層。回傳 reference 依第 7.4 節連到 self；裡面的 static 限制原因型別所含借用，兩者要分開。

docs/chapters/11-07-lesson.md:13

> trait 也可以替 method 提供 body，稱為**預設實作**。Error 的 source 預設回傳 None，所以沒有下層原因時，可以寫空的 `impl Error for CountError {}`；不是缺少必要實作。

docs/chapters/11-07-lesson.md:15

> 先沿用第 8 章0到100的件數規則與 Display，新增 Error 能力。下面這個階段只有 leaf error，不載入後面的原因鏈：

docs/chapters/11-07-lesson.md:25

> caller 引入 Error，才能依 trait method 呼叫 source；完整根檔案如下：

- Q1：Error是Debug+Display及source原因借用介面；Result本來無此要求。
- Q2機制／性質（已足夠）：sourceOption借下一層，receiver外ref與innerstatic分開；預設bodyNone可空impl。
- Q2獨立需要／用途（已足夠）：報告器不認每個variant仍要顯示哪項工作失敗與下層原因。
- Q3：leaf沿8規則只加Error；use trait叫source。
- Q4：Negative-1 Debug/Display/None，缺Debug E0277，完整helper未先載chain。

### 原因鏈、context 與 report 的分工

首次必要：11.7 chain60/64、report76前。

docs/chapters/11-07-lesson.md:53

> ParseCountError 延續第 8.5 節的 Integer／Range，分別擁有 ParseIntError 或 CountError。標準 ParseIntError 也有 Error；因此 **source method** 可以對 `&self` 作 match，將兩種欄位 reference unsize 成共同的 `&dyn Error`。

docs/chapters/11-07-lesson.md:55

> 這次 Display 只說目前這層工作，例如「解析後的件數不合法」，下層原數字留在 source。若外層 Display 已完整顯示原因，再沿 source 顯示一次，就會重複；這是報告器的分工選擇，沒有移除原錯誤。

docs/chapters/11-07-lesson.md:57

> 再用 ReadCountError 保存輸入 String 與 ParseCountError。read_count 失敗時以第 8.5 節的 to_owned 保存輸入，將 cause move 進欄位；這裡只處理傳入文字，沒有檔案 I/O。

docs/chapters/11-07-lesson.md:67

> 圖對應 abc 的三層資料。每個 source 箭頭借讀下一層，不複製或接手它：

docs/chapters/11-07-lesson.md:73

> report 收到 `&dyn Error`，先 format 外層文字，再用 loop／match 追蹤 source。Some 時加入這層 Display，再取下一層；None 時交回 owned String。這些例子的原因鏈都向下前進、最後結束，沒有循環回自己。

docs/chapters/11-07-lesson.md:79

> 以下是這一步的**完整** helper，包含 leaf、解析、輸入 context、report；沒有 borrowed context 或 boxed boundary：

docs/chapters/11-07-lesson.md:105

> 兩條失敗路徑都用同一份 report。abc 尚未取得整數，−1 已取得整數卻不合法；報告保留每層工作與最終原因，沒有猜訊息文字來分類。`?` 只離開當次 helper，caller 仍可處理下一筆。

- Q1：ParseCountError擁原原因，ReadCountError另擁輸入context；source借而非clone/move。
- Q2機制／性質（已足夠）：每Display只本層，report有限loop沿source直到None→ownedString，不重複原因。
- Q2獨立需要／用途（已足夠）：保工作層與最終原因，不靠猜診斷文字分類。
- Q3：From/?沿8；read_count只是文字非FileIO，caller還能處理下一筆。
- Q4：abc三層與-1三層，圖箭頭Some&cause/Some&error/None，完整chainhelpers/main可重現。

### 原因 static 不延長外層 reference

首次必要：11.7 cause_lifetime負例114/borrowedhelper130前。

docs/chapters/11-07-lesson.md:109

> `Option<&(dyn Error + 'static)>` 的外層 reference 借自這次 self；內層 bound 則要求**原因的具體型別**不依賴短命借用。只含 owned 數字的 CountError 與 ParseIntError 符合後者，reference 仍不能超過 error 的實際有效範圍。

docs/chapters/11-07-lesson.md:111

> 下面 Cause 符合 static，卻由內層 Outer 擁有；Outer 結束後還使用 source 借回的 reference，仍得到 **E0597**：

docs/chapters/11-07-lesson.md:121

> 它不是 `&'static dyn Error`。需要在 error 結束後保留報告，可以取得前面的 owned String，而不是把原因 reference 延長。

docs/chapters/11-07-lesson.md:125

> Error 本身沒有整體 static 要求。BorrowedParseError 借一份 label，另外擁有 ParseIntError；整個錯誤依賴 label，source 借出的 cause 型別仍符合 static。

docs/chapters/11-07-lesson.md:127

> with_context 只做整數解析，不做0到100的驗證。label 的 lifetime 明寫到回傳錯誤；text 只用來取得數字，不保存在錯誤中。完整 helper 只有這份錯誤、with_context 與已教的 report：

docs/chapters/11-07-lesson.md:150

> report 借讀外層錯誤，沒有要求它整體符合 static。若**cause 自己**也借短命文字，就不符合標準 source 的內層 bound；以下完整負例得到 `lifetime may not live long enough`：

docs/chapters/11-07-lesson.md:160

> BorrowedLeaf 仍能實作 Error；不成立的是 Outer 想把它當成 static 原因借出。外層 context、原因型別與這次 reference 是三個要分辨的角色。

- Q1：外ref依self短借；內部cause型別static不要求Error本身整體static。
- Q2機制／性質（已足夠）：借label的BorrowedParseError仍能Error，ownedParseInt cause可static；借label的cause自身不行。
- Q2獨立需要／用途（已足夠）：外層需要borrowed context仍要報告，不能把來源borrow永久延長。
- Q3：with_context只整數解析，label進回傳錯誤、text不保存；reportowned可離error。
- Q4：E0597與borrowedcause lifetime負例各對不同限制，full4tests含borrowedlabel與cause原內容。

### BoxdynError 邊界與 ? From

首次必要：11.7 boxedhelper175/root181前。

docs/chapters/11-07-lesson.md:164

> 需要以 match 穩定分類時，具體 enum 仍直接。若某個邊界只需接收不同來源並報告，可以回傳共同的 **`Box<dyn Error>`**；本頁簽名採預設 static object bound。

docs/chapters/11-07-lesson.md:166

> 標準函式庫替符合 `Error + 'static` 的具體錯誤提供到這個 Box 的 From。因此第 8.6 節的 `?` 可以把 ParseIntError 或 CountError 交進 Box，保留原資料；不用為每個錯誤另寫 boxing。

docs/chapters/11-07-lesson.md:172

> 這個對照省略 ReadCountError 的輸入 context，只展示兩種來源如何交給報告邊界。完整 helper 與 caller 如下：

docs/chapters/11-07-lesson.md:192

> caller 用 `&*error` 借 Box 內的 dyn Error，report 不接手 Box。共同 handle 沒把兩種失敗改成同一段文字，也沒有要求所有 library 介面都改用 dyn。

- Q1：報告邊界可用共同owned錯誤handle，分類仍可選具體enum。
- Q2機制／性質（已足夠）：標準From(Error+static)→BoxdynError，?保原錯誤，static不是外borrow長度。
- Q2獨立需要／用途（已足夠）：只需接不同來源並report的邊界，免每一variantboxing手寫；非所有library必須dyn。
- Q3：count_for_report parse? thencheckedcount?；report(&*error)不接owner。
- Q4：3成功、abc/101兩來源原訊息；completeOrderError id7 cause101各顯示一次。

### Rc owner 與 strong 計數

首次必要：11.8 rc_handles31前。

docs/chapters/11-08-lesson.md:3

> 第 11.2 節每段鏈由前一份 Box 管理。現在要兩條鏈分別是80接120、40接120，而且尾端120是**同一份資料**；任何一條先結束，另一條仍能讀它。重建兩份120能算相同數字，卻沒有保留這項共享關係；只借同一尾段則需另外維持 owner。

docs/chapters/11-08-lesson.md:7

> **`Rc<T>`** 位於 std::rc，Rc 是 **Reference Counted**。它用 **reference counting** 記下有多少個共享 owner 持有同一份 heap 資料。保活 T 的 owner 稱 **strong handle**；handle 自己仍是普通值，受 move 與 scope 規則管理。

docs/chapters/11-08-lesson.md:9

> 最後一個 strong handle 結束，才清理內部 T。數的是 Rc owner，不是每次 `&T` 借用；借用不會自行增加 owner。Rc 是單一執行緒內的共享型別，跨執行緒的管理留後章。

docs/chapters/11-08-lesson.md:15

> | `Rc::new(value)` | 接手 T，建立第一個 strong handle，數量1。 |

docs/chapters/11-08-lesson.md:16

> | `Rc::clone(&handle)` | 新增指向同一配置的 handle，strong加1，不 clone T。 |

docs/chapters/11-08-lesson.md:17

> | `Rc::strong_count(&handle)` | 借讀目前 strong 數量，交回 usize。 |

docs/chapters/11-08-lesson.md:18

> | `Rc::ptr_eq(&a, &b)` | 檢查兩個 handle 是否指向同一份配置，交回 bool；不是比較 T 的內容。 |

docs/chapters/11-08-lesson.md:20

> Rc 的 Clone 只複製 handle，因此 T 不必有 Clone。Rc 也提供 `Deref<Target = T>`，可借讀內部資料，並沿第 11.4 節的 method／欄位查找使用它。

docs/chapters/11-08-lesson.md:48

> drop(first) 只移除一個 owner；second 仍讀同一份 id1。drop(second) 才讓 strong 降到0、清理 T。若寫 `let second = first`，則只 move 同一個 handle，first 不能再用，strong 也不會增加。

- Q1：Rc ReferenceCounted管理同heap資料共同owner，stronghandle是普通可move值。
- Q2機制／性質（已足夠）：clone只handle不用TClone，borrow不增owner，最後strong清T；單執行緒此處契約足夠。
- Q2獨立需要／用途（已足夠）：兩條鏈需要同一尾段與獨立存活，不是只複製相同120或另維持借用owner。
- Q3：new1/clone2/count/ptr_eq，Deref沿11.4讀，movehandle數不加。
- Q4：Tracked無CloneCopy仍可共同持有；dropfirst後second可讀，最後owner才DropT。

### Rc 共用尾段與不可任意移改

首次必要：11.8 sharedhelper59/root69與Rc負例91前。

docs/chapters/11-08-lesson.md:52

> SharedChain 保留 End／Link，只把 next 改成 `Rc<SharedChain>`，讓不同開頭共同持有尾段。total 這次接受 `&SharedChain`，不消耗它。

docs/chapters/11-08-lesson.md:54

> match 借讀 enum，value 是 `&i32`，next 是 `&Rc<SharedChain>`。累加 `*value` 後，`chain = next` 在需要 `&SharedChain` 的位置利用 Rc 的 Deref 轉換；沒有 move 或 clone 原鏈。

docs/chapters/11-08-lesson.md:62

> 圖中的 tail、first.next、second.next 是三個 strong handle，指向同一份120；尾段自己的 next 再管理 End：

docs/chapters/11-08-lesson.md:84

> first 結束時，其 next 一起清理，尾段由3份 owner 變2份；再移除 tail，second.next 仍保活共同尾段。這回應了任一條都能先結束的需要，代價是 clone／清理需更新計數，配置也需管理資訊。

docs/chapters/11-08-lesson.md:88

> Rc 有 Deref，沒有 DerefMut。宣告 mutable handle 只允許修改 handle binding，不會替共享 T 取得獨占修改借用。以下完整負例執行 `cargo check` 得到 **E0594**：

docs/chapters/11-08-lesson.md:98

> 也不能像 Box 一樣，直接以星號移出非 Copy 的 String；Deref 借用沒有給我們取走其他 owner 仍需資料的權限。以下得到 **E0507**：

docs/chapters/11-08-lesson.md:108

> 本節只需共同讀取，這個介面已足夠；下一節才建立共享修改狀態所需的受控介面。

- Q1：SharedChain nextRc，total改借整鏈不消耗。
- Q2機制／性質（已足夠）：borrowmatchvalue&i32/next&Rc，Derefcoercion接下一段；Rc無DerefMut/非Copy不能move出。
- Q2獨立需要／用途（已足夠）：任何一條先結束另一條仍讀原尾段，需要共同保活不只是算同總額。
- Q3：三handle tail/firstnext/secondnext指120；firstdrop3→2，再droptailsecond保活。
- Q4：圖三箭頭與nextEnd清楚，200/160；mutablehandle/E0594與moveString/E0507完整。

### Weak、upgrade 與配置存活

首次必要：11.8 weak_observer127前。

docs/chapters/11-08-lesson.md:112

> **`Weak<T>`** 也位於 std::rc，保存不保活 T 的觀察關係。它能在 T 結束後繼續存在，因此不能直接當成有效的 `&T` 讀取，必須先嘗試取得 strong handle：

docs/chapters/11-08-lesson.md:116

> | `Rc::downgrade(&owner)` | 建立同配置的 Weak，不移除 owner、不增加 strong。 |

docs/chapters/11-08-lesson.md:117

> | `Weak::new()` | 建立空 Weak，不指向資料。 |

docs/chapters/11-08-lesson.md:118

> | `observer.upgrade()` | 借讀 Weak，回傳 `Option<Rc<T>>`；仍有 strong 時取得新的 strong，否則 None。 |

docs/chapters/11-08-lesson.md:120

> 成功取得的 Some 裡面是新的 Rc owner，不是普通 reference，所以 strong 會加1；保留它也會保活 T。

docs/chapters/11-08-lesson.md:122

> **值與配置的清理要分開**：最後 strong 結束就清理 T；若還有 Weak，管理計數與位置所需的配置仍保留。這不表示 T 還能讀，upgrade 也不會復活已清理的值；Weak 結束後才不再因它保留配置。

docs/chapters/11-08-lesson.md:124

> 下面的完整程式先試空 Weak，再把成功 upgrade 得到的 temporary 限在 match 分支。離開分支會移除這個暫時 owner，接著才能觀察最後 owner 結束的情況：

docs/chapters/11-08-lesson.md:145

> strong 的1→2→1對應建立、結束暫時 owner。drop(owner) 後立即清理 T，observer 還在卻只能取得 None。若把第一次升級的 Rc 留在外層，它本身就能繼續保活 T，不能只算最早那個 owner。

- Q1：Weak不保T的觀察關係，不能直接當有效&T。
- Q2機制／性質（已足夠）：upgradeSome新增Rcowner，None已無T；T清理與Weak保留計數位置配置分開，不能復活。
- Q2獨立需要／用途（已足夠）：想找到資料卻不強迫它存活，必須處理對象已結束。
- Q3：downgrade不加strong、Weaknew空；temporary限match讓strong1→2→1。
- Q4：dropowner即DropTracked，Weak仍存但upgradeNone；保留upgrade則它真能接替owner，兩test完整。

### reference cycle 與 Weak 責任方向

首次必要：11.8 假設圖151；實際可執行樹在11.9:202後。

docs/chapters/11-08-lesson.md:149

> **reference cycle** 是沿連結又回到原對象的關係。下圖是用來推理的**假設資料圖**，不是本節已建立的可執行程式：A 裡的 Rc 保活 B，B 裡的 Rc 也保活 A。

docs/chapters/11-08-lesson.md:153

> 外部 handle 全結束，A、B 仍各有 strong1，兩邊都不能開始清理，內部 Rc 也無法跟著移除，形成 **memory leak**：沒有外部使用者，資料仍未釋放。Rust 的 borrowing 檢查不會自動清除這種 ownership 循環。

docs/chapters/11-08-lesson.md:155

> 若 B→A 的責任只是觀察，就讓它使用 Weak，不替 A 增加 strong；A 結束後，B 再找它就應接受 None。要看實際哪些方向負責保活，才能安排 Rc 或 Weak。

docs/chapters/11-08-lesson.md:159

> 建立保存120的 Rc，clone 成三個 owner，再建立一個 Weak。確認 strong3；drop 前兩份，第三份仍讀120、strong1。用短暫 match upgrade 後，讓新增 owner 離開分支；最後 drop 第三份，確認 Weak 得到 None。

- Q1：cycle沿連結回原對象，memoryleak無外用者但未釋資料。
- Q2機制／性質（已足夠）：內A→B/B→A各strong1無法啟cleanup；borrowchecker不自動拆owner循環。
- Q2獨立需要／用途（已足夠）：反向若只觀察就不該增加strong，為下節實際父子關係預備必要方向選擇。
- Q3：明標假設不是已能執行之構造，不要求提前懂RefCell才能讀圖。
- Q4：兩邊箭頭與strong來源對稱，答案new+兩clone=三owner，未把題目誤讀三次clone。

### Cell 的值交接

首次必要：11.9 Counter34/caller58前。

docs/chapters/11-09-lesson.md:3

> 記錄器的 trait 若規定 `record(&self, amount: i32)`，呼叫端只有 shared receiver，實作卻想把金額加入清單。普通 Vec 欄位的 push 需要獨占借用，不能直接從 `&self` 取得：

docs/chapters/11-09-lesson.md:15

> 若介面由自己決定，可以改用 `&mut self`；但要實作既定的 shared 介面，就需要在型別內安排受控的修改方式。**interior mutability** 指能經 shared reference 修改內部狀態的設計。Cell 和 RefCell 是標準庫提供的兩種介面，仍需遵守各自的安全條件；普通 `&T` 不會因此都能修改 T。

docs/chapters/11-09-lesson.md:19

> **`Cell<T>`** 位於 std::cell。它不以這裡的操作交出指向內部 T 的 reference，而是讓我們交入、複製或取回值：

docs/chapters/11-09-lesson.md:23

> | `Cell::new(value)` | 接手 T，建立 Cell。 |

docs/chapters/11-09-lesson.md:24

> | `get(&self)` | 當 T: Copy，複製出一份 T。 |

docs/chapters/11-09-lesson.md:25

> | `set(&self, value)` | 接手新 T，取代並清理舊值。 |

docs/chapters/11-09-lesson.md:26

> | `replace(&self, value)` | 接手新 T，回傳原本的 owned T，不要求 Copy。 |

docs/chapters/11-09-lesson.md:27

> | `into_inner(self)` | 消耗 Cell，取出 owned T。 |

docs/chapters/11-09-lesson.md:29

> 計數器每次只需讀一個 i32、加1、寫回；get 交回副本後沒有借用仍指向內部數字，因此同一個 shared receiver 可以完成更新。

docs/chapters/11-09-lesson.md:47

> 仍可用 replace 交入另一份 String、接回原字串，最後用 into_inner 取出目前字串。這不複製文字，也沒有讓兩份 owner 同時管理同一個 String。

- Q1：interiormutability經sharedref安排受控內部修改；普通&T不自動可改。
- Q2機制／性質（已足夠）：Cell這組操作不交內部ref，getCopy副本，set換值清舊，replace返owned舊，into_inner消耗。
- Q2獨立需要／用途（已足夠）：既定record&self或sharedcounter需要更新；若自行定介面也可選&mutself。
- Q3：Counterget+1/set；String非Copy以replace/into_inner交接。
- Q4：shared連叫1/2，Rust換Rust教學，getString E0599，完整helper只有Counter。

### RefCell、Ref/RefMut guard 與 Drop

首次必要：11.9 refcell_guards86/故意runtimefail103前。

docs/chapters/11-09-lesson.md:74

> 清單 push 需要借用 Vec；每次都把整個 Vec 換出來不一定是合適的介面。**`RefCell<T>`** 也位於 std::cell，保存 T 和目前的借用狀態。`RefCell::new(value)` 接手 T；以下兩個方法都接受 `&self`，但會先檢查執行期狀態：

docs/chapters/11-09-lesson.md:78

> | `borrow()` | `Ref<'_, T>` | 可同時有多份 shared guard，不能有 mutable guard。 |

docs/chapters/11-09-lesson.md:79

> | `borrow_mut()` | `RefMut<'_, T>` | 只能有一份 mutable guard，也不能有 shared guard。 |

docs/chapters/11-09-lesson.md:81

> Ref、RefMut 是 **borrow guard**：持有它，就維持這次執行期借用；guard 結束並執行 Drop，才釋放借用狀態。它們也借自 RefCell，不能活得比來源久。Ref 提供 `Deref<Target = T>`，RefMut 另外提供 DerefMut，讓我們以星號或 method 操作 T。它們不是取走內部 T 的 owned 值。

docs/chapters/11-09-lesson.md:83

> 衝突時，borrow／borrow_mut 會 panic。檢查發生在呼叫時，不能因程式通過編譯就假設一定成功。以下完整程式先建立兩份 shared guard，兩份都 drop 後才取得 mutable guard；修改完成再 drop writable，讓讀取恢復：

docs/chapters/11-09-lesson.md:98

> 普通 reference 的借用可在最後使用後結束；guard 是有 Drop 行為的值，不能把「最後讀取」當成 guard 已釋放。

docs/chapters/11-09-lesson.md:100

> 下面刻意省略 drop(view)。即使 println! 已是 view 的最後讀取，接下來仍會在 borrow_mut panic：

- Q1：RefCell保存T與執行期借用狀態，Ref/RefMut是借用guard值。
- Q2機制／性質（已足夠）：多shared或單mutable互斥，call衝突panic；guardDrop才釋放，最後讀不是已Drop。
- Q2獨立需要／用途（已足夠）：Vecpush要借內容，不適合每次整Vec取換；在既定shared介面受控修改。
- Q3：borrow/borrowmut取guard，Deref/DerefMut操作T；兩shared都drop後write再drop。
- Q4：80→80/120總200；省drop能編但runtimefail alreadyborrowed，lifetimeescape仍E0515。

### try_borrow 把衝突交回 Result

首次必要：11.9 try_borrows128前。

docs/chapters/11-09-lesson.md:114

> 若衝突是呼叫端要處理的可能情況，可以使用不因借用衝突而 panic 的版本：

docs/chapters/11-09-lesson.md:116

> - `try_borrow()` 回傳 `Result<Ref<'_, T>, BorrowError>`。

docs/chapters/11-09-lesson.md:117

> - `try_borrow_mut()` 回傳 `Result<RefMut<'_, T>, BorrowMutError>`。

docs/chapters/11-09-lesson.md:119

> 成功的 Ok 保存相同 guard；Err 保存 std::cell 的對應錯誤值，沒有取得新借用。本例不需要錯誤的內容，便以 `Err(_)` 處理。它不會等待另一份 guard 結束，也不會替我們釋放它。

docs/chapters/11-09-lesson.md:121

> 下圖追蹤一份 view：取得後即維持 shared 借用，最後一次 println! 沒有釋放它，所以 try_borrow_mut 回 Err；drop(view) 後才可成功重試。向下箭頭是程式順序：

docs/chapters/11-09-lesson.md:125

> 完整程式先留下 view，觀察 try_borrow_mut 被拒；drop 後重試成功。接著留下 writable，觀察 try_borrow 被拒：

- Q1：tryborrow兩版本結果guard或對應stdcell錯誤值。
- Q2機制／性質（已足夠）：Err沒取得新borrow，不等待/不替你drop，Ok保存相同guard。
- Q2獨立需要／用途（已足夠）：當衝突是caller要處理之可能路徑，選回報而非panic。
- Q3：Err_不需要細error，dropview後重試成功；writable在時shared被拒。
- Q4：圖明最後println仍Err→drop→Ok，程式順序箭頭非資料move；80→100→120。

### 回傳 view 的 Recorder 與 RcRefCell 分責

首次必要：11.9 Recorder152/root158、sharedcells187前。

docs/chapters/11-09-lesson.md:145

> 現在回到 shared 介面的 Record。Recorder 將欄位改為 `RefCell<Vec<i32>>`；record 取得短暫 mutable guard、push、在函式結束時釋放。view 則回傳 `Ref<'_, Vec<i32>>`：`'_` 依 receiver 省略規則，表示這份 guard 借自 self。

docs/chapters/11-09-lesson.md:147

> 這份 view 沒有 clone 清單；它讓呼叫端借看原資料，代價是呼叫端保留 view 時，record 不能取得 mutable guard。先 drop view，再新增40，才能再次讀清單。

docs/chapters/11-09-lesson.md:170

> RefCell 的執行期檢查沒有取代來源的 lifetime 檢查。以下從區域 RefCell 回傳 guard，來源在函式結束就消失，編譯時便得到 E0515：

docs/chapters/11-09-lesson.md:182

> 把兩者組成 `Rc<RefCell<Vec<i32>>>`，Rc 讓多個 owner 保活同一個 RefCell；RefCell 則管理內部 Vec 的借用。clone Rc 增加 owner，不增加 RefCell 的 shared guard。borrow／borrow_mut 增加執行期借用，也不增加 Rc owner。

docs/chapters/11-09-lesson.md:184

> 因此能經由 second 修改，再由 first 讀到同一清單。完整程式中的 mutable guard 限在短 block，先釋放才建立 view：

docs/chapters/11-09-lesson.md:233

> 用兩個 Rc owner 共用一個 Recorder。分別記錄80、120，確認 view 有兩項、總額200；釋放 view 後再記錄40，重新讀取三項、總額240。解釋為什麼只需 `Rc<Recorder>`，不必額外包一層 `RefCell<Recorder>`。

docs/chapters/11-09-lesson.md:235

> Recorder 已把需要修改的 Vec 放入 RefCell，record 也接受 shared receiver，因此 Rc 的 Deref 就足以呼叫它。以下完整 `examples/cells_complete/cells/mod.rs` 此時才合併 Counter、Recorder 與三項測試：

- Q1：viewRef借self，Recordsharedreceiver短mutableguard後return釋放。
- Q2機制／性質（已足夠）：view不clone，caller保留時阻record；Rc增owner不增guard，借用也不加owner。
- Q2獨立需要／用途（已足夠）：多caller觀察/修改同清單仍要清楚保活與當下互斥兩責任。
- Q3：先dropview再40；RcRefCell短blockwrite，RcRecorder已內包RefCell故不需外包。
- Q4：200→240，兩owner與兩sharedguardtests分開驗；sourceescape編譯拒而不是只runtime。

### Weak parent 的實際非保活反向連結

首次必要：11.9 weak_parent209前。

docs/chapters/11-09-lesson.md:202

> 第 11.8 節的假設 strong 循環，現在可以換成實際的樹：父節點的 children 以 Rc 保活子節點；子節點的 parent 只用 Weak 觀察父節點。parent 包在 RefCell 中，是因為 leaf 建立時還沒有父節點，建立 branch 後才經 shared leaf 更新這個連結。

docs/chapters/11-09-lesson.md:204

> `Node` 的 children 是普通 Vec，本例建立時就填妥，之後只讀，不需要把每個欄位都改成 RefCell。

docs/chapters/11-09-lesson.md:206

> 完整程式先把空 Weak 換成觀察 branch 的 Weak，再借讀 parent、upgrade。成功取得的暫時 Rc 限在 match 分支；block 結束時，shared guard 也結束。計算 strong 時已沒有這份暫時 owner：

docs/chapters/11-09-lesson.md:227

> branch 只有區域 owner，離開內層 scope 就清理；其 children 移除一份 leaf owner，但外層 leaf 還在。leaf 的 Weak 不保活 branch，所以再次 upgrade 得到 None。清楚安排哪個方向負責保活，才解決循環；只把更多值放入 Rc 並不會自動處理。

- Q1：父childrenRc保子，子parentWeak觀察父，RefCell只放需要後改欄。
- Q2機制／性質（已足夠）：leaf先無父，branch建立後更新Weak；upgradeRc限match，guard限block，計count時已釋放。
- Q2獨立需要／用途（已足夠）：落實11.8假設循環的方向修法，父結束不被子阻止，子可仍活。
- Q3：借mutparent換downgrade，後借parent升級；children普通Vec建立填妥不需RefCell。
- Q4：branchstrong1/leaf2→Dropbranch2後leaf1、upgradeNone、最後Dropleaf1，全碼同頁。

### alias、newtype 與 tuple struct 身分

首次必要：11.10 aliashelper14/identity47前。

docs/chapters/11-10-lesson.md:3

> 路程80公尺與等待80秒，都能用 i32 保存。現在希望「顯示路程」的函式只接受路程，誤傳等待時間時就由編譯器指出。type alias 能改善名字，但不會建立兩種型別身分；這次需要用 struct 建立邊界。

docs/chapters/11-10-lesson.md:7

> `type MeterAlias = i32` 與 `type SecondAlias = i32` 都替同一個 i32 取名字。因此需要 MeterAlias 的函式仍接受 SecondAlias。

docs/chapters/11-10-lesson.md:9

> alias 適合替重複的長型別取短名，例如第 11.6 節的 `Box<dyn Fn(i32) -> i32>`。取名 PriceFn 後，呼叫、ownership 與 object lifetime 的契約都不變。

docs/chapters/11-10-lesson.md:38

> 第一行沒有攔住單位誤用；第二行將 PriceFn 直接交給完整型別，不需重新 Box 或 clone，因為兩個名稱指同一型別。

docs/chapters/11-10-lesson.md:40

> **newtype pattern** 是用新型別包住原資料的設計方式，不是 Rust 關鍵字。本例使用 **tuple struct**：欄位有順序、沒有名稱。`pub struct Meters(pub i32);` 宣告公有型別，內層 pub 則公開第一個 i32 欄位。`Meters(80)` 建立值，`.0` 取第一個欄位，沿用 tuple 的編號方式。

docs/chapters/11-10-lesson.md:42

> Meters、Seconds 是不同 struct，也都不同於 i32。derive 只選已教的 Debug／PartialEq；沒有選 Copy，所以交接整體仍會 move。借用 struct 時可以複製出 `.0` 的 i32，並不等於 struct 本身有 Copy。

docs/chapters/11-10-lesson.md:76

> caller 必須明確交來路程，才通過這道型別邊界。公有 i32 欄位仍允許負數；這個 newtype 只區分單位，沒有自動完成數值驗證。

- Q1：alias同型別別名；newtypepattern非keyword，tuple struct有順序無名字。
- Q2機制／性質（已足夠）：Meters/Seconds/i32皆不同身分；pub型別與pub欄分開，沒有deriveCopy整體仍move。
- Q2獨立需要／用途（已足夠）：80公尺/80秒需由compiler擋誤傳，alias無法達到；只分單位不自動驗負數。
- Q3：Meters(80)/.0，借Meters可Copy i32field；PriceFn仍原BoxdynFn。
- Q4：alias誤傳接受vsnewtype E0308；ownerclosure直接交完整型別沒rebox/clone，完整stagedmodules。

### Add 的預設 Rhs、Output 與 move

首次必要：11.10 addhelper97/root103前。

docs/chapters/11-10-lesson.md:80

> 兩段路程相接，希望結果仍帶著公尺單位。**`std::ops::Add`** 是 `+` 的 trait 介面；實作它就能定義這種加法。

docs/chapters/11-10-lesson.md:84

> | `Add<Rhs = Self>` | Rhs 是 right-hand side，右運算元型別。等號指定預設型別參數：未另選 Rhs 時，右邊與 Self 同型別。不是替 method 提供預設 body。 |

docs/chapters/11-10-lesson.md:85

> | `type Output` | 實作選定相加結果型別，可以是 Self，也可以另選。 |

docs/chapters/11-10-lesson.md:86

> | `add(self, rhs: Rhs) -> Self::Output` | 接手左右兩值，交回結果。非 Copy 輸入在本例會 move。 |

docs/chapters/11-10-lesson.md:88

> `impl Add for Meters` 以預設 Rhs 接收另一個 Meters，Output 選 Self。`Self(...)` 在 tuple struct 的 impl 中就是該型別的建構方式。

docs/chapters/11-10-lesson.md:90

> 同一 helper 的 `add_same<T: Add<Output = T>>` 另外要求「結果仍是 T」。這是 associated type 的等式 bound，不能只憑型別有 Add 就假定輸出一定相同。

docs/chapters/11-10-lesson.md:92

> 接著有500毫米與2公尺，要交回毫米。`impl Add<Meters> for Millimeters` 明選右型別，先把2乘1000，再加500，得到2500毫米。它只提供這個方向，不會自動產生反向加法或其他運算子。

docs/chapters/11-10-lesson.md:116

> 這些小整數沒有超過 i32 範圍；wrapper 的計算仍受整數規則限制。換算後的型別是 Millimeters，保留了2500的單位。

docs/chapters/11-10-lesson.md:128

> 若要保留輸入，可以另設計借用 method，或依型別語意選擇 Copy；本例沒有暗中 clone。反向 `Meters + Millimeters` 也不符合目前 Meters 的右型別，得到 E0308：

- Q1：Add定義+，Rhsright-handside預設Self不同method預設body。
- Q2機制／性質（已足夠）：Output可Self或別種；add接左右值，非Copy move；associatedequal要求結果T。
- Q2獨立需要／用途（已足夠）：接兩段公尺仍保單位，500毫米+2公尺需先換右單位結果2500毫米。
- Q3：implAdd forMeters，Add<Meters>forMillimeters，Self tuple構造；不自生反向/其他op。
- Q4：200/100/2500、零換算；E0382left再讀、反向E0308，已限制小數不超i32。

### orphan local 與選擇公開介面

首次必要：11.10 foreignalias147/interface165前。

docs/chapters/11-10-lesson.md:140

> 路程清單用 `Vec<Meters>` 保存，報告時想顯示「2段路程，共200公尺」。能否直接為標準 Vec 實作標準 Display？

docs/chapters/11-10-lesson.md:142

> **orphan rules（孤兒規則）** 限制 trait 實作能放在哪個 crate。**local（本地）** 指型別或 trait 定義在目前寫 impl 的 crate。import 標準 Vec 或替它取 alias，不會改變定義來源。

docs/chapters/11-10-lesson.md:144

> 對本例沒有泛型參數的 `impl Display for ...`，Display 或實作型別至少一方必須本地。否則不同 library 都能替同一對標準 trait／型別提供不同實作，合用時便有衝突。以下只替 Vec 取 CountList 別名，仍得到 E0117：

docs/chapters/11-10-lesson.md:156

> 改用 `pub struct MeterBatch(Vec<Meters>);`，就有本地 struct 可實作 Display。內層欄位沒有 pub，helper module 外不能直接建立或讀寫這個欄位。

docs/chapters/11-10-lesson.md:158

> from_values 接手 Vec；count 借讀段數，total 借讀各 i32 後交回 Meters。Display 沿第 8.3 節使用 Formatter／write!，沒有消耗集合。這層 wrapper 接手原 Vec，沒有為包裝另建 Box 或 clone 集合。

docs/chapters/11-10-lesson.md:160

> 另一個合法方向是**本地 trait，外部型別**。本例宣告 BatchSize，再為 `Vec<i32>` 實作 batch_size；它只是借讀 len。

docs/chapters/11-10-lesson.md:184

> newtype 不會自動繼承內部 Vec 的 method。MeterBatch 沒有 push，下面完整負例即使使用 mutable binding，仍得到 E0599：

docs/chapters/11-10-lesson.md:194

> 第 11.4 節的 Deref 能提供借用式 method 查找，但本例沒有替 MeterBatch 實作它。這次只承諾建立、計算、報告；公開整套 Vec 操作會增加 caller 可依賴的介面，不是包住 Vec 就必須公開全部操作。

- Q1：orphanrules管implcrate責任，local定義來源不隨use或alias變。
- Q2機制／性質（已足夠）：本例無impl泛型時trait/self至少一本地；newtype私欄設邊界，不自動繼承Vecmethod。
- Q2獨立需要／用途（已足夠）：想為路程清單Display而標準Vec/Display皆foreign；需避免不同library對同對產生衝突。
- Q3：MeterBatchownVec借count/total/Display；另localBatchSize可implforeignVec。
- Q4：2段200、可再讀，aliasE0117、wrapperpush E0599；不強制Deref公開整套Vec。

### generic orphan、uncovered 與 fundamental

首次必要：11.10 WrappedFrom205/逆From226前。

docs/chapters/11-10-lesson.md:198

> 外部 trait 的泛型實作還要檢查次序：先看實作的 self 型別，再依序看 trait 的型別 arguments。在第一個本地型別以前，不能出現 **uncovered type parameter**，也就是尚未被一般型別包住的 impl 型別參數。例如單獨的 T 就是 uncovered；`Wrapped<T>` 則是本地 wrapper。

docs/chapters/11-10-lesson.md:200

> `impl<T> From<T> for Wrapped<T>` 的 self 型別首先就是本地 Wrapped，符合這項來源檢查。Wrapped 是泛型 tuple struct，欄位型別為 T；from 接手它，into_inner 再整體交回。標準 From 也提供對應 Into，沿第 8.4 節的轉換關係。

docs/chapters/11-10-lesson.md:223

> 反過來 `impl<T> From<Wrapped<T>> for T`，self 的 T 出現在本地 Wrapped 以前，得到 E0210：

docs/chapters/11-10-lesson.md:233

> into_inner 是自己型別的 inherent method，不是在為任意 T 新增外部 trait 實作，所以能完成取回工作。不能只因 trait argument 有本地型別，就忽略前面的 uncovered T；也不能反推所有外部 self 型別都禁止。

docs/chapters/11-10-lesson.md:235

> 規則對 **fundamental types** 有特殊處理：在本地性判斷上，`Box<LocalType>` 視為本地；但 `Box<T>` 不會把 T 視為 covered。相對地，一般 Vec 外層可以 cover T，卻不因內部型別本地就讓 Vec 本身變本地。這是判讀泛型 orphan 診斷的區別，本例沒有另外依這項特殊規則增加 impl。

- Q1：先self再traitarguments的來源檢查；單Tuncovered、Wrapped本地一般wrapper。
- Q2機制／性質（已足夠）：首本地以前不能uncoveredimplT；BoxLocal視local但BoxT不cover，一般Vec可cover但不變local。
- Q2獨立需要／用途（已足夠）：需要泛型wrap/取回任意owned值時，不能只看argument含local就推From反向合法。
- Q3：From<T>forWrapped<T>可配Into，逆forT E0210；自己into_inner不新增foreignimpl。
- Q4：i32/String完整交回；fundamental只診斷角色預告，當前未額外依特殊規則寫新impl，無需深內部。

### coherence 與 blanket overlap

首次必要：11.10 overlappingfixture244前。

docs/chapters/11-10-lesson.md:239

> **coherence（一致性）** 要求實作既通過 orphan 檢查，也沒有互相重疊。**blanket implementation** 是涵蓋一整類型別的泛型實作；例如 `impl<T> Label for T` 已涵蓋每個 T，包括 Meters。

docs/chapters/11-10-lesson.md:241

> 再寫 `impl Label for Meters` 不會覆寫前者，兩份都能匹配 Meters，因此得到 E0119。即使 Label 是本地 trait，也不能這樣重疊：

docs/chapters/11-10-lesson.md:251

> 若不同型別要不同標籤，就不要先承諾所有 T 共用同一份實作。orphan 限制跨 crate 的實作責任，重疊檢查確保適用 impl 的唯一性，兩項都需要成立。

- Q1：coherence兼orphan與唯一適用impl；blanket泛型涵蓋整類。
- Q2機制／性質（已足夠）：impl<T>LabelforT已含Meters，再具體impl不覆寫而重疊。
- Q2獨立需要／用途（已足夠）：需各型別不同label就不先承諾全部T同實作，避免呼叫不唯一。
- Q3：完整本地Label兩impl負例E0119，不能以trait本地忽略overlap。
- Q4：已知trait/generic語法組成，沒有專精/優先規則等後章依賴；完整final三test全已教。

### Any static、payload 與確切型別借回

首次必要：11.11 borrowed29前。

docs/chapters/11-11-lesson.md:3

> 第 11.6 節依共同 amount 能力處理不同型別。現在事件帶著 id 與呼叫端自訂的附加資料；傳遞層只保存它，處理者才取回 i32、String 或自訂 struct。這份附加資料稱為 **payload**，也就是隨事件一起交給後續處理的內容。

docs/chapters/11-11-lesson.md:5

> 已知種類可用 enum；有共同業務操作可用相應 trait。若種類由 caller 擴充，傳遞層不想每次增加 variant，也沒有共同業務操作，就可考慮辨識確切型別再取回。**`std::any::Any`** 提供這項型別辨識能力。

docs/chapters/11-11-lesson.md:9

> Any 要求實作型別符合 `'static`；標準庫已為符合條件的型別提供實作，本節數字、String、沒有 borrow 欄位的自訂 struct 都能直接使用，不另手寫 Any impl。

docs/chapters/11-11-lesson.md:11

> 沿第 11.5 節，String 符合 `'static` 型別 bound，仍可在局部 scope 建立、使用、清理。把它借成 `&dyn Any`，這次 reference 仍只能在原 String 有效期間使用。內部型別不依賴短命借用，與 reference 此次能使用多久，是兩個問題。

docs/chapters/11-11-lesson.md:17

> **downcast** 在這裡指從 dyn Any 的共同視圖，嘗試取回確切型別 T 的資料。caller 在編譯時指定 T，執行時檢查原型別是否相同；不把數字或文字轉成 T。

docs/chapters/11-11-lesson.md:21

> | `view.is::<T>()` | T 須符合 Any；回傳 bool，判斷內部是否就是 T，不取走內容。 |

docs/chapters/11-11-lesson.md:22

> | `view.downcast_ref::<T>()` | 回傳 `Option<&T>`；相同時借原 T，否則 None。reference 依這次 shared borrow 有效。 |

docs/chapters/11-11-lesson.md:24

> Any 的 trait method 提供辨識資訊；標準庫另為 dyn Any 提供這些泛型 helper。它們是對 dyn Any 定義的操作，不是第 11.6 節禁止放入動態呼叫介面的泛型 trait method。

docs/chapters/11-11-lesson.md:53

> Any 不能查詢「有沒有實作任意 trait」，也不能按文字欄位名找到資料。取回 String 後能用 as_str，是因為現在有已知的 String reference。若始終只要 amount，直接要求 Charge 就能表達需要，不必猜每一種原型別。

- Q1：payload事件附加內容，Any標準型別辨識，static具體型別已有標準impl。
- Q2機制／性質（已足夠）：downcast compile選T/runtime比原type，不轉內容；isbool/refOption只借原資料，dyn泛型helper不同trait泛型method。
- Q2獨立需要／用途（已足夠）：caller擴充未知種類且無共同業務操作時辨確切型別；若只要amount仍用Charge。
- Q3：&dynAny由局部ownedString借，Tstatic不讓此次borrow永久；i32/usize同80區別。
- Q4：80同值不同type結果true/false，String原字未移；不承諾任意trait反射或欄位名字查詢。

### Event BoxAny 與 downcast_mut

首次必要：11.11 eventhelper68/root74前。

docs/chapters/11-11-lesson.md:57

> Event 保存 id 與 `Box<dyn Any>`。event 的 `payload: impl Any` 沿第 9.3 節的參數形式，由 caller 選具體型別；函式接手它、交入 Box，再轉成共同的 owned handle，沒有 clone。

docs/chapters/11-11-lesson.md:59

> payload_number 借 Event，以 `&*event.payload` 取得 `&dyn Any`，再 downcast_ref。回傳 `Option<&i32>` 的來源依單一輸入省略規則連回 Event。

docs/chapters/11-11-lesson.md:61

> 修改則使用 **`downcast_mut::<T>()`**：需要 `&mut dyn Any`，回傳 `Option<&mut T>`。成功時借原 T 供修改，失敗時 None；沒有新建 T，也不是經 shared reference 任意修改。

docs/chapters/11-11-lesson.md:63

> add_to_number 接受 `&mut Event`，透過 Box 的 `&mut *` 借內容，找到 i32 才加 extra、回傳 true；其他型別不動、回傳 false。

docs/chapters/11-11-lesson.md:92

> 以下交入的內容是借自局部 String 的 `&str`，不能符合 Any 的 `'static` 要求，得到 E0597：

docs/chapters/11-11-lesson.md:102

> 外層 Box 是 owned，內部 reference 卻仍借原文字。寫 `+ '_` 也不會取消 Any 自己的 static bound。若需要 owned 文字就交入 String；若資料本來必須借來源，就保留能描述來源的具體型別或其他合適介面。

- Q1：EventownedBoxpayload、implAny參數caller選T並移交。
- Q2機制／性質（已足夠）：downcast_mut需&mutdynAny，Option&mutT成功改原、失敗不動，不是shared任意修改。
- Q2獨立需要／用途（已足夠）：傳遞層保不同payload，handler僅處理理解的type。
- Q3：&*Box共同借/view與&mut*Box共同借改；單輸入elision連Event。
- Q4：i3280→100、StringRust不變；借局部as_str不滿Anystatic E0597，Boxowned/+placeholder不消限制。

### owned downcast 的兩種 owner 結果

首次必要：11.11 圖115/any_owned120前。

docs/chapters/11-11-lesson.md:106

> **`boxed.downcast::<T>()`** 接手整份 `Box<dyn Any>`，回傳 `Result<Box<T>, Box<dyn Any>>`：

docs/chapters/11-11-lesson.md:108

> - Ok 保存符合 T 的 owned Box，可以使用或交出 T。

docs/chapters/11-11-lesson.md:109

> - Err 保存原 Box，內容仍在，可以保留或再試其他 T。

docs/chapters/11-11-lesson.md:111

> 它與 downcast_ref 的 `Option<&T>` 責任不同。呼叫後原 boxed 已 move，成功失敗都要從 Result 接手 owner。

docs/chapters/11-11-lesson.md:113

> 圖中 String 原值沒有變；第一次請求 i32 失敗，接回 Err 的原 handle，第二次請求 String 成功。箭頭表示 ownership 交接：

docs/chapters/11-11-lesson.md:117

> 完整程式在成功取得 `Box<String>` 後以 DerefMut 追加文字，再沿第 11.2 節 Box 的特殊 move-out 取出 owned String：

docs/chapters/11-11-lesson.md:132

> 沒有為了辨識而 clone 資料。下面在 downcast 後繼續讀原 boxed，得到 E0382；正確流程需接手 Ok 或 Err：

- Q1：接BoxdynAny回ResultBoxT或原BoxdynAny，兩支均交owner。
- Q2機制／性質（已足夠）：失敗不丟原資料，成功取得具體Box後可借改/Box特殊moveout；原binding都已move。
- Q2獨立需要／用途（已足夠）：不知道原type但需取得owned內容時仍可依次嘗試，不能把失敗當資料遺失或clone。
- Q3：先i32Err接original，再StringOk追加教學，*BoxString取owned。
- Q4：圖Rust保持同文字，兩箭頭handle交接；E0382原Box再用，ExtraInfo2→3完整答案。

### TypeId 的查詢對象與邊界

首次必要：11.11 concrete_id151/root159前。

docs/chapters/11-11-lesson.md:144

> **`std::any::TypeId`** 是用來比較型別身分的值。`TypeId::of::<T>()` 取得符合 `'static` 的 T 之身分，`==` 比較是否同型別。TypeId 提供 PartialEq 與 Debug，所以也能用 assert_eq!；不用解讀內部表示，它不是事件 id 或資料內容。

docs/chapters/11-11-lesson.md:146

> Any 的 **`type_id(&self)`** 回傳 receiver 的 TypeId。透過 `&dyn Any` 呼叫，得到內部具體型別的身分。下面的 concrete_id 則由 `&T` 決定 T，再查 T 的 Any 身分。

docs/chapters/11-11-lesson.md:154

> **Box 自己也是型別。** `Box<dyn Any>` 本身符合 Any，直接 `boxed.type_id()` 會使用 Box 自己的 Any 能力，得到 Box 的身分。要查內容，就先以 `&*boxed` 取得 `&dyn Any`，再呼叫 type_id。

docs/chapters/11-11-lesson.md:156

> 完整 caller 明列兩種對象；concrete_id(&boxed) 的 T 明確是 `Box<dyn Any>`：

docs/chapters/11-11-lesson.md:171

> 若把直接 boxed.type_id 的結果當成 i32，以下完整負例能編譯，執行時 assert_eq! 失敗：

docs/chapters/11-11-lesson.md:183

> 它在比較 Box 與 i32 的身分。TypeId 適合程式內判斷確切型別；其內部表示、hash 與排序沒有跨 Rust 版本的固定保證，不拿它作持久保存格式或外部事件編號。一般處理 payload 時，多半直接用 is／downcast helper 就足夠。

- Q1：TypeId型別身分值，PartialEq/Debug；ofT要求static，不是事件id或內容。
- Q2機制／性質（已足夠）：&dynAny.type_id得內type；Box本身亦Any，直接boxed.type_id得Boxtype。
- Q2獨立需要／用途（已足夠）：比較身分前要先選對receiver，避免把包裝型別當payload型別。
- Q3：concrete_id&T令T明為BoxdynAny；&*boxed改查內i32。
- Q4：正例兩true，故意Boxvs i32 assert runtimefail完整；不用其hash/排序作持久跨版本格式。

## 完整來源閉包與負荷

本章只有 intro-type-stages 一個最小package，manifest四行無dependencies/library。每一早期target只mod其當步helper；後文合併helper直到本節練習才提供，沒有在第一個run命令偷偷編入後文新介面。所有package實檔都列snapshot，另補查未被snippet引用的預設src/main.rs，只有println一句，不隱藏library/build.rs/.cargo。workspace Cargo/lock/toolchain只作設定先備，不算後章Rust已教。

11.1 size_support只有兩查詢函式和一test，array_support另只fixed_len；11.2 chain_support只有Chain/into_total與兩test；11.3每個scope/drop/field失敗/成功target各完整。11.4 basic顯式deref/deref_mut，coercions另只三個借用函式，complete才into_inner/tests。重要實際核對：coercions caller的文字讀法是wrapped.value.as_str()，沒有在97行method lookup之前使用wrapped.as_str()；後者直到已教後的完整答案才用，故沒有把想像的早用列finding。11.5 type_bounds不載後static helper，最終三函式三test，此處Wrapper與11.4 completeviews byte-identical，仍核完整根檔所有操作。

11.6 borrowed→dyn_list→factories→dyn_lifetimes→dyn_compatibility→dyn_complete各具獨立module/完整caller。完整helper的四test只在全部能力說完後出現；compatibility caller沒有另外未教trait-upcast操作。11.7 leaf→chain→borrowed→boxed→complete亦各獨立閉包；每層Display只本層工作，source借原cause，無隱藏clone/規則。完整四test含具體原因、逐字報告、borrowedcontext、boxed兩來源；OrderError答案完整。11.8 sharedtail只有當步資料/total，complete才加兩Weak/shared tests；假設循環圖明標不是可執行實例，不要求先教11.9內部修改才能讀。11.9 Counter早helper與Recorder晚helper分開，complete才三tests；Weakparent全程短blockguard/暫時Rc位置與計數吻合。

11.10 aliases、identity、add、interface、from、complete分階段，不讓前面alias命令帶入Add/orphan的未解釋實作。最終TimeBatch根答案與helper三test全讀。fundamental types段只是當前診斷辨別且明說無額外新impl，這裡不需深特殊規則內部。11.11 borrowed、events、owned、type_id、complete依序；Event第一次完整helper不含稍後TypeId或tests，完整兩test與ExtraInfo答案才合併。Anystatic與source/static、Boxmoveout等均準確回用已讀前綴，不以先前尾預露當已教。

負例40expected與對應完整檔都讀，編譯失敗與runtime失敗區別明標（guard衝突、BoxTypeId assert）；沒有因fixture注定失敗便放過其中未教語法。完整解答/測試全部使用當時已解釋的操作：Copy小值、moveBox、match借用、Deref、Result/From/?、有界while/loop、format/Display、Vec/slice與assert。全章逐步延長來源長度而保持當步閉包；長helper有清楚同頁完整標籤，不把「只run單target」誤當能不理解同target helper/tests。

## 實頁與圖文

actual preview 11頁各1280×800與390×844，22頁record時點/HTMLhash詳visual metadata，最後capture09:46:44–09:47:31 UTC。30張pair圖的60原始screenshots全部親看，完整檔名JSON列明。所有10張表雙尺寸實看，手機table343px、wrapperclient=scroll375px，沒有左右切換才能對齊資訊的表。部分長英文API格內折行，對應角色/行仍可辨，未列無具體收益的格式nit。

必要八圖每張實際桌面/手機都看，servedhash與snapshot吻合；桌面390px原生寬，手機約342.98px，24px字約縮至21.1px，沒有跨多屏才能追整圖的放大。圖的標籤與箭頭均可讀：

- 11.2 boxed-chain 390×570／343×501：chainLink80→nextBox owned管理→heapLink120→nextBox→heapEnd三位置。箭頭不是shared借用，框與相鄰文字不冒充ABI。圖和下一個loop的80/120/End同材料。
- 11.4 deref-coercion 390×625／343×550：&WrapperString借外層→WrapperDeref TargetString→&String借value→StringDeref Targetstr→&str同UTF8文字，底部明不複文字。兩條箭頭與text_bytes(&wrapped)同一呼叫，手機整關係可見。
- 11.6 dyn-dispatch 390×690／343×607：values0/1各同型BoxdynCharge，分別下連資料與method表入口；Order80×1→80、Bundleprice120→120。兩種箭頭角色有文字標，正文說非move/clone與無精確ABI保證。
- 11.7 error-chain 390×650／343×572：ReadCountError abc→Some&cause→ParseCountErrorInteger→Some&error→ParseIntError原文→None。每層Display各自工作與借下一層原因兩用途同時可見，與abc實際報告吻合。
- 11.8 shared-tail 390×640／343×563：tail、firstnext、secondnext三箭頭落同Link120 strong3，該Linknext再指End；與first200/second160及逐個drop計數一致。strong-cycle390×490／343×431：A/B左右反向Rc箭頭、兩來源strong1清楚，題頭「假設」與底部無外handle仍保活明確，沒有暗示已有可執行建環碼。
- 11.9 guard-lifetime 390×595／343×523：建立view維持shared→最後println仍未Drop所以trymutErr→dropview解除才Ok，向下箭頭明是程式順序，非move或reference線。它精確處理普通ref最後使用與guardDrop不同的當下困難。
- 11.11 owned-downcast390×560／343×493：BoxdynAny內StringRust→請i32失敗Erroriginal仍Rust→請String成功OkBoxString；文字不變、owner交接不同於轉型資料內容。相鄰117先解成功借改/Boxmoveout，fullprogram照圖路徑追加教學。

其餘1/3/5/10無SVG；大小資料表、實際Drop輸出、lifetime型別表與單位API表已直接表所需關係，未見必補空間圖。代表長source位置親看含11.3 Pairowned欄位、11.6 finalhelper四test尾+完整ExtraFee、11.7 finalhelper四test尾+完整OrderError、11.9 alltests尾+RcRecorder答案+Weakparent完整流程尾、11.10 final三test尾+TimeBatch答案、11.11兩test尾+ExtraInfo答案。沒有details折疊，所有174完整include每viewport source比對都成立；長碼手機仍橫捲，不聲稱一屏能讀所有行。

## 限制與預露

本角色不是blindfirstreader。先前root授權ACREF02的11.11footer窄核，曾tail22行看到末尾完整payloads/main includes/commands/test摘要，實頁也露相鄰Any答案片段與章11nav；round08-crossref-followup-final已封存此限制。此次11.11正式初判仍從已讀1–11.10建立需要/操作，沒有倒算預露為已教。

本次盤點一次rg docs/chapters/11-*.md誤帶出舊async頁部分table行，未讀其body、未用其能力或結論；之後只明選canonical11-01-lesson至11-11-lesson。Root只提未來12.4修後版本提醒，未讀12body且不作11先備。最初capture只印output漏記running session，因此同一readonlycapture重跑一次；最後成功metadata和60原圖同source，sourcehash始末一致。

沒有執行Cargo/build/tests或第三方內部驗證；partialpreview不是finalstrictsite、完整導航或全書驗收。所有判斷限定上述source版本與兩實際尺寸。
