# 第 4 章銜接交叉與 v7 局部修後核對

這不是新的 blind 首讀／獨立來源初判。第4章v5原初判與SHA側檔保留不變：ch04-continuity-v5.md SHA-256 d7e6309342f17f7b88d5073299f044559e8715ccc69eb916cfc78639cf4b63c9；側檔 SHA-256 0e221fea419325b107cdb95e3ea8aecba9d1513dd668e1c8b33e9b6bd9224411。

本輪先讀協調者指定四份同行／處置紀錄，工具初次輸出截斷部分已分段補讀；之後才核ch04-review-fixed-v7五單位與指定source／SVG。現在已受同行N1和修正方式提示，不把下列新增核對回算為本人原初判重現，也不把v7說明補回v5。必要前文版本實況沿原initial（完整原讀1～3＋指定局部修後，不假裝整章全新版重讀）。只做局部，不開後章／live稿／其他作者notes，不重跑全套。

## 交叉配對：來源、分級與範圍

| 本人v5初判 | 同行已封存初判 | 本輪處理 |
| --- | --- | --- |
| C1兩位置，選讀改善 | technical I1兩位置；first-reader的I2是GroupKey位置，4.2已能分辨count材料作用而未另報此處問題 | 同缺少的例子歸因關係，分級均可選；核v7的4.2窄化與4.3新增材料，不把兩位置混成一種修法 |
| C2圖線避字，選讀改善 | first-reader I1後到v5 crop實看；technical原未看渲染、cross只核source | 本人原initial親看自己以v5 SVG渲染的native圖，是獨立同關係發現；本輪仍需自己核v7圖，不能只引用同行crop替實看 |
| C3文字比較限定，選讀改善 | first-reader I3、technical I2 | 皆保主選來源／長度檢查已可完成；核局部文字明確承認4.2已教能力 |
| 未提N1；本人原003-005／core-source列必要依賴足夠 | first-reader N1、technical N1獨立報let counts;延後首次初始化，分級必要／增加理解負擔 | 本輪受提示新增核查。本人原單位沒有保存這項宣告形式的機制與無mut前提，不能以當時看懂scope意圖當已介紹證據；承認漏核此局部依賴，保留原「無必要缺口」封存，不回填N1命中 |

N1不把主get_mut／缺鍵Clone／owned鍵用途推翻；已教部分仍保留。同行處置提供兩種修法：block expression可避新形式，root選擇保留形式並在首次程式前補教；本輪只核後者的最小關係與新增負擔，不必猜未見技術答案。

交叉檔案的SHA-256（此時讀取版本）：
- ch04-first-reading-v1.md：0f3425336cf7beaaa3bdbae85f27c38b951e0e7a0c67b8e9c955bf4c45fb4222。
- ch04-technical-v5.md：2464e1c1c8ac4225067bd1593ca86fdd56383dd980e8c020cb829d0f0b21cebd。
- ch04-technical-followup.md：5c036e23bfd0e3c62ba0a5b05ca0ec5c5fe3bc53f8b1dbdc1872a20a0efbfb7c。
- ch04-disposition.md：2480e47f4a4e450c7e50d87bc6b7c9b06f997ce698e8e9965badd8442658ccad。

## 修後局部核對

### 002-003 C1a：文字收斂到固定金額下的數量差別

- Q1：PartialEq／derived全欄、enum同形態後比資料、整體assert能力與比較借用不變；不是新型別或新比較契約。
- Q2 機制：第70行改「顯示相同金額下，數量不同仍是不相等的摘要」，吻合360／3對360／4。需要：完整摘要不能只看金額，這組直接證明count的作用，全欄一般規則仍由第9行derived說明支持。
- Q3：真／假／假／假四輸出與操作不改，archived equality.rs與v5 bytes相同；source SHA-256：c3a730043d5774e5462a5cfd54fedc8332f0fcdb44a5e63ef3ec80cae101ce0b（已由程式計算）。
- Q4：措辭改動修復本人C1這一位置的證據歸因，沒有弱化完整欄位契約／新增例子或先備。局部修後足夠；本人、technical原可選分級保留，first-reader此處未報問題也不補計為其命中。

