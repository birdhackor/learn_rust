# 第4章 4.1 首次閱讀紀錄 v1

日期：2026-10-14。AI 輔助讀者模擬，以基本 Python 語法讀者為起點。指定前文已另存 ch04-entry-reading.md；本紀錄只判斷實際正文提供了什麼，未執行程式。每個單位在開啟下一單位前保存；後文若補足，只追加，不改寫初判。

## 001-001.md｜4.1 開場（初判已保存）

SHA-256：`83ef7ffa4fbcce9d9b2f9e96dd8c78309da7993458ea46ae2e3c1f7dc3fe18a6`

引文：「兩者的資料欄位不同，但接下來的工作相同：**判斷這筆金額是否至少為 300**。」

1. 角色／前文：這是本節原問題。2.5 已有 Order 計算小計，現加入直接存固定價的組合商品；兩種資料需要同一門檻判斷。
2. 機制／原需分存：原需是讓門檻規則集中，免去各資料型別重寫。尚未提供 trait 的定義或實作機制，不能先以標題推論操作。
3. 使用／結果：目前材料是非負的小整數單價、數量與組合價，期待判斷是否至少300；具体函式輸入與結果型別仍未知。
4. 證據／界線：沒有圖；正文只支持欄位不同而後續判斷相同。數值約定限制範圍，本節沒有提供運費、缺值政策。新概念尚未用於規則或操作；等待如何取得共同金額。

## 001-002.md｜兩種資料與公開欄位（初判已保存）

SHA-256：`9a9d0d152b165fb33c0f99a5e42a811e44e556d6b58c5bc748b4f8a4bd2a3f1b`

引文：「`Bundle` 的金額直接取已保存的價格。」「欄位前的 `pub` 讓 library 外的呼叫者也能建立與讀取這些欄位。」

1. 角色／前文：Bundle 是新的 struct 實例形狀，price 已代表整組價格；Order 仍以兩欄相乘。struct 的定義／建立已見2.1，library／example 分工已見1.9。
2. 機制／原需分存：首次操作的公開型別與公開欄位分開：pub struct 公開型別，各欄pub再允許外部提供及讀取材料。公開的原需是呼叫程式須建立訂單／組合價，不是 trait 目前產生的新要求。
3. 使用／結果：外部可提供三個具體 i32 欄位建立兩種值；它們仍不同型別。共同判斷如何呼叫尚未給出。
4. 證據／界線：兩個 struct 定義支持欄位與金額角色各自保留，沒有圖；只知道共同行為不要求同欄位，仍不能由此說它們已具備共同能力。接回原問題：不同的金額來源要服務同一門檻。

## 001-003.md｜trait 與 method 契約（初判已保存）

SHA-256：`b6e4a595e778c474647bbb058d003d4a64e530b81d66dd811cea3a6360d79aa3`

引文：「`trait` 用來描述一個型別必須提供哪些能力。」「定義 `Charge` 並沒有替所有型別自動加上 `amount`。」

1. 角色／前文：首次得到 trait 的角色：說明型別須提供的操作契約；Charge 要求取得金額。它使用2.5已教的method、shared receiver與數值回傳。
2. 機制／原需分存：trait Charge 宣告契約；fn amount(&self)->i32;只寫簽名，以分號結束、暫無body，做法待各型別提供。原需仍是只讀資料取得金額，使門檻使用者不用知道欄位算法；不是要統一資料形狀。pub trait 將契約公開。
3. 使用／結果：實作型別上的&self將借用Order或Bundle，amount承諾交出i32，供原問題比較300；目前僅宣告，尚未知道實作與共同函式的寫法。
4. 證據／界線：單項method簽名支持所需材料與結果；無圖。明文限制宣告的效力，不可當成兩個struct已自动得到method。先前未知的「能力如何正式宣告」於此得到宣告部分，其餘保留。

## 001-004.md｜impl Trait for Type（初判已保存）

SHA-256：`32cc23ecb7d6df02bcd58f9e4c297fa65eea73683078b3efaeb6b20673ecac1a`

引文：「和前文 `impl Order` 相比，現在額外指出遵循哪項 trait 契約。」「這裡不在 `fn` 前再加 `pub`」。

1. 角色／前文：首次用impl Charge for Order將某型別接到某trait；與2.5只組織Order行為的impl Order相比，多了要符合的契約。Bundle另有自己的實作。
2. 機制／原需分存：各method簽名符合amount契約，再填入各自body；trait method依公開契約呼叫，fn不另加pub。原需是保留两種金額來源，同時讓使用者只須取得金額，不必知道欄位。
3. 使用／結果：兩種實作都借用self；Order以unit_price*count回傳i32，Bundle回傳price。兩者現已提供amount，可交出相同型別的金額；共同門檻函式仍未出現。
4. 證據／界線：無圖。兩個body支持「不同欄位／做法提供同角色結果」；相同簽名本身不能代替算法正確性的檢查（1.10已有此區分）。追加補足上一單位的實作未知，不改先前初判。

## 001-005.md｜bound 與共同判斷（初判已保存）

SHA-256：`df71a63fa5ab3a4534f54477f6a53f75f9d9ff2ded06f23a54448be8eef1ac52`

引文：「`T: Charge` 的意思是：這個 `T` 必須實作 `Charge`。」「中間方框是同一份泛型判斷規則，不是把兩筆資料合成一個值」。

1. 角色／前文：首次用trait bound限制3.3的型別參數T：編譯器檢查具體型別實作Charge，body才可依契約呼叫value.amount()。T仍代表型別，Charge是要求的能力。
2. 機制／原需分存：`<T: Charge>`聲明要求；amount各依Order／Bundle實作取得金額，再共用>=比較。原需是門檻規則只寫一處，兩份資料仍各自保留；未限定T不能假定這项操作（3.3）。
3. 使用／結果：meets_threshold需要&T與i32門檻，回傳bool；只讀借用保留原值。回傳可供後續條件判斷，具體呼叫仍待下一單位。程式用threshold參數，圖中的300是這次材料。
4. 圖／界線：已當場讀原SVG並view指定Chromium圖PNG。上方藍Order框：120×3、amount()、360；綠Bundle框：price=160、amount()、160。左右兩線各自向中央橙meets_threshold框，&Order／&Bundle標記借用輸入，T:Charge標能力要求；沿各線到下方同色true／false，分別是360>=300、160>=300。支持兩次呼叫共用規則；沒有資料合併或運費結果。單看共用框也不能推論兩次呼叫的執行順序。

圖SHA-256：SVG `b3411edef987179eedd841cad5bd7afda48f1f58deb55bf97dca21b79951f204`；指定PNG `195723860eaa20dedbef8ebd19db98fe6fe0cfcb4387cc0bdd012be6ff4ad923`。未看整頁。

## 001-006.md｜外部呼叫與移除bound（初判已保存）

