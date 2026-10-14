# 第 5 章技術審閱：ch05-visual-fixed-v3 初判

審閱者：`/root/chapter04_traits`。日期：2026-10-14。這是獨立於第 5 章作者的技術初判，不是新的無背景盲讀。我曾撰寫 4.1–4.3，也為寫作與技術審閱實讀過第 1–3 章；本輪按指定版本回讀需要的前文。第 5 章正文先依 reading-order 讀完，再補讀完整來源與答案，最後才查正式 API 與做局部實驗。

初判封存前沒有閱讀第 5 章 first-reader、disposition、作者 ledger／source-notes、其他技術／銜接清單或 live 修稿。root 提供的「5.6 圖已有原位 QA」只作證據界線背景，沒有拿其結果代替本角色看圖或閱讀。以下發現均來自本輪凍結正文／來源的自行核對。

## 固定範圍與指紋

- 唯一被審稿：`/workspace/work/tutorial-implementation/frozen/ch05-visual-fixed-v3`。全六頁、35 units、40 archived sources，含全部完整答案、library、normal examples、三個 compile-fail／expected、三個 runtime-fail／expected、package manifest 與一 SVG。全部 chunk／source bytes 已逐項核對 manifest hash。
- manifest SHA-256：`6f8ea0ff68bcffa9d5af744ac7670304a5eb59c0e5dfa6b486194afee2811d9e`。完整指紋保存在 `reviewer-ch05/technical-v3/source-fingerprints.json`，本檔封存後另留 initial hash 與副本。
- 必要前文只用 actual `ch01-complete-source-v8`、`ch02-complete-source-v4`、`ch03-prerequisite-v3`、`ch04-review-fixed-v7`。依實際需要回讀數值範圍／推導、unit／尾端 expression／return、module／use／tests、Self／receiver／field shorthand、enum／借用 pattern、標準容器、trait scope／bound、具體標準型別能力、lifetime placeholder／receiver／模型來源；沒有以未來大綱或第 6 章內容作先備。
- 先備 manifest 指紋依序為 `2ffad984aceb0e9fd822e5f0bb01a6958b6f063319a576e229df2da68c09e2cd`、`9a20a66ce1cd36f9c61e533d1621a43f6e0808fd1e67b639db8218f04297e263`、`0f2135203a1d97836c10810406ce3635a189d33915d91929439430f7edf491c8`、`d672bf58e8f197b3c603129adc3f9a72619d383a4a987a08c0db04e0b0333f14`。
- 沒有改教材、範例或共享設定。僅對 trait scope 疑點做 reviewer-only 小實驗，沒有重跑完整 chapter package、全部 fixtures、fmt／Clippy 或站點建置。

## 主要 API／語法的首次需要與契約

單位均指本輪第 5 章凍結包；`1v8` 等指上述實際前文。這份表由正文與 source 重建，沒有從作者帳冊取清單。

