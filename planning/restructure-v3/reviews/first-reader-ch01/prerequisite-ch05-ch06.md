# 第5–6章實際必要前文閱讀報告

## 路線與實際範圍

我按合法append的同一feed依序讀了**150–160，共11個完整頁**，每頁保存record後才取下一頁。160之後feed明確回覆STOP。這次是 **ordinary prerequisite page reading; not staged first-use evidence**，用於將來第7章逐段首讀的實際先備，沒有把第5／6章補稱逐段盲讀。

|單位|實際完整頁|已讀的必要內容|
|---|---|---|
|150|5.1 泛型|Holder、泛型函式／impl、turbofish、Pair同型與雙型參數、完整例與答案|
|151|5.2 Option／Result|標準enum與prelude、有值零／缺值、錯誤資料、完整呼叫者和答案|
|152|5.3 char／UTF-8|byte／scalar／字形、文字get與直接索引、必要圖、分檔答案與四測試|
|153|5.4 Vec|新增／查詢／移除／pop／借改、各自失敗契約、完整main與答案|
|154|5.5 Vec ownership|String移入取回、元素借用衝突、非負整理、必要摺疊全部text/list source與六測試、上限練習|
|155|6.1 trait／impl／bound|Charge與兩種資料、generic要求、trait scope、完整module／測試及答案|
|156|6.2 Debug／PartialEq／derive|整筆顯示比較、文字／reference／Option的標準規則、完整程式與答案|
|157|6.3 Clone／Copy|String與含參照容器的Clone、自訂數值Copy、String不能Copy、完整例與答案|
|158|6.4 associated type|Extract／Output、泛型實作、輸出相等要求、owned取回、完整source／測試及答案|
|159|6.5 HashMap|Eq／Hash／Borrow、insert／get／get_mut、query與結果的借用來源、必要圖、完整例與答案|
|160|6.6 分組與owned鍵|借清單計數、狀態表、完整三測試、自訂雙欄鍵、庫存數量合計答案|

所有原位置完整source、失敗例、測試、manifest與154摺疊完整text/list均已讀；摺疊source屬必要教材。練習和答案同頁可見，沒有宣稱我作過隔離解題。records只保存當下真正讀到與恢復的規則、合理小變化和未知，不拿這些完整頁資料當首次使用逐題驗收。

我沒有跑cargo或其他教學命令、安裝工具、看repo章節／queue／作者ledger／其他review／外部教材；沒有追第7章連結。沒有新的未釋出內容曝光。先前報告和初判records未改寫。

## 現在真正有的操作先備

泛型的型別參數與值參數是不同輸入；同一T在多個位置維持同一型別關係，兩個欄位要可不同型別需兩個參數。`impl<T>`是先宣告，`Holder<T>`才使用；Self依目前實作型別。generic body不能只憑未知T就乘法或顯示。155的trait bound才讓我實際讀到怎樣寫需要的能力；158的associated type則保留實作者與輸出型別的關係，`Output=i32`要求已有輸出是i32，不是幫呼叫改型別。

Option與Result現在是實際讀過的標準enum：Some(0)和None不同，Ok(0)和Err不同。Err是交回的資料，非建立它就panic或結束程式。呼叫者需決定如何處理；prelude只讓本頁列出的常用名字可直接用，不代表一切標準型別都自動匯入。151尚未直接比較整份Option，156才實際讀到T具相應能力時Option可Debug／PartialEq，含reference時比較被借值。

文字方面，我知道str.len數UTF-8 bytes，char是scalar，完整顯示字形又可含多scalar。&str.get(range)按byte分界，成功借&str、失敗None，不另建String。合法空段是Some空文字，scalar內空段仍無效；直接range索引的相同錯誤會panic。selected_byte_len交回owned usize，不是我已經會寫借用回傳簽名。