SHA-256：`08ccc48fc7db170d09c8a264bf7b1f5f46e18fa657667476503d5947eae37ac8`

引文：「用點號呼叫 trait method 時，這項 trait 也要在目前的名稱範圍中。」「問題發生在編譯這份泛型定義時」。

1. 角色／前文：首次實際外部呼叫trait method，需要use Charge使契約在目前scope；use與crate路徑已見1.9，這裡新增的是method查找所需的trait。它不執行、下載或接手資料。
2. 機制／原需分存：具體order／bundle用點號amount；泛型函式靠T:Charge取得呼叫保證。外部的原需是顯示各金額、借用同規則判斷後繼續使用原訂單。
3. 使用／結果：呼叫meets_threshold(&order,300)／(&bundle,300)，按正文預期得到true／false；amount分別360／160，原order.count仍3。cargo選package及example沿1.3–1.4，未實際執行。
4. 證據／界線：無圖。無bound的fixture連main都沒有呼叫，仍預期在泛型定義報E0599；因此僅宣告Charge不等於任意T具備amount。這回應3.3未知T不能假定操作的限制，也補足前一單位尚未看到的具體呼叫。

## 001-007.md｜契約與計算規則的測試（初判已保存）

SHA-256：`847caacf65c7670708a9ab1052d59dfbe4f50c6c0b2f447c5eff51ab3451c617`

引文：「先對各型別交回的數字做 assertion，再驗它們能交給同一個門檻函式。」「trait 檢查簽名和能力」。

1. 角色／前文：沿用1.10的測試module、attribute與數值／bool assertion；本單位未新增操作概念，把已宣告與實作的能力接到需求驗證。
2. 機制／原需分存：先驗amount取得預期數字，再驗共用函式結果。原需是確保各資料算法正確，且「至少」包含相等邊界；不是僅讓trait簽名過關。
3. 使用／結果：第一項材料預期360達300、160不達；第二項兩種資料金額皆300，門檻300為true、301為false。這也確認門檻來自參數。正文要求cargo test，這輪只讀未跑。
4. 證據／界線：無圖。兩項測試支持兩份實作的這些金額與邊界；沒有直接比較struct，也沒有涵蓋每個實作者。正文明說商業規則仍需實作與測試確認，追加支持001-004的簽名／算法界線。

## 001-008.md｜新增型別的練習與小節連結（初判已保存）

SHA-256：`e5a575984f039c28f37744e7256eec46cfac2fea5d46d3ec8c07ab9285a68b8b`

引文：「為它實作 `Charge`，再使用既有 `meets_threshold`。」「確認沒有為這個型別再複製一次門檻比較。」

1. 角色／前文：DiscountedBundle是第三種資料，新欄位為原價與折扣；trait／impl／bound皆前段已教。本練習檢查能否把新的金額来源接到原規則。
2. 機制／原需分存：amount的body按此資料以原價減折扣，既有泛型函式仍比較回傳金額。原需仍是讓不同資料共享門檻規則，此處新增折扣資料，沒有新增門檻算法。
3. 使用／結果：約定非負小數且折扣不超原價；360−80=280，300門檻預期false、280門檻true。可依前段寫實作與assertion，但這輪未寫／執行練習，未開外連參考答案。
4. 證據／界線：無圖。兩個預期結果连回相等邊界；沒有涵蓋折扣大於原價政策。小節連起原問題：先宣告只讀取得金額的能力，各資料實作，再由bound保障共同函式可用amount，門檻比較只保留一處。標準比較、顯示、複製能力仍尚未學。

## 4.1 封存

4.1 的初判已完成並封存：snapshot `/workspace/work/tutorial-implementation/frozen/ch04-trait-prerequisite-v1`，reading-order 的8單位全部依序讀取並各在下一單位開啟前保存引文、hash與當時理解。001-005所遇圖已查閱原SVG及指定獨立PNG；沒有看整頁。後文若補充，另新增紀錄，不回改上述初判。

原問題已由正文接通：不同欄位來源 → 各型別amount實作 → T:Charge的共同判斷 → 分別回傳bool且保留原值。目前未見阻止理解本節操作的缺口；數值與兩次圖線界線均可在實際材料追蹤。這只表示正文理解，並非技術驗證或真實新手成效。

## 4.2 追加閱讀開始

4.1封存回報後，依root新指派才開啟snapshot `/workspace/work/tutorial-implementation/frozen/ch04-capabilities-prerequisite-v3` 的reading-order。前8單位由root指明是已讀4.1同內容，跳過、不重做初判；新讀4.2的002-001至002-007。4.3與4.4未讀。

## 002-001.md｜4.2 原需要（初判已保存）

SHA-256：`c04c6a7f435ca114f127c117d3cb7a74c123fe539aafb585eb6a4dba50c7927d`

引文：「把整筆資料印出來檢查、判斷兩筆是否相同，以及保留原資料再改另一份。」「各 example 中的 `OrderSummary` 是各自的型別」。

1. 角色／前文：4.1用自訂Charge表示計算能力，現在介紹標準函式庫已定義的能力。Copy先前見2.2，其餘目前只有名稱，尚未讀到規則。
2. 機制／原需分存：原需是開發時看整筆、比較兩筆、保留原資料修改另一份；標準traits為Debug／PartialEq／Clone／Copy，尚未提供宣告或欄位處理的機制。
3. 使用／結果：讀者需依資料需要選能力，將有四個獨立example；同名OrderSummary不能當成單一型別逐步累積全部能力。具体輸入、語法及結果仍待正文。
4. 證據／界線：無圖／程式。這個開場支持三種原需求及分開範例的閱讀界線，還不能只由名稱推論顯示格式、比較條件或複製成本。

## 002-002.md｜Debug、格式與derive（初判已保存）

SHA-256：`d6903812ffa25c37c463c718ab58fae4df128365f2265eff1600930602e12c3d`

引文：「`{summary:?}` 要求以 `Debug` 印出 `summary`。」「每個欄位型別也必須有 `Debug`」。

1. 角色／前文：Debug首次用於整筆開發檢查，提供型別名、欄位名與值的文字表示；原來1.4、2.1只印各欄位，現在原需是一次檢查整筆。
2. 機制分存：Debug是trait契約。`:?`首次成為println格式要求，原有{}／{summary}不自動採用它。`#[derive(Debug)]`首次要求編譯器生成實作；attribute外框已見1.10，derive括號列traits。生成逐欄使用Debug，所有欄位型別都須具備；兩個i32符合，卻不使struct自動具備。
3. 使用／結果：attribute附在struct前，再以{summary:?}格式印amount360、count3，正文輸出為OrderSummary與兩欄名稱／值；之後仍讀欄位assert。可用於一起查看資料，未執行example。
4. 證據／界線：無圖。移除attribute的fixture保留同欄位及格式，預期E0277；支持「欄位可印」不等於整體有Debug。正文限制格式可能改變，因此不作固定存檔／對外資料契約；這補足上一單位未知的顯示方式與用途界線。

