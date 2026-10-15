# 第4章逐段首讀紀錄報告

## 實際讀取範圍與路線

我依同一個 staged feed 讀完 **106–149，共44個H2自然單位**；每單位先保存初判，再取下一份。`records/106.md`至`records/149.md`已保存。149之後再次呼叫feed，收到明確的：

> STOP: no more frozen material has been released. Save current state and report to root; do not search later files.

|實際節次|單位|這一段讀到的主題|
|---|---|---|
|4.1|106–110|enum、variant與三種payload形式、建構值|
|4.2|111–115|match、pattern、綁定、窮盡與分支結果|
|4.3|116–122|tuple／array／struct／參數／for／巢狀解構，可反駁與不可反駁模式|
|4.4|123–129|or pattern、範圍模式、guard、遮蔽與@|
|4.5|130–137|模式中的Copy／move／借用、部分移動、自動借用、ref與&、2024 edition限制、_與_name|
|4.6|138–142|if let、批次helper、while let與終止條件|
|4.7|143–149|let else、離開延續路徑、return／continue／break、章末整合與練習|

背景是基本Python，加上我真正讀過的第1、2章逐段教材，以及098–105的第3章完整頁先備閱讀。第3章不是逐段首用盲讀；我沒有把那次整頁閱讀改稱逐概念首次使用證據。

本章沒有必要PNG。原位置的完整source與141的state table都有讀，沒有把它們當成選讀。練習與答案若在同一packet中可見，紀錄明說這個條件；我的小變化預測不冒稱外部隔離的未提示解題。我沒有執行教學、安裝工具、查repo／queue／作者ledger／其他review／外部教材，也沒有追進第5章。沒有新增的未釋出內容曝光。

## 我現在能用自己的話說清楚的部分

我把enum理解成一個型別有多種互斥情況，一個值當下屬於其中一種variant；struct則是同一個值同時有那些欄位。variant可沒有payload、帶位置式payload，或帶具名payload。`Amount(0)`仍是Amount，不會因數字為0自動變Missing或Rejected；把不同情況在統計函式裡都映成0，也不會使原本的variant失去區別。

match讓我先確定形狀，再在成功的arm裡取得資料。pattern裡的新名字是綁定，不是拿外部同名值作比較。從前到後第一個成功arm決定動作；guard是在形狀成功後再判斷。窮盡與arm結果型別相容是兩個問題：補足所有variant不會自動修好i32／bool結果不一致；有guard的Amount也不能代替所有Amount的保底處理。

我能把已讀的解構寫法連起來：tuple依位置、struct依欄位名稱、巢狀pattern依資料形狀走入payload。忽略部分資料不會改造原值的型別或tuple長度。普通let、參數和for需要不會失敗的pattern；match、if let與let else各有處理失敗的路徑。因此`(price, count)`對任意二元i32 tuple可用，`(0, count)`則可能失敗，不能直接當普通let的通用綁定。

模式裡的名字拿到什麼，要一起看被檢查的值、那一部分的型別，以及綁定模式。擁有的String欄位可被移走；i32欄位可Copy；對`&QuoteText`作教材中的自動配對時，Label裡的名字拿到`&String`；對`&mut QuoteText`則拿到`&mut String`。這不是「match都會move」或「名字前沒有&就一定擁有String」。部分移走String後，另一個i32欄位仍可讀，但不能再把缺欄位的整個struct借出去。

我能區分expression中的`&`、pattern中的`&`、以及`ref`：前者建立借用，pattern的`&`配對既有參照層，`ref`指定綁定借用某部分。134讓我看見顯式寫法；135才教我2024 edition下，已進入自動借用模式時不能任意再加ref／ref mut／mut或混入內層&。我現在只對教材展示的自動與完整顯式寫法有具體把握，沒有據此聲稱能判斷所有組合。

if let是一次嘗試，失敗可以略過body或走else；while let會每輪重新求右式，成功才進body。141中是helper改變remaining，使More最後變Done，不能把終止歸因為while let自動替我減工作量。state table讓我讀到7→4→1→0的三次body，另外還有一次回傳Done的helper呼叫。142的兩個assert是真正執行的前置檢查，所以`work=0, capacity=0`仍會在capacity檢查失敗，不能只憑「沒有工作」跳過它。

