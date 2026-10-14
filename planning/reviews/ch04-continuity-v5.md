# 第 4 章 v5 獨立銜接審閱：來源初判

角色：未參與第 4 章寫作的獨立銜接審閱者；不是另一角色 first-reader4，也不宣稱第 4 章 blind 首讀／真人測試。沿正文順序先建立必要依賴與「原需 → 改變 → 用途」，保存當下來源判斷後才開後單位；全正文後再核對本凍結 source archives／完整答案／11組 compile-fail fixtures 與 markers。只審、不修改教材，不看 live 修稿、作者 notes／ledger、其他第4章 review／first-reading／disposition 或第5章。

先備版本實況：第1章首三頁原讀v1、尾七頁v2，之後v6指定八單位與v8指定五單位做局部複核；不是整章v8重讀。第2章primary實讀v3全章，之後v5只核對002-006。第3章primary實讀v3全章，之後v6核對004-003／004-004／005-002／006-002／006-003／006-004，並指定補讀單一archive Cargo.toml。其餘前文不假設已看新版。本輪不改任何原始答題／分級／hash。

沿用 clear-tutorial review-protocol/calibration 通用判準。Q1 身分／關係，Q2 分存機制與需要用途，Q3 操作／結果，Q4 示例／圖的支持界線。新必要概念按實際首次使用記，不以標題代覆蓋；限定另查其主詞／對象與當下作用。合理正常推論保留最短前提，不以未教知識補答案。必要／可選／合理待教／未驗證分判。

範圍：ch04-complete-prerequisite-v5 的38正文單位／5頁、對應source archives與完整答案、11 fixtures及expected markers、4 SVG。檔名清單只作定位；未開freeze manifest.json。4圖只在正文原位讀其v5 SVG，必要渲染僅用該SVG位元組；不混live或其他版本。未親看實際完整手機／桌面頁面、不實跑命令／全套測試，不能據此宣稱那些檢查通過。

reading-order SHA-256：980c379d0ca6ab150ad1c5545bc109b781c52b8ceb5fc62e20d8f87b1b525bbc。

## 正文來源初判

### 001-001 4.1 原需：異欄位資料共用門檻規則

- Q1：Order 小計沿用2.5；新組合商品只保存固定價格，兩種資料待共用「金額至少300」。
- Q2 機制：尚未提出 trait 實作，合理待教。需要：獨立於做法的是同一門檻規則有一處，不為每個模型重寫判斷。
- Q3：已能用價格×數量／固定組合價判金額；新共同接口待下文。非負小整數／不越 i32，及不追加運費／缺報價，界定本節工作而不混1.6／3.1政策。
- Q4：導入足夠說明要保留與要改變的關係，未聲稱只靠一種資料展示重用收益。無必要缺口。

正文 SHA-256：83ef7ffa4fbcce9d9b2f9e96dd8c78309da7993458ea46ae2e3c1f7dc3fe18a6。

### 001-002 模型保留與公開型別／欄位

- Q1：Order、Bundle 是不同 struct；前者金額相乘，後者 price 已是整組價。charges library／example 分工依1.9。
- Q2 機制：pub struct 公開型別、各欄 pub 讓 library 外可建與讀；沒有把型別pub等同所有欄位自動pub。需要：example 要提供材料；共同行為不要求把Bundle欄位改成Order。
- Q3：可建兩種給定i32欄位資料，保留各自金額來源。新公開層級在首次建模前已教。
- Q4：兩模型與逐欄pub支持資料差異／跨crate入口，不證明共同規則如何避免重寫，該問題仍合理待教。無必要缺口。

正文 SHA-256：9a9d0d152b165fb33c0f99a5e42a811e44e556d6b58c5bc748b4f8a4bd2a3f1b。

### 001-003 Charge trait 的共同契約

- Q1：trait 是型別必須提供的能力契約，Charge 只要求 amount；&self／i32 身分依2.5。
- Q2 機制：method 簽名無body、分號收束，稍後各型別供實作；pub trait 供library外使用，不自動替全部型別加method。需要：門檻工作只須「只讀原資料、取得金額」，免知道相乘／固定欄位的差異。
- Q3：能讀 fn amount(&self)->i32 的輸入／結果，暫無body不能據此算值；實作處會使self成&Order或&Bundle。
- Q4：介面已與001-001同一需要連起，不把宣告當已實現重用。實作／共用函式仍待後文，當前不需猜。

正文 SHA-256：b6e4a595e778c474647bbb058d003d4a64e530b81d66dd811cea3a6360d79aa3。

### 001-004 impl Trait for Type 與異構取金額

- Q1：impl Charge for Order／Bundle 是各型別實作同契約，與2.5固有impl相比多了trait名稱。
- Q2 機制：簽名符合契約、body依各自欄位；method供契約呼叫不另pub。需要：保留異欄位資料而向後續工作提供同意義i32金額，不是強迫body／資料相同。
- Q3：Order讀兩欄相乘，Bundle讀price；shared借用不消耗資料。必要接收／公開規則在code前明示。
- Q4：兩種不同做法實際滿足同一amount，不單憑名稱一致推共同意義；門檻仍未寫，不能提前宣稱判斷重用完成。無必要缺口。

正文 SHA-256：32cc23ecb7d6df02bcd58f9e4c297fa65eea73683078b3efaeb6b20673ecac1a。

### 001-005 trait bound 與分流／共用判斷圖

- Q1：T:Charge 是型別參數能力要求，3.3未知T不能任意操作的限制由此滿足；meets_threshold 借&T與門檻、回bool。
- Q2 機制：編譯器檢查具體T實作Charge，body才能amount再>=；T各選Order／Bundle仍共用比較。需要：原需是同一門檻規則一處；「如何取得金額」留各實作，規則函式不讀私有欄位、不改模型，是實際分工理由。
- Q3：360>=300真、160>=300假；兩呼叫各借資料，不合成一值。公開泛型fn與前文pubtrait／impl形成可用入口；實際caller後續核對。
- Q4：已讀原位v5 SVG並用其本體渲染400×630。左藍Order=120×3→amount360，&Order線到中央T:Charge／金額>=300，再沿左線true；右綠Bundle price160→amount160，&Bundle線同規則，右線false。文字明說兩次呼叫、圖兩縱線與不同結果對應，不把合流框誤當合資料；它支持關係，不是完整頁面／執行證據。

SVG SHA-256：b3411edef987179eedd841cad5bd7afda48f1f58deb55bf97dca21b79951f204；同v5渲染PNG：9755447c80f159c5a74d39fe52a99dcc8ac8947ce5efa5d5ad42480bb2bc2006。此接口／理由／圖對應足夠，未主張原方法算不對。

正文 SHA-256：c60a8205318c034eff2b564eea9d7ef43e84788fb467cf7c9d2f311472fbaa25。

