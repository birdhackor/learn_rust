# 第6章尾段獨立技術初判：6.3–6.6

從已實讀第1–5章、6.1–6.2接續，順序核對6.3→6.6正文及全部實際selected source／helper／tests／fixtures／expected／manifest，另讀map-query SVG原文。本身尾段判斷先在tool payload寫好，期間未讀作者verification或其他review／continuity／reader；未讀第7、8章當先備。正式落檔被metadata／SVG版本漂移延後，此間收到root修訂通知，詳細時序與版本另記下方；不聲稱提示前已正式seal。

**必要問題0項；初見選讀觀察1項，封存時實際source已修正。** 本輪採靜態型別、借用、狀態與輸出推演，加上snippet／hash／TOML／SVG XML查核；沒有執行Cargo、rustc、Clippy、Rust測試或瀏覽器，不聲稱目標讀者實驗。

## TC06T-O1：回指PartialEq契約時可明指6.2（選讀改善）

本角色最初實讀的6.5:9寫「上一節已用PartialEq比較內容」。PartialEq的明確身分、比較規則及標準文字內容比較，實際教於6.2:35–41、62–86；緊鄰的6.4主題是associated type。

6.4的assert_eq!確實也間接使用PartialEq，所以不把這句判為Rust事實錯誤或必要先備缺口。若讀者想回顧Eq正在延伸的相等契約，改成「第6.2節已用……」會更直接指到解說。6.5:71、89及6.6:58本來也已明指6.2，因此現有主線充分；此項僅選讀的交叉指涉精準度建議。後來hash snapshot的實際source已改成「第6.2節」，本次封存沒有未解項。

## 完整閉包與首次使用

intro-trait-stages仍只有name／version／edition三欄manifest，無library、build script、dependencies或repo內額外Cargo設定。11個run目標中，cloning／copying／cloning_answer／custom_keys各單檔；associated與答案共用完整associated_support；三個map目標及grouping_answer載入同頁完整map_support；只有grouping另載grouping_support。全部來源已逐檔實讀，沒有後章library、Iterator／entry／closure API或未展示helper。2個精確example test命令各載入3項test；沒有all-targets或後段member的source入選。35個snippet引用及起訖標記皆存在。

- **Clone／Copy：** 6.3:7–19在首次完整程式前交代Clone借讀原值、String獨立文字、derive欄位需求及Vec元素逐一Clone；reference只複製借用資訊的界線清楚。Vec<&str>的兩份容器仍借同一份原文字，沒有被說成String深複製。Copy之前接上3.2，說明所有欄位Copy與型別Clone需求；String資源責任與不可由stack／heap位置推導Copy的解釋正確。clone／copy／答案的獨立修改與assertions相符。
- **Associated type：** 6.4:17–21在首個trait source前定義type Output與Self::Output，區別型別位置與值欄位，且說明owned self。泛型impl宣告／使用T、每份impl固定Output、S::Output及equality bound皆先說後用。i32／bool結果保留原型別；未知Output沒有被假定可算術；numeric-only bound才允許乘二。String取出測試的Rust 教學確為11 bytes。FirstPair答案接手來源只交回first，second隨其餘來源結束，符合所有權與題意。
- **HashMap／Borrow：** 6.5先交代K／V、Eq反身性、Hash與碰撞及相等／hash一致性，再使用String標準鍵。std::collections引入、new、insert取得鍵值與回傳舊值、同鍵保留舊鍵而取代值，皆符合實際source。get／get_mut與len的借用、結果型別、缺鍵不新增也明確。Borrow<str>先給視圖及Eq／Hash契約，沒有把任意reference當可替代鍵身分；不需手寫Borrow才能使用本例。
- **借用來源：** get(query.as_str())的結果借map值而非query，query追加文字後found仍有效；圖標記的查找當下與後續變化分清。反例在insert後仍讀found，E0502成立；先完成最後借讀再insert的修法符合既有規則。
- **分組／自訂鍵：** 6.6在首個函式前說明for借得&String、String的clone建立owned鍵、僅缺鍵才複製；Some更新與None後重新insert的借用可成立。表格真正呈現重複Rust累加，不是每次覆蓋成1。owned map在輸入block結束後仍有效的測試可支持鍵所有權。derived PartialEq／Eq／Hash同欄位與兩個欄位各自改變的查詢對照充分；缺Hash的insert失敗碼正確。quantity_by_name答案只借讀&StockItem，Copy quantity、缺鍵clone name，Rust2+6=8／Python3、原輸入3筆均相符。

## 結果與圖

11份成功目標的輸出及assertions由實際source靜態推演，與正文相符。associated及grouping各3項test，測試數與所有來源一致。4份完整compile-fail及其非空expected分別為E0204、E0271、E0502、E0599，與其具體失敗操作吻合；本輪沒有重新執行。

map-query.svg的title／desc、顏色線型legend、箭頭與alt一致：查詢視圖藍色實線借query，綠色虛線依文字查map，found藍色實線借counts內usize值2。兩份Rust文字與被借的數字是不同框，圖在code之前明確標為「查找當下」，與後續query改為Rust 課程不衝突。XML可解析；沒有瀏覽器或圖像視覺驗證。

同名JSON保存38份SHA-256、逐目標完整載入、13條命令、四題概念依據及逐目標靜態輸出。intro workspace新增較後member只作metadata核對，未讀其source或當作先備；本結論限本次四節技術與依賴，不是後章或全站驗收。

封存時另偵測intro workspace新增iterators member及對應無dependency lock entry。已實讀兩份metadata新版本，原snapshot hash保留，另記封存hash；本章Rust／正文／設定均未變，未讀新增member source。

版本時序補記：上述初見選讀觀察與必要0的判斷稿在root後續提示前已寫入tool payload；正式報告落檔因workspace metadata及SVG hash改動而延後，root提示到達時尚未完成檔案seal。本角色未讀其他report。另確認較後的hash/text snapshot其實已包含6.5「第6.2節」修訂，故保存初見引文、snapshot與最終hash，不虛構舊版實測hash，也不聲稱root提示前已正式落檔。map-query.svg僅新增width=420／height=680，移除兩attribute即可重現舊hash；已再讀原文與XML，但desktop尺寸觀察屬root而非本角色。6.2新增Debug借讀說明的root提示不回填成我自己的原版發現，將另作窄followup。
