# 第6章前段獨立技術初判：6.1–6.2

從已實讀第1–5章正文接續，按6.1、6.2核對實際完整source／helper／tests／manifests／fixtures與expected。封存前未讀第6章作者evidence／validation、continuity、reader、其他review或已知問題；6.3–6.6未讀，也未當作先備。5.5窄修採納通知與本章判斷分開，待本初判封存後再做該項source followup。

**必要問題0項；選讀建議1項。** 本輪是整頁技術重讀與靜態source推理，沒有執行Cargo、rustc、Clippy、Rust測試或瀏覽器，不聲稱目標讀者實驗或獨立實跑結果。

## TC06P-O1：「非負小數」可精確為「非負小整數」（選讀改善）

6.1:119的練習寫「材料採非負小數，折扣不超過原價」，但同一題欄位是original_price:i32、discount:i32；本節5行也明定非負小整數，答案使用360與80。小數通常指帶小數部分的數值，容易短暫讓人以為價格可含小數，與i32不合。

這是局部用詞精確度：型別註記、已教i32及實際材料已足以辨認整數，不阻礙trait主線，也不是程式語義錯誤。建議僅把「非負小數」改成「非負小整數」，不用增加decimal型別或額外規則。

## 實際載入範圍與完整性

intro-trait-stages沒有library、build script、dependency或repo內額外Cargo設定。charges與charges_answer各載入自己的main source及同一份charge_support/mod.rs；其餘debug、equality、standard_equality、equality_answer皆單檔。charge_support只有mod.rs而沒有main.rs，不是Cargo另個example；所有未選後段example不會進入本次selected閉包。

6.1四個前置snippet已涵蓋normal helper全部定義，再展示完整charges.rs；完整manifest只含先前已教的name／version／edition，package／no-lib／module角色在11行即說明，沒有晚教的必要依賴設定。完整helper（包含cfg(test)）於108行列出，在112行精確example test前；答案也重用此已展示完整helper。6.2每個example均完整列出，沒有隱藏helper或後章API。18個snippet引用及起訖標記存在。

## 首次使用與技術結論

- trait簽名、無body分號、pub契約、impl Trait for Type及impl方法不再加pub，皆在首個相關source前說明。T:Charge先定義能力要求，再讓body呼叫amount；&T、&self及field讀取可接上已讀第3、5章。
- 共用門檻的需要、每種資料不同取金額方式與比較只有一個位置，有實際Order／Bundle對照及300／301邊界測試。正文沒有把trait說成保證商業結果，也沒有宣稱原本無法各寫函式。
- Caller的trait scope引入在首次完整main前交代；use不取得ownership。DiscountedBundle答案加入新實作者並重用門檻，既有Order／Bundle都得到280，輸出與assertions相符。
- Debug的developer表示、:?、derive及欄位能力需求先於首份source；PartialEq的==／!=、struct逐欄、enum同variant再比payload，以及assert_eq需要比較與失敗顯示兩種能力，也均先講後用。比較借讀而不move的說法與後續使用相符，Debug外觀不被當作穩定外部格式。
- String／&str按內容比較、Debug引號、usize及shared reference按所借值比較，在各自片段前已說明；Option的形態與內容比較也在使用前定義。Option<&usize>來源不同但都借到2的例子確實比較相等，沒有把位置相等或None當成數值0。
- equality_answer獨立建立兩份文字，沒有偷用Clone／Copy方法。只改第二筆amount為300，原280保留、標籤仍相等而整筆不等，與題意、assertions及Debug輸出一致。

本範圍6個run目標與1個精確test命令。charges兩項test、missing_bound的E0599與not_debug的E0277，皆與實際source／非空expected的靜態契約一致；未宣稱本輪重跑。24份SHA-256、逐目標完整閉包、7條命令與四題概念依據存於同名JSON。沒有必要的型別／載入疑點需要擴大測試範圍。
