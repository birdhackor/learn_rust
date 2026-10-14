# 第 2 章：獨立技術初判 v4

日期：2026-10-14。審閱者：`chapter10_concurrency`。角色：clear-tutorial 第二輪技術與證據，未參與第 2 章作者工作。本輪僅審閱、不修改教材。

## 凍結範圍、先備與獨立性

- 凍結稿：`/workspace/work/tutorial-implementation/frozen/ch02-complete-source-v4/`，六頁、31 個正文單位，按 reading-order 讀完。
- manifest SHA-256：`9a20a66ce1cd36f9c61e533d1621a43f6e0808fd1e67b639db8218f04297e263`；reading-order SHA-256：`6a888df89ea53238eec0847c5025738c46c7aa00203fc3cf6802ec2dc00029d2`。
- 必要前文是已實讀的 `ch01-complete-source-v8`：數值與型別、變數／mut／shadowing、函式／參數／回傳／unit／block／scope、tuple／array／range／索引、for、數值顯示與 assertion、Cargo 的 package／example／locked 選擇。未以課綱標題取代真正的前文能力。
- 本輪重新讀 review-protocol 與 Rust 術語指引；未讀第 2 章首讀者答案、作者帳冊、來源筆記、其他 review、root 問題清單或修後稿。
- 本角色此前有第 1 章技術與交叉審閱歷史、曾研究第 10 章規劃；這不是無技術背景的首讀盲測。既有專家知識只用來查核，不當成學生先備。
- 實讀 archive 的正常 source、六份完整答案、六份 compile-fail、單一 runtime-fail、manifest、預期標記及四份 SVG 來源。31 個 chunk、34 份 archive source 的 hash 全與 manifest 相符。指紋明細另存 `/workspace/work/tutorial-implementation/technical-ch02-v4/fingerprints.json`。
- 沒有擴至第 3 章，沒有寫第 10 章。沒有重跑已通過全套來代替教學判斷；本輪未新增執行實驗，數字與 diagnostics 的判斷是 source／官方契約核對，不冒稱自己的完整執行驗收。

## 初判結果

**本輪未核實到必要修正。** 重要新操作在首次正常 code／失敗 fixture／答案前都有足夠的當下契約，主要數值與使用限制相符。這只表示下面列明的來源與技術支持範圍，不保證獨立首讀者沒有其他理解缺口。

有一項可選改善 I1：empty mutable slice 的「每項資料不變」可以用更精確的自動檢查回收。原程式正確，這不是缺少 API、不能編譯或無法理解的必要問題。

## 可選改善 I1：空段不變的 assertion 只回收了總和

- 正文位置／原句：`006-005.md:15`：「再把空段 `2..2` 傳入同一函式，**資料應完全不變**。」此前同段已要求五個數值 assertion，分清借用內外各位置。
- 完整答案位置：archive `examples/ch02-ownership/examples/slices_solution.rs:17–23`，第一次 increase 後有五項值與總和的檢查；`:25–26` 的空段呼叫之後，只有 `assert_eq!(sum_values(&prices[..]), 520);`。`:27–31` 仍顯示五項實際值，讀者可以另外目視對照。
- 已教／已足夠部分：mutable slice for 每次得到 `&mut i32`，空 range 沒有元素，因此正確 source 的空段呼叫不執行 body。正文的 range／for 規則已足以推出本次資料不變；現行答案的實作沒有錯，也不需先新增測試才能看懂。
- 可改善的關係：自動檢查的「總和沒變」弱於正文的「每項資料完全不變」。例如 130 與 210 變為 140 與 200 時，總和仍相同；不應把單一總和判準誤當每個位置值不變的完整檢查。
- 實際收益：讀者自己練習時，若空段處理誤動某些位置但保留總和，逐項檢查可指出這個問題。這是驗證空段 identity 行為，並非重寫演算法或鏡像測實作步驟。
- 最小建議：在空段呼叫後再以已教的五個 i32 assertion 檢查 80、130、210、60、40，保留前一次修改後的檢查，讓兩個動作各有可觀察判準。避免直接引入尚未教的整個 array 比較能力、Clone 或新測試 helper。
- 分級：**可選／局部驗證判準**；原 source＋文字規則已完成操作與因果理解，未升必要。不要求這時教所有測試方法或新增大量案例。
- 發現者／時點：本角色互看前對照正文與完整答案獨立提出；未看其他發現。未修改，修後複查未做。

