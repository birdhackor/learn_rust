# 第5章有提示的修後局部複查

日期：2026-10-14。AI輔助讀者模擬，沿已封存的35單位初判及其已報問題回讀；不是新盲讀，不改ch05-first-reading-v1.md。僅按root指定讀ch05-review-fixed-v7受影響單位，原closed答案仍預先遮蔽，未實跑或讀作者／技術／外部資料。

來源：`/workspace/work/tutorial-implementation/frozen/ch05-review-fixed-v7`。reading-order SHA-256：`de9c5f94af5b474485815eef2c4e09240003903115416a835661855fecbbc47c`。本輪選取其第4、11、17、18、28、29、31位置的001-004、003-002、004-002、004-003、005-007、006-001、006-003，不讀其他正文／答案。

## 001-004｜借用數字先dereference

SHA-256：`20f77e14f23ae2b94185f83fb553b74f4b9184c1478f881e5882092e6f2c4f01`

引文：`Ok(count) => println!("待取的件數={}", *count)`；`println!("被拒絕的件數={}", *value)`。

- Q1：仍是先報錯誤數字再unwrap觀察panic，借&result的policy不變。
- Q2a：pattern取&i32後先*讀出Copy數字，再用已教{}；不需要在5.1先假設ref的Display規則。
- Q2b：原需要是失敗前看被拒絕輸入，不是增加驗證或轉移result。
- Q3：101仍先報，之後unwrap仍panic；借用結束後交出原result，未實跑。
- Q4：001-004原局部契約缺口由已教dereference解除，無新概念负擔。wrapper中error ref部分待003-002實際補文確認；不回改原未知。

## 003-002｜Display的shared reference實作

SHA-256：`19926bf23d0c4ba3be12050c8c8b58536246054529e7e319f2c38f2b43b95c9b`

引文：「當 T 提供 Display，shared reference &T 也能用 {} 顯示所指的值」。

- Q1：新補關係是原資料有Display時，借它的ref也能顯示，為5.5 wrapper借原因的格式化提供前置。
- Q2a：標準庫為reference提供格式化實作，顯示所指值、不取得ownership；與拷資料／move分開。
- Q2b：原需是readonly顯示既有原因並保留分類資料，不是為每個wrapper複製錯誤或改成String。
- Q3：&CountError／&ParseIntError可在write!直接{error}；兩原型別的Display已由正文提供。
- Q4：原wrapper ref格式未知解除；此刻補文在首次該用法前。增加一項短的T→&T能力關係，前置traits／借用已教，沒有新資料或接口。v1初判維持原未知，這是修後理解。

## 004-002｜prelude與轉換trait scope

SHA-256：`5d5b126a5c1c5ebab9bdff241a70fd7e2b528ede6b1f17f8c675af4fad63a9f2`

引文：「edition 2024 會透過 prelude（預先匯入的常用名稱）提供 From、Into 與稍後的 TryFrom」。

- Q1：prelude補的是這些標準名稱如何已在caller scope，不是轉換新算法。
- Q2a：本edition預先提供三trait；明寫use仍追來源路徑，自訂trait沿4.1引入。未推所有std名稱都自動可用。
- Q2b：原需要是追蹤顯式use與後例省use為何都可找轉換方法，保留4.1 scope規則。
- Q3：只引Count的caller也能找TryFrom／Into；結果及Copy／move作用不變。
- Q4：原scope必要未知解除。新名稱prelude直接以熟悉的引入工作解釋，負擔小，沒有讓自訂trait免use或改ownership。

## 004-003｜私有field限制的主詞收窄

SHA-256：`e468f862509bebf5890d0f0addd90724707f1641db24beb184e833db340f350b`

引文：「library 外的呼叫者能使用 Count，卻不能直接寫 Count { value: -1 }」。

- Q1：Count API的使用者現在明指library外caller，與本章example target分工一致。
- Q2a：pub型別／私有field／pub借讀method機制未變，不再用「所有其他module」概括。
- Q2b：原需仍是外部不能繞過檢查自行給field，接合法Count後可只讀。
- Q3：外部經轉換取得值，value()借讀；Copy／move及內部須維持規則的背景不變。
- Q4：主詞範圍更精確、沒有新增負擔；不以此句聲稱每個不同module的所有存取關係已教，亦不把私有field當自動驗證。

## 005-007｜for tuple與名稱pattern的首用補文

SHA-256：`828947b1052e854cf8b9edf5b34b835690a5bd1e80bbfbfd3ceae2950ecf37a0`

引文：「for 的名稱位置也能放 1.7 的 tuple pattern」。 「和 _ 略過而不保存值不同」。