### 001-006 trait 的名稱範圍、caller 與缺bound

- Q1：直接點號trait method需Charge在名稱範圍，use引契約，不取ownership／下載；crate路徑轉名規則依1.9。
- Q2 機制：外部caller導入公開模型、Charge與共用fn；移除T:Charge使泛型定義本身不能假設amount、E0599。需要：能在library外取得共同能力；bound是所需保證，不是硬塞答案。
- Q3：caller兩種型別各調同meets_threshold、360真／160假，最後order.count=3仍可讀；同一需要的規則重用於不同欄位資料已實際發揮。
- Q4：source片段／輸出覆蓋雙型別共用與shared保留，錯fixture只驗缺bound的保障，不假設runtime才檢查。無必要缺口；凍結source／命令後續核對，未實跑。

正文 SHA-256：08ccc48fc7db170d09c8a264bf7b1f5f46e18fa657667476503d5947eae37ac8。

### 001-007 契約／計算正確性的測試界線

- Q1：兩個同檔測試沿1.10，Charge use沿前單位；先驗各模型amount，再驗同門檻。
- Q2 機制：300>=300真、300>=301假；trait只保能力／簽名，不自動保商業計算。需要：同契約可重用仍須保存各實作答案與邊界，契約存在不能代替需求驗證。
- Q3：兩model金額360／160與邊界同300各過／301各不過，全部可由已教>=算出，不需整struct比較。
- Q4：新限定改談編譯器保障範圍，最短依據是fn簽名只列輸入輸出型別、body由各實作供給（001-003～004），兼1.10型別≠規則。兩測試支持指定規則，不驗所有實作者，限定有當下用途且無猜測。

正文 SHA-256：847caacf65c7670708a9ab1052d59dfbe4f50c6c0b2f447c5eff51ab3451c617。

### 001-008 4.1 收束：新增模型不複製規則

- Q1：自訂DiscountedBundle是第三資料角色，Charge／bound／shared借用皆已教；非負、discount≤original排除無效折扣／越界。
- Q2 機制：amount=original-discount，其餘用既有meets_threshold。需要：保留「每模型供金額、共同規則一處」的原需，新型別可加入而不複製>=。
- Q3：360-80=280，threshold300假／280真，可先驗amount再驗兩bool；當前語法足夠，完整答案稍後在凍結archives核。
- Q4：小節已用兩異構模型示重用，再用第三模型練加入關係；未聲稱trait自動驗商業政策。延伸外連未開，不作必要先備；無必要／可選缺口。

正文 SHA-256：e5a575984f039c28f37744e7256eec46cfac2fea5d46d3ec8c07ab9285a68b8b。

### 002-001 4.2 原需與四個獨立範例

- Q1：標準Debug／PartialEq／Clone／Copy同屬能力契約，接前節trait；具體規則待各子節，這裡只需路線。
- Q2 機制：四example各有獨立OrderSummary，不把後設能力算到前型別。需要：整筆檢查、相同判斷、保留原資料改另一份是獨立於trait的開發需求。
- Q3：可以分別選example，不拼成重複定義同型別；每能力首次使用仍需下文。
- Q4：導入不把四能力等同，不聲稱只列trait名已教其差異。時序分界明確，無必要缺口。

正文 SHA-256：c04c6a7f435ca114f127c117d3cb7a74c123fe539aafb585eb6a4dba50c7927d。

### 002-002 Debug、:? 與 derive

- Q1：Debug供開發檢查整筆文字，:?指定此格式；derive attribute要求編譯器生成trait實作，屬性外框依1.10，不等同普通{}。
- Q2 機制：生成Debug需每欄也有Debug，兩i32可逐欄加名稱；自訂struct不因欄能印就自動Debug。需要：一次看型別／欄名／欄值，代替分別印欄；生成省去自寫這份欄位表示。
- Q3：加#[derive(Debug)]得OrderSummary { amount:360,count:3 }；移除則整筆:?處E0277、欄仍可用。參數、格式、正常／錯例都在首次需要處說明。
- Q4：限定「文字格式可以…改變」針對輸出穩定性，不否定開發檢查用途；特定印出不保固定存檔／對外格式。錯例把欄能力與整型別能力分開，支持本需要，無新增猜測。

正文 SHA-256：d6903812ffa25c37c463c718ab58fae4df128365f2265eff1600930602e12c3d。

### 002-003 PartialEq、完整摘要與enum的相等

- Q1：PartialEq供==／!= bool，比較借用兩邊；derive逐欄，enum還先同variant。Debug+PartialEq讓assert_eq!整筆可判斷／失敗可顯示，首次所需能力已明說。
- Q2 機制：全部欄位相等，enum同形態且附帶資料相等；需要：完整摘要相同，不能只同金額而忽略數量。生成依結構而非自懂業務「相同」，需忽略欄位則此規則不符，限定有用途。
- Q3：同欄真、同金額異count假；Waiting對Ready假、Ready3對Ready4假。比較後仍讀ready等名，借用契約實際發揮；全欄參與也可從明教derive逐欄正常推得。
- Q4：選讀改善 C1（非必要）：原句「第一組…固定金額…顯示兩個欄位都參與判斷」把例子支持範圍說得稍寬。這組變量只直接顯示count會影響結果；amount參與的依據是前面的逐欄規則，不是這組變動。最小改為「顯示數量也參與判斷」並回指逐欄規則即可，避免混淆證據來源；不需要補樣例才能理解全欄比較，也不改其必要通過。

正文 SHA-256：9ac20c072d0b5fa350b374e8e2994d31fbba719fc522e67d82ab7f53baccf368。

### 002-004 Clone 的型別行為與借用元素清單

owned文字副本：
- Q1：Clone提供借原值、回同型別新值的clone，預先在scope；String具Debug／PartialEq／Clone，&str也可Debug／比文字內容，先教再使用。
- Q2 機制：String複製內容到自己的配置；derive逐欄Clone，需每欄能Clone。需要：保留原標籤再改副本，普通賦值move不能留下原owner（3.4），明確clone提供此關係。
- Q3：original→duplicate先相等，改duplicate.label成Rust 副本，original仍Rust、金額仍相等、整摘要不等。
- Q4：單改label且原內容保留，實際呈現此次owned文字獨立修改用途；沒有從此推所有Clone都是全內容獨立副本。