| 首次需要／來源 | 已教位置與必要關係 | 核對結果 |
| --- | --- | --- |
| 001-002：pub enum／帶資料 variants／derive(Debug, PartialEq) | 本單位先說 public enum 與 variants；3.1 已教資料形狀／pattern，4v7/002-002、003 已教生成能力／每欄要求／enum 比較 | CountError 的兩類都保留原 i32，兩欄能力足夠；沒有偷用 Copy |
| 001-002：Result<i32, CountError>／for／match／Debug 報告 | 3.2–3.3 結果與完整 signature；1.8 迴圈；4.2 Debug。新驗證先聲明 0–100 全合法 | <0、>100 只比較，不做會溢位的算術；任何 i32 都有明確結果。Err 是資料，不會自己中止迴圈 |
| 001-003：panic!／格式參數／中止 | 在 fixture 前定義 macro 與效果，回收前文 panic／assertion；特別限定本 main 的流程 | 正數內部契約與件數允許零分開。take_positive(0) 不是把所有外部零都改成錯誤；未泛稱任何 panic 都必然終止所有 threads |
| 001-004：Result／Option unwrap、expect | 表先給 by-value 接收、成功值、各自 panic 行為；Result 錯誤需要 Debug 也先說 | 固定成功資料與已知 slice 索引提供具體保證；expect 訊息沒有被說成使工作不可失敗。完整三份相關 fixtures 未用未教 API |
| 001-005：Result PartialEq／Debug／assert_eq | 本單位明說兩側能力與 variant 比較，接4.2／4.3；比較借用不移值已在4v7/002-003 | CountError／i32 皆符合；Ok／Err 推導由另一側完整結果提供。測試只驗已列邊界與原因，不宣稱窮舉所有值 |
| 002-001／002：type alias／pub type | 在 source 前分清型別名字、等號右邊既有型別與 runtime 值 | CountResult↔完整Result不需轉換；CountNumber沒有把−1變成非法i32。短名稱不冒充驗證機制 |
| 003-002：Display／fmt module／fmt method | 身分／用途與必要 method 先教；4.1 trait impl、2.5 Self／receiver 先備 | Debug 開發檢查與選定使用者文字分開；沒有以 derive 決定業務顯示 |
| 003-003：Formatter<'_>／外層&mut／fmt::Result | 本單位逐項說輸出狀態、兩種借用角色、Result<(), fmt::Error> alias；4v7/004-007、005-004已有placeholder／模型與receiver區分 | f 由格式化流程提供；fmt::Error 是出口失敗，不是 CountError 的業務原因。沒有提前 Box／dyn／io::Write 或要求讀者理解 Formatter 內部佈局 |
| 003-004：write!／format!／match尾值 | macro 的輸入／出口／結果／換行先說；1.5尾端expression、3.1借用pattern、4.2文字比較先備 | write 結果直接交出，沒有提前?；format交回ownedString。負數業務錯誤也正常完成顯示，不自行製造fmt::Error |
| 003-005：同一error Debug／Display／保存 | 本段說格式化借用、saved擁有新String；4.2具體String／&str能力 | error連續使用合法；String訊息不等於把error型別改成String；測試明記沒有模擬出口寫入失敗 |
| 004-002：From<i32>／Into<i64>／from與into | 泛型trait來源／目的／Self、infallible約定、by-value與目的推導都在code前；1v8/004-002已教i64等整數命名 | i32→i64保留所有值，i32 Copy使來源仍能用；此初例明確use From／Into。後續省略use的scope有N1 |
| 004-003：pub Count、private field與value(&self) | 2.5借用method、4.1 public型別／欄位；本單位說沒有Copy | 外部consumer不能直接塞入−1，只讀方法沒有變更路徑。泛稱「其他module」的精確性見I1 |
| 004-004：TryFrom／associated type Error／Self::Error | 在impl之前逐項教角色、實作指定type Error、完整Result實際型別與不同於獨立alias | 只在checked_count成功後建Count；associated type最低角色已實際教，不能因第6章仍會深入而判此處未教。trait scope仍需N1 |
| 004-005／006：From<Count> for i32／into move／比較結果 | code前說反向保留數字；段後先說Count非Copy、value借用與轉換接手不同 | 此實作合法；借用value之後才move，再未使用Count。私有field保證限定公開介面，沒有說實作者能免除維護規則 |
| 005-002：FromStr／Err／i32::from_str／parse::<i32>／ParseIntError | 先教文字格式、i32有限範圍、標準錯誤Debug／Display／PartialEq與同一解析；3.3 turbofish先備 | malformed、空白、超i32範圍各失敗；不接受任意Python int可讀文字。沒有先顯示陌生錯誤再事後介紹 |
| 005-003／004：ParseCountError／兩個From封裝／Count FromStr | 原需是分清整數解析與件數規則；先教保存兩個來源與必定成功封裝，再寫解析impl | 保留原error值與ownership，不依顯示文字猜分類；Self指當前型別沿用2.5，Self::variant是已教Type::variant的應用 |
| 005-004：成功Count與輸入文字lifetime | 先說Count只有owned i32、沒有referencefield；原String先as_str | 實際型別沒有輸入借用；解析不接手原String。Result／Count比較為借用，assert_eq後仍讀direct或from_owned，依4v7/002-003成立 |
| 005-005：AsRef<str>／T:AsRef<str>／&T | 新trait先給低成本借用、as_ref(&self)→&T、String與&str的具體能力；4.1 bound、3.5 byte len、4.5 receiver借用先備 | &borrowed是借用reference值，&owned借String，兩種參數層次已明說；函式不需未教的?Sized |
| 005-005：Label的完整Eq／Hash與文字投影 | code前指定id+text身分、AsRef只借field；4.3已有Eq／Hash／Borrow身分一致性 | id不同而投影文字相同，足以反駁此例Borrow<str>；沒有把AsRef泛稱等同Borrow，也沒有手寫未知Hasher API |
| 005-006：ToString／ToOwned／Owned／Clone對照 | Display提供to_string，具體str→String／[i32]→Vec<i32>與內容複製先說；associatedtype角色5.4已教 | 選定產物不借原資料，原值仍能用；沒有將所有Clone或所有ToOwned產物一概說成深層複製。std文字／numeric slice操作都已教 |
| 005-007：延後初始化let count;／多行method call／測試 | 4v7/003-005先說外層名稱延後初值、不是重新指派；其餘只是已教API串接 | 內層文字結束後讀Count合法；測試查結果規則、分類及來源結束，沒有用未知完整Vec比較 |
| 006-002／003：手動return Err／Result上的?／From轉換 | return1.5、兩個From5.5，先完整手動機制再說?角色、成功值與整函式早退 | i32與Count不是Result殼；ParseIntError、CountError分別封裝，第一步失敗沒有執行件數檢查。scope問題不影響已教的?轉換契約 |
| 006-004／005：caller政策／E0277兩種failure | Result比較5.1；?接不住unit與缺From分開說、後者原生解析型別先教 | main仍逐筆處理，helper早退不替main停迴圈；兩個compile-fail真正缺的條件不同，沒有假造負數原因讓解析失敗能編譯 |
| 006-006／007：Option上的?／numeric sharedreference／Debug與比較 | 在code前給Some／None路徑、Option<&i32>與Copy數字；4v7/003-005已明說Option／數值reference Debug及Some(2)格式 | 空／一／兩筆與Some(20)一致；Option沒有error payload，不替Result挑原因。沒有因本角色記得原舊4稿而錯判最新先備未教Option Debug |
| 六個完整答案／全部fixture／lib／Cargo | 答案在本節教完後交付；pub mod／package繼承欄位／use／cfg(test)來自1–4章 | 沒有closure、map、Box／dyn／?Sized黑箱。package manifest只有已教繼承欄位，未新增未說明lint設定。library的逐節module不改較早案例使用的既有能力或驗證規則 |

