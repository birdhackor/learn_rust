# 4.4–4.7 獨立技術初判

未發現必要技術缺陷、實際選取來源閉包／先備缺口、edition2024規則、狀態更新、結果或失敗原因不一致。另有一項可選的先備指涉用語建議；本句已解釋夠用，不列必要問題。

## 獨立性、方法與版本

以基本Python、實際已讀1–3章與4.1–4.3為先備，依4.4→4.7順序完整讀正文，以及全部實際引用source、helper、fixtures與manifests。未讀章4作者ledger／validation／freeze結論、首讀、continuity／audit、其他角色或歷史問題，亦未用未完成章5補缺口。

本輪是整頁技術重讀，非盲讀／目標讀者實驗。來源於2026-10-15T07:18:30.584290+00:00保存42個SHA-256、34個snippet核對；同名JSON保存20個成功target命令、結果靜態預測與七份失敗例檢查。

為核對當前工具鏈的精確2024診斷原句，只做一次focused rustc --edition=2024 --emit=metadata -o /dev/null預期失敗檢查。先source scripts/env.sh並使用要求的compact env；exit1且實際輸出「cannot explicitly borrow within an implicitly-borrowing pattern」，位置為fixture9:26，help建議移除多餘ref。沒有Cargo／成功target／Clippy／tests或browser執行，也沒有輸出執行檔；其餘結果都是靜態核對，不借作者驗證作自己的證據。

## 可選 TC04T-O1：return; 的「已教」指涉可更精準

4.7:9寫「失敗時用已教的 return; 結束這個回傳unit的函式」。在實際先備正文與早期引用source中，前面出現的是帶值return；1.6也已教unit，但沒有早先的裸return;形式。

若「已教」指return這個概念，前文確實已教；若指完整的無值形式，字面指涉略超前。這句本身在snippet前已說它結束unit函式，加上已知return／unit可以正常接起，因此不構成必要的概念或操作缺口。可只刪「已教的」，保留現有解釋，避免讀者誤以為漏讀過這個確切形式；無須新增小節、return()變體或其他先備。分級為可選文字準確性改善，不是Rust語意錯誤。

## 概念、用途與技術證據