reference／Vec的新限定：
- Q1：shared reference的Clone複製借用資訊；Vec<T>另建Vec逐元素Clone，需T:Clone。清單與元素的管理層次依3.6。
- Q2 機制：Vec<&str>的新清單內仍是reference，指同original.label，不變String owner。需要：回答clone到底保留／改變哪一層，免用「新值」推成所有指向資料也獨立；限定對象已明教，不需猜深淺演算法。
- Q3：views=[original.label.as_str()]，copied_views.clone所得清單len1，兩者讀Rust；&str索引Copy借用資訊依3.4／3.6。
- Q4：共同來源由建立式+元素Clone契約支持，不只憑兩輸出相同。文本足以對應原來源／新清單，不需自行想像未給圖。當前必要關係均足夠。

正文 SHA-256：0ca8c2572a412589b58162b6434cd96a2d8ee45f645e1a0d4829b1350fc09fc6。

### 002-005 Copy 的生成條件與管理責任

- Q1：Copy讓原本轉移值的動作隱含複製值資訊，兩整數欄struct需明選，不自動取得；依2.2已有數值Copy。
- Q2 機制：欄全Copy且型別也有Clone，才可此處derive；Clone不反推Copy。需要：指定後保留原摘要並改另一份，整數欄可獨立；String只複製持有資訊會使同配置雙owner，故另份owned文字用Clone。
- Q3：second=first後改count4、first仍3、amount兩者360；帶String derive(Clone,Copy)錯E0204，原因在label型別不符。
- Q4：正常例實際呈現隱含複製與各自修改，錯例與上一Clone示範界定契約。新增「不只看stack/heap」由2.2已教型別決定+String owner／shared ref資訊差異支撐，不要求猜其他配置型別；必要關系足夠。

正文 SHA-256：6c0ed09bebf82ef653aa709e0c92d80d7f2027536601771e4949944076ec0ef6。

### 002-006 四能力回收與改金額練習

- Q1：LabelledSummary指含String的模型，應沿002-004成功的Debug／PartialEq／Clone，不能拿002-005明示不能編譯的Copy設定當可用版本；這是由正／錯標記正常推得。
- Q2 機制：clone逐欄、新文字owner；+=20只改副本金額，PartialEq全欄因此整體不同。需要：原摘要保留、比較標籤相同但整筆不同、Debug一起檢查，各能力皆對應原開發需求。
- Q3：原280、副本300，label.as_str()兩者「訂單A」，!=真；可組derive與已教方法，不需知道未教Clone底層演算法。
- Q4：練習改amount而非先前改label，能檢查全欄比較規則的另一材料；表把契約與生成行為分列，沒有把Copy／Clone混同。最強支持為指定兩欄需求，無必要缺口；完整答案待archives核。

正文 SHA-256：4ecb31865a4ee9d43a9a5cef3a4b09e34ae56b847f23f6215c5d473346b3f6d9。

### 002-007 4.2 選讀分界

Q1／Q3：只有可衍生trait／Clone／Copy外連，未要求本文操作，沒有開。Q2機制／需要均已在002-002～006由正文完成，不拿外連補前提。Q4：小節最強通過範圍是整數／String／借用文字與Vec逐元素的已述契約；不推全部標準型別、完整Display或自寫所有標準trait。保留C1為例子說明範圍的可選改善，沒有必要未知。

正文 SHA-256：6f2cedef0fca55c5eb90a3d14f81c94f79ed705f286709d846ffc515f51021ad。

### 003-001 4.3 原需與HashMap鍵值形狀

- Q1：HashMap<K,V>保存相等鍵唯一對應值，兩種型別參數依3.3；String鍵 owned、usize次數，Vec仍按位置。
- Q2 機制：同map鍵／值各同型別，相等鍵只一筆。需要：名稱身分查數量，不把位置當名稱；舊迴圈也可掃描，改的是保存可按名稱查的結果表示。
- Q3：Rust/Python/Rust應變Rust2、Python1；建立／插入／查詢API尚待下文，未要求先猜hash演算法。
- Q4：表明任務與重複名稱要合併的關係，未證明HashMap一定比掃描快，亦無此宣稱。當前角色與用途足夠。

正文 SHA-256：61123c48aa02868267970347e8ae82405e4b957013bb8fd101a88327c871aa9e。

### 003-002 Eq／Hash 的鍵契約

- Q1：Eq建立在PartialEq之上，承諾每值自相等、無新method；Hash提供鍵資訊供定位數值計算，collision是異鍵可能同hash。
- Q2 機制：PartialEq還需對稱／傳遞、!=相反；HashMap先hash縮小位置後Eq確認，同條件相等鍵須同hash，反向不成立。需要：同名稱可再次找到，不被錯當另一鍵；同hash還須辨異鍵，不能代替相等。
- Q3：String／str／i32／usize標準Eq可用，標準Hash流程本節直接使用，不需算hash或挑演算法。自行鍵稍後需同套欄位支持兩契約。
- Q4：新增「同一套計算條件」限定比較的是同流程hash，不要求跨條件數值一致；當下只用既有實作，沒有操作上需猜的條件。定位與確認的最小關係已教，不需要hash細節才能理解分組。

正文 SHA-256：37478a4106e7a784ce6eaeeb690e2890dd7f6f2423379d031f70d9566d0323d7。

### 003-003 HashMap接口、Borrow一致性與查詢來源圖

接口／helper：
- Q1：std::collections::HashMap需use，不加第三方依賴；new空建、insert接owned鍵值、get回Option<&usize>、len數不同鍵。owned_text只把3.4 new／push_str／owned回傳包成短helper。
- Q2 機制：相等鍵insert保原鍵、替值回舊值，不自動加；get借map值、查詢字只作查找。需要：owned名稱保存、用借來名稱查，不每次新建鍵；「insert不是加總」避免原分組需求被錯操作代答。
- Q3：Rust1新建回None，Rust2替換回Some(1)、get新2，Python1使len2，未有JavaScript則None；query改Rust 課程仍能讀found的2。
- Q4：caller實際改query並讀found，展示回借來源不綁query；map不承諾印出順序的限定只針整map輸出，不否定固定按名稱查答案。接口與結果足夠。

Borrow：
- Q1：Borrow<str>讓String透過borrow提供&str，型別參數指出借用表示，trait／型別參數依4.1／3.3。
- Q2 機制：文字Eq／Hash與owned鍵一致才可借表示辨同鍵；「能取得reference」本身不足。需要：同名稱查到原鍵，省每查建立String而不改變身份；大小寫假想對照明示不一致時不能直接稱同身份。
- Q3：本例用已教as_str，map內用標準Borrow，不需自己寫borrow或選hash算法。
- Q4：新一致性限定已接Eq／Hash定位與確認，不靠課外Borrow知識；沒有把任意視圖都當可查鍵。必要關係足夠。