## 原需要、機制、用途與材料的分開裁定

| 節／原需引文 | Q1／Q3：角色與使用契約 | Q2a：設計改變／機制 | Q2b：為何回應原需要 | Q4：材料能支持什麼／界線 |
| --- | --- | --- | --- | --- |
| 5.1：「逐筆報告，不能因第一筆不合法就略過」 | Result交成功／具體原因；caller match，panic／unwrap／expect選停止政策 | 失敗保存為值，與是否停止分開 | caller每筆報告後繼續；只在另有內部保證的情境選panic。不是聲稱Result只適合繼續 | −1／0／3／100／101與尾句形成完整對照；內部分支與固定unwrap值各有不同契約，不把fixture當推薦外部資料政策 |
| 5.2：「同樣型別…數個函式重複出現」 | type alias為既有型別名字，pub使外部可用 | 用短名字代替重複Result拼寫，值與規則不改 | 讓每項工作輸入／輸出較容易讀；CountNumber仍可−1，指出命名不能替驗證 | 同一值兩個型別名稱接收、alias-is-name反例與tuple答案。沒有主張不同型別轉換或自動新不變條件 |
| 5.3：「避免每個地方重寫報告規則」 | Display定義{}，Formatter提供出口，fmt::Result報寫入結果 | 將訊息形成規則集中在型別；顯示與保存出口分開 | 不同caller用同一資料訊息契約，format把產物交後續工作；比呼叫處重寫match少一處重複定義 | 兩類錯誤與data-driven LimitError，借用同一error作三種表示。只驗訊息內容，不驗出口失敗，不以Debug格式當固定對外資料 |
| 5.4：「無法從型別辨認…已經通過檢查」 | From保成功、Into來源方向、TryFrom保失敗、associatedError選型別 | 成功後用Count表示已驗證；privatefield／只讀公開方法保介面邊界 | 接收Count的工作可依既有構造規則，不用重新驗每個普通i32 | 0／100與−1／101、保值往返、move／借用差異均可追；沒有Copy、沒有公開直接改field。N1只缺scope條件，I1只需收窄module措辭 |
| 5.5：「先…解析…再檢查…兩種失敗…不同原因」；「解析內容、取得借用表示與建立owned產物」 | FromStr／AsRef／ToOwned／ToString各有輸入／產物／ownership契約 | 解析兩階段合成具體enum；另以相同文字材料比較借用投影／獨立產物 | error分類服務caller政策；只數bytes不必解析；要保存或修改才建立owned產物 | abc與101使階段不同可見；Label id不同而text相同使Borrow限制可見；文字／Vec副本修改使獨立產物可見。不存在parse必為Display逆運算的假承諾 |
| 5.6：「同一項工作…手動傳遞…使用?簡化」 | ?成功取值，失敗依函式回傳錯誤From轉換並整函式早退；Option另有None路徑 | 將反覆match的早退樣板交運算子，工作與錯誤形狀不改 | 少重寫相同分支，caller仍決定逐筆繼續；要在此層取替代值仍用match | 相同inputs比完整Result，成功／整數錯誤／範圍錯誤三路；unit與缺From兩failure、Option缺第二筆、pair答案接起多步。等價性限定選定輸入，沒有宣稱窮舉證明所有情境 |

