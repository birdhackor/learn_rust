# 第8–9章普通完整頁先備閱讀報告

## 實際路線與界線

合法釋出的**187–199共13頁**已按同一feed原順序完整實讀，每頁保存record後才取下一頁。199保存後feed明確回覆STOP；我沒有取200或搜尋後文。這次是 **ordinary prerequisite page reading; not staged first-use evidence**，為之後閱讀建立真實前綴，不是為第8／9章補做逐段盲首讀。

每頁的四題、機制／用途、引文、未知／推論與小變化都寫在`records/187.md`至`records/199.md`。但頁內完整正文、source與答案在寫record前已同頁可見；不能把它們當作概念首次必要使用位置的逐段隔離證據。頁末的「已解釋」是完整頁讀後的理解，不是聲稱答案尚未見時已獨立解題。

|單位|完整實讀頁|主要實讀內容|
|---|---|---|
|187|8.1 Result／panic|CountError驗證、逐筆政策、panic!、unwrap／expect、Result條件能力、fixtures／全部policy快照與答案|
|188|8.2 type alias|同型取名、非新型別／非驗證、ownership保留、tuple別名與完整答案|
|189|8.3 Display|fmt／Formatter兩層借用、fmt::Result、write!／format!、完整載入來源／訊息測試與答案|
|190|8.4 From／Into／TryFrom|i32→i64、私有Count欄位、validated入口、associated Error、消耗與借讀、六測試／全部快照與答案|
|191|8.5 FromStr／AsRef／ToOwned|解析規則、Integer／Range分類、&&str、AsRef與Borrow、ToString、兩種ToOwned、全部來源／十項解析target測試／答案|
|192|8.6 ?／傳遞|手動與?、From條件、unit main／錯型別fixtures、Option?、兩筆順序、全部六模組與十二測試／答案|
|193|9.1 closure|語法／capture／推導、shared與mutable、move與body移出、三fixtures、必要圖、同頁答案|
|194|9.2 Fn系列|callback、FnOnce／FnMut／Fn包含、工作receiver、具體closure型別、function pointer與完整例／答案|
|195|9.3 impl Trait|參數／回傳選型別方向、opaque單一型別、回傳工作、2024 RPIT capture、precise use清單、完整例／fixtures／counter答案|
|196|9.4 Iterator|Item／next／狀態與lazy、IntoIterator／IntoIter／for、三ownership路徑、必要圖、fixtures／StepDown答案|
|197|9.5 adaptors|map／filter參照層、collect／FromIterator、Vec條件能力、fold、Result短路收集、必要圖、完整例／答案|
|198|9.6 文字／Entry|chars／Unicode escape、lines各種結尾、is_empty、Entry回呼、owned鍵、map三Item、完整例／fixture／答案|
|199|9.7 排序|Ordering／PartialOrd／Ord／NaN、stable與unstable、callback契約、完整次數／姓名規則、完整報表／答案|

所有inline manifests、完整source、fixtures、tests、folds及答案均已實讀，包括190–192重現的完整helper快照。早期target沒有被我當成已提前載入後期內容。第9章所有例子是頁內完整no-lib程式，預設入口只提示選example，沒有據此跑任何工作。

沒有執行教學／安裝／測試／編譯，也沒有直接讀repo／queue／作者ledger／他人報告／外部連結。第10章只見199末段Cargo／檔案／命令列預告，正文未讀；第10以後及第14正文未讀。原161–186與`report-ch07.md`保持原樣。

## 必須披露的兩處自行預先命名

整理報告時我發現兩處不應帶入的背景詞名，已在原record末**APPEND**更正，沒有修改原回答：

- 190第3點Q4寫了「TryInto」，當頁沒有提供這個名稱。它不是實際已讀先備，這次也沒有用它推導任何能力。
- 194最後一句先寫了「impl Trait」，當頁正文／下一節連結未教出此名稱。我沒有先看195或外部來源，但不應提前命名；不作194理解證據。195合法整頁釋出後才真正讀到其語法、opaque與capture。

這是我扮演讀者時的命名失誤，已向root回報；不是發現偷開未釋出source的情況，也不能拿來宣稱完全沒有任何背景詞名混入。所有有內容的impl Trait理解都以195實讀為據，TryInto仍不在本次教材讀到的知識範圍。

## 我現在真正能恢復的操作規則

第8章讓我把**錯誤資料、驗證、顯示、轉換和停止政策**分開。0到100的件數驗證交Result，Negative／TooLarge保存原因與原輸入；Err本身不停止。單一main示例中的panic!或unwrap／expect失敗才停止正常往下，expect訊息說明成功預期而不保證成功。Result是否Copy要看成功與錯誤型別兩者，不能只看i32；unwrap／expect對Result還要求錯誤Debug。整份Result的Debug／PartialEq也有雙邊條件。

type alias只是同型別另名，沒有驗證或增加Copy。Display則真的新增型別能力，定義{}使用者句子；Formatter的外層&mut與其內部借用角色分開，fmt::Result是寫入成功／失敗，非件數業務錯誤。write!交指定出口，format!與to_string建獨立String。這些顯示契約不承諾可逆解析。

From／Into是保證成功的轉換安排，From提供對應Into，不能反推。Count是私有i32欄位的新struct，由TryFrom先checked_count才建；公開借讀不提供破壞規則修改，但同module新增操作仍須維持規則。Count→i32可保證成功，into／from接手非Copy Count，value方法只借讀。

FromStr／parse先依文字規則取得整數，再驗證件數；格式／i32不可表示屬Integer，整數已得但範圍不合法屬Range，均封裝原原因。Count只有owned數字，不借輸入文字。AsRef只借現有view，可投影欄位，不具Borrow要求的完整鍵Eq／Hash身分承諾；&str變數傳給接&T介面時會是&&str。ToOwned本頁str→String、[i32]→Vec<i32>可獨立，但不是任意產物一律無內含借用。