原位圖：
- Q1／Q3：已讀v5 SVG並渲染400×730。頂部query:String擁Rust；其下query.as_str()藍實線向上指此來源。綠虛線向下查counts內String鍵Rust；底found:Option<&usize>的藍實線向上指map值2，不指query。
- Q2 機制／需要：實線借資料、虛線查找明列；不同來源解釋為何可改query但用found期間不能衝突改map。表面兩份Rust相同不代表同object，owned_text兩次建構和兩獨立框支持此區別。
- Q4：對象／箭頭／相鄰文字可核。選讀改善C2：渲染中綠虛線x112進map時穿過「counts: HashMap」標題，仍可辨，未阻礙來源判斷；移查找線避開標題可減少視覺干擾，不需改借用關係。未看實頁版面。

SVG SHA-256：aae0087e2a06d48d21083276f039b37267ff2c5859489efbb2ce609335f2dd37；同v5 PNG：1b9e6afb76b1732c2d85cb042b43996c9d9e8db1269a46528b3788a1c837088c。

正文 SHA-256：f9b9512645c4f9b7724c941cad4f279995f24f56c3b9ea0d1a6c946c97e34445。

### 003-004 不同鍵仍屬同map借用契約

- Q1：get成功的&usize借map，不是query；E0502 shared／mutable衝突依2.4與3.6。
- Q2 機制：found最後讀延伸shared借用到insert後，insert需mutable；需要：先報舊數再新增時要安排先後，不靠知道鍵名不同放寬API契約。
- Q3：Rust2先get、插Python1、後if let found會錯；將found最後讀提前再insert可行，名稱建構依3.4。
- Q4：新限定「另一個名稱…仍借用這個map」相較前文補清衝突對象是collection而非只同鍵，get表明shared借map已足以前提。錯例覆蓋異鍵仍衝突，無必要猜測。

正文 SHA-256：a28c59e972fb7c47c18c14f9d8d8f30b6823d457f400ee9a44dcce05d21c0bc1。

### 003-005 count_labels分組、分支借用與結果測試

分組的來源／改變：
- Q1：get_mut查同名稱，回Option<&mut usize>；&[String]迴圈得&String依2.6，owned來源不能移出。
- Q2 機制：Some透過*count加1不另借map，None無存留值reference可結束查詢再insert；僅缺鍵(*label).clone建owned名稱。需要：重複加1、保留輸入、回傳結果不依輸入存活，而非每次insert1替換。
- Q3：Rust首入1、Python首入1、Rust再加至2；map鍵是String新owner，input scope結束仍可查。固定&str array三查依array by-value取得&str，缺名報0但map不增鍵、len2。
- Q4：表與雙分支具體實現同原需；clone借原String而非*直接搬值，符合2.5／002-004。None分支新借用界線當下明教，無必要猜測。

Option／reference比較：
- Q1：Option比較沿enum規則，None同None、Some比較內值；shared數值ref比較所指數字，不比位置。usize／Option／數值ref可Debug，說在assert_eq!之前。
- Q2 機制：內型別可比較／顯示才有相應Option能力，get結果可比Some(&2)。需要：簡潔檢查查到的數量／未收錄，不略None或只驗len。
- Q3：三測驗重複／空／輸入scope結束後結果可查；&2只在當下assert比較，不由此推可回傳任意短命借用。
- Q4：scope測試真正發揮owned鍵的獨立存活，未只用原labels仍在時宣稱此用途；合前節兩測共五的宣告待source核。必要依賴已教，未實跑。

正文 SHA-256：7fb59b1bbce8c64945ee399e5edf7c3fe1d7721e6fb799f46e2bd8da0667bcb0。

### 003-006 自訂複合鍵與derived Eq／Hash

- Q1：GroupKey兩i32欄作完整身分；derivedEq需欄Eq、derivedHash取全部欄Hash，PartialEq逐欄依4.2。
- Q2 機制：同套欄結構使相等鍵同hash，Hash不保異鍵必異hash（003-002）；需要：地區／類別共同分組，不只名稱。自行忽略一欄時需重核一致性，限定接已教契約。
- Q3：&same借完整鍵本身查到Some(&3)，&different類別3查None；只Eq／PartialEq缺Hash的insert錯E0599。同型別借用入口在caller前明說，不需猜Borrow blanket實作。
- Q4：C1同類可選位置：「固定地區…只改類別…檢查…兩個欄位都參與分組」直接測的變量只有category，完整region+category規則依據在前述全欄derived。改成「確認類別也參與鍵身分」可對準例子證據；完整規則可正常推得，不列必要缺口／不要求增案例。錯fixture支持缺能力，未驗任意手寫Eq／Hash一致。

正文 SHA-256：6d46fe97adb160a909e895201ca1ce1dddf4dc5407575f22b8106c66256f1a75。

### 003-007 原需改為庫存量、保留分組表示

- Q1：StockItem是owned name+usize quantity，借&[StockItem]迴圈得&StockItem；reference讀欄沿2.3，String clone沿002-004。
- Q2 機制：已有鍵*count+=item.quantity，缺鍵clone name插該quantity；需要：把「一次」改成「附帶件數」而保留同名累加與原輸入owner，不每次insert覆值。
- Q3：Rust2+6=8、Python3、len2、input len3、JavaScript None；name.as_str查／name.clone建owned可由已教field access+method借receiver正常組合。小數量不越usize限定排除未處理overflow。
- Q4：指定材料真的區分加一／加量及覆值錯誤，回傳owned與借輸入關係沿本節成果，不依下一節lifetime符號才能實作。無必要缺口，凍結完整答案後核。

正文 SHA-256：16ec364a69ab8d51f02c167389ac6bd3707bd361363b9552c0c12caa2fad9026。

### 003-008 4.3 選讀分界／整體關係

Q1／Q3：只有HashMap／Borrow外連，未開。Q2機制：owned map鍵、Borrow一致查詢、get／get_mut借map值、None可另insert，必要前提由正文實教；需要：名稱身份分組與累加，不改輸入又讓結果獨立。Q4：最強支持是String文字鍵與全欄GroupKey、兩種固定報表材料，不是任意自訂Borrow／hash算法或map順序。C1／C2保留可選；無必要缺口。

正文 SHA-256：2ed46856e564e09e47684af220c653f27ac785baddf64f0ba401f0fcd804d0ef。

### 004-001 4.4 原需：回傳借用本身

- Q1：由3.5 owned長度結果轉為&str結果，caller之後要用它；borrow不延owner依2.4。
- Q2 機制：需連輸入與輸出的借用來源，符號未教合理待後文。需要：不重建文字但caller知道哪個owner必須存活，不只是能在fn內顯示。
- Q3：既有String／&str／len byte比較足夠材料，沒有未教走訪／容器前提；不能從此導入自行寫尚未教簽名。
- Q4：工作改變與保留材料明確，尚未以任何owned結果代替借用來源問題。當前導入足夠。

正文 SHA-256：454fbb4b86372d4f392769f766b63e12e9e72c288855d031fd317cb5d1b1753f。