修後unit SHA-256：ad28bc288c3488938a6ea43594ec0af0c93c525a31625a00ccafdca0fb65a92b。

### 003-003 C2：查找線繞開標題、保留兩來源

- Q1：query String／查詢&str、counts鍵String／值usize2、found Option<&usize>的五角色與v5相同；正文除凍結資產路徑外不變。
- Q2 機制：兩藍實線仍指各自被借來源；綠虛線從query.as_str框底向下，轉至map左外側，再向右進鍵框。需要：保留查找與借值關係，同時讀counts標題不受線穿字干擾。
- Q3：已讀v7原SVG並以其bytes渲染native400×730、親看此圖。新路徑M112,265→112,335→8,335→8,448→40,448繞過標題y395；箭頭仍指Rust鍵而不是值2，兩借用線、found、圖例清楚。
- Q4：本人的C2在這張同版native圖已改善，沒有新增owner／新概念，也未因線轉彎需猜關係。本人此輪的圖實看不是借同行crop冒充；未看完整頁面／手機布局，所以只判局部圖，不判實頁避字或發布QA。

SVG SHA-256：75f1b7da233283b4a81ba86a4e8f26533b077893171de262c729f12725ac015c；同v7渲染PNG：4d31945cdb7d5f3f7b07f205d09aa7bcb8847abb0e91cd006ec83d63cea4cfb7。

修後unit SHA-256：15a166d61e56ff583efe33a70bba0404e4b7021ad0d1d621e463c255207ac179。

### 003-005 受提示N1：宣告／首次初始化前置

- Q1：第85行先解釋let counts;是外層名稱宣告，尚未給map初值，內層counts=...是初始化；不等同已建空map，也不是shadowing新counts。
- Q2 機制：第一次給值不是修改既有值，無需mut；型別由後面初值推導，讀取前Rust檢查已初始化。需要：先讓外層保存owned結果，再結束內層labels，scope測試才能實際展示結果不借輸入；保留這種形式就必須先教這些操作規則，不能只憑測試意圖猜合法性。
- Q3：前置段在第85行，首次測試code的let counts;第114行、初始化117行、外讀119行。source counts.rs:55／58／60仍同操作，回傳HashMap<String,usize>移入外層counts；函式缺鍵Clone建owned String已教，labels結束不結束map鍵。外層名稱本身不能延長任意borrowed來源，這裡同時依owned輸出成立。
- Q4：補文回答宣告與給值分開、無mut、型別、讀前有效四項，不改主get_mut分組／結果／source；保留新宣告形式增加一項語法，但在當下用前完整說明，沒有新增隱藏API。本人受first／tech N1提示後才明確核此，認同其必要局部關係已補；不改v5原無必要初判、不補算N1重現。source bytes相同的證據在下面另程式核對，未實跑。

已教部分保留：查詢比較、缺鍵Clone、map owned鍵與三組測試目的均維持；本輪不以未讀第5章的沿用作本章前置充分性的理由。

修後unit SHA-256：91024462cbc73ddb1d1d3db86f8334146338ba5fb9e13afb9dcd89e0501fc705。

### 003-006 C1b：新增只改region的可見對照

- Q1：新增different_region=(2,2)，舊same=(1,2)／different=(1,3)及同型別借用鍵、derived Eq／Hash皆不變。
- Q2 機制：以已存(1,2)為基準，單改category或region各成不同鍵、get回None；需要：兩欄共同身份的原主張有各自可見材料，不再從只變category的結果代稱兩欄均驗。
- Q3：第39～42行建新鍵、51～54行查與顯示、57行assert None；預期三行3／類別3未收錄／地區2類別2未收錄。archived custom_keys.rs:26～29／38～44有同樣新增，已親讀source對應。
- Q4：root選擇加同形材料而不是本人原提的窄化文字，兩者可各有收益；此新增只用已教struct／get／Option／println／assert，負擔限一筆鍵與一組查詢，能直接看region作用。C1b局部修後足夠，但不推出hash不碰撞／任意手寫鍵契約皆正確；本人未實跑，同行root執行只算其證據。

修後unit SHA-256：77af1428e5eea5fe46ee5d0984b45ab455729e449869d039ab9926a71d706a50。

