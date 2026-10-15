# 第7章逐段首讀報告

## 實際範圍與保存路線

我依合法釋出的feed讀完**161–186，共26個staged單位**，每包先保存自己的引文、Q1／Q2a機制／Q2b用途／Q3操作／Q4例子界線、當時未知及適合的小變化，再取下一包。`records/161.md`至`records/186.md`均保留；161列點編號與後續稍不同，但必要面向均有記，162另註明之，沒有重寫原答案。186保存後feed明確回覆：

> STOP: no more frozen material has been released. Save current state and report to root; do not search later files.

|實讀節次|feed單位|實際路線|
|---|---|---|
|7.1|161–167|借用回傳需求、單來源elision、兩來源失敗、'a關係、共同有效區間、過短來源、shorter|
|7.2|168–173|只借first、local reference失敗、owned結果、'_、outlives、first_nonempty與完整source|
|7.3|174–180|借用模型需求、TextView宣告與必要圖、make_view、泛型impl／數字方法、完整caller、模型過短、Label數字結果|
|7.4|181–186|兩層借用、receiver省略與原來源方法、scope對照、雙來源模型、完整library、Label reference結果|

先備背景是基本Python及我真正讀過的1–4章，加上第3章、150–160第5／6章的普通完整頁先備閱讀。第5／6章路線已另存`prerequisite-ch05-ch06.md`；我沒有把它改寫成逐段首次使用證據。

當包必要完整source、manifest、表格與測試均已實讀；185才見到完整library全部檔案，沒有把它倒算為早期no-lib例子已載入內容。第7章練習答案在167、173、180、186與題目同包可見，紀錄如實說已見答案，不稱外部隔離解題。所有小變化均為紙上閱讀預測，沒有執行cargo或其他教學命令。

我只用feed讀正文，只看當包列出的必要PNG，沒有檢索repo／queue／作者ledger／其他review／外部教材，也沒有讀第8章正文。沒有新發現的未釋出內容曝光；前章原始報告與records未改。

## 在這些例子中，我能恢復的問題與操作規則

7.1的核心問題是：回傳仍借原文字的&str時，呼叫者如何知道使用結果要保留哪些來源。same的一個輸入lifetime使輸出可依elision連到它；longer兩候選就不能只用省略。`<'a>`先宣告、`&'a str`在簽名使用，描述有效範圍關係而非型別或文字byte長度。兩候選與輸出同'a，表示結果使用區間須由兩來源都支持，不要求兩String同時建立／清理，也不延長資料。

我能分開E0106與E0597：163是簽名缺輸出來源關係；166已寫'a，仍因inside結束後用selected而過短。即使這筆資料實際會選outside，不能把body當次比較結果變成較窄公開契約。longer／shorter比較byte長度與lifetime關係是兩件事，等長規則需要看具體文字或body，不能只比長度。