### 004-002 單一來源與lifetime elision

- Q1：same借&str回同來源&str，shared ref Copy借用資訊依3.4；elision是省輸出符號而連唯一輸入reference lifetime的新簽名規則。
- Q2 機制：省略不消滅有效要求／延owner；需要：caller能取同文字借用繼續用，不重建String。函式結束只結束自身工作，不把caller仍持有的text結束，依2.2／2.3正常推得。
- Q3：新ch04-lifetimes／source module／pub與use依1.9；text仍在、view印Rust／len4，single_source命令具目標，不需新外依賴。
- Q4：caller實際在fn返回後讀view，展示跨fn用借用且來源存活；不是只示fn內讀。elision如何處理多來源尚不需要、後續教；本案必要依賴足夠。

正文 SHA-256：5bdeaf38e791a8456b89e41ef4db190e1c53a59bf3552920846b5246137a80fa。

### 004-003 雙來源的介面欠關係

- Q1：longer依byte長度選文字，並非比較lifetime；同長選first是明定工作政策。
- Q2 機制：省兩輸入會分成不同關係，單一來源elision無法定輸出，E0106；caller依簽名、不是逐次執行body猜來源。需要：可能選任一段的借用都須能滿足後用有效性，不能只假設此例固定選Python。
- Q3：if >=／尾值／len已有先備；目前fixture不能編譯，缺的是描述來源關係，非byte演算法，標註待下一單位。
- Q4：完整錯例與簽名說明對準雙來源新問題，不用文字長短推owner生命長短；沒有必要立即猜修法。

正文 SHA-256：5d5036fd8ee4ead13b608ea5994fd386e6446828603ed3979f3b2efac26b384b。

### 004-004 'a共同使用要求與雙來源圖

- Q1：lifetime annotation以'a參數描述reference有效關係，不是秒數／字資料；fn名稱後宣告、&'a str使用，位置可接3.3型別參數但角色不同。
- Q2 機制：輸出用到哪，兩候選來源都須可借到哪；同'a不要求owner同時建立／結束，也不延短命owner。需要：兩種可能來源均可支持caller後用而不重建內容，這是共同要求，不是猜本次實際分支。
- Q3：Rust4／Python6先選參數second；交換後first是caller的Python owner，仍選它。兩次都得到Python但經不同候選位置，兩owner在兩次use都在。
- Q4：v5 SVG本體+340×420渲染：上first & 'a str／中second同'a，first沿右外側線到輸出，second向下到同綠輸出&'a str；footer「箭頭=可能的借用來源」限定此圖不是同時兩結果或文字複製。相鄰文字與source/輸出標記足夠，同名參數與caller變數交換以code可分辨，未看整頁。

SVG SHA-256：b998606f6f12113e38fe18efab59cca38029be305512083ac608ecd2c172c83c；同v5 PNG：8802e3f75e5ec748635348c22055df896b4167da7641ccde98842b26ff250a97。新限定的主詞／有效範圍關係已解釋，不需猜生命被強制等長。

正文 SHA-256：f4c0eced628092fc8bf720208eea1fe5eecbeef6a2bdbb06c775edaa642fcb53。

### 004-005 同結果／不同來源契約與scope

- Q1：longer候選兩來源；first_text只接第一輸出，第二用_不綁名，pattern依3.1。這是功能改為只返first，明示非同需求修正。
- Q2 機制：outside較長仍不能取消longer對inside共同要求，annotation不延owner；first_text的輸出'a只連first，第二先結束不牽輸出。需要：保留任一候選則結果在共同有效區用；改只返first才可用另一契約，對照區分結果字串與有效理由。
- Q3：錯fixture外讀selected E0597；合法longer留內層，only_first出內層後仍用outside，兩次Rust tutorial。
- Q4：兩個相同輸出不能證明同lifetime關係；code的use位置+不同簽名實際展示用途，第二未教額外規則，直接從標注連接可知。_不省參數、仍借有效during call，可由現有接口推得；無必要缺口。

正文 SHA-256：a333ea8de8ed9917cdd075ed5cb6cc22caac0328ac65daf00b72c20b2545ba95。

### 004-006 區域來源失效與owned結果選擇

- Q1：new_text想回區域String的&str，copy_text則回新owned String，後者轉管理依2.2／3.4。
- Q2 機制：任意'a不能讓inside延命、E0515；copied new+push_str保存另一份，尾值交owner給caller。需要：來源結束仍獨立保存結果，owned滿足此改變，borrowed用於續讀仍有效的已有內容。
- Q3：inside block結束後saved.as_str()仍Rust／len4；copy_text名字不代表Copy能力，工作名稱與trait已明分，無需猜複製種類。
- Q4：獨立owned的scope例真正展示來源結束後可用，非只重複原來源仍在的顯示。代價另存文字對應設計改變；不宣傳annotation越多越好。必要關系足夠。

正文 SHA-256：639e2cffe4e15444fd9afacdf80e6fb253ba5082862b2f5136ddf3d5460c0989。

### 004-007 '_ placeholder與唯一來源省略

- Q1：'_是推導用位置符號，不是宣告名稱'a、也不是pattern _或任意時間；新版語法角色在code前說清。
- Q2 機制：still用elision，因此same唯一輸入連輸出；多候選歧義不靠placeholder消失。需要：本子段辨認保留標注位置但交推導的形式，原工作仍是正確說明來源，不另編功能優勢。
- Q3：same_placeholder(&Rust內容)回同來源顯示Rust；無需fn<'a>，不是把任意lifetime放進去。
- Q4：唯一來源示例只支持此規則；雙源仍失敗可由placeholder依elision+004-003規則最短推得，不需實際新fixture才能理解。無必要缺口。

正文 SHA-256：0838061bd55e82137afc134d572f1cdf943b2e792cc6b8a81ae2aa65618c6e11。

### 004-008 lifetime bound與Option來源候補

- Q1：fallback preferred是Option<&'a str>、backup:&'b str、輸出'a；'b:'a描述至少涵蓋，可同範圍，不是秒數。
- Q2 機制：Some取preferred、None取backup，後者需'b保證能供'a輸出。需要：有片段就用、缺時候補且滿足結果後用；不同名稱提供範圍關系的表達，不主張舊共同'a無能力做到fallback。
- Q3：可用Some片段／None與backup調用，預期Rust／backup；Some空仍片段不換備用，沿3.2有0≠缺值。缺bound fixture所述錯誤就在None將'b供'a。
- Q4：新含蓋限定明確解釋對應branch及用途，不把annotation當實際延owner；能從fn身體知道哪項保證必要。外連fixture未開，後續只查archive；範圍關系已足夠，不需更多秒數／複雜型別。