- Q1：新增位置說明服務配對測試，以及報告非預期的整個Result。
- Q2a：for每次拆tuple兩項在當次body命名；末arm other接先前未匹配全值并綁名字，_不保存。前文解構／pattern知識有明確連接。
- Q2b：原需是每組文字對應預期數字，以及看見任何偏離Integer預期的完整結果，不是新增解析政策。
- Q3：測試材料及結果未改，現在不需只靠code推其位置規則；答案header仍closed，內容已預先遮蔽，未讀／展開。
- Q4：原optional推論負擔解除。只多兩個已用形式的解說，沒有新data或新檢查規則，不擴稱所有pattern位置皆相同。

## 006-001｜指定位置核對

SHA-256：`d49c9114e88403f5c540f444102267eac3e13499b00a8d19d34e72c20b766cc9`

引文：「這個決定仍放在呼叫者」。

- Q1：本單仍是5.6開場，定義簡化傳遞而保caller政策的需要。
- Q2a：未新增機制文字；hash亦與原tail的006-001相同。
- Q2b：原需要與兩步檢查固定的對照目的未變。
- Q3：這個指定位置沒有return的章節回讀句，不能用它確認「1.5→1.6」修正。
- Q4：原optional定位問題其實記在006-002（reading-order第30）；目前未開未指定的006-002，已向root回報此定位差異，先不宣稱該問題解除。不影響既有return機制背景。

## 006-003｜同型錯誤的標準From

SHA-256：`16f62326b8e17a8a9f969768ea3d4e81a75ce456d88bc64683c68244af60cf4a`

引文：「標準庫已為每個 T 提供從 T 到同一個 T 的 From，原樣交回收到的值」。

- Q1：新增的T→T標準實作補上同E傳遞如何滿足?條件，與兩種異型來源的自訂封裝分開。
- Q2a：E與目標E相同時，已有From可原樣交接ownership；不需另寫轉換，也不複製或改錯誤原因。
- Q2b：原count_pair要依序取兩Count、首個失敗回同ParseCountError，需要合法的同E傳遞，不是創新錯誤分類。
- Q3：兩個parse_count的Err可經?回count_pair；原成功／早退預測不變，完整From條件現在可只依正文解釋。
- Q4：原必要未知解除；只增加一個標準保證，不擴成異型轉換一律可用。此輪未重看SVG／整頁或展開答案，檢查的是新增文字關係，圖的原初見仍保留。

### root更正指定位置後的補讀時點

006-003實際讀取後收到root更正：先前006-001指定錯誤，需另補006-002核return位置；開場不算該修法已核。本輪因此新增reading-order第30的已讀相鄰單位006-002，接在006-003之後回讀，不倒填為原指定正確。

## 006-002｜更正後補核return前文位置

SHA-256：`af9691289ac9eddd04217669d2d76a99c4ee0820c848c21797b8b0c41627c7fc`

引文：「return 的作用與第 1.6 節相同」。

- Q1：仍是手動兩次整函式早退，這次實際核到所報前文指涉。
- Q2a：return不只離開match／block的規則未變，修後位置1.6符合已讀正文。
- Q2b：需要可靠回讀定位，不是改helper的失敗政策。
- Q3：成功值／From封裝／後步跳過均同原例，未實跑。
- Q4：optional定位問題在這個實際補讀單位解除；先前006-001不能算驗證。只有定位修正，沒有新概念負擔。

## 局部followup封存

已按指定七單位及root後來更正追加的006-002，實際回讀八單位並記各hash。實際順序為001-004→003-002→004-002→004-003→005-007→006-001→006-003→（收到更正後）006-002。未展開005-007答案，未讀其他正文、答案、作者／技術或外部資料，未實跑。原35初判沒有回改。

| 原問題／追加收窄 | 修後核對 |
| --- | --- |
| 數值ref／原錯誤ref的Display關係 | 解除：5.1先明用*數字，5.3在借錯原因首用前教&T顯示所指值。 |
| Into／TryFrom caller scope | 解除：edition 2024 prelude提供三名稱，顯式use仍可追路徑。 |
| same-E ?的From條件 | 解除：明教每T到同T的標準From，原樣交ownership。 |
| for tuple／末arm名稱pattern位置 | 解除：測試前明說兩位置與_的不同。 |
| return的前文位置 | 解除：更正指定後才實核006-002指1.6，不把006-001算修法核過。 |
| 私有field限制主詞過廣 | 收窄為library外caller，符合範例使用者，不擴稱所有其他module。 |

未見此局部修訂引入新必要缺口：增加的ref Display、prelude、identity From及pattern位置均在首用前就地解釋，原報告／轉換／傳遞需要未變。這是有提示修後複查，不是新盲讀，也不回填原35初判。以上補教現在可作第6章進入背景；接下來才開始第6章唯一稿的真正順序首讀。