## 002-003.md｜PartialEq與整體assertion（初判已保存）

SHA-256：`9ac20c072d0b5fa350b374e8e2994d31fbba719fc522e67d82ab7f53baccf368`

引文：「全部欄位都相等才算相等。」「需要 `PartialEq` 來判斷，並需要 `Debug` 在失敗時顯示左右兩個值」。

1. 角色／原需：首次用PartialEq讓自訂整筆資料可==／!=，結果是bool；整數已有此能力（1.6）。原需明確選擇比較完整摘要，所以同金額、不同數量也須不同。
2. 機制分存：derive(PartialEq)逐欄比較，所有欄位須有同能力；enum先同variant再比該variant全部資料。derive(Debug,PartialEq)首次用逗號列多項生成。整體assert_eq!首次同時需要PartialEq判定與Debug失敗表示；已教assert外框不重述。
3. 使用／結果：比較借用兩邊，不move，之後可再比較／assert。摘要360/3對相同欄位為true、對360/4為false；Shipment的Waiting對Ready為false，Ready3對Ready4亦false，可作完整資料驗證。
4. 證據／界線：無圖。固定amount只改count使第二欄作用可見；enum材料分別改形態及內容，支持兩階段比較。derive依結構生成，沒有推導業務的「相同」；若需忽略欄位，本例逐欄契約不適用。這接回開場的相同性原需，尚未涉及複製。

## 002-004.md｜Clone及欄位／元素的行為（初判已保存）

SHA-256：`0ca8c2572a412589b58162b6434cd96a2d8ee45f645e1a0d4829b1350fc09fc6`

引文：「我們現在要保留原標籤，另外修改一份副本」。 「`Clone` 不保證每種型別都複製它所指向的全部資料」。

1. 角色／前文：首次使用Clone的原需是保留含String的原摘要，再修改另一份；依2.2、3.4原本指派會move。新補入String的Debug／PartialEq／Clone，以及&str的Debug／PartialEq；文字比較按內容，供此處assert使用。
2. 機制逐項：clone借用原值，交回同型別新值，不消耗原值。String的clone複製文字內容並有自己的配置。derive(Clone)逐欄clone後組成新struct，欄位都須Clone。Clone已預先在scope，與4.1的Charge需use相容。shared reference的clone只複製借用資訊；Vec<T>的clone另建Vec並逐元素clone，要求T:Clone。
3. 使用／結果：original.clone()先得相同內容，再只對duplicate.label追加「 副本」；原標籤仍Rust、金額仍360、兩摘要不相等。Vec<&str>的views.clone()得到另一份長度1清單，其元素仍借original.label；取索引0的&str依3.4可Copy借用資訊。結果可各自管理清單，不能把元素當成owned String。
4. 證據／界線：無圖。先相等、改副本後不相等而金額相等，支持本次只改獨立副本文字的原需。兩份清單都印Rust本身只支持內容相同；共同來源關係由正文對Vec及reference契約說明，不能靠相同輸出反推。新值包含什麼，須依型別／欄位／元素分別判斷；未執行。

## 002-005.md｜自訂struct的Copy條件（初判已保存）

SHA-256：`6c0ed09bebf82ef653aa709e0c92d80d7f2027536601771e4949944076ec0ef6`

引文：「所有欄位都必須是 `Copy`」。 「具有 `Copy` 的型別也必須具有 `Clone`」。

1. 角色／原需：Copy的整數行為已見2.2；此處首次為自訂摘要生成能力。需要回到兩個i32欄位，保留原摘要並在另一份增加數量。
2. 機制逐項：Copy在原會move的指定中隱含複製值本身資訊；let second=first不呼叫clone。derive(Copy)要求全部欄位Copy；型別還須Clone，因此同列生成。Clone不反向保證Copy，文字摘要就是材料。
3. 使用／結果：生成四項能力的這個獨立OrderSummary可指定後保留first；只對mutable second.count加一，預期first360/3、second360/4。保留下來的兩份數值供分別使用／修改，未執行。
4. 證據／界線：無圖。原3與副本4支持本次獨立整數欄位；String會持有heap配置，不能只複製持有資訊讓兩份都當owner。帶String的derive(Clone,Copy)fixture預期E0204，即使沒呼叫也不符合欄位條件。界線依型別契約，不由stack／heap推斷；接回開場的保留原值需求。

## 002-006.md｜四能力接回原工作（初判已保存）

SHA-256：`4ecb31865a4ee9d43a9a5cef3a4b09e34ae56b847f23f6215c5d473346b3f6d9`

引文：「兩份標籤文字相同，但整筆摘要不相等」。 「是否複製底層資料依型別而定」。

1. 角色／前文：本練習仍用含String的LabelledSummary，整合前段Clone、文字PartialEq、整體比較與Debug；沒有新增能力或操作概念。
2. 機制／原需分存：原工作先明確要求保留280原金額，只在另一份加20；按已讀契約選clone，不能用會move的普通指定。各能力表分別整理印出、比較、明確複製與隱含複製，不能以其中一項替代所有需要。
3. 使用／結果：原值「訂單A」/280，副本同標籤/300；可依前段用clone及+=，以文字比較驗標籤相等、以整體比較驗不相等，再:?查看兩筆。這是閱讀上可形成的操作路線，未寫／跑練習，未開答案。
4. 證據／界線：無圖。相同標籤而不同金額連回逐欄比較，表格保留Clone依型別的界線以及Copy須Clone的單向關係。它彙整已教機制，沒有新增底層副本保證。小節原需「看整筆／判相同／保留再改」已逐項連上；文字鍵查詢仍未學。

## 002-007.md｜延伸資源（初判已保存）

SHA-256：`6f2cedef0fca55c5eb90a3d14f81c94f79ed705f286709d846ffc515f51021ad`

引文：「Rust Book：Derivable Traits」與「std：Clone、Copy」。

1. 角色：此單位僅列相關外連延伸資源。
2. 機制／原需：沒有新增正文規則或操作，不能拿未開啟的資源補足本輪判斷。
3. 使用／結果：提供日後可選讀來源；本輪遵指派只讀單位文字，不開外連。
4. 證據／界線：沒有圖、程式或新材料；本小節理解仍依前六單位，沒有援引外部答案。

## 4.2 封存與停止界線

4.2的7單位已依序各自保存首次理解、引文、SHA-256後再開下一單位，來源為上述ch04-capabilities-prerequisite-v3 snapshot。這7單位未遇圖片，未額外找圖／整頁／後文。4.1原初判未改寫。