正文 SHA-256：893dd880023490b910a31743b8b789e641ce53d911137713b9a4ded6f7d3c6cf。

### 004-009 4.4收束與first_nonempty

- Q1：兩個候選&str需共同'a輸出，函式依len==0選第二／否則第一；shared借用和byte概念已教。
- Q2 機制：共同來源契約依004-004，不由哪段當下空來取消另候選；需要：第一段非空優先，雙空仍返第二空，不同於byte較長政策。
- Q3：("",Rust)→Rust、(Rust,Python)→Rust、("","")→len0；numeric assertion+顯示可按指定方法核，無實作缺口。
- Q4：可選C3：原句「不直接拿…文字結果作尚未介紹的比較」未指明哪種比較；002-004已教&str PartialEq按內容比較。若意指本練習只用長度／顯示，最小改成局部驗證選擇，避免暗示已教&str相等尚未教。主線有清楚可行方法，未因這句增加必要實作負擔，故不是必要缺口。小節原需／雙源／單源／owned選擇與限定均已完成；提到library其他測試待archives核。

正文 SHA-256：3a25cff40a863a53377fe3ef26a926b8b857b4e190508357210d31172dbbf791。

### 005-001 4.5原需：保存借用的模型視圖

- Q1：從fn返回&str改為struct保存borrow資訊，原內容仍另有owner；沿3.4／4.4。
- Q2 機制：新model型別／method未教，合理待後文。需要：把借來文字與後續操作集中成視圖，不重建owned內容；不是單純替原文字換管理者。
- Q3：知道須保存原內容入口並保證owner有效，不能把外層model當新文字owner。
- Q4：導入需求清楚但未要求先寫含lifetime字段，當前沒有必要未知。

正文 SHA-256：266ced4fdc92de40db07d9a97f01273808f3fc431bc0b3dec5c104d9e804e885。

### 005-002 TextView<'a> 欄位／owner與重名圖

- Q1：TextView<'a> 宣告欄位使用的lifetime，text:&'a str把有效性帶入型別；持reference值不是持被借String的ownership。pub型別／欄／views module依4.1／1.9。
- Q2 機制：模型使用借文字時原owner必有效，包struct不延String存活。需要：保存既有內容入口供後用，而不重建文字，新的模型也應約束同一owner有效性。
- Q3：可建view.text借外部text.as_str()；本單位只定義，caller在後文。相同text名稱被明分模型欄位reference／外變數String，不用由名字相同猜同物件。
- Q4：已核v5 SVG+340×330渲染，上藍view:TextView<'a>內列view.text:&'a str，下綠text:String「仍是owner」管理Rust tutorial；從上向下借用箭頭以相鄰標「借用原內容」解釋。圖不是ownership移入／第二份文字／完整布局，對應足夠。

SVG SHA-256：b6c5e7c89ca18a4e22522f2ff94a7e21b9af1c725b7da94a63eb6c60b4d2b4d0；同v5 PNG：3c391b8bfa7b83e1951240c45d11c66c7af96efbdf3f0180c9ceca0feac8d437。必要概念與圖都已在首次使用處接前文，未看實頁。

正文 SHA-256：88219e9a4886a807243ea9f58de0395d925a28844c828aa040981439549041ae。

### 005-003 make_view的placeholder型別路徑與外逃借用

- Q1：TextView<'_>在回傳型別路徑使用placeholder，模型內reference接唯一text輸入；規則由004-002／007接到005-002。
- Q2 機制：構造TextView { text }只保存借用資訊，fn結束不結束caller來源；內層String結束後把模型外讀仍E0597。需要：把視圖建立集中到fn而不重建內容，model不應被當延owner工具。
- Q3：直接view與built都指同caller text，印Rust tutorial／bytes13；source短命fixture須讓source留到外讀完，或改獨立owned內容。
- Q4：兩建法由明確as_str來源支撐同份，非只因印相同字；「不是忘了複製struct」由reference包模型不改target ownership正常推得。只顯示text欄，不偷用TextView尚未給的Debug／PartialEq；必要關系足夠。

正文 SHA-256：58b4fbbc321de85017d57d6c3138dbf31b6a618a2e80a48b9d9328f4359b577e。

### 005-004 receiver借模型／欄位借來源兩關係

- Q1：impl<'a>宣告參數、TextView<'a>指定適用型別，不是固定全域生命；&self是再借模型，與欄位&'a str借原來源不同。
- Q2 機制：method省輸出連receiver（新elision規則），original顯式&'a str則連欄來源；self.text可Copy借用資訊，無需move模型。需要：只在借模型時讀用text()，要把原來源視圖交到model結束後則用original契約；byte_len回owned數字無需來源連接。
- Q3：內層view.text()顯示、view.original()交saved外讀，來源String仍外存，len13；改為view.text()外讀則E0597在view，並非原String短命。
- Q4：兩method同body而對照正常／錯scope，實際展示簽名契約的不同用途；模型壽命與原來源沒有偷換。「不能&self一概固定欄'a」回應此兩角色，不要求更多lifetime參數才正確。table+code能定位兩次借用，無缺必要圖／前提。

正文 SHA-256：8d5a1826a98edec2198126851af50dbaa638d44a38cb337321c377bb78ca0870。

### 005-005 兩欄不同來源的可用輸出

- Q1：TextPair<'a,'b>兩欄各連來源，impl宣告同兩名稱；各method輸出&a／&b，byte_len回owned數字。
- Q2 機制：模型使用所需欄需有效、各輸出只連自己來源，不綁無關來源；需要：內層用兩段後，第一來源的借用可在第二結束後繼續用。若兩欄共一'a則共同要求可能綁短來源，可由004-004共同範圍規則推得，非額外課外理由。
- Q3：first外存Rust tutorial13、second內存note4，內層pair總17、讀second，再first_source交saved外讀13；輸出不持續借pair／second。
- Q4：實際source壽命不同並在短者結束後用長者，呈現分開參數的關鍵用途，不單列兩符號。不推多標註讓所有data一起存活，已教限制與當前關係足夠。

正文 SHA-256：951cf801053dbda0543f00f92ba00aa92de4db4cd8bedc2e3bba5ddd0b777b5f。

### 005-006 4.5與全章收束

- Q1：Label<'a>文字欄與impl原來源method是TextView已教模式；原來源外層、model內層，receiver可先結束。
- Q2 機制：original(&self)->&'a str Copy欄借用資訊，saved不續借Label；需要：短暫模型包已有文字而保留原來源的後用，source若一起移內層則改變前提，不是同練習。
- Q3：外String Rust、內Label、回original、外顯示與len4；所需impl／pub／lifetime語法與測試方法已教，不靠第5章。
- Q4：全章最強通過是異模型金額契約、標準能力按欄位、名稱分組與owned鍵、回傳ref來源及receiver分離；下一章只交代格式／轉換／傳遞目的，不提前要求未教介面。38正文已保存；C1／C2／C3為可選最小措辭／呈現改善，暫無必要缺口。下面只補本v5 sources驗對應，不用後查回填正文判斷。