7.2改變工作就能改契約。first_text保證只回first，second在呼叫當下仍要有效，但不為結果後續使用負責。local_text借函式內String交出去則E0515，任寫'a也不建立可借的文字空間；copy_text回String，是複製內容建立獨立owner並move交出，原文字可先結束或修改。`'_`只是明示讓省略規則推導，不是三個相同token便把多來源連在一起。

fallback把Option<&str>與來源關係結合：Some空文字仍有值，None才用backup。`'b: 'a`要求backup的借用至少支持輸出'a需要，允許一樣長，非嚴格更長，也不是把owner完整scope定為'b。first_nonempty又是另一個需求，判現有文字len==0而非Option缺值；可能交兩來源，所以仍需共同關係。

7.3的TextView本身持有reference值，source:String仍管理文字。`TextView<'a>`把欄位來源要求帶在型別中；make_view回`TextView<'_>`依唯一輸入連來源；impl也先宣告lifetime再用於型別。移動模型不會移動或延長String。byte_len借模型、沿欄位查bytes，回owned usize，數字可在模型與來源都結束後繼續保存。

7.4才正式回答我從174保留的問題：模型欄位借文字，method的&self另借模型。receiver就是self參數；省略method輸出連receiver借用，因此text(&self)->&str可能使借模型持續到輸出後續使用。original(&self)->&'a str則用欄位原來源，沒有將&self固定成'a，模型可先結束而輸出繼續借source。兩body都讀self.text，是此shared reference可Copy借用資訊，不是move String、move模型或複製文字。

我能在183辨識這次E0597太早結束的是view，與179 source／inside太早結束不同。source有效不會自動放寬text方法的receiver契約。TextPair再用'a／'b分開兩欄，first_source與second_source各連對應來源；沒有第二來源要支持第一輸出的需求，就不必加'b:'a。185的完整caller先讀兩欄，再只將第一視圖帶出，沒有保留已結束的pair或second。

## 先有未知、後釐清的時間線

- 161只有問題預告；162先實際看單來源函式，當時我把lifetime理解為有效期間是短推論，164才讀到「借用有效範圍之間關係的泛型參數」和語法，沒有把這個正式定義倒填到162。
- 163知道兩來源省略不足，但還不會補簽名；164才學同'a。164記下短second即使未選中也會限制外層使用的預測；166才見完整失敗例與E0597。
- 168只預告不同需求，first_text具體參數安排到169、owned String到170才讀到。沒有將導入判成已教完機制。
- 171的struct placeholder仍是預告，176的TextView<'_>才真正補上；175的欄位初始化推論也到176才有正文直接說明。
- 174開始保留「method回reference究竟連模型還是欄位來源」疑問，177–180只教數字，沒有假裝解開。181仍只有問題；182才實際教receiver規則、顯式原來源與shared reference Copy，183再看完整scope對照。
- 184只見雙來源模型定義與方法，185才見實際第一輸出跨第二來源及模型結束的caller。錯換body／換second_source的變化仍是我自己的推測，不冒稱有對應fixture。

## 可辨識的自身小變化與仍屬推論的內容

這些預測記在取後包之前，未執行；後包若提供支持，我另外記後釐清，不重寫初判：

|record|自己的變化或預測|實讀後續／目前界線|
|---|---|---|
|164|外層較長Rust、內層hi，結果在hi結束後用仍不合法；內層用完可行|165成功、166失敗其後才讀到|
|169|交換first_text兩實參，第一改內層來源後，外層使用不再可行|由已教契約推得，未見此變化fixture|
|170|local_text改回String並直接交inside owner，與借外來源另建copy_text不同|已有owned回傳規則的短推論，未執行|
|172|反寫'a:'b不能保證None分支的'b支持輸出'a|明列為方向推論，未見反向fixture|
|175／177|只移模型不延長內層source；只交owned byte數則可獨立|179／180後來才明示對照|
|178|建模前追加!會使兩視圖長14；模型仍待使用時插入source.push_str則借用衝突|套用既有bytes與borrow規則，未執行|
|182|original結果不依模型，不等於可在結果尚待用時改source|仍為既有shared借用規則推論，非本包另展示的source|
|183|直接讀公開view.text欄位不帶括號，應交欄位原來源，而非text()的receiver契約|由欄位型別與Copy推得，未見具體fixture|
|184|first_source若誤回self.second，無'b:'a時不能保證'a|generic關係推論，未執行|
|185|make_view／original不能替longer的輸入借用消除原有共同來源要求|組合已讀契約的短推論，非作者逐字結論|
|186|若尾端改label.original().len()交usize，來源可再次留內層|組合已讀owned數字規則的推測|

資料政策的小變化也保留，例如shorter("中","ab")按3／2 bytes選ab；first_nonempty(" ","Rust")仍選空格，因未教trim且len非0；Some("")和first_nonempty空字串的選擇政策不同。這些不是替作者發明新需求。

## 必要圖片與例子觀察的限度

第7章feed實際只列出175的必要PNG。我已用view_image親看`ch07-borrowed-field.png`：上藍框view:TextView<'a>／view.text:&'a str，藍箭頭向下借綠框source:String所管理Rust tutorial；旁註沒有另建內容，下框明說source仍為owner。圖交代角色與借用方向，沒有時間軸，不能單靠它回答後來method輸出跨模型scope問題。

165的兩來源／byte比較表、182的三角色表與185完整source、測試都讀了。相同輸出字串本身不能區分是否複製或契約相同，要連型別、建立路徑與簽名看；兩個空文字長0也不能用長度判斷選哪來源。185改用內容assertion能辨別等長ab/CD政策，但測試選定材料不是所有情況的證明。此報告沒有聲稱任何程式已建置或測試已實跑。

## 本次理解的界線與STOP

我能追完本章提供的具體函式、模型、兩層借用及scope對照，並恢復各節提出的問題。初判中的必要未知都有在原序後段得到明示或保持為有標記的短推論，沒有必要圖遺失，也沒有需要靠未釋出章節補完才能讀懂這些例子的承諾。這是單位層級理解，不是所有Rust lifetime情況或整書品質的通過宣告。

我仍未讀更複雜方法／泛型／多參照組合的完整規則，也沒有由這些例子學到如何設計任意借用介面。annotation不等於執行期延壽、&self不一概在呼叫結束便不再借用、原來源長命不一概足夠，這些只在目前契約上逐例判。下一章錯誤資料的格式化與傳遞只有186末段預告／連結可見，正文未讀，不預先補知。

已STOP。第5／6章普通整頁先備路線、早期所有原始報告及records維持原狀，等待後續合法釋出。