Vec是owner且長度不入型別，整個Vec不Copy，即使元素i32。push移入／Copy元素，get借單項、pop交owned末項或None、remove交owned指定項且越界panic；不能把所有查取契約統一成Option。as_slice借讀、as_mut_slice借改既有元素但不能增長。push可能換存放區，借出元素尚待使用時不能跨著push，不能靠猜這次不會重新配置。非負整理函式借輸入建新結果，保持順序、重複和零，負值略過，原輸入不變。

trait是共同操作契約，不合併資料型別，也不保證商業算法正確；泛型使用操作前要有bound。module外點號呼叫自訂trait方法要把trait引到scope。Debug是檢查表示，非固定輸出協定；derive PartialEq按全部欄位、enum先variant。assert_eq!整筆比較需要相等判斷和失敗顯示能力，比較與格式化此處都借讀，非一般指派也因此不move。

Clone要看型別契約。String clone建立自己的文字；Vec<T> clone另建容器並clone元素，所以Vec<&str> clone後仍借同一文字，沒有自動變成owned文字或延長來源生命。Copy允許原本move動作隱含複製，不能只看stack／heap判定；自訂struct不自動Copy，derive需每欄Copy且型別有Clone。String有Clone而無Copy。

HashMap依鍵身分查值，不保證插入順序。Eq／Hash須配合，相同hash仍可能碰撞；不同鍵也可同hash，所以仍需相等確認。String鍵可用&str查，依Borrow<str>的身分一致契約。insert替換值且回舊值，不會自動相加；get借讀，get_mut借改既有值，缺鍵不新增。分組在None時clone owned鍵，在Some時累加，回傳map不借輸入，內層輸入結束後結果仍可用。自訂GroupKey兩欄一起derive相等與Hash，不能只任改其中一個契約。

## 兩張必要圖的實際閱讀

152已親看當包UTF-8 PNG，不只讀alt。三色塊分別對應A的0..1、中的1..4、crab的4..8與各byte位置；標題列合法分界0、1、4、8。下方綠框1..4包含完整中，紅框1..2終點2切在scalar內，無效。圖未教按顯示字形切分或用整數取第幾個char。

159已親看當包map-query PNG。query:String在上，查詢視圖&str以藍實線向上借query文字；綠虛線向下表示按內容查counts。found:Some在下，以另一藍實線向上指map中的usize值2。found沒有箭頭指query。圖是查找當下；正文接著說明暫時query借用結束後可改query，而found仍借map。這使我能區分「查詢用的文字」與「結果的借用來源」。

## 保留的推測與尚未知

records有紙上小變化，沒有假稱執行驗證。例如151若main多加成功材料5，checked_amount可成功，但原本只配0材料的`assert_eq!(value,0)`就不適用；152的4..4應是Some空文字，而4..1端點雖各為合法分界，順序反向仍None；154的ceiling=0保留三個非負輸入會得到三個零，不是去重。159改query不會讓found重新查找；160新名稱quantity=0仍建立Some(&0)，不等於未見名稱None。

這些頁面部分解開了早期真實未知：泛型能力要求、UTF-8 range細節、整份自訂資料與Option的比較、以及自訂Copy，現在都有實際讀到的依據。早期原答案仍保留原狀，沒有用這次內容倒填成當時已懂。

我仍不知道一般自訂函式如何在簽名寫出借用回傳的來源關係、lifetime annotation語法、任意複雜借用／綁定模式分析、trait物件或一般動態調度、所有generic／associated type設計取捨、手寫Hash／Eq／Borrow的完整做法、各容器完整接口與配置策略、scalar／字形迭代或完整Unicode處理。不能用標準get的已述契約，冒稱我已會自行宣告那種借用回傳函式。第7章只有最後連結與預告可見，正文未讀。

此報告只確立實際先備讀取範圍，不是第5／6章逐概念首用通過判定、全書品質判定或建置成功聲明。已STOP，等待第7章另行合法釋出。