正文 SHA-256：af04f6368d081ef2f804944344c2ffad6d8df6df25c2709eb06e17ff1e87ea76。

## 正文階段原判封存（source archives 前）

38單位按順序完成；截至此點沒有讀archives／完整答案／markers／他人審閱。四圖已在原位只用v5原SVG閱讀及渲染。正文階段已記C1兩處的例子支持範圍、C2查找箭頭過標題、C3已教文字比較與練習限定措辭，均為選讀改善，無必要缺口；原紀錄不因下查sources覆蓋。

前38單位紀錄 SHA-256：2ff962737ac6085b1443e327ab3388f85d1f95be188e3a548d191ee3b2e22b99。

## 凍結source archives核對（非新blind正文首讀）

### Core archives：兩個package／library／helpers／tests／主binary

- Q1：兩Cargo.toml各name=ch04-traits／ch04-lifetimes，四項.workspace=true沿實讀1.3及v6局部；本freeze只存成員manifest，沒有根workspace／lock，不能宣稱完整環境或繼承數值已重核。traits lib公開charges／counts；lifetimes lib公開source／views，與正文路徑一致。
- Q2機制／需要：charges.rs:2–35完整保留兩模型、Charge、兩impl、共用bound fn，回應4.1規則一處；counts.rs:6–25的owned_text純3.4建構、count_labels依缺鍵Clone／重複借改回owned map，回應4.3結果不借原材料。沒有額外未教API。
- Q3：charges兩測、counts三測就是正文展示五項；source.rs:2–51依序same／longer／first_text／copy_text／same_placeholder／fallback／first_nonempty，最後用match len 0／_而非if，模式由3.1已教，與練習規則等價。views.rs:2–46為TextView／make_view／三method／TextPair三method，與正文一致；lifetimes src/main.rs按兩參數順序印Python兩次，對應004-004無--example命令。
- Q4：source.rs四測實查雙輸入位置、owned副本原source修改後仍4／原13、Some空與None／first_nonempty三組；views.rs兩測查view三入口長度與第一輸出跨越短第二源／模型。合為六項。這些是靜態核對展示／依賴與指定材料，不是實跑通過／所有字串內容或pointer身份的實驗證明；來源身份另外由實際body self.text／self.first及契約支持。全部required操作已在當下正文或實讀1～3教過。

Rust源檔的 // --8<-- start/end 是archive中snippet邊界註記，正文未靠其作語言推理；本輪不把讀懂出版抽取標記當初學者必要前提／不據此稱建置流程已驗。未讀外部Cargo設定或其他sources。

### 21個完整examples／五頁完整答案的依賴與輸出核對

本階段已讀兩package所有example源檔：traits 11個、lifetimes 10個。四題合併位置如下：Q1以表的角色／正文定位分辨各獨立型別；Q2機制／需要以既有契約與工作分開；Q3逐檔核操作／可推輸出；Q4只判源碼／正文對應與已教前提，不以未實跑來源當結果實驗。

| package／檔案 | 已教必要前提／機制 | 保留的工作與可推結果（非實跑） |
| --- | --- | --- |
| traits / traits.rs | 001-002～006公開模型／Charge scope／共用bound fn | 360、160、達300真／假，原count3仍可用；兩caller實際重用 |
| traits / traits_solution.rs | 001-008本地DiscountedBundle與外部Charge實作 | amount360-80=280、300假／280真；只供amount、不複製門檻 |
| traits / debug.rs | 002-002 derive Debug／:? | 整筆OrderSummary360／3與個欄斷言；不承諾穩定對外格式 |
| traits / equality.rs | 002-003 PartialEq+Debug／借用比較 | 真／假／假／假，struct欄與enum形態附帶數字參與；C1的支持範圍不因看source被覆蓋 |
| traits / cloning.rs | 002-004 String Clone／Vec逐元素reference Clone | 原Rust、副Rust 副本，兩Vec借原Rust；same source依建立式+元素契約，不只同字串輸出 |
| traits / copying.rs | 002-005所有數字欄Copy+Clone | 指定後改second.count4、first3與兩amount360；不推String也Copy |
| traits / capabilities_solution.rs | 002-006成功LabelledSummary三derive；&str內容PartialEq早在002-004教 | 原280、updated300、labels同、整筆不等與兩Debug；只改amount、不把原資料移走 |
| traits / hashmaps.rs | 003-003標準HashMap、owned_text、String Borrow<str>、insert/get | None、舊1、新2、query可改Rust 課程、缺JS、len2；回借map與query不同 |
| traits / label_counts.rs | 003-005count_labels、固定&str array與Option branches | Rust2／Python1／JS報0未收、原len3；不使用map排列順序 |
| traits / custom_keys.rs | 003-006Eq／Hash按兩i32欄生成、同型別借查 | (1,2)Some3、(1,3)None；C1只直接變category的限制仍記 |
| traits / hashmaps_solution.rs | 003-007StockItem、field auto-follow、String method借receiver、get_mut／Clone／Option Debug | Rust8／Python3、len2、缺JS，原input3；Debug查詢顯示Some(8)／Some(3)由003-005已明教，未偷渡unwrap或新轉換方法 |
| lifetimes / single_source.rs | 004-002same唯一來源elision | Rust／len4；來源在caller，借回不重建 |
| lifetimes / source_scope.rs | 004-005共同來源vs第一來源 | 內longer與外first_text皆Rust tutorial、位置不同；不把改需求當同方法修正 |
| lifetimes / owned_output.rs | 004-006copy_text new/push/owned move | inner結束後saved Rust／4，無任意'a延命 |
| lifetimes / placeholder.rs | 004-007'_依elision | Rust，同來源約束仍在，不宣稱多來源省略可行 |
| lifetimes / outlives.rs | 004-008Option<&str>與'b:'a | backup外層、preferred內層，Some選Rust／None選backup、均內層使用；Option類型註記依3.3 |
| lifetimes / lifetime_solution.rs | 004-009first_nonempty helper與數值檢查 | 第一空Rust、第一非空Rust、雙空len0；不借用未教API，C3是限定措辭而非缺實作 |
| lifetimes / borrowed_model.rs | 005-002～003pub TextView／make_view<'_> | 兩視圖都Rust tutorial／13，字串owner仍外變量 |
| lifetimes / method_source.rs | 005-004來源'a輸出與receiver借用區分 | 內text()讀、original()外讀13，不能替為省略text()後外讀 |
| lifetimes / two_models.rs | 005-005兩來源參數／三method | 17、note、短來源／model結束後saved Rust tutorial13 |
| lifetimes / borrowed_model_solution.rs | 005-006Label<'a>／impl<'a>／original返回&'a str | 內label結束後外saved Rust／4，來源外存；沒有clone／延owner |

