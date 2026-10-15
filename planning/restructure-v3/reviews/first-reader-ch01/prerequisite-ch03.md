# 第3章實際必要前文閱讀

098–105八份完整頁已依序讀完、保存各頁簡短紀錄後遇到明確STOP。這次是ordinary prerequisite page reading：包括同頁完整來源與練習答案，不是第3章逐段盲讀，也不作逐概念首次使用四題的證據。第1／2章原報告與000–097紀錄未修改。

## 實讀範圍

| 單位 | 真正讀到的完整頁 |
| --- | --- |
| 098 | 3.1 struct、實例／欄位、mut、跨module公開型別／欄位、positional格式佔位 |
| 099 | 3.2 ownership、move／Copy、值參數接手、mut owned參數修改與交回 |
| 100 | 3.3 shared reference、借讀參數、解參照與欄位自動跟隨 |
| 101 | 3.4 mutable reference、&mut權限與binding前mut的差別 |
| 102 | 3.5 scope／lifetime／last use、衝突例、reborrow、reference不能延長原資料 |
| 103 | 3.6 impl、三種self receiver、associated function、Self與欄位簡寫 |
| 104 | 3.7 shared／mutable slice、起點長度／局部索引、bounds、borrowed逐項加總／修改 |
| 105 | 3.8 String／str／&str、UTF-8概念、追加／視圖／move、完整LabelledOrder模型 |

每頁的成功程式、預期編譯／執行失敗來源、練習完整答案與命令說明均讀了；沒有實際建立教學檔、執行、安裝或驗證任何工具。只使用指定feed及本頁metadata的view_image，未看repo來源、queue、作者ledger、其他review或外部教材。跨語言選讀只看見連結和非先備的說明，未追連結；未讀第4章正文。

## 已恢復、可作第4章實際前文的操作規則

- struct先定型別再建實例；欄位按名稱對應、可點讀，mut binding可改欄位。型別與欄位各有pub邊界。已能使用{}接後方expression顯示數字／文字。
- ownership是使用與清理責任，不是物理bytes一定搬動。此Order／String移交後原名稱不可用，值參數及owned self可接手；數字／bool等已讀Copy型別可保留獨立值，自訂struct不因數字欄位就自動Copy。
- &借讀不接手owner，*可沿reference讀值；i32讀出可Copy。&mut借改與reference binding自身的mut分開；借用仍在使用時不能有衝突存取。
- 在本頁的直線局部例子，borrow可在最後一次使用後結束，名字scope仍較長。兩個shared可同時讀，重疊的排他借用／尚待使用的shared與修改會衝突。reference不延長inside的存在。
- 已實讀&mut *parent的短reborrow，以及參數已明定&mut i32的呼叫位置自動reborrow；不能將此推成&mut reference有Copy，let next=parent仍會move該reference。
- method的&self／&mut self／self分別借讀／借改／接手；點呼叫由接收者提供self。Order::new是自訂associated function，不是自動產生。&self不能搬出非Copy整筆值；owned self能交出其owned欄位。
- &[i32]是帶起點／長度的借用連續段，不是新的元素array。slice索引從自己的0起，自己的len決定界線；切片須start<=end及end<=原長度，合法空段可在結尾。shared slice逐項for得到&i32，mutable slice得到&mut i32；已實讀明寫*讀／改原元素。借用視圖不取得原array的owner。
- str是UTF-8文字slice；&str帶位置與byte長度，普通literal是靜態資料的&str。String持有文字資源，最後owner負責清理；空String可尚無heap配置。push_str借讀來源並追加內容，as_str借出目前文字視圖，不造另一份String；視圖後面還用時不能先衝突追加。
- LabelledOrder把owned String移入欄位、以&self算小計／&mut self追加或改數量、再以self接手並交出String。這些完整來源已讀，非僅看章節摘要。

## 必要圖片均實際查看

| 單位／PNG | 親見的必要對應 |
| --- | --- |
| 099 ch03-move-copy.png | first失去使用權、實線責任轉交到second；下半count3與another_count3→4為獨立數值 |
| 100 ch03-reference-owner.png | reference:&Order借用箭頭指同一筆、order仍owner；無第二份Order |
| 102 ch03-borrow-scope.png | 藍線借用到最後讀，灰線名稱scope延到main末；圖只示shared／owner，正文另有中間mutable步驟 |
| 104 ch03-slice-part.png | 原索引1–3／120、200、60被同框選中，局部slice索引0–2；上界4不含 |
| 105 ch03-string-view.png | String owner向同一heap文字作管理箭頭，show的&str向同一內容作借用箭頭；責任不同 |

每張在所屬頁保存／下一頁釋出前以view_image親看，不靠alt代替。沒有缺失或不可讀標籤；不由示意箭頭推斷物理地址／固定bytes布局。其餘三頁無必要PNG。

## 尚未知與證據界線

這些頁沒有實作一般自訂函式「回傳借來reference」的簽名關係；雖已使用as_str取得視圖，不因此聲稱會寫所有回傳borrow的函式。複雜分支／迴圈lifetime分析與命名lifetime標記未教。如何替自訂型別提供Copy明確延到第6章；整筆自訂型別的通用格式化支援也未實作。

文字目前只整段顯示／追加；byte、char、UTF-8切字界線明確留第5.3節。String容量／配置細節未學，不由heap示意圖自行補。尚未讀enum與依形態拆開處理的新語法，只見下一章預告。沒有用任何未釋出內容反過來補本次前文。

狀態：STOP已到，必要前文實讀完成。等待root釋出第4章分段主文；屆時恢復必要新概念的四題／自身預測／初判先存後取，不把本次第3章整頁補讀宣稱成逐段首次使用驗收。
