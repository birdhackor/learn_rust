# 4.1–4.3 獨立技術初判

未發現必要技術缺陷、所選target閉包／首次使用先備缺口、輸出或預期失敗碼不一致，也未另提可選項。這是靜態技術整頁審閱，不是命令執行、盲讀或目標讀者／版面實驗。

## 範圍與獨立性

以基本Python與實際已讀第1–3章主文作先備，完整讀4.1–4.3、九個成功example、三個compile-fail source、manifest與workspace metadata。未讀4.4–4.7正文／後續source當先備；未讀章4作者ledger、validation／freeze結論、首讀、continuity／audit、他角審閱或歷史問題。root在期間提供的是章3 Copy後續任務，沒有用它提示本輪章4判斷。

所有來源hash於2026-10-15T07:00:41.505813+00:00保存；同名JSON含26個來源指紋、24個snippet引用、九條命令與靜態結果。未執行Cargo/rustc/Clippy/tests或fixtures，不借作者已跑過的結論作自己的證據。

## 主要概念四題與技術來源核對

| 單位／原文依據 | 身分與機制／性質 | 需要／用途 | 操作、例子及技術結論 |
| --- | --- | --- | --- |
| 4.1:3、7–25 enum／variant | enum列出可選形態，一次只是一個variant；struct則同時有各欄位。Missing無附帶值、Amount是一個i32、Rejected有命名code。 | 缺報價不能和真正報價0混在一起，拒絕也不應硬塞金額。 | enums.rs完整定義三形態與三個值；對應表、型別、payload形狀正確。Amount(0)仍有金額而非Missing，文字先說明表示差異，未要求現在就讀payload。 |
| 4.1:17、31、61 局部allow與未使用名稱 | #[allow(dead_code)]沿用2.7屬性形式，精確限enum的暫未讀欄位；_開頭仍是binding、不取消ownership。 | 先建立形態，下一節才讀資料，保留這個階段而不全域關閉檢查。 | 兩個建值程式只在enum加allow，註解理由相鄰；兩份source中的所有variant都已建立。allow和_名稱作用都先於完整source使用解釋；固定println明說不是依資料內容決定顯示。 |
| 4.1:65–77 Shipment答案 | Waiting無資料、Ready有count；正確形態才可帶該資料。 | 把等待狀態保留，不改存為假count0。 | enums_answer含完整enum/main與相同已解釋allow，Waiting與Ready3建立正確，沒有hidden helper／derive。 |
| 4.2:7–19 pattern／match／arm | pattern描述值或形狀，match邏輯上選第一個符合arm；=>左右角色、literal pattern與_ wildcard先說明；結果來自arm expression。 | 先以數字學選分支，再分辨報價形態才能用其附帶金額。 | match_numbers的0/1/_交20/10/0，tier1不落_；三個arm結果i32，作函式尾端值。match不要求被配對值是bool，與if區別正確。 |
| 4.2:45–55 enum payload binding | Amount(amount)在該arm建立新名稱；Rejected{code}是code:code簡寫；Missing無payload。 | 形態不同，要取得該情況真的存在的資料，再完成顯示。 | match_quotes完整Quote/show_quote/main全內嵌；四呼叫分別Missing、Amount0、Amount120、Rejected37。每arm println得unit，show_quote owned參數接手值；未偷用借用pattern。 |
| 4.2:74、96 Missing與統計0 | 原始enum仍保存Waiting／Ready區別；ready_count對Waiting回0是這次統計工作的輸出。 | 統計可出貨數需要每種狀態的數值貢獻，但不改原始資料表示。 | match_answer顯示0與3，兩項assertion正確。這不是撤回4.1「缺少與0不可混存」的理由，正文已明確區分。 |
| 4.2:78–92 涵蓋與結果型別 | match需exhaustive涵蓋其型別可能情況，且有結果的arms型別需相容；兩項各自成立。 | 未知Quote可能是Missing，不能只因某次預計傳Amount就省略處理；運費還需要數字結果。 | missing_variant完整例靜態E0004；arm_type有_卻i32/bool不合，E0308。正文要求先決定Missing工作，未建議任意補0而破壞原始語意。 |
| 4.3:7–19 tuple／array pattern | let先取得右值再依左pattern建立binding；tuple逐位置，_不建名；array[first,..,last]取首尾，..忽略剩餘且只一次。 | 只命名本次需要的部分，不把忽略一格誤當改變容器形狀。 | destructure的tuple120/3/true只取數字得360，array首尾120/40。..在pattern的角色與range expression／切片範圍分開，沒有偷用range pattern。 |
| 4.3:23–37 struct pattern | 欄位名對應，新binding可用unit_price:price另取名，count是同名簡寫；..忽略未列欄位。 | 改以具名資料拆出所需材料，順序不交換欄位角色。 | 80/2解構為price80/count2；另一筆只取unit_price200，沒有說原來的外層count名稱變得不可用。資料全是已教Copy數字；未使用String pattern的move／borrow規則，所以4.5可待教。 |
| 4.3:64–78 參數／for pattern | 參數名稱位置可換pattern，一個(i32,i32)在進函式時拆開；for每次取得一個tuple再解構。 | 把已知tuple的形狀直接接到計算函式與逐項工作，而非另開新容器。 | 由1.6的fn(name:T)、1.8的tuple型別及64行「參數位置可放pattern」即可組成subtotal((price,count):(i32,i32))；70行再釐清外括號／一個tuple參數與呼叫。pattern_places算360與360+160=520，不需要未教任意約定。 |
| 4.3:94–104 nested pattern | Request::Order持有已有Order型別，或Cancel；建構與解構都由外到內對應。 | 請求可能有訂單才計算，也可能取消而無Order可讀。 | nested_patterns含完整struct/enum/show_request/main；Request::Order(Order{...})算360，Cancel只顯示取消。Order variant與Order型別位置由外框可辨，解構不另建訂單。 |
| 4.3:108–118 irrefutable／refutable | 上述固定tuple／array／struct pattern對其具體型別每值可配對；Amount pattern可能遇Missing而失敗。普通let／參數／for無失敗分支，故需irrefutable。 | 不能無條件建立一個在某形態不存在的amount名稱。 | refutable_let完整例的show接受整個Quote，Amount pattern未涵蓋Missing，E0005正確。先用已教match安排缺值工作，4.6／4.7的新形式不作現在先備。 |
| 4.3:122–134 完整答案 | 仍用tuple pattern、參數tuple、array首尾與具名struct pattern。 | 換成80/2、四項價目表後檢查對應規則保持。 | destructure_answer含全部Order/subtotal/main；兩種小計160、80+60=140，三項數字assertion正確。沒有隱藏測試或外部來源。 |

## 所選命令與完整來源

所有九個公開成功命令都明選 `-p intro-patterns --example <本頁target> --locked`，manifest指向已教的入門workspace。patterns的完整member manifest只有name/version/edition；無src/lib.rs、build.rs、dependency或repo Cargo config。每個target各自一個完整.rs，內含所需型別、helper與main，沒有mod/include、Option/Result/Vec、泛型、derive或未解釋方法。

3份失敗程式也各自完整；E0004／E0308／E0005均由source靜態核對，不宣稱實際診斷已執行。這些頁面不要求跑整個package的all-targets，之後新加的未選範例不會變成當前Rust source閉包。

三頁沒有引用必要圖片；enum形態表、短pattern與具體結果足以追蹤當下資料對應，技術檢查未發現必須補圖的關係。沒有做browser、實際字體／折疊／手機可讀性或全站建置驗證；本輪不外推為實際初學讀者已順暢或後續章節已驗收。