五完整答案均只用其當頁或已教前文能力，普通完整example亦與正文源碼／文字預期對應。traits_solution／capabilities_solution／hashmaps_solution與兩lifetimes solution沒有隱藏需要讀第5章的helper。Positional {:?}是已教普通{}傳參+002-002的:?格式設定，003-005還明示Option reference Debug，可正常組合。

### 11 fixtures與expected markers：錯誤目的／已教前提

| package／fixture | 正文與故意缺項／最小作用 | expected檔內容 |
| --- | --- | --- |
| traits / missing_bound | 001-006；泛型T未保Charge，body.amount無契約保證，非等某caller才錯 | E0599 |
| traits / not_debug | 002-002；移除Debug derive，:?整筆無能力，欄位本身仍可用 | E0277 |
| traits / string_not_copy | 002-005；String欄不Copy，不能生成整struct Copy | E0204 |
| traits / map_borrow_conflict | 003-004；found借counts、插別名後再讀，shared／mutable重疊 | E0502 |
| traits / key_without_hash | 003-006；Eq已有但缺Hash，insert所需鍵能力不全 | E0599 |
| lifetimes / ambiguous_output | 004-003；兩省略輸入，輸出未接來源，即使body可看出選長也欠簽名關系 | E0106 |
| lifetimes / source_too_short | 004-005；inside內層結束、輸出仍需兩候選共同有效，outside較長不解除 | E0597 |
| lifetimes / local_output | 004-006；inside局部owner未交出，借用不能用任意'a外逃 | E0515 |
| lifetimes / missing_outlives | 004-008；None要由'b供'a輸出，缺'b:'a涵蓋保證 | lifetime may not live long enough |
| lifetimes / model_source_too_short | 005-003；TextView含借用不延inner String，外讀field仍失效 | E0597 |
| lifetimes / method_receiver_too_short | 005-004；省略text()輸出連receiver view，view內層結束，雖然原String外存仍不符 | E0597 |

Q1：markers是各獨立故意失敗源檔的期望錯誤標記，不是讀者應把失敗片段放入正常examples的指示。Q2機制：每fixture差別對準表中缺項與該處已教保證；需要：具體界定trait能力／來源契約被拿掉時會失去什麼，不用錯誤碼替代原因。Q3：已逐個開.rs與對應.expected，11個各有main；缺outlives仍在泛型定義的None分支失保，不因main用literal backup就補掉。Q4：本輪確認archive代碼、正文解釋與marker文本對準；沒有運行rustc／harness，不能宣稱本工具鏈實際產生marker、診斷行號或其餘warnings已驗。未發現把未教依賴誤當fixture錯誤原因。

### 靜態對應與資產邊界核對

正文共有43段Rust fenced blocks；以只去除snippet註記、空行與行首尾縮排的比對，43段全部可在此凍結Rust archives找到，沒有以live檔補對應。這是片段來源一致性檢查，不是語法／執行測試，也未把空白正規化當文字輸出驗證。四個assets SVG與各自sources/docs/assets中原SVG的bytes相同；原位閱讀／渲染均使用這份v5，沒有混別版。兩member manifest、7個library／主binary源檔、21 examples、11 fixtures及11 expected、4 archived SVG共56 source檔，全部已讀或同bytes核對；所有指紋另存ch04-continuity-v5.sha256。

正文的Cargo命令按-p對上兩member name，--example對上21個檔名、不加.rs；ch04-lifetimes無--example命令對上src/main.rs。繼承根設定與--locked的既有含義沿實讀1.3／修後指定單位；本freeze未包含根Cargo.toml／Cargo.lock，本輪沒有另開live根補證。cargo test的5項traits／6項lifetimes是所讀library #[test]數量；不把外部fixture expected標記當它們自動執行的測試。fixture harness與實際stderr未驗。

## 獨立initial最終封存

必要缺口：本輪未發現。此結論的通過範圍是38單位正文的實際依賴與來源、目的、轉換、限定用途，以及56 source檔與4原位SVG的靜態對應；不能擴成完整環境／實頁／編譯器診斷／真人首讀驗證。

可選最小改善如下，三項皆保留原先「選讀改善」分級，不因已能理解便丟掉，也不升為必要：

| ID／位置 | 已教且可通過的關係 | 最小改善／實際收益與邊界 |
| --- | --- | --- |
| C1：002-003、003-006 | derived全欄比較／Eq與Hash同欄位規則已明教，不需猜完整身分；固定一欄、變另一欄例子直接展示後者影響 | 把「兩個欄位都參與」的例子歸因改為「數量／類別也參與」，全欄主張回指derived規則；精確區分例子支持與語言契約，不必加更多例子 |
| C2：003-003 map-query.svg | query文字borrow、查找、found借map值的三條關係均可定位 | 綠虛線查找路徑避開counts:HashMap標題；同v5 native400×730渲染已見交疊，仍可辨，不說整頁手機／桌面遮擋已驗 |
| C3：004-009 | 本練習numeric長度／顯示方法可行，且002-004已教&str內容PartialEq | 「文字結果作尚未介紹的比較」改明是本練習選用的驗證方式，或指明是哪種尚未教比較；不暗示已教文字相等尚未教，無需改演算法／增加trait先備 |

關鍵通過已獨立從來源建構：4.1各模型供amount、共用門檻有一處與實際雙caller；4.2逐欄生成、整型別能力不自動、Clone按各型別及reference層次；4.3 Eq／Hash／Borrow身分一致、查詢與結果不同來源、get_mut分支借用、owned鍵跨輸入結束；4.4共同候選／單來源／owned／placeholder／outlives各有工作與契約，限定不依猜實際分支；4.5模型借原來源與receiver再借模型分離，兩來源可各自輸出不牽無關短命來源。每項最強支持與未驗界線已在相應unit／archive表保留。

本輪未看任何第4章first-reading／review／disposition、作者notes／ledger、freeze manifest.json、live修稿、外部解答或第5章；沒有交叉提示。初判先封存，若後續cross另開追加紀錄，不改此原判。封存前僅統一這份新紀錄的正體字形，判斷、分級、原引文關係與source hashes不改；第1～3章原紀錄一律未改。

未驗：完整手機／桌面頁面、Cargo／Rust實跑與fixture marker實際產生、根workspace／lock與環境版本、所有輸入／任意自訂trait／hash／Borrow、真人學習。獨立銜接角色與已讀背景實況已明記，不宣稱盲讀首讀或全套技術驗收。完成即停止，等待協調者指定cross或下一任務。