正文已連起三項原需要：Debug以生成實作與:?提供整筆開發檢查；PartialEq逐欄／variant比較並與Debug合用於整體assert；Clone依欄位／元素契約保留原值並建立新值，Copy另受全部欄位及Clone條件限制，在指定動作隱含複製。String副本與Vec<&str>元素仍共同借用原文字的兩種材料，足以保留「新值」與「所指資料是否另有副本」的界線。

目前没有阻止理解這段正文操作的缺口。沒有宣稱真實新手效果、程式已執行或完整技術驗證；沒有查外連答案或研究資料。4.3未交，4.4雖據root已完成亦不開；至此停止，等待root交付下一段實際正文。

## 4.3–4.5 追加閱讀開始

依root接續指派才開啟snapshot `/workspace/work/tutorial-implementation/frozen/ch04-complete-prerequisite-v5` 的reading-order-tail.txt，只讀新交付的23單位。root指出此前4.1–4.2的15單位未變，因此不重讀／改寫既有初判。沿用指定背景補讀與上述初判；不宣稱前1–3章已首讀驗收。本輪仍只判正文，未技術驗證或實跑。

## 003-001.md｜HashMap與原分組需要（初判已保存）

SHA-256：`61123c48aa02868267970347e8ae82405e4b957013bb8fd101a88327c871aa9e`

引文：「把結果保存成『名稱對應次數』，之後就可以拿名稱查詢」。

1. 角色／前文：首次得到HashMap<K,V>的角色：保存key與value對應。3.6的Vec按位置讀取；此處原需是統計重複名稱，後續依名稱查次数，不把位置當名稱身分。
2. 機制分存：K與V各是型別參數，同map的鍵各同型、值各同型；相等鍵只一筆值。名稱清單也可由既有迴圈掃描，原需不以HashMap產物反推。
3. 使用／結果：本例選String鍵、usize次數，map擁有名稱；Rust／Python／Rust預期成Rust2、Python1。怎麼判相等、建立／查詢／累加尚未知。
4. 證據／界線：表格支持本次名稱與計數關係，沒有圖／程式；未給出鍵查找規則、操作契約或輸出順序。局部資料形狀可懂，如何完成原工作待後段。

## 003-002.md｜Eq、Hash與collision（初判已保存）

SHA-256：`37478a4106e7a784ce6eaeeb690e2890dd7f6f2423379d031f70d9566d0323d7`

引文：「`Eq` 以 `PartialEq` 為基礎，另外承諾**每個值都與自己相等**。」「最後用相等比較確認是否為同一個鍵」。

1. 角色／原需：為名稱查找補上同鍵辨認需要。4.2已有PartialEq；此處補充左右交換、連鎖相等的一致性及!=相反。原需是同名稱查詢能找到同一筆次數。
2. 機制逐項：Eq新增自我相等承諾、沒有方法。Hash把鍵資訊交給計算流程，所得hash縮小尋找位置，再以相等確認。同條件下相等鍵須同hash；collision是不同鍵也可能同hash，因此仍須Eq確認。
3. 使用／結果：鍵須Eq+Hash且相互配合；String／str／i32／usize標準實作符合本節。現可理解為何選String鍵，不須自己算hash或選算法；建立查詢方法仍未給。
4. 證據／界線：無圖／操作材料。正文支持單向關係「相等→同hash」，不支持「同hash→相等」。已能解釋鍵契約如何服務查名稱原需；尚不能從契約直接完成統計。自訂鍵如何讓相等及Hash採同欄位待後段。

## 003-003.md｜map四操作、Borrow與兩個借用來源（初判已保存）

SHA-256：`f9b9512645c4f9b7724c941cad4f279995f24f56c3b9ea0d1a6c946c97e34445`

引文：「查詢文字只供查找，回傳的 `&usize` 是借用 **map 裡的值**」。另句原文：`單純「能取得一個 reference」還不夠滿足這個 Borrow 契約。`

1. 角色／原需：首次實用map建立／保存／查詢操作。原需是保存owned名称、查既有名稱而不用再擁有查詢文字；use std::collections::HashMap依1.9引入標準名稱，不需第三方依賴。owned_text沿3.4建立文字，非新概念。
2. 機制逐項：new需K/V線索而得空map；insert需mutable map並接手String鍵、Copy usize值，回傳Option<usize>舊值，相等鍵保留舊鍵／替換值，不自動相加。get需shared map及&str，回傳Option<&usize>；len是键值對數。Borrow<str>首次提供String的&str借用表示，borrow取得shared reference；相等與Hash須與owned表示一致。Rust／rust的大小寫假設說明有reference本身不足以保證鍵身分一致。
3. 使用／結果：初insert Rust1得None，第二次Rust2得Some(1)，再插Python1；get Rust得借用2、缺JavaScript得None、len2。query改成Rust課程後仍可讀found，因結果借map而非query；使用found期間仍守map借用限制。局部查詢方法可懂，整串名稱如何累加仍待下一段。
4. 圖／界線：原SVG已當場查閱：上query:String框擁有Rust；中上query.as_str()框以藍實線向上借query文字；綠虛線向下查counts框中的String Rust鍵；counts內另有usize2值框。下found:Option<&usize>／Some框以藍實線向上借值2，圖例實線借用／虛線查找。各線方向及來源支持query與found不同來源；查找線不是回傳借query。尚未看PNG，未判字級／渲染passed。正文不承諾插入順序，整體印出順序不能當固定答案；未執行。

圖SVG SHA-256：`aae0087e2a06d48d21083276f039b37267ff2c5859489efbb2ce609335f2dd37`。已向root索取同版獨立圖crop，若後到另追加觀察，不改初判。

### 003-003後到PNG的追加觀察

root提供同v5獨立手機crop `/workspace/work/tutorial-implementation/screenshots/ch04-map-query-v5-mobile.png`，已view；SHA-256：`53324ac4b3c9301192978e58c2f5a9e50e439d26d6129672e982189dafb1752a`。未看整頁。crop可辨query、query.as_str、counts鍵／值、found名稱及實線／虛線圖例，兩條藍借用箭頭分別向上指不同來源；綠虛線向下找String鍵。綠查找線穿過counts:HashMap標題所在區域，看名稱時線與字靠在一起；仍可從鍵框及圖例追到Rust，不阻止兩來源理解。這是可選的圖線避字問題，影響局部閱讀順暢，不改已封存的關係初判，未判整頁字級passed。

## 003-004.md｜查詢借用與修改順序（初判已保存）

SHA-256：`a28c59e972fb7c47c18c14f9d8d8f30b6823d457f400ee9a44dcce05d21c0bc1`

引文：「雖然新增的是另一個名稱，`get` 的回傳契約仍然借用這個 map」。