let else則讓成功綁定留在宣告後面；失敗路徑必須離開那條延續，不能單純println後繼續往下。return離開函式，continue離開本輪，break離開迴圈，三者不代表後面任何地方都永遠不執行。147把這個差別放在同一個借用slice的例子中；148–149又把enum、巢狀借用、let else、普通早退與費用計算組合起來。我能依提供的函式追到結果，並看到反覆借用後原值仍可再次使用。

## 首用時的未知與後續釐清

這些不是用後文倒填的通過判斷；當時的未知仍保留在各份record，後續解釋另外寫：

- 106–110只知道如何表達和建構不同情況，如何取payload到113才具體讀到。109的輸出只是固定字串；我當時預測只改建構數字，不會自動改那行輸出。
- 108的`_name`仍是綁定，112才讀到bare `_`不綁定；136才用String實際區分兩者對所有權的後果。沒有提前把136的解釋算作108已教。
- 116–120已會看解構形狀，但哪些位置允許會失敗的pattern尚未知；121的irrefutable／refutable才解開這個問題。
- 124提到or alternatives需要相同借用模式時，我尚未讀到具體借用綁定語法，沒有把那一小句判為已完全學會；133–135依序補了自動配對、顯式寫法和edition限制。
- 132的部分移動是依欄位型別分別判斷。我的「只解構Copy的count並忽略label，可保留完整owner」是短推論，紀錄沒有冒充作者在該例中逐字展示。
- 137裡push_str那個arm可直接作unit函式的結果，我由相鄰程式結構推得它在這裡產生unit；這是明列的合理推論，不是我已讀過標準函式完整簽名。
- 140先讓我理解helper一次拿多少、先減remaining再回傳；141才展示while let反覆呼叫和完整表格。沒有把完整loop當成140已讀。
- 143先提出成功後繼續、失敗提早離開的需求；144才見到實際提示與bare return；146才命名diverge，147才增加continue／break。145的分支結果理解只依可見return路徑，沒有宣稱一般never type／coercion規則已學會。

## 我實際作出的可辨識小變化預測

各record有當時引文與四題；以下列出能看出我是在追規則，而不只是重述同包答案的例子。這些是紙上閱讀預測，沒有冒稱編譯或執行驗證：

- 118完整main的最後一個pattern只綁unit_price；若此時額外讀count，仍會讀到前一個count綁定2，不會自動取得新order裡未綁出的1。
- 119把tuple參數誤寫成兩個獨立實參，不是同一種呼叫；解構也不會把array中的二元tuple攤成四個整數。
- 125把前一個range上界299改成300，300就被第一個arm先接走；301才進下一個arm。
- 127把無guard的Amount fallback移到前面，會先接走所有Amount，即使形狀窮盡，業務分流仍已改變。
- 133若先留住共享參照、插入可變借用，再於後面使用共享參照，會碰到先前學過的借用衝突，而不是因pattern新寫法就免除。
- 135加allow不能讓edition本身不允許的綁定寫法變合法；移掉自動借用情境中多餘的ref仍可取得`&String`。
- 136把被配對主體改成`&quote`，`_text`就不再表示搬走那個擁有的String；名字有無底線不是唯一判準。
- 145若在三個函式結尾加「完成」，Missing在if let版本會走到它，在match／let else內已有return的版本則不會。
- 147把Missing放第一筆，略過版仍累加後面得到200，停止版得到0；把Missing換Amount(0)，兩版都可繼續到200。
- 149 Ready裡若是單價0、數量2，依可見規則會是20，不是empty分支0；單價299、數量1則是319。不能從統計結果0反推原variant。

## 範圍內判斷與仍未知的邊界

這44單位讓我可以恢復各節提出的具體問題，並沿著完整範例說出每個名字當下代表資料、參照或統計結果。enum取資料、先前延後的String模式所有權、單一情況處理、以及成功綁定要在後面繼續用等預告，在本章後段都有實際教到。我沒有留下會阻止我追完這些具體程式的必要圖像缺失或未實現的本章承諾。這是單位層級的閱讀理解，不是整本書品質或程式已通過建置的判定。

我仍未學到任意複雜pattern的完整綁定模式／edition判定表、一般自訂函式回傳借用參照的簽名與lifetime寫法、never type／coercion的完整機制、或所有可能輸入的業務驗證。這些範例中非負小數字契約也不等於全部都有執行期檢查；142的assert則有明確執行效果。第5章常用型別等只有章末預告，沒有正文閱讀或外部補知。這些邊界不拿後來可能的教材預先補過。

本報告另存，先前章1報告、章2報告、第3章先備報告與全部原始records均保留。到此STOP，等待後續合法釋出。