| 原文單位／source | 身分與機制／性質 | 需要／用途 | 例子與判定 |
| --- | --- | --- | --- |
| 4.4:7–29 or pattern | 單一\|組合pattern，非bool的\|\|；兩種選項須建立相同名稱、型別和借用方式的binding。 | Amount／Estimate接受同類金額，共用處理；Missing無金額不能硬交value。 | or_patterns回120/80/0。or_missing_binding在Missing側沒value，E0408合理；不宣稱只有此一診斷，後續未初始化錯誤不改其原因。 |
| 4.4:33–45 range pattern | pattern的0..=299只檢查一個值，不像for逐項走訪；char範圍同樣合法。 | 依連續金額區間選運費。 | 非負小數值約定明示；299/300為20/10，499/500為10/0，端點包含正確。沒有把負數也免運當成承諾，因其不在本節輸入約定。 |
| 4.4:49–85 shadow與guard | Amount(limit)建立arm內新binding；guard在pattern符合後檢查bool，false繼續後續arm；guard作用整個or。 | 要比較執行時門檻，而非把外名誤當pattern中的既有值；也需把ready限制套到兩個選項。 | shadow Amount50顯示內50、外100；guards的80不過100後走Amount(_)為0，120通過；1且未ready為0，能分辨guard是否只限制or右側。guard_not_exhaustive仍E0004，未把guard當完整涵蓋。 |
| 4.4:89–117 @ | 名稱@pattern既篩選又在arm綁定實際值，不是綁整個range。 | 保留1..3中的實際數量；另將4或5且ready限制為3。 | at_bindings的1／3保留自身，0／4走0；combinations_answer的(2,true)/(4,true)/(4,false)/(0,true)為2/3/0/0。第三例確實區分guard涵蓋整個or，第一例區分實值與上限。 |
| 4.5:7–43 owned／partial move | binding依取得的型別Copy或move；String不因在pattern就複製。取label只移該欄，未移count仍可讀，但完整Order不能再借出。 | 用已教String看見僅數字時不明顯的交接限制。 | owned_patterns在consume的Label arm接手A；partial_move取label後讀order.count3合法，失敗例借整筆order為E0382。每檔String建立／helper完整，未用Clone遮掉關係。 |
| 4.5:47–67 match ergonomics | 配對&QuoteText時內text為&String；&mut時為&mut String，自動沿外reference配對enum；仍受借用衝突規則。 | show只讀並保留原資料；append修改同一份文字。 | borrow_patterns兩次讀A後追加再讀A：已備妥；沒有move原String。move_from_pattern_borrow嘗試*text交出String，E0507與3.6借讀不可移出規則相同。 |
| 4.5:71–97 ref／ref mut／& pattern | ref決定新binding借讀，ref mut借改；單獨mut只改binding可否重新指派。& pattern配對已有reference層，非建立借用。 | owned資料只借某些欄位，或以完整形式對照自動借用。 | explicit_bindings分借label／count兩個不同欄位，數量3→4，借用最後使用後原訂單可再讀。reference_pattern的&QuoteText::Label(ref text)合法，兩項具名Clippy allow的各自理由在87行先於source90，僅限show。 |
| 4.5:101–111 2024規則 | 自動略過reference後預設已借用，不能在該位置再任意加ref/ref mut/mut或內嵌& pattern；binding mode的當下角色有說明。 | 分清自動／完整明寫兩條合法路線，避免為修錯誤改成接手整份資料。 | 反例對&QuoteText使用Label(ref text)；唯一實際rustc檢查驗證了書中診斷原句。改Label(text)，或從外層明寫&QuoteText::Label(ref text)，符合本例2024規則。未聲稱所有舊edition情況已完整推導。 |
| 4.5:115–145 _／_name及答案 | _不對該欄建立Copy/move/borrow binding；_text仍是String binding，會move。 | 只確認形態但保留原值，與接手但避免未使用警告不同。 | ignore_patterns的Label(_)後show(&quote)可用；_text例再借whole Quote為partial move E0382。borrow_patterns_answer的append text:&mut String、show text:&String，兩次顯示B：完成；Missing分支也完整。 |
| 4.6:7–29 if let | 先算expression，再以一個pattern配對；成功binding只在body，失敗略過或交else。對&Quote所得amount是&i32。 | 只對有金額做一小段工作，不需要為所有其他形態分類。 | if_let同時列完整match、iflet與有else版本：前兩行120，Missing無else不多印，有else顯示缺值。沒有把iflet說成取代本來需區分Missing/Rejected的match。 |
| 4.6:33–60 helper／while let狀態 | next_batch借改remaining，正值先扣processed再回More，0回Done；whilelet每次重新呼叫、重新配對。 | 每次取一批並持續更新剩餘量，並非藏了待取清單或新API。 | helper完整定義；7→4→1→0回More3/3/1，第四次Done不進body。借改呼叫返回後body才印remaining，沒有借用重疊；whilelet本身不保證永遠More的helper會停止。 |
| 4.6:85–97 批量答案 | work非負、capacity正的約定先說，程式開始即assert；processed不超remaining。 | capacity0可能不前進，約定直接支撐停止。 | 全部Batch/helper/batch_count/main同檔，(7,3)/(6,3)/(0,3)為3/2/0；零項不多算body，最後不足一批只扣剩餘量。 |
| 4.7:7–44 let else／unit return | 成功binding留在後續目前scope；失敗else須離開繼續路徑；此處return;結束unit函式。前述可選指涉不影響本句基本解釋。 | 先排除缺值，使成功路徑依正常順序使用amount，免再包一層成功if。 | let_else同一Quote做match／iflet／letelse，Amount120均140，Missing均略過；match的Missing直接return，沒有繼續交回amount，因此不用先教never型別也能理解控制去向。 |
| 4.7:48–76 diverge／continue／break | println會沿路徑繼續，不足以處理未建立binding；return、continue、break均可離開該繼續路徑，後兩者不等於離開函式。 | 失敗略過當項，或整個走訪到此為止。 | let_else_must_leave的unit else會E0308，原因正確。let_else_loops借&[Quote]每次取得&Quote，amount:&i32；continue保留最後80合200，break於Missing止合120，之後都由函式尾端交total。 |
| 4.7:80–119 章末模型與答案 | Quote保留Missing／Offered(Order)／Rejected差異；show完整分類，letelse只留可計價形態；都接受&Quote。 | 組合owned標籤與借讀計價，非成功形態的0只是本次統計貢獻，不抹掉原分類。 | quote_report全型別/helper/main完整，OfferedA為180，Missing0、Rejected37為0；借slice走訪後仍能再借quotes[0]。規則明確回到300免運／空單0。let_else_answer檢查Waiting0、small180、threshold300、empty0，再借small仍180，完整source無hidden helper。 |

## 完整來源與執行邊界

20個成功targets均明選intro-patterns及本頁--example／--locked；manifest只有已教name/version/edition，沒有library、build.rs、external dependency、mod/include或repo Cargo config。每個target都是完整單一.rs，包含自有模型、全部helper與main；不因同package其他未選例子而編譯其source。String::new、push_str、as_str和數值assertions均已由必要前文教。

七份失敗source完整，六份錯誤碼／原因靜態符合，2024 ref反例另有實際metadata診斷。沒有用Option、Result、Vec、泛型、derive或章5才能理解的API。這四頁無必要圖檔；短source、狀態表與已教資料模型足以追蹤當前關係，未做任何browser／手機／版面驗證，也不把XML或能編譯當成讀者實驗。

原初判先於其他角色結論封存；可選項的最低收益與修正成本已分開寫明，不把它升級成主要概念缺陷或Rust錯誤。