1. 角色／前文：將2.4最後使用與3.6collection借用限制接到HashMap；沒有新增操作概念。get結果含reference，所以found的最後讀取仍需原map有效。
2. 機制／原需分存：get shared借用持續到結果最後使用，insert需mutable，兩者不能重疊。若原工作是先報舊次數再增名稱，就把讀取排在insert前；不因鍵名不同跳過此契約。
3. 使用／結果：Rust2的found取得後，insert Python1再讀found，正文預期E0502；先if let讀次數再insert可讓借用依序結束／重建。這是正文操作判斷，未實跑。
4. 證據／界線：無新圖。fixture刻意新增不同名稱仍衝突，支持限制以map借用契約而非名字猜測。此處可懂如何修正順序，也知道修正須符合「先報再增」工作需要；未宣稱能任意調換所有任務順序。

## 003-005.md｜get_mut累加、owned結果及查詢比較（初判已保存）

SHA-256：`7fb59b1bbce8c64945ee399e5edf7c3fe1d7721e6fb799f46e2bd8da0667bcb0`

引文：「遇到同名標籤應該加一，而不是把舊次數換成 `1`。」 「只有缺鍵需要讓 map 擁有名稱時，才用 `(*label).clone()`」。

1. 角色／原需：原名稱統計現在接到完整方法：已有同名加一，缺名存1。get_mut首次需mutable map，回傳Option<&mut usize>；Some透過*count改原map值，None再插新鍵。
2. 機制逐項：&[String]迴圈每次&String，as_str查詢；缺鍵時*label指原值、clone借它而另建owned String，保留原清單。Some只用已借reference；None無後續值reference，查詢借用可結束再insert。首次整體Option比較按Some內值／None形態；shared數值reference比較所借數字，Some(&2)可比get結果。Option及數值reference有Debug供assert失敗表示。
3. 使用／結果：Rust→1、Python→1、Rust→2，回傳map鍵不借輸入；三個查詢預期2、1、未收錄，最後一項報表顯示0但不新增鍵。測試覆盖重複、空輸入、內層清單結束後仍查map；usize值依4.2可比較／顯示。主算法已接通原需要，不依insert覆寫冒充累加。
4. 證據／界線：無圖。逐步表支持何時clone與修改，scope測試意圖支持owned鍵可脫離輸入；Option比較的結果反映值與缺值，未提供缺失原因。未執行。

當時不確定（必要的局部新語法說明）：測試首次出現`let counts;`，後在內層寫`counts = ...`，沒有前置解說這種分開宣告／給初值的形式。我能由後續程式猜到要讓外層保存map，但不能以已教正文確定無mut而稍後指派的規則；不以既有Rust知識補足。這影響scope測試操作為何合法的理解，不影響已完整說明的get_mut／缺鍵clone統計方法。保留猜測，待實際後文或針對此點修訂。

## 003-006.md｜自訂複合鍵與生成Eq／Hash（初判已保存）

SHA-256：`6d46fe97adb160a909e895201ca1ce1dddf4dc5407575f22b8106c66256f1a75`

引文：「把 `PartialEq`、`Eq` 與 `Hash` 一起生成，就能讓相等判斷和 Hash 遵守同一份欄位結構」。

1. 角色／原需：分組需要同時辨認地區與類別，單一名稱不足；GroupKey以兩欄共同決定身分。PartialEq逐欄生成已見4.2。
2. 機制逐項：derived Eq首次要求所有欄位Eq以保證組合自我相等；derived Hash首次取全部欄位Hash資訊，欄位須Hash。與PartialEq一起生成使身分規則一致；自行忽略欄位須重新核對同鍵同hash。此次查詢借用鍵自己，get接收&GroupKey。
3. 使用／結果：map接手GroupKey(1,2)與3；查相同兩欄得Some(&3)，只改類別成(1,3)得None。少Hash的fixture預期insert報E0599，操作仍需鍵能力，未執行。
4. 證據／界線：無圖。固定地區、只改類別的材料直接顯示類別有作用；「兩欄都參與」還依據derive文字規則，這組對照沒有另改地區來驗它。可選的材料補強問題，影響驗證說法的範圍，不阻止複合鍵方法理解。原需可由鍵形狀、共同生成、借同型查詢連起。

## 003-007.md｜以每筆數量替換次數（初判已保存）

SHA-256：`16ec364a69ab8d51f02c167389ac6bd3707bd361363b9552c0c12caa2fad9026`

引文：「把每次加一改成加上這筆的 `quantity`」。

1. 角色／原需：新工作是按名稱合庫存，不再把每個標籤只計一次；StockItem以name與quantity保存這個不同規則的材料。struct／&slice／欄位借讀已見2章，無新操作概念。
2. 機制分存：沿本節get_mut分支；已有鍵累加item.quantity，缺鍵複製名稱並保存首次quantity（由庫存規則推得，不再初始化1）。直接insert每筆數量會覆寫舊量。
3. 使用／結果：借&[StockItem]保持原清單，回傳owned鍵map；Rust2+6=8、Python3、len2、原len3、JavaScript查得None。小數量約定排除加總超範圍；未写／跑练习或開答案。
4. 證據／界線：無圖。重複Rust材料使累加與覆寫差異可見，結果仍按名稱身分。小節已把原需、鍵契約、借用查找、缺鍵擁有名稱與累加分支連上；查詢結果來源的簽名描述仍待下節。

## 003-008.md｜延伸資源（初判已保存）

SHA-256：`2ed46856e564e09e47684af220c653f27ac785baddf64f0ba401f0fcd804d0ef`

引文：「std：HashMap、Borrow」。

1. 角色：列出HashMap與Borrow延伸來源。
2. 機制／原需：無新正文規則，不用外連補足局部疑問。
3. 使用／結果：本輪只讀列出的連結文字，未開資源或答案。
4. 證據／界線：無圖／材料。4.3初判範圍至此，名稱查詢／統計的理解與問題依上述實際正文保留。

## 004-001.md｜回傳借用的原需要（初判已保存）

SHA-256：`454fbb4b86372d4f392769f766b63e12e9e72c288855d031fd317cb5d1b1753f`

引文：「呼叫後的 reference 借自哪份資料，哪個 owner 必須繼續存活？」

1. 角色／前文：3.5借文字而交回owned長度，現在原需是交回文字借用，供呼叫者繼續使用而不用重建內容；問題由函式內擴到輸入／回傳關係。
2. 機制分存：2.4已教reference不延長來源存活，這個性質沿用；來源如何寫入簽名仍未教，不能由標題先當作有新語法可操作。
3. 使用／結果：材料仍是String／&str，選擇規則將用byte長度與比較（3.5已教）；預期留下借用，須知道哪個owner活到使用完。
4. 證據／界線：無圖／程式。正文支持回傳借用的必要問題，尚未支持多來源如何連結；局部開場可懂，但可用方法仍待後段，不能把byte長度當人眼字數。

## 004-002.md｜單來源與lifetime elision（初判已保存）