主要原因均由原文任務給出，不是本審閱者從API產物反推需求。中途子問題與本節收束連得起；5.5的幾種轉換共享文字材料但目的已逐項分開，沒有只靠一張API清單替代用途。

## 自行發現

### N1：省略 use 的轉換範例沒有說明這些 trait 已在 scope

- 版本／原位置：`004-005.md:22–25`，完整 `examples/conversions.rs` 只引入Count，卻呼叫 `Count::try_from(value)`／`count.into()`。unit hash `a2c710d4eec95243e4ee57a632ca593033e4d72bdf52cbb076d3e1266f92246f`；source hash `d928259e6353e3ff34bb60a09176d5e2cb04ddf6e1cdd2d465e0ef819c16be48`。後續 `005-003`的完整parsing module也直接用From，`005-004`再用TryFrom。
- 已教部分：4v7/001-006明確說 trait method需trait在名稱scope，Charge用use引入；4v7/002-004另外說Clone由Rust預先提供。本章004-002示範明確use From／Into；004-004說library的conversions module逐條use From／TryFrom。轉換機制、所有方法、Copy／move、Self與Error角色已教，並非整節未教。
- 缺少關係：library module的use不會轉給不同example crate；Clone自動在scope也不等於所有新標準trait都自動在scope。此時沒有說From／Into／TryFrom各自已由本專案edition預先提供，因此consumer省略use的原因仍需猜。不能把Count::try_from誤認成前文那種不需trait參與的inherent associated function。
- 實際影響／分級：**必要的selected trait scope契約，增加理解負擔**。正常source可以編譯，問題不是技術非法，也不阻礙Count範圍或?核心機制；但讀者在新介面首次跨crate呼叫時，要能依已教條件解釋方法如何被找到。
- 正式核對：[std prelude](https://doc.rust-lang.org/std/prelude/index.html)說自動引入與edition關係；[v1](https://doc.rust-lang.org/std/prelude/v1/index.html)列From／Into，[core rust_2021](https://doc.rust-lang.org/core/prelude/rust_2021/index.html)列TryFrom，而2024保留前述內容。查到的頁面均標std/core 1.99.0，與工具鏈一致。
- 相稱實驗：reviewer-only小程式以全名實作TryFrom，再省略trait use呼叫Count::try_from。rustc1.99.0的edition2024編譯／執行成功，edition2018得到E0599並提示TryFrom不在scope。這只核實名稱解析條件，不是把舊edition當學生路線，也不是重跑本章。程式與結果存 `reviewer-ch05/technical-v3/trait-scope-probe.rs`／`.json`。
- 最小修法：在004-002或最遲004-005完整consumer之前，說本專案edition預先把From、Into、TryFrom放進scope，這些例子可省略use；明確use仍可沿用。只列已教的這三項，不展開完整prelude或提前TryInto／Iterator。另一種修法是consumer逐份明確use，但改動檔案更多。
- 發現者／提示狀態：本角色正文與來源自行發現，沒有看其他第5章審閱；未修改或驗修後稿。

### I1：「其他 module」不能直接建 Count 的措辭可收窄

- 原位置／引文：`004-003.md:5`：「field 沒有 pub，所以其他 module 能使用 Count，卻不能直接寫 Count { value: -1 }」。004-005後段又提醒同一module能接觸私有field，實作者要保持規則。
- 已教與主要通過範圍：pub型別／field、對外consumer與實作者不同責任已交代；本章consumer確實不能直接寫field，公開value方法也不修改。因此Count公開介面承諾在現有source成立。
- 精確性缺口：private並非只在定義module可見，其子module也能存取；「其他module」若解讀成所有不同module，就包含已熟悉的巢狀tests module，範圍過大。[Reference privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)的private存取規則與unit-test子module例直接支持這點。
- 影響／分級：**可選的技術精確化**。這次範例沒有需要子module直接構造Count，也已限定保證來自公開介面；未造成當前運算或外部client錯誤。不以此要求提前完整module visibility章。
- 最小修法：把主詞收窄到「library外的呼叫者」，正好對應本節實際consumer；或短寫「定義module及其子module以外」。不需要新增pub(crate)／pub(super)或新範例。收益是避免讀者把外部邊界誤記為所有module一律隔絕。
- 發現者／提示狀態：本角色本輪自行注意並查Reference，未受其他review提示；沒有做子module執行實驗或修改教材。

## 正式 API 查核與圖的界線

除N1／I1所列頁面，查 [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) 的trait signature、ToString關係、parseability與fmt error規則，對照003-002–004／005-004。正文正確區分業務錯誤與出口錯誤，Display不保證可解析回原值；本輪不把該頁的後續所有implementors當已逐項查完。另查 [i32 FromStr](https://doc.rust-lang.org/std/primitive.i32.html#impl-FromStr-for-i32) 的十進位格式／符號／空白與範圍，及頁面相鄰的i32→i64無損轉換；未讀整份i32全部方法，也未跑所有數值邊界。這些正式來源只供技術核對，不算學生正文先備。

`question-paths.svg` 的frozen原位asset與archived source bytes一致，SHA-256 `a94a79cc4eb5f56cb515e567613830b407e818c568079e71d550ad2a85773934`。親自讀source：340×410、字24，左Ok→取value→下一行，右Err→From轉回傳錯誤→整函式早退。正文在圖前定義兩路，圖後立即指定只走一條與右側來源的From；沒有反向合流或把錯誤重新送回成功路。箭頭表示這次執行方向，不是同時產生兩份結果。

這份來源與位置核對可以支持機制一致，但**本角色未親看實際桌面／手機頁面或渲染PNG**，不把尺寸／source／root已做QA當成自己視覺通過。其餘小節的型別、ownership或轉換差異已有明確契約／程式／前後文字，沒有找到只有自行想像才能完成的必要素材缺圖；這是來源閱讀判斷，不是整站視覺驗收。

## 封存結論與未查範圍

全35單位與40份完整來源的選定API／語法／契約核對已完成。必要N1待處理，可選I1供coordinator裁定；其他主要通過項如表，不能把一處scope缺口擴大成轉換／associated type／錯誤傳遞都未教。

沒有看第5章其他review或修稿，沒有做修改。只有上述trait scope小實驗；未重新執行本章正常examples／答案／fixtures、全套測試、fmt／Clippy或文件建置，也未親看實頁。fixture expected目前只保存E0277或指定panic片段，來源與正文原因一致，但本輪不宣稱重新實測所有診斷／退出狀態。

此檔是互看前初判，封存後才交叉；所有新增提示、裁定或修後複查另存，不回寫本檔。沒有審第6章或續寫第14章。