?對Result是取Ok或以From轉錯型別後提早離開**目前函式**；尾端成功Ok仍要寫，helper早退不決定main是否繼續整批。unit main與缺錯誤From的Result helper雖同E0277，理由不同。Option?取Some或回None，不自動造任意Err。count_pair按順序兩步都成功才交tuple，首失敗不做第二筆，無半份結果。

第9章讓我把**建立工作、保存環境、呼叫權限、逐筆Item與消費目標**分開。closure可capture外部資料；推導出的參數仍固定，不是泛型。shared／mutable capture持續受來源與最後使用規則約束，原資料mut與closure binding mut各有作用。move決定按值保存，Copy reference仍只是借用資訊；是否消耗工作看body有沒有移出非Copy內容。

FnOnce／FnMut／Fn分別承諾按值／mutable／shared呼叫，能力包含而非互斥；FnOnce不是所有符合者本質只能一次，Fn也不保證純函式或每次同值。共用函式只需依權限與次數選所需能力。function pointer的小寫fn是具體型別，普通fn與無capture closure可轉；capture環境不能只塞進同一pointer。

參數impl Trait由caller供具體型別，回傳impl Trait由實作選；opaque仍有單一具體型別，兩個不同closure分支即使同Fn也不相同，可將分支放同一closure body。move回傳owned設定／計數讓工廠結束後仍可用；move借用文字則仍需原來源。2024 RPIT自動capture允許隱型別依賴scope內generic參數，包含省略lifetime，與建立closure時實際capture資料不同。fixed_reader即使不存ref，未限介面仍會連來源；use清單可明限。use<>不能消除真實借用，有型別參數時還有全部列入的語法要求，不能把這個只有lifetime例子泛化。

Iterator持推進狀態，next(&mut self)交Option<Item>；Item可owned或reference，零不是結束訊號。這些倒數／slice例子結束後仍None，但一般Iterator可在None後再Some，for遇第一None就停。IntoIterator建立走訪，已是Iterator時交同份進度；owned輸入與reference輸入各接自己那份值。iter／iter_mut／into_iter的Item分別讀／改／交接，包進iterator不取消借用或move規則。

map接Item變新Item，filter借&Item作bool、保留仍原Item；原Item已&i32時filter是&&i32，map則&i32。collect由目標FromIterator決定產物，沒有自動把&i32變i32。fold只要累計狀態，不必建中間Vec；空保持初值。Result collect採首Err短路、不交部分成功、不收全部錯誤，且保持原錯型別，和?的From轉型不同。

chars交scalar值，lines交原文&str，兩種collect不等於同樣owned。LF／CRLF行尾移除、末行尾不多一行、單CR保留、空行可過濾，is_empty不trim。Entry接候選鍵並借改map，and_modify／or_insert／or_insert_with分別依已有或缺鍵工作；值callback延後不等於候選String鍵也沒建。map.iter_mut只借改值、鍵仍shared，任意走訪序沒有改變。

Ordering三態／PartialOrd不可比／Ord一致完整比較的角色已可分。stable只保比較Equal項的來源序，來源map本來任意，單按count保序仍不能固定報表；須count降序且同count按名稱byte字典升序。比較器借tuple，owned鍵由map移到Vec，排序不必clone文字。unstable不承諾Equal原序、非故意每次打亂；不另heap暫存的敘述只針對排序工作。

## 圖片與自己的小變化

只依當包meta親看三張PNG，沒有提前開其他圖：

- 193 closure-transfer：text→take在建立時move capture，take→saved在呼叫時body移出；同一文字、兩時點、無clone。
- 196 iterator-next：同一Countdown四次remaining 3→2→1→0→0，Item Some3／2／1／None；不是每次建立新來源，for遇第一None停。
- 197 adaptor-pipeline：Item &i32→filter callback &&i32→保留&i32→map回i32→collect Vec[0,30,30]，底註collect反覆next帶動前面；箭頭不是每站完整中間清單的建立時序。

records有下列可辨識的小變化，均未執行，不是同頁答案的盲解：Count別名101仍合法i32而需另驗證；LimitError即使99/100仍會照fmt說超過，顯示不驗證；+101解析成功但Count範圍失敗，前空白+3屬Integer錯；count_pair(101,abc)先回Range。take body若改text.clone可重複另建文字，但原String仍先move入closure；給capture closure加move不會變無環境pointer。

也有回傳契約／Item的短推論：fixed_reader +use<>後可修改原String再call獨立工作，precise_reader仍有真實borrow則不行；sum_source的Item=i32不接受&Vec交&i32；parse_all只解析i32，所以101可成功，不套Count驗證；bad移第一筆visits1，不回滾計數。空格行不是空行；Rust CRLF與單CR結尾可成不同名稱。Rust與Rust加空白同count時按前綴短者先，stable單比count不足以補此規則。這些推論在各record以自身預測標記，沒有假作作者逐字教法或編譯結果。

## 尚未知與停止狀態

我能追到本輪具體例子的規則與責任，但沒有因此學完panic內部行為、所有標準轉換／borrow實作、任意RPIT泛型capture清單、所有closure能力判定、Iterator各種實作、Unicode完整字形處理、排序底層算法或浮點NaN排序政策。這些未知不拿外部Rust知識補過；第10以後也沒有真實正文前綴。

本報告只保存普通完整頁閱讀的事實與當下理解，不是第8／9章逐段首次必要使用驗收、全書品質判定或程式建置／測試通過聲明。兩處不當預先命名已另列且保留原文。已STOP，等待第10以後實際freeze並另行合法釋出。