SHA-256：`5bdeaf38e791a8456b89e41ef4db190e1c53a59bf3552920846b5246137a80fa`

引文：「省掉的是符號，資料仍必須在回傳借用使用期間有效」。

1. 角色／原需：same收到&str再交回原借用，先以單來源實現回傳借用需要；只轉交shared借用資訊（Copy已見2.6），不接手String／複製內容。
2. 機制分存：首次lifetime elision讓只有一個輸入reference lifetime時，省略的輸出lifetime連到它。省略標註仍有來源關係，不消除資料有效性要求。ch04-lifetimes的source module、pub／use沿1.9。
3. 使用／結果：same(text.as_str())回view借原text；預期Rust、byte長度4，text仍存在所以函式結束後能讀view。可把借用交回呼叫者繼續檢查，未執行。
4. 證據／界線：無圖。簽名、body與外層owned text共同支持單來源；函式參數結束不等於外層String結束。此規則只在本單輸入条件下說明，尚不能用來選多個來源；方法需要與合法範圍可連回開場。

## 004-003.md｜多來源的原選擇規則與省略失敗（初判已保存）

SHA-256：`5d5036fd8ee4ead13b608ea5994fd386e6446828603ed3979f3b2efac26b384b`

引文：「這裡比較的是……byte 數量，和 reference 的 lifetime 長短是兩個不同問題」。

1. 角色／原需：新的原工作是回傳兩文字中byte較長者，同長選first；不是選存活較久者。依body可能回first或second，因此單來源規則不足。
2. 機制分存：兩個省略輸入lifetime被視為分開關係，不能決定省略輸出來源；呼叫依簽名檢查，不按每次body分支猜來源。首次將elision實用界線放在兩輸入fixture。
3. 使用／結果：first、second兩個&str與&str輸出尚缺連結，正文預期E0106；即使例中Rust／Python可算byte4／6，也不能當程式已選到Python。合法簽名寫法仍未知。
4. 證據／界線：無圖。if分支支持兩可能來源；編譯失敗支持介面須描述共同滿足的要求。局部算法可懂，但方法仍欠來源契約，保留而不以既有標註知識補足。

## 004-004.md｜lifetime annotation與兩來源圖（初判已保存）

SHA-256：`f4c0eced628092fc8bf720208eea1fe5eecbeef6a2bdbb06c775edaa642fcb53`

引文：「呼叫者需要把結果用到哪裡，兩個候選來源就都必須可借到那裡。」

1. 角色／原需：首次標註reference有效性關係，使兩可能來源的選字函式有可用介面；接回上一單位缺輸出來源的問題，算法仍只比byte。
2. 機制逐項：longer<'a>宣告lifetime參數，單引號a不是文字／秒數；位置類似3.3型別參數。&'a str使用它，資料至少滿足其範圍。兩輸入／輸出同'a描述共同可用要求，不要求原值同時建立／結束，也不延長短命資料。
3. 使用／結果：兩String在呼叫及結果使用時都存在；Rust4、Python6選Python，交換參數也選Python。回傳一個借用供顯示，未複製文字／執行程式。此單補足合法簽名，不能反改上一單的未知。
4. 圖／界線：已讀原SVG：上first藍框與中second藍框各標&'a str、來源滿足'a；first的線沿右側繞下，second線向下，各指同一綠輸出&'a str框。底標「箭頭=可能的借用來源」，綠框標共同要求、任一輸入。支持兩可能來源與一個輸出，不表示兩值同返或內容複製；圖未表示真實建立／結束時刻。尚未看PNG，不判渲染／字級passed。

圖SVG SHA-256：`b998606f6f12113e38fe18efab59cca38029be305512083ac608ecd2c172c83c`；已索取同版獨立crop，若後到另追加。

## 004-005.md｜維持兩來源功能與改成只回first（初判已保存）

SHA-256：`a333ea8de8ed9917cdd075ed5cb6cc22caac0328ac65daf00b72c20b2545ba95`

引文：「若保留『可能選任一來源』的功能，就在兩來源都存在的區域使用結果。」

1. 角色／原需：實用上一單annotation界線：仍需要選byte較長者時，兩來源都須活到結果使用；另改成只回first，是不同工作需要。
2. 機制逐項：outside文字即使確定較長，longer簽名仍許兩來源，inside在內block結束使外讀selected預期E0597。first_text只連first與输出'a，第二&str不約束它；參數的_首次用已教pattern忽略值、不命名。annotation或換名都不延長資料。
3. 使用／結果：保留longer就內層使用；只回first可在inside結束後外讀。兩行預期Rust tutorial，但契約理由不同；未執行，不把first_text當保留原需求的同一修正。
4. 證據／界線：無圖。短hi對長outside刻意排除「人能看出分支就可外讀」推論；同樣文字輸出也不足以證明同使用範圍，需看簽名及scope。局部合法性與方法選擇都可連到需要，正文已把改需求與保需求的操作分開。

## 004-006.md｜獨立保存需要owned結果（初判已保存）

SHA-256：`639e2cffe4e15444fd9afacdf80e6fb253ba5082862b2f5136ddf3d5460c0989`

引文：「若呼叫者要在來源結束後獨立保存文字，就回傳 owned String」。

1. 角色／原需：此處原需改為來源結束後仍獨立保存結果；owned回傳回應這個需要。2.4已有不能借內層已結束值，建構／move沿2–3章，無新操作概念。
2. 機制分存：new_text只寫任意'a仍回區域String的&str，正文預期E0515。copy_text另建String、push_str拷內容、尾端交ownership；名稱copy不表示String有Copy，annotation不替代來源存活。
3. 使用／結果：inside.as_str()供拷入新值，block結束後saved仍owned；預期Rust與len4。代價是保存另一份文字，可由呼叫者管理而不借原來源，未執行。
4. 證據／界線：無圖。新建body與來源scope結束後外讀共同支持獨立owned結果；不能僅靠Rust輸出判別借用／擁有。方法選擇直接連到來源是否須結束後保留，不由annotation數量或copy名稱反推能力。

## 004-007.md｜lifetime placeholder（初判已保存）

SHA-256：`0838061bd55e82137afc134d572f1cdf943b2e792cc6b8a81ae2aa65618c6e11`

引文：「它不是像 `'a` 一樣宣告的新名稱，也不表示任意存活時間」。

1. 角色／原需：首次用'_保留lifetime位置並讓Rust按省略規則推導；它是same單來源契約的另一寫法，本單沒有新增一定要placeholder的任務。
2. 機制分存：&'_ str不是新宣告的'a，不把各個_綁成任意共同範圍；單來源輸入／輸出仍依004-002連結。單引號形式也與上一單函式參數_忽略pattern分開。
3. 使用／結果：same_placeholder借入再交回同資料，正文命令預期顯示Rust；source有效性要求未變，未執行。
4. 證據／界線：無圖。簽名展示保留位置，不提供多來源解歧義方法；正文明說兩獨立來源仍須清楚契約。局部新符號可懂，但不能將它當解決longer原問題的捷徑。

