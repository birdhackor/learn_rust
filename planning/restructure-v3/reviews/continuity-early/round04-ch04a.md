# Chapter 4.1–4.3 銜接與實際頁面：獨立部分初判（round 04a）

本輪只審已freeze的4.1、4.2、4.3，從實際已讀1–3章接續。沒有讀4.4以後正文／source，也沒有讀本章作者ledger、technical／firstreader初判、舊audit或root問題。角色仍是獨立銜接／頁面審閱者；不是盲讀或完整第四章驗收。

初判：**1項必要局部順序調整、1項可選排版改善**。enum→match→一般解構的主線與完整來源大致連續，沒有需要未freeze後文才能看懂的隱藏helper。

## 範圍與版本

19份來源hash在`round04-ch04a-sourcehashes.json`，初始完整副本在`round04-ch04a-frozen/`。三份教材與所引Rust在本輪未變；workspace及lock在審閱中新增後續data-steps/data成員，已保存末版metadata並還原初始兩檔、精確核對完整hash。只查新增成員manifest仍使用已教的package/name/version/edition，沒有讀後續章節的程式。

`intro-patterns`沒有library或build.rs，預設main只印文字；本輪各Cargo run都選明確example，沒有提前編譯其他未教的patterns範例。所有實際例子、答案、失敗fixture都讀完整來源，沒有執行Cargo、build或測試，也沒有改稿。

## C4a-01：match整體如何取得值，應先於第一個以它回傳運費的函式（必要；增加理解負擔；信心中等）

- 首次使用：`04-02-lesson.md:13–15`引用`patterns/examples/match_numbers.rs:2–8`，函式`shipping(tier: i32) -> i32`以整個match作尾端，arm分別為`0 => 20`、`1 => 10`、`_ => 0`。
- 片段前已說：:7「match將一個值依序與各個pattern配對，只執行第一個符合的分支」，且定義`pattern => expression`的arm；:9給三段運費規則；:11給literal／`_`的配對與順序。這些足以解釋哪個arm被選中，不應報成沒有match角色或沒有例子動機。
- 片段後:19才說：「match本身是expression，結果來自被選中arm的expression。這裡各arm都交出i32，整個match又是函式尾端expression，所以成為運費的回傳值。」
- 實際前文：1.6教block尾端值，2.2明說if是expression，2.5則另教loop靠break交出值。這些不能自動替尚未介紹的match決定整體結果；arm本身是expression，也尚未說它的值如何成為整個match的值。
- 最小缺口與影響：第一個函式已依賴「選中arm的值成為match整體值」，但正文還只交代分支執行。讀者必須先假定match的結果規則，再在下一段得到確認。完整閱讀單位後解釋已足夠，因此只是本輪先解釋、再在程式使用的局部時序項，不是整節概念阻礙。
- 最小修正：把:19第一句的整體expression／值規則移到第一段程式前。原例「各arm是i32、又在函式尾端」的分析可留在程式後；不增新例子或篇幅。
- 頁面證據：`desktop-match-first-expression.png`、`mobile-match-first-expression.png`均實看此先後。未修、未做修後複查。

## C4a-02：pattern首次定義的粗體標記直接顯示（可選；信心高）

- 位置與引文：`04-02-lesson.md:7`的`**pattern（模式）**描述要符合的值或資料形狀`。
- 桌面1280×800與手機390×844均顯示字面`**`，而非預期粗體；實際截圖同上，metadata保存rawAsteriskParagraphs。是在檢查本頁首個match例子時看到；先前本角色C1-02也曾學到相同parser邊界問題，未據其他本章報告提示。
- 定義與中文含義仍能讀到，沒有必要概念缺漏。具體收益是減少初次核心詞旁的語法雜訊、恢復掃讀線索。
- 最小改動：例如用`**pattern**（模式）描述…`，不需擴寫pattern定義。未修、未複查。

## 主要通過證據