## 依實際正文建立的第一次使用與支持範圍

以下「通過」只限眼前具體資料、介面與閱讀路線。未知的內部 trait 設計不作隱藏先備，沒有把後續課綱位置當作已教證據。

| 首次需要的能力／位置 | 先備與 code 前的最低解釋 | 技術、用意與結果支持範圍 |
| --- | --- | --- |
| struct／欄位／實例：`001-001–002` | 前章已有 tuple 的位置角色與 i32。先指出 order.0 需記位置，再定義 struct、Order 的欄位型別、實例與欄位點號。 | 改變的是資料角色的表示，不說 tuple 不能算。型別定義不等於具體資料；欄位名決定實例對應，不依列出順序。 |
| 修改欄位與保留原計算：`001-003–004` | source 前說 `let mut order`、欄位 +=、mut 不放 struct 定義，並沿用前章「保存結果不會自動重算」。 | 120×3=360，count增4後480；練習80×2=160、增3後240。main 與答案只顯示／比較 i32，未加入整個 Order 的 Debug／derive。 |
| stack／heap／pointer 模型：`002-002` | 函式／參數／block／scope 已教；本段先給呼叫工作區域、獨立配置區塊、配置器／歸還與位置資訊。 | 僅用來建立角色；明說 Order 沒有額外 heap 資料、編譯器可用處理器暫存空間，不當作實測布局。沒有 Box 程式或物理地址主張。 |
| ownership／owner／move：`002-003` | Order 的完整值與 scope 已教；先教誰管理／轉交／清理，再給 `let second=first` 的行為。 | 此 Order 沒有 Copy，完整指派／參數傳入會 move；原 binding 不能再讀同筆值。清理責任不用另存 owner 名稱欄位；本例沒有聲稱發生額外 heap 釋放。 |
| Copy 與欄位讀取：`002-004` | 先說 i32 等 primitive 可 Copy、獨立值保留、自訂 struct 不因全是數字就自動 Copy；圖在此最低解釋後。 | 這是當下能力辨識，不要求先會實作 trait。[Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html) 支持區別；整數 count 複製後改另一個值不改原值。 |
| 函式接手／move 後失效：`002-005` | code 前明確說 subtotal 接手完整 Order、只回傳小計；fixture 前說在 move 後再讀。 | first→second→參數，回傳360；primitive兩個count為3與4。fixture後讀first符合E0382。未把move的語意箭頭當實際bytes搬動或heap專屬操作。 |
| mutable 參數／owned 回傳：`002-006` | 前章已有參數與回傳；本段先說 add_one 的參數需mut、接手修改再回傳完整Order。 | ownership_solution 的 add_one(mut order:Order)->Order 合法，數量3／小計240；不把原函式的 stack frame 說成永久存活。 |
| reference／borrowing／dereference：`003-001–002` | 原需要是只讀算小計又保留原訂單。先定義reference、&的expression／型別角色、*跟隨資料、i32讀取會Copy與shared限制。 | reference借用不接手referent；*在此是reference操作，不靠未教Box／raw pointer使用。shared對本章普通數值不能寫，限定沒抹掉後續interior mutability的可能性。 |
| 借用參數、欄位自動跟隨：`003-003` | code前先教 `order:&Order`／`&order`，再明示 `(*order).count` 可寫為 `order.count`。 | 兩種具體存取形式已在此教，未暗靠學生懂Deref trait。讀到i32欄位不會使Order整體Copy；小計360後原count3仍可讀。官方[Field access](https://doc.rust-lang.org/reference/expressions/field-expr.html)支持此操作。 |
| mutable reference與指向資料：`003-004:5–44` | source前先教 &mut、原變數mut、獨占／衝突存取、修改的是所指資料而非reference重指位置。 | 訂單count變4而unit_price仍120；reference binding沒有mut亦可修改所指i32。普通數值契約與[Pointer types](https://doc.rust-lang.org/reference/types/pointer.html)相符。 |
| mut binding與shared寫入失敗：`003-004:46–82` | 第二份source前分開 `let mut reference:&i32` 重新指向與 &mut型別的權限。fixture前已說shared不能寫。 | 重指後讀3／8、原數字不變；*shared +=1符合E0594。不是只在名稱前加mut就可修改shared referent。 |
| 讀／改分工練習：`003-005` | 函式型別、&Order、&mutOrder、欄位重新指派均已教，題目先固定操作需求與材料。 | borrow_solution 使用set_count與subtotal，得160／240且保留同一Order；沒有derive、完整Clone或提前Generic helper。 |
| lifetime與最後使用：`004-001–002` | source前定義有效程式範圍而非秒數／名稱scope；把first-last模型限定在無分支簡單程式。 | shared讀3／3→mutable讀4→owner改5，已說每份借用何時最後使用。不是以原始碼含幾個&mut計衝突；[官方Book borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)支持這種先用完再修改。 |
| 衝突兩類fixture：`004-002:42–73` | 在code前分別說後續讀取會使區間涵蓋owner修改、兩mutable references都後續使用會重疊。 | shared後改再讀對應E0506；兩個mut還使用對應E0499。不是所有owner永遠不能再用或任何兩次&mut都錯。 |
| reborrow／自動短借用／reference move：`004-003` | source前先教 &mut*parent 與子借用限制，再教明確&mut參數的呼叫可短暫重新借用。 | child加到4、parent到5、兩呼叫到6及7，修改同一count。未把所有指派說成自動reborrow；未註記的let next=parent與mut reference非Copy的區別明說。 |
| referent存活與數字回傳：`004-004–005` | block、scope、Copy、reference非owner已教；fixture前先提出內層資料結束後不能從外層借用。 | named local inside的reference逃出符合E0597；答案改交owned i32保存8。判斷限本例局部變數，不需先教lifetime annotation、temporary promotion或完整靜態分析。 |
| impl／Self／self／receiver：`005-001–003` | 前章array.len的method使用、函式與本章借用已教；source前介紹impl、型別Self／參數self、receiver的三種需要與普通參數。 | 具體Order的&self讀、&mutself改、self接手，點號語法不避開ownership。與[Associated items](https://doc.rust-lang.org/reference/items/associated-items.html)相符；不要求學生先會trait impl。 |
| associated function／new／欄位簡寫：`005-004` | 完整source前教無既有實例的工作、new不是關鍵字、Order::new與Self欄位簡寫的兩個角色。 | 產生80／2→讀160→加一→讀240→接手tuple80／3，所有assert僅i32。沒有把into名稱本身當ownership保證；依self簽名判斷。 |
| 不能從shared借用搬出Order：`005-005` | fixture前明說i32欄位可Copy，但完整非Copy Order不能從shared位置move。 | *self造成E0507是能力／借用契約衝突，不是所有dereference錯。若工作接手才選self，若借用就交可讀數值結果，沒有盲加Clone修法。 |
| reset_count練習：`005-006` | receiver選取／欄位寫入／assertion已教；題目先給「不結束訂單、不改單價」需求。 | methods_solution 用&mutself；count0、unit_price80、小計0逐項測，能避免只見小計0卻把單價錯改0。 |
| slice位置／長度／相對索引：`006-001–002` | 先給原固定三項接口的限制與同加總規則；source前教&[i32]、位置＋長度、不複製／接手、range、相對索引與slice.len。 | 1..4借120／200／60，新索引0／1／2，長度3與array5分清。slice這裡是借用視圖，不叫它另一份array。 |
| 不同range／shared Copy／for item：`006-003` | code前教[..]、省一端、空段、shared ref Copy只複製借用資訊、forvalues取得&i32再*讀數字。 | 同一sum_values得到380／500／0／200，實際使用不同長度且空段body不執行。與[slice](https://doc.rust-lang.org/std/primitive.slice.html)的具體契約／IntoIterator item核對相符；學生程式沒有未教trait簽名。 |
| slice自己的邊界／borrow存取期：`006-004` | fixture前說start≤end、原邊界、slice自身len；後面接回先前最後使用規則。 | selected長度3取索引3即越界，不能因底層array有第五項就放行。只使用直接索引與panic，不先用未教Option／get。 |
| mutable slice／逐項修改與完整答案：`006-005` | 答案前先教&mut[i32]、原arraymut、for取得&muti32、*value+=amount及修改原元素。 | 1..3加10只得到80／130／210／60／40，和520；empty2..2不跑body。接口改的是可借範圍而不是新加總演算法。局部自動檢查可選改善另列I1。 |

小節的需要與例子用途各自成立：struct改善角色辨認；borrow讓原資料保留；method整理行為；slice讓同一函式處理不同長度／部分資料。沒有虛構tuple、普通函式或固定array原本不能完成數學計算，也沒有以某一個成功輸出承諾任意i32輸入／性能結論。

## 完整答案、正常source、fixtures與manifest

六份完整答案在相應練習連結之前都只用已教材料：

| 答案 | 當時已教的新增能力 | 有限結果 |
| --- | --- | --- |
| structs_solution | struct／欄位與mut修改；先前數字assert | 160→240；不derive整個Order。 |
| ownership_solution | 完整值傳入／owned回傳、mut參數 | updatedcount3，小計240；不再使用交出的order。 |
| borrow_solution | shared／mutable參數、欄位重新指派 | before160／after240，同筆order保留。 |
| lifetime_solution | last-use順序、block回ownedCopy數字 | 借用先讀3、owner後改4、外層保存8；不回傳失效reference。 |
| methods_solution | impl／Self／associatednew／&mutself | 原240，reset後count0、price80、subtotal0；不用新測試框架。 |
| slices_solution | shared／mutable slice具體for、*讀／改 | 兩項增加、其餘不變，和520；empty不修改。I1限額外自動檢查收益。 |

六個compile-fail的原始碼與預期marker分別是use_after_move/E0382、shared_write/E0594、borrow_overlap/E0506、double_mutable/E0499、dangling_block/E0597、move_from_borrow/E0507。單一runtime-fail以slice長度3、position3對應index out of bounds。各失敗前均已教其原因與階段，沒有拿失敗程式混作正常Cargo example。

所有已提供run命令沿用prefix的-p／--example／--locked；指定「自己的練習main.rs」的fixture操作可以沿用獨立練習的cargo check，沒有無提示切到根workspace全體測試。新package manifest只沿用第1.3已教的共同欄位繼承，沒有dependencies、新build script或其他設定能力。

## 四張原位圖的技術關係

本輪讀的是凍結SVG原始碼與正文原位置，不是實際圖片／頁面觀看。以下是內容對應查核，不是視覺驗收。

| 圖與出現位置 | 必要對象／連接與正文對應 | 能支持什麼／不能支持什麼 |
| --- | --- | --- |
| move-copy／`002-004:11` | 上半first已交出→second現在持有120／3；下半count3→another_count3到4，分別標轉交／複製。正文先說兩類能力與箭頭只表語意。 | 能辨使用權與獨立數值；不能由此推實際地址、搬動bytes、stack／heap布局。 |
| reference-owner／`003-002:15` | 上框reference:&Order，借用箭頭指向下框order owner的120／3；底部*reference找到同一Order。 | 原order與借用是不同角色，沒有第二個Order；找到值不等於取得owner或可move完整非Copy值。 |
| borrow-scope／`004-002:40` | 四個觀察點：建兩borrow、最後讀、直接改count、main結尾；藍線只到讀完、灰線到結尾，標first／second的名稱scope。 | 顯示兩種範圍不等長，支援最後讀完後的修改；不是聲稱完整程式只有四項動作，也沒有把writable的使用區間混成first／second的線。 |
| slice-part／`006-002:11` | 原索引0..4列80／120／200／60／40；框只包120／200／60，右欄新索引0..2；文字selected=&prices[1..4]與長度3。 | 身分由同行值／框／兩索引明確對應；沒有另建array。原索引4與slice上界4不同對象，圖沒有宣稱slice可索引4。 |

圖的原位置都有當下角色說明，沒有靠後圖或外鏈補reference／Copy／slice的必要意義；但source清楚不等於手機字級、文字遮擋、箭頭可辨或整頁閱讀順暢已通過。

## 未做與結論邊界

本輪只保存互看前初判：0項核實必要修正、1項可選I1，並列明主要通過項的真實支持範圍。全6頁31單位及34份archive已查source與指紋；不因沒有必要項就宣稱所有真實讀者一定好懂。

沒有自行執行正常程式、compiler fixtures或runtime fixture；沒有親看四張渲染圖或桌面／手機頁面、沒有做真人讀者測試、沒有重做全站建置或跨到第3章。執行／視覺證據需要由實際完成該檢查的角色另存。若後續互看發現新增必要關係，另記提示來源與時點，不覆寫這份獨立初判。