## 004-008.md｜fallback需要與lifetime bound（初判已保存）

SHA-256：`893dd880023490b910a31743b8b789e641ce53d911137713b9a4ded6f7d3c6cf`

引文：「`'b` 至少涵蓋 `'a` 的要求」。 「允許同樣範圍，不是必須多活幾秒」。

1. 角色／原需：原工作先有可能缺少的已選片段，Some沿用它、None才用backup；不是重新依長度選。這個需要先于新bound明示。
2. 機制逐項：首次lifetime bound 'b:'a表示涵蓋關係，outlives也可相等；fallback<'a,'b:'a>宣告兩名與要求。preferred含&'a str、backup為&'b str、輸出&'a str，所以None分支須由'b供應'a；不延長backup生命。與4.1 T:Charge的能力bound分開理解。
3. 使用／結果：Some回原片段，None回backup；合法空文字仍Some，不觸發fallback（3.2／3.5已有有值與缺值之別）。正文命令預期Rust／backup，未執行。
4. 證據／界線：無圖。match兩來源與簽名共同支持涵蓋保證；缺bound的錯誤只依正文敘述，未開外連fixture。保證服務回傳所需範圍，未變更實際存活；局部語法與方法需要均可連回fallback規則。

## 004-009.md｜first_nonempty與小節連結（初判已保存）

SHA-256：`3a25cff40a863a53377fe3ef26a926b8b857b4e190508357210d31172dbbf791`

引文：「第一段 byte 長度為零才回傳第二段，否則回傳第一段」。

1. 角色／原需：新的明確選擇規則是first空才改second，與longer／fallback不同；兩者皆為可能來源。沿用已教兩輸入同'a契約，無新必要概念。
2. 機制分存：由選擇規則先辨兩來源，再寫lifetime參數；依first.len()==0分支回借用，不能以重建文字代替此借用任務。兩段都須滿足結果使用範圍。
3. 使用／結果：空／Rust→Rust，Rust／Python→Rust，皆空→second空、len0。可形成簽名與分支思路，但未寫／執行练习、測試或開答案。
4. 證據／界線：無圖。len及显示支持內容，皆空len0本身不辨來源；借用來源仍依規則與簽名。小節已連起原借回需要、單來源省略、多來源標註、使用範圍、owned需求及涵蓋bound。

可選的前文指涉問題：本單寫「文字結果作尚未介紹的比較」，但指定已讀4.2的002-004已明教String／&str的PartialEq比較文字內容。我按本練習要求用長度仍可完成，沒有算法／來源契約缺口；該說法讓已教能力的適用位置稍不確定，宜分清「這次只要求長度」與「尚未教文字比較」。

## 005-001.md｜保存借用的原需要（初判已保存）

SHA-256：`266ced4fdc92de40db07d9a97f01273808f3fc431bc0b3dec5c104d9e804e885`

引文：「需要保存的是『如何找到原內容』，沒有需要重建另一份文字」。

1. 角色／前文：從4.4函式交回reference，改要把借用文字及操作收在資料模型，供後續工作用視圖。
2. 機制／原需：此處先給保存借用的原需；reference找到原內容、不接手原資料的性質沿2.3與3.4。struct如何連來源尚未教。
3. 使用／結果：希望保存借用資訊供後續使用，預期仍須來源有效；具体欄位／操作未知。
4. 證據／界線：無圖／程式。開場可懂其需要，不能先推論模型會建立新文字或已延長owner生命。

### 004-004後到PNG的追加觀察（005-001保存後）

已view root交付的同v5獨立crop `/workspace/work/tutorial-implementation/screenshots/ch04-two-sources-v5-mobile.png`，SHA-256：`568417954216123912702272fca030c95d0c498cd7ad36cdf09229d0256f6a19`。first、second、輸出三個&'a str及各要求文字可辨；first的右側繞線與second向下線分別指一個綠輸出框，底部「可能的借用來源」標記可讀。此crop未見阻止關係追蹤的局部問題；仍只支持可能來源，不顯示實際scope時間或當次選哪支。未看整頁，未將此後到觀察回填先前初見。

## 005-002.md｜struct的lifetime參數與借用欄位（初判已保存）

SHA-256：`88219e9a4886a807243ea9f58de0395d925a28844c828aa040981439549041ae`

引文：「這個模型**持有 reference 值，但沒有取得被借文字的 ownership**」。

1. 角色／原需：TextView保存通往已有文字的視圖，接回不用重建內容的原需。首次把lifetime成為struct型別的一部分，位置類似3.3 struct型別參數。
2. 機制逐項：TextView<'a>宣告欄位用的lifetime；text:&'a str保存shared reference。模型持有reference值與原文字owner分開，仍須來源有效，不延長String生命。views module與pub型別／欄位讓外部建讀模型，沿1.9、4.1。
3. 使用／結果：需先有原String提供&str，模型可保存借用資訊供後續操作；外text是String owner，view.text是reference欄位，不能因同名認為同角色。建立實例及method仍待後段。
4. 圖／界線：已遇原位才讀SVG並view同v5 crop：上藍框view:TextView<'a>、view.text:&'a str，下綠框text:String仍owner、文字Rust tutorial；一條藍向下箭頭標「借用原內容」。支持欄位借原文字，沒有ownership進模型或第二份文字。crop名稱／標記可辨，未見局部理解問題；圖不表達source延長或實際存放布局，未看整頁。

圖SHA-256：SVG `b6c5e7c89ca18a4e22522f2ff94a7e21b9af1c725b7da94a63eb6c60b4d2b4d0`；PNG `6a642cad05ec000391fd6e35d7d62cde883665a1151bc97b2a99e14a0a07584c`。

## 005-003.md｜型別路徑placeholder與建立視圖（初判已保存）

SHA-256：`58b4fbbc321de85017d57d6c3138dbf31b6a618a2e80a48b9d9328f4359b577e`

引文：「`TextView` 裡的 reference 借自唯一輸入，所以 `'_` 依省略規則連到該來源」。

1. 角色／原需：要把建立視圖工作命名為函式；直建同樣可用。make_view回傳含借用的模型值，原保存視圖需要不變。
2. 機制逐項：首次將4.4的placeholder用於TextView<'_>型別路徑，唯一&str輸入連到模型內reference。body TextView{text}用2.5同名欄位簡寫；函式交出模型，不接手原String或重建內容。
3. 使用／結果：直建TextView{text:text.as_str()}及make_view同借外String，預期都Rust tutorial，built.text.len為13bytes。若借內層inside後帶模型外讀，正文預期E0597；需把來源留到外層或選owned結果，未執行。
4. 證據／界線：無新圖。兩種建立方式與失敗scope共同支持「模型內借用仍連來源」；有'a／複製模型不等於延長inside。局部路徑語法及方法需要可懂，TextView任意存活仍不成立；上一單模型建構未知於此補足。