| 單位與需要 | 身分／機制／操作的實際依據 | 需要／用途及例子支持範圍 | 判定與待教界線 |
| --- | --- | --- | --- |
| 4.1保存「哪一種情況及其資料」 | :7先定enum／variant與一次一形態；:9–15先列三形態及payload；:17在完整source前交代具名dead_code allow與範圍；:29、31在建立前教型別::variant及底線開頭仍是binding | :3先給Missing與Amount(0)若只用0會混淆的獨立需要；:25說無假金額與不同payload形狀；:37分Amount(0)與Missing。固定println沒有實際讀欄位，:61據實限制輸出證據。 | 足夠的表示與建立教學。4.2才真正讀取／分流，沒有把固定輸出當enum資料已被辨識。Shipment答案完整，局部allow與_變數都先教後用。 |
| 4.2先配對再取資料、檢查完整涵蓋與結果型別 | :7／11在第一例前教pattern／arm／literal／wildcard及順序；:45–47在Quote例前教binding、arm scope、命名欄位簡寫；:53連到unit；:78在失敗例前教exhaustive；:86–92分覆蓋與型別相容 | :3說先用數字觀察分支再回報價，:9明定此次tier規則；Quote例包含Missing、Amount(0)、Amount120、Rejected37，:74完成4.1尚未實作的0與Missing區別。E0004不是因某次呼叫不傳Missing就可略過；:84要求先決定缺報價時的工作。Shipment等待貢獻0明說是統計結果，未倒回把原資料改成數字0。 | 除C4a-01的整體值規則位置外足夠。owned參數沿用3.2，不需要提前借用enum的pattern；本輪沒有依賴4.5。 |
| 4.3把既有解構統一成pattern規則 | :7給let右值／左pattern；:11在tuple前教_不建立名稱；:13在array前教rest及與range／slice的差別；:25在struct前分欄位名與新binding名；:31教欄位rest；:64在參數前教pattern位置；:70在完整呼叫前分一個tuple參數與兩參數；:72在for前教tuple array；:94在nested例前交代外enum／內struct；:108–110在失敗fixture前教irrefutable／refutable及為何let沒有失敗分支 | :3承接Amount命名與1.8tuple，例子逐步保留已知120×3／80×2。array source先給[120,80,200,60,40]再實際取首尾，不讓pattern改變容器形狀。nested的Order請求與Cancel兩例顯示先外層、後欄位；取消沒有硬找不存在的Order。完整答案三種材料均附型別／函式／main及assertion。 | 足夠。全是已教數字資料，沒有在本輪用原物件再使用去偷渡partial move；String binding ownership明確待4.5。普通let失敗已可用本輪match修正，4.6/4.7只是後續路線，不作本輪先備。 |

## 實際頁面與必要圖解

實看8097的`04-01-lesson/`、`04-02-lesson/`、`04-03-lesson/`，桌面1280×800與手機390×844。三頁沒有img／SVG，不是只因沒圖便跳過關係檢查：

- 4.1的variant／情況／資料三欄表兩尺寸可同時對照；Rejected欄位寫法在手機換行，三種資料形狀仍可辨。
- 4.2首個match函式及前後文字兩尺寸均實看，因而確認C4a-01與C4a-02。
- 4.3參數與for段兩尺寸可讀到「一個tuple參數」對照與兩筆小計520；外enum／內struct段兩尺寸均看過，Request::Order的外框、Order的內部欄位及Cancel另一分支都有具體正文／完整程式支持。手機長程式行可橫向捲動；前後說明也明寫完整nested pattern。此處不需自行想像空間位置，未發現必須補SVG才能理解的資料對應。

目視截圖為兩尺寸的enum-forms-table、match-first-expression、pattern-parameter-and-for、nested-pattern。三頁首頁另有保存，但未目視的首頁不算版面通過；不宣稱逐像素全長檢查。

沒有開4.4、4.5草稿；預覽導覽標題可見但未當已教前提。沒有真人首讀、Cargo執行、最終strict站點或第四章後半的通過結論。此部分初判封存後才能比較其他角色，後續章節／修正另存。