### 004-009 C3：承認已教內容比較、保留局部焦點

- Q1：first_nonempty兩個可能&str來源與共同lifetime、空第一才選第二的規則不變；新第7行不把4.2已教能力退回未知狀態。
- Q2 機制：改成「本練習用數值assertion檢查長度…先聚焦可能的借用來源；4.2已教的文字內容比較也能用來檢查結果」。需要：清楚區分本題選哪種驗證方式與讀者已有哪種能力，保留借回來源任務，不新增比較trait／API。
- Q3：("",Rust)→Rust、(Rust,Python)→Rust、雙空len0仍可用原長度／顯示路線；若用文字相等也有002-004的&str內容PartialEq前提，答案source無需改。
- Q4：本人的C3與first I3／tech I2的最小前文指涉已修，不需猜已教相等目前能否用。內容相等只核值，不單獨證明owner或借用來源身份；該來源依規則／簽名／scope。無新增必要負擔，局部修後足夠。

修後unit SHA-256：e9651b73b68e741a166c68434a35d554fae8e45d4f1bc5a24fad9f66a3b20e7f。

## Source影響與核對邊界

本輪對v7全部56 archived sources只作bytes版本比對（有改的檔案與counts.rs另親讀）。只有custom_keys.rs新增region材料、map-query.svg改綠查找線；其餘54檔與本人已讀v5相同，不能把指紋一致稱成本輪重跑／重新逐段閱讀全部sources。

N1的counts.rs與原v5 bytes相同，修的是使用前說明：外層先宣告、內層首次初始化，之後仍從外層map查Some(&2)。這維持owned鍵與原輸入生命分開的測試意義，不改分組算法、key Copy／Clone、測試材料、數量或輸出。new paragraph在第一次let counts; code前，沒有把「改說法」誤當「修了Rust計算bug」。equality.rs亦不變；C1a收斂結果文字，不收窄derived的一般契約。C1b的new instance／lookup與正文程式／預期輸出一致，都是已教API。map-query assets副本與v7 archived SVG bytes相同。

指定source／圖指紋：
- sources/examples/ch04-traits/src/counts.rs：f5d6044ec8110498d47689d27953e6014722bc604747a5509fa32982f0661283。
- sources/examples/ch04-traits/examples/equality.rs：c3a730043d5774e5462a5cfd54fedc8332f0fcdb44a5e63ef3ec80cae101ce0b。
- sources/examples/ch04-traits/examples/custom_keys.rs：3b8ff510cbeee0f954659ea3bb98b8b33c3eb907a905d99c95fc1880008dd416。
- assets/map-query.svg：75f1b7da233283b4a81ba86a4e8f26533b077893171de262c729f12725ac015c。
- sources/docs/assets/ch04/map-query.svg：75f1b7da233283b4a81ba86a4e8f26533b077893171de262c729f12725ac015c。

## 交叉主要通過項與封存

同一來源支持的主要關係與first／technical紀錄相容：4.1的異資料amount／共同門檻、4.2的逐欄能力與Clone型別層次、4.3的Eq／Hash／Borrow與兩借用來源／缺鍵Clone、4.4共同候選與單來源／owned／placeholder／outlives、4.5receiver與欄位來源及兩欄各自輸出，都沒有在此次局部改動引入新必需先備。這是原initial與同行的交叉支持範圍，不把讀同行的技術來源替正文補教，也不把未回讀的其餘33個v7正文當本輪全章重驗。

結果：本人C1兩位置、C2、C3均在v7指定範圍改善／完成；同行必要N1的前置最小關係也已補齊。N1是受提示新增核查、原initial漏核的操作前提，不算本角色獨立重現；原無必要初判／原可選分級／SHA不變。沒有新局部必要問題。

本人這輪未執行cargo／rustc／fixtures或全套測試，未親看完整手機／桌面頁面；只親看以v7 SVG自身渲染的native400×730圖。root與first-reader既有實跑／crop證據屬他們，本輪不冒充。未核第5／6實際正文、更未首讀第7章；後續需先由root交付實際背景再開始，不提前開章。

完成後暫停。此follow-up與其SHA側檔另存，v5 initial／側檔與第1～3章紀錄均不改。