## 005-004.md｜impl參數、receiver省略與原來源回傳（初判已保存）

SHA-256：`8d5a1826a98edec2198126851af50dbaa638d44a38cb337321c377bb78ca0870`

引文：「method 的 `&self` 是**對這個模型的另一次 shared borrow**」。 「两個文字 method 的 body 都讀 `self.text`，但簽名給呼叫者的契約不同」。

1. 角色／原需：為模型組織byte_len與讀文字操作，延續2.5method。若只在借模型期間用結果，與要在模型結束後仍借原文字，是兩種已明說的使用需要。
2. 機制逐項：impl<'a>宣告參數，TextView<'a>把它用於適用型別。receiver &self另借模型，可短於欄位'a。首次receiver省略規則將text(&self)->&str輸出接這次借模型；original(&self)->&'a str明接原欄位來源。byte_len回owned usize不交借用；self.text是Copy shared reference，可交它而不move整模型。
3. 使用／結果：外String存活、內TextView短存；text()在內讀，original()的借用可帶外讀，預期同Rust tutorial、13bytes。換成text()交外則正文預期view不夠久E0597，未執行。
4. 證據／界線：無新圖。相同body／相同內容不等於相同來源契約，對照簽名、內外使用及fixture支持兩借用角色。原String也仍必須有效，original未延長view或資料；不可一概固定&self lifetime為'a。局部語法與保留借用的選擇理由均已接通。

## 005-005.md｜兩欄來源分開與方法所需關係（初判已保存）

SHA-256：`951cf801053dbda0543f00f92ba00aa92de4db4cd8bedc2e3bba5ddd0b777b5f`

引文：「不必把無關來源也綁到每項結果」。

1. 角色／原需：TextPair同時保存兩文字，內層需讀兩份，後續只帶第一來源借用到外層；原需是讓各輸出只連相關來源，並非多名參數使文字延命。
2. 機制逐項：首次struct用'a／'b各接first／second，impl也宣告並使用兩名；first_source回&'a str、second_source回&'b str。用整模型仍需其欄位有效；獨立回傳來源借用後，不要求無關來源活到每個結果用完。byte_len回owned usize，相加沿前文。
3. 使用／結果：外first為Rust tutorial13bytes，內second為note4；內pair.byte_len預期17、second_source為note，pair／second結束後saved仍借first、len13。沒有重建文字或搬整模型到外層，未執行。
4. 證據／界線：無圖。內部同時讀兩欄、外部只讀first，支持分開命名的實際用途；不是單憑來源不同时結束就宣稱所有模型都須兩名。完整pair使用與單一已取出的借用使用分開，局部語法及方法需要都可連起。

## 005-006.md｜Label練習與原問題連結（初判已保存）

SHA-256：`af04f6368d081ef2f804944344c2ffad6d8df6df25c2709eb06e17ff1e87ea76`

引文：「先說明為什麼這段 reference 不依賴 Label 繼續存在」。

1. 角色／原需：Label以短暫模型整理借用，但需要後續在模型結束後仍讀原文字；沿005-004來源method，無新必要概念。
2. 機制分存：struct參數接text欄位'a，original明回&'a str；receiver只借Label，交回Copy shared reference後，結果繼續借來源。把String也放內層會破壞原條件，不是同需求修法。
3. 使用／結果：外String為Rust，內Label結束，外結果仍借原String，預期顯示Rust、len4。可由已教契約說明理由，未寫／實跑練習、測試或開答案。
4. 證據／界線：無新圖。內容與長度只核對結果，來源契約及scope才支持不依賴Label存活。小節已把保存借用需要、型別欄位lifetime、建構函式、receiver／source分離、兩欄輸出各連來源接回原工作；未讀下一章。

## 4.3–4.5與第4章指定路線封存

本輪完成ch04-complete-prerequisite-v5 tail的23單位：4.3八單位、4.4九單位、4.5六單位。每單位均先保存當時四題判斷、引文與SHA-256再開下一正文；既有4.1–4.2初判未用後文重寫。新原SVG三張均只在原位遇到後查閱；同版Chromium獨立crop三張均實際view，其中two-sources後到的觀察已另記時點。沒有看整頁、manifest／sources、作者notes／reviews、外連答案、研究資料或其他章；没有實跑／技術驗證。前1–3章仍只算指定背景補讀。

### 方法需要與局部理解分開的結果

- 4.3：名稱對應次數的原需要先成立；Eq／Hash與Borrow身分一致性說明查詢條件，get_mut及缺鍵clone則實際完成累加並保留原清單。複合鍵依完整欄位身分分組。主方法與用途已接通。
- 4.4：借回現有文字的原需要先成立；一來源省略、兩來源共同標註、明接first與owned結果分別對應使用條件。placeholder與lifetime bound的機制沒有被誤當成延長生命。主方法與何時需改契約／改結果表示已接通。
- 4.5：保存找到原內容的視圖需要先成立；struct持有reference與原String ownership分開，再把receiver借模型與欄位借來源分開。單欄／雙欄method的輸出契約連到是否須跨模型或另一來源結束。主方法與用途已接通。

### 尚存問題及實際影響

| 分類 | 原位 | 具體問題 | 實際影響 |
| --- | --- | --- | --- |
| necessary（局部新語法） | 003-005 scope測試 | 首次`let counts;`再於內層`counts = ...`，未解說分開宣告／給初值以及此處無mut的規則。 | 可猜測試要把map留在外層，但不能只依已教正文說明該操作為何合法；主get_mut統計方法仍可理解。後文未明補這個語法，疑問保留。 |
| optional（圖線避字） | 003-003 map-query實際crop | 綠查找線穿過counts:HashMap標題區。 | 局部看名稱時線字靠近；仍能沿鍵框／圖例辨查找及兩個借用來源，不阻止方法理解。 |
| optional（材料結論界線） | 003-006複合鍵 | 對照固定region只改category，但文字稱檢查两欄都參與。 | 實際材料直接顯示category作用；完整欄位規則另由derive解說支持，對照不能單獨當兩欄驗證。不阻止建鍵／查詢。 |
| optional（前文指涉） | 004-009練習 | 說文字比較尚未介紹，但4.2已明教String／&str PartialEq內容比較。 | 用指定len檢查仍可完成；對已教能力是否可用的說法稍不確定，不影響兩來源契約。 |

除此未見須以外部答案補足的必要新概念／方法需要缺口。這是AI輔助首次正文理解紀錄與其材料界線，不能當真實Python新手成效或完整技術正確性認證。初判已封存；後續修訂若交付，應另追加對該修訂的回讀觀察，不改上述首次判斷。
