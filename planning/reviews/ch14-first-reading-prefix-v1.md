# 第 14 章 14.1–14.5 原始順序首讀 prefix

本紀錄為 AI 模擬首次順序閱讀，不宣稱真人測試。基本 Python 加本人實際已讀第1–9章正文與既有局部補讀可作背景；實讀版本沿封存 checkpoint，不假定完整讀過前章各最新版。第9章 original v5 全27單位已封存，v6僅002-001／004-001兩段 local，不能說完整已讀v6。本輪開始只核 own counter（27／2）及原檔 SHA，未重讀原 review。第10–13章未實讀；不得用模型本來知道的 Rust／前章名詞補答案。

本輪先重讀 clear-tutorial/SKILL.md 四問與 review-protocol 一般準則，未讀 Rust 參考答案、作者安排或技術問題清單。所獲提示只限路線、版本、scope及通用判準，沒有 peer14／root14疑點或預期答案。

唯一正文依序為 frozen：14.1 ch14-macro-rules-prerequisite-v3（6單位）、14.2 ch14-hygiene-prerequisite-v4（7）、14.3 ch14-derive-prerequisite-v5（10）、14.4 ch14-api-prerequisite-v1（11）、14.5 ch14-property-prerequisite-v3（7），共41單位。各包 renumber 001 起，故紀錄／gate 用14.x/001-xxx複合鍵保持唯一。14.6不在範圍。

新 ch14prefix route／state／ledger 獨立；每個 current BODY 在輸出前先經同 visible_filter 的 closed 答案過濾，每必要新概念首次四問，Q2a機制／Q2b需要分存，附當下引文、最短已教推論與未知，保存後才NEXT；小節末回收整體。不開 source／archive（包括14.3兩package完整檔）／peer／root disposition／author／live／outline／外連。圖片僅當前原位 hook 之後核同 frozen 圖；未實頁或Rust執行不聲稱通過。

每小節 entry 查必要前置是否在本人實讀1–9或本輪前文已教；未讀10–13若只是待教／非必要名稱，不猜機制，必要前置缺口則當時保存並停回ROOT。初判／分級／hash原樣保留，後文釐清只追加。完成 prefix 原始紀錄封存後停，不補答案或讀14.6；完整14 QA與其他entry待另安排。FIRST9 original與local SHA保持。

### 14.1/001-001 — 保留單位型別，減少宣告樣板的需要

entry 前置：本人實讀7.9的可重用型別／函式及8.9的Meters／Seconds、tuple newtype與借讀i32；不需要未讀10–13章。println!／vec!只是已用過的macro呼叫，不當成已學過定義規則。
Q1：宣告式macro依輸入語法產生Rust宣告，這次要重用的是struct／impl形狀，仍保各單位的型別身分。引文：「每個型別都包一個i32」。
Q2a（機制）：當下只給語法→同款宣告的基本角色，尚未教匹配或展開步驟；正文先寫普通答案的安排足以導入，操作細節合理待下一單位。
Q2b（需要／用途）：更多單位都需借讀數字而不接手原值，想「少維護幾份相同的struct／impl樣板」；這是獨立的維護需要，沒有將新語法本身當目的。
Q3：先核普通Rust答案，再定自己的macro規則；package ch14-macros、library ch14_macros，指令從repo根執行，沿已有命名／命令規則。
Q4：沿用Meters／Seconds保留資料模型與單位用途，先減宣告重複而不換問題；本導入未展示macro結果，不冒稱已理解完整實作。當下無必要未知或問題。

原始SHA-256：9c83a8c1775f2923dbc43af1b43487a5698e361cb61337051c2fd94bf5d05731；visible SHA-256：9c83a8c1775f2923dbc43af1b43487a5698e361cb61337051c2fd94bf5d05731。

### 14.1/001-002 — 手寫 Reading 與待減少的宣告重複

Q1：Reading提供借讀i32的共同能力，read_value<T:Reading>重用呼叫邏輯；Meters／Seconds仍為兩個newtype，不混單位。前置trait／泛型／derive／本地impl／tuple欄位皆在本人實讀4、8章。
Q2a（機制）：&self保留wrapper，.0只Copy內部i32；Debug／PartialEq用於觀察／assert，沒有讓wrapper自動Copy。引文：「沒有選Copy」。
Q2b（需要／用途）：函式／泛型已完成借讀，仍重複型別與impl宣告；需改宣告來源而保留型別邊界與行為。引文：「重複的是上面的型別與實作宣告」，清楚避免把原函式說成做不到計算。
Q3：library提供trait／函式，example的本地型別可impl引入trait；main借distance／waiting得80／30，之後仍觀察並比較原值。
Q4：兩份同款struct／impl與兩個assert呈現宣告形狀相同、資料／單位不同、owner仍在；支持原任務可行與重複的位置，不支持任何效能優勢。沒有必要未知／問題，macro操作仍待下一單位。

原始SHA-256：55a9771d0ef656dcb75529f02fe5f6ed2f91f9c3031a71a2a136c400179f9d1c；visible SHA-256：55a9771d0ef656dcb75529f02fe5f6ed2f91f9c3031a71a2a136c400179f9d1c。

### 14.1/001-003 — 展開時點、token匹配與單一ident規則

檢查點A：compile-time expansion。
Q1：macro展開是編譯時產生Rust語法，再受型別／ownership檢查；token是identifier、標點、literal等語法記號，非runtime結果值。引文：「不是程式啟動後用字串決定新型別」。
Q2a（機制）：先匹配輸入語法，再把名字放入宣告樣板，最後編譯及執行借讀；不先執行輸入作普通match比較。
Q2b（需要／用途）：前段要減少不同單位的同款宣告維護，compile-time產生宣告回應它，保留已看懂的newtype／Reading行為；普通函式處理不同資料數量並非此處採macro理由。
Q3：Meters／Seconds各自展開一組struct／impl，再用原main得到80／30與仍在的原值；仍非同型，也未增加Copy／換算。
Q4（原位圖）：hook後核同SVG原生圖；第一框輸入Meters名字→箭頭「編譯時匹配ident」→name=Meters、同名字兩位置→箭頭「編譯時展開宣告」→struct Meters／impl Reading→最後「編譯後執行」→read_value借Meters(80)交80。框／三箭頭支持時點與名字重用，不表示複製值。

檢查點B：macro_rules規則／metavariable。
Q1：規則由輸入模式、=>、樣板組成；$name保存匹配語法片段，ident指定種類，不是let變數。這是當下新教的定義語法。
Q2a（機制）：($name:ident)只接一個識別字，樣板兩處$name同時替換；定義名字不加!、呼叫加!，規則末分號結束它。
Q2b（需要／用途）：一份樣板能對不同單位名字產生一致介面，同時保留各型別；代價是追蹤輸入／展開，沒有虛稱比普通函式一定容易。
Q3：同example先引Reading、先定reading_type再呼叫兩次；read_value名稱沿前段相同main的import即可，不需跨crate規則來補此例。
Q4：表列每個符號角色、兩個呼叫對到手寫兩組，main的原assert讓宣告來源改變與行為保留可對照；尚未實跑。
圖SHA：`c67dbdbf122f64ace7860e3ad0eecf059b68d589b72bdcd15b3a0cba19739f2b`；原生PNG：`877205591ed0171c684d6404092be7e02a63888c6a1d1f77bcb7e95a2ef49b14`。只看單圖，未驗實頁。單位末：必要新角色／時點／符號／採用理由已足，無未讀10–13依賴或問題。

原始SHA-256：e54396d2304cd76d9810691d96dee6eb5081509bb75f21874bde1b8d8b512124；visible SHA-256：e54396d2304cd76d9810691d96dee6eb5081509bb75f21874bde1b8d8b512124。

### 14.1/001-004 — 一層 repetition 與可選尾逗號

Q1：repetition重複的是匹配／產生語法的範圍，不是runtime Vec；新增Millimeters仍採相同i32／Reading模型。
Q2a（機制）：$(...)圈範圍，+至少一次、*零次以上、?零或一次；本例$name:ident以逗號分隔至少一個，再以$(,)?容許尾逗號。樣板相同一層+各展開一組，這些符號不當數值加法／Result的?。
Q2b（需要／用途）：一次列三個名字而保留三個型別，減少每個呼叫／宣告樣板的維護；「清單的用途是決定要產生哪幾組原始碼」接回原宣告重用需要，非資料數量迫使使用macro。
Q3：reading_types!(Meters,Seconds,Millimeters,)得到三組宣告，main借讀80／30／500；最短已教預測：去掉尾逗號仍匹配相同名字，零個名字不滿足+，不猜實際診斷。
Q4：輸入表、樣板與三個assert呈現一個名字一組宣告，借讀後wrapper保留；正文指出零／負數也只是取值，沿8.9不把型別身分當非負驗證。未讀完整tests或實跑。
單位末：新增重複符號、匹配次數與展開層次都有當下意義；原型別／borrow行為不改。無必要未知或問題。

原始SHA-256：84a5956042e803a733766f748d844037c0a5fed90eee632fafa5a9480964588d；visible SHA-256：84a5956042e803a733766f748d844037c0a5fed90eee632fafa5a9480964588d。

### 14.1/001-005 — 匹配失敗與產生程式的型別失敗

Q1：輸入語法是否匹配與展開程式是否合型別是兩個階段；生成newtype仍受已讀8.9的型別邊界約束。
Q2a（機制）：ident拒80 literal，+拒零個名字；名字合法可成功展開，但&Seconds不能交需要&Meters的distance_value。引文：「後者是產生的程式能否滿足型別契約」。
Q2b（需要／用途）：宣告重用不能取消原單位保護，且查失敗需知道修輸入還是修生成程式的使用；保留原任務的型別契約而非讓macro放寬它。
Q3：第一fixture預期no rules expected；空repetition預期unexpected end；第二fixture用合法名字產生兩型別，等待值借入路程函式預期E0308。依正文辨認階段，不猜診斷的其他文字。
Q4：模板刻意只生成struct，隔離匹配／型別關係；不含Reading impl也不影響該&Meters契約的展示，Reading的共用能力是前段背景。這是可見失敗材料，未實際編譯。無必要未知／問題。

原始SHA-256：070b966f175618c0bf143e9300bb5a089132ed83dbe96ae8d12834e0d2acfb31；visible SHA-256：070b966f175618c0bf143e9300bb5a089132ed83dbe96ae8d12834e0d2acfb31。

### 14.1/001-006 — 零次 repetition 練習與14.1小節末

Q1（小節末）：macro以名字語法產生獨立newtype與Reading宣告，再由普通Rust借讀值；不是runtime清單或新型別字串解譯。
Q2a（機制）：輸入／樣板同一層+一起改*便可零次，選尾逗號仍可一次；空呼叫不產生struct／impl，單獨逗號也匹配零名字。引文：「不能產生只剩下部分語法的impl」。
Q2b（需要／用途）：原需是保單位身分／借讀行為、少維護重複宣告，單一／多名字模式都已回應；新增練習是允許空清單而不製造假型別，不改泛型函式已能讀值的分工。
Q3：最短已教修改只換兩個重複操作符；先空呼叫，再原Meters／Seconds仍得到80／30。沿輸入→匹配→同名兩位置展開→編譯檢查→runtime借讀的順序解釋，無須開答案。
Q4：本節手寫／macro同main、展開圖與兩類失敗支持維護位置、時點與型別保護；有限模板不等於已教完整vec!，也不支持速度優勢。答案為closed marker，延伸連結未開。
14.1收束：必要前提都在實讀1–9／本節，四題兩面已連成原問題的答案；necessary0／optional0，scope／名稱來源下一節合理待教，不提前猜。

原始SHA-256：901b953c0b8925147f8d74fa658400f10b22db5e1d5a2bd0413a5bfba3c835b4；visible SHA-256：6e4f5cf965580b359d73f8c01ab67daddfaaa6084fe654f7dc4a297c06dc234a。

### 14.2/001-001 — 跨crate模板與同名來源的入口需要

entry 前置：14.1六單位已實讀，library／module／crate／公開名稱與tuple亦在1–9背景；不用10–13章。hygiene目前只是新module名字，不自補其語言規則。
Q1：原同example的宣告模板要交其他crate使用，並新增一次借讀兩來源回(80,30)的例子；現在查名稱來源與同名變數的關係。
Q2a（機制）：本導入尚未給跨crate／同名解析規則；不知道模板項目找library還是caller，保存為合理待下一單位的新問題，不先用專家知識答。
Q2b（需要／用途）：「讓其他crate使用」需正確找到依賴名稱；兩個來源必須仍各自借讀，不能讓等待誤變路程。這兩個具體需要先於新規則。
Q3：沿原package新增hygiene module，命令仍由repo根執行；舊例不改，尚未要求新macro呼叫操作。
Q4：(80,30)保留已教材料，能辨認來源錯配的後果；尚未展示結果或比普通函式更好的主張。當下無必要前提缺口，名稱／作用域機制待教。

原始SHA-256：6eddfdaa2ea39d0b8c4879bf108e101bb078899e8086f846ea77e410e22b7514；visible SHA-256：6eddfdaa2ea39d0b8c4879bf108e101bb078899e8086f846ea77e410e22b7514。

### 14.2/001-002 — 公有普通函式與私有helper的基準

Q1：read_pair_values以分別的T／U Reading借兩種wrapper，read_internal私有且只供同module函式呼叫；承接已讀1.9的pub邊界。
Q2a（機制）：公有函式內兩次私有helper各讀&self，回兩個Copy i32，不交出私有函式供外crate直接用。crate::Reading與pub mod hygiene的名稱來源明列。
Q2b（需要／用途）：先證明普通函式已完成兩來源借讀，再用私有helper作後續名稱／可見性觀察材料。引文：「不是因為普通函式無法借讀兩份資料」，沒有把macro用途虛構為新計算能力。
Q3：read_pair_values(&distance,&waiting)得(80,30)，原wrappers仍在；外部caller只需要公有路徑，不能直接呼叫read_internal。
Q4：兩段函式及來源句支持普通呼叫的分工；尚未展示macro生成位置，所以001-001的macro名稱規則未知仍待下一段，未偷補。無當下必要缺口／問題。

原始SHA-256：f4fedc40f20c4ee5e4f4c086c39510638d3d2461b0a7a7ef2610f66ea9c8dc6b；visible SHA-256：f4fedc40f20c4ee5e4f4c086c39510638d3d2461b0a7a7ef2610f66ea9c8dc6b。

### 14.2/001-003 — macro自身的scope與模板裸名稱

檢查點A：textual／path-based scope。
Q1：本地macro依文字順序、同包圍scope查名稱；跨crate export則以定義crate根路徑查macro，和它原本寫在哪個module分開。
Q2a（機制）：本地定義前呼叫尚找不到macro；#[macro_export]提供public根名稱，可用use或完整路徑呼叫。引文：「即使定義寫在hygiene module裡」。
Q2b（需要／用途）：讓其他crate使用模板，需要可找到macro的公開契約；本地先後順序不能當跨crate路徑來源。
Q3：前置呼叫fixture預期cannot find macro；當段實際export的reading_type_unqualified以根路徑引入，先成功找到macro，再匹配Meters。
Q4：先後次序fixture及attribute／use對照展示macro是否找到；不把名稱查找失敗當14.1的pattern失敗，未實跑。

檢查點B：模板的裸Reading。
Q1：macro名字可見不等於生成impl的trait名也可見；裸Reading是另一份名稱契約。
Q2a（機制）：只import當段macro、沒有Reading，展開impl Reading仍找不到trait，預期E0405；「沒有讓模板裡所有名字都自動使用library的來源」。
Q2b（需要／用途）：跨crate的宣告必須仍impl正確Reading，不能只解決macro本身的來源就認為生成程式合法。
Q3：caller引reading_type_unqualified並呼叫Meters，失敗發生在生成trait名稱；當下不猜修正版或其他名稱的解析規則，待下一段。
Q4：fixture隔離只import macro而缺trait，支持export的有限作用，沒有聲稱所有名字採同一規則。001-001的跨cratemacro來源已釐清；裸名稱修法尚待教，無必要前置缺口。

原始SHA-256：5482418773a987cd30549f8757370d9b8e917540cc1f138fe3b0b434ea7334c8；visible SHA-256：5482418773a987cd30549f8757370d9b8e917540cc1f138fe3b0b434ea7334c8。

格式註記：只校正本輪尚未封存新紀錄中的正體用字，沒有改理解、分級、來源引文或source hash；既有封存紀錄均未寫入。

### 14.2/001-004 — $crate的定義來源與private邊界

檢查點A：固定trait來源。
Q1：$crate是macro的特殊metavariable，指定義macro的crate，不是caller crate或runtime參數。
Q2a（機制）：impl $crate::Reading固定library trait，caller即使有同名unrelated介面也不改目標；caller無須import library Reading。
Q2b（需要／用途）：解決001-001／003的跨crate名稱問題，生成型別仍滿足library read_value能力，避免同名trait改契約。
Q3：caller只引macro／read_value，生成Meters／Seconds再借讀80／30；普通函式的Reading bound連到固定trait，原值仍可assert。
Q4：caller另一Reading要求unrelated與成功讀值的對照，使來源差別可見；這是文字／程式支持，未實跑。

檢查點B：生成位置與存取權。
Q1：公有函式body在library內，macro生成程式則在caller；找到私有helper不等於caller能存取。
Q2a（機制）：$crate::hygiene::read_internal路徑指對來源，仍因private而E0603；生成程式須呼叫公有read_value，module內item另需完整路徑。
Q2b（需要／用途）：讓外crate模板可用，同時保留1.9已教的library封裝；引文：「沒有把caller變成library的私有實作區塊」。
Q3：固定例private_reading!(&distance)將$value放到helper參數位置，caller無權直接呼叫，不能靠$crate繞過；公有函式原呼叫仍合法。
Q4：普通private helper／export macro兩個位置對照支持權限差異，不把E0603混同之前E0405。
當下未知：expr首次出現在模式中，未有名稱／一般匹配範圍解說；只由可見固定&distance呼叫及預期存取權錯誤推得此例匹配成功，再由已教metavariable替換讀出callee。此段的來源／private結論不依賴接受其他輸入的規則，一般expr語法尚未算通過，記合理待教，不補專家答案。無未讀10–13必要前置。

原始SHA-256：f71b296d7634c6bdcf31e58aae321370ef01687e77f432b48294a28c4ed6ee71；visible SHA-256：f71b296d7634c6bdcf31e58aae321370ef01687e77f432b48294a28c4ed6ee71。

### 14.2/001-005 — expr、mixed-site hygiene與求值／名稱的限定

檢查點A：expr與結果block。
Q1：expr匹配一個Rust expression，不是ident名字；read_pair接兩份產生shared reference的expression，回兩個i32。004的未明名稱在此追加釐清，不改先前待教紀錄。
Q2a（機制）：外層括號是模板，內層生成block；let先保存左reference，最後tuple為block結果，tuple兩項依左、右求值。
Q2b（需要／用途）：把兩來源的借讀形成單一expression結果，保來源reference與原owner；不是為解決普通函式做不到的工作。
Q3：$left代&distance先存，read_value讀該reference，再求值$right並讀；來源可用會加calls再交reference的block。
Q4：可見雙大括號／let／tuple及普通函式同結果，支持生成block的角色，沒有借助未讀章節。

檢查點B：hygiene名稱來源。
Q1：hygiene是展開時名稱隔離／解析；mixed-site把模板局部變數依定義上下文、插入片段依caller上下文處理，其他裸項目名如Reading則caller解析。
Q2a（機制）：模板let value與caller value拼法同而來源不同；右輸入保caller，故不是被新let換成左來源。$crate固定其他項目來源，不能把所有名字都套局部規則。
Q2b（需要／用途）：解決入口「第二份來源會否變第一份」的問題，讓來源不同的80／30保留，不只得到兩個可用數字。
Q3：caller value借waiting；read_pair!(&distance,value)結果(80,30)，普通函式相同，waiting仍可assert。
Q4（原位圖）：同SVG原生圖的綠框caller value借Seconds(30)，右側線接讀右30；藍框模板value存Meters reference，中間線接讀左80；紫框逐列指來源，最後到(80,30)／原值仍在。箭線追名稱來源，不是move或求值順序。對象由框／正文／讀左讀右標籤對上，未驗整頁。

檢查點C：只求值一次的來源。
Q1：hygiene管名稱，不保證輸入工作只做一次；求值次數是這份樣板如何插入expression的性質。
Q2a（機制）：left／right各只插一次，兩個加calls的輸入合計2；若同expression插兩次，計數也兩次。let先左、tuple右項才取得右來源。
Q2b（需要／用途）：新增可觀察副作用材料，避免只看80／30就漏掉重複取得來源或誤將once歸因hygiene。
Q3：兩個block各加1、交既有reference，observed=(80,30)；文字另列[1,2]驗順序，我沒讀其完整tests。
Q4：calls=2 assert支持每輸入once；單憑最終2不驗順序，所以順序依正文／程式及另列測試目的，未聲稱實測。

限定D：Q1生成item名稱仍是caller提供的identifier；Q2a同module兩次Meters仍同名，局部變數隔離不產生新型別名字；Q2b防止把名稱隔離誤作自動唯一type；Q3重用模板需自己選不同名字／範圍；Q4此為正文明說的規則，當段未有duplicate fixture，不先報實際diagnostic。
圖SHA：`ee1710566e1eba0c5c906261dea8945b0d35bfdfa9a3c288cfa632642b7c4aec`；原生PNG：`50804ba5881d3dc82affdced18e6fc2b6a7b0702c5d4a25494f49ccebd75eb8e`。單位末：入口兩來源問題已接到規則與材料，必要新動作／限定可理解，無necessary／optional。

原始SHA-256：4d546c9128d059cd5feb9e67f00da4a1a7331390433656fb78911b079e224335；visible SHA-256：4d546c9128d059cd5feb9e67f00da4a1a7331390433656fb78911b079e224335。

### 14.2/001-006 — fallback診斷與隔離的跨cratefixture

檢查點A：tt／fallback／compile_error。
Q1：tt是token或配對括號圈住的tokens；fallback用零次以上tt接住其他輸入，compile_error在編譯時給文字診斷且令建置失敗，不是panic／成功值。
Q2a（機制）：按順序先ident，未匹配才試fallback；第一條呼叫已教的生成macro會再展開。生成程式錯誤不退回另一pattern。引文：「不會因生成程式失敗就回退」。
Q2b（需要／用途）：模板要明說需要一個型別名字，讓錯誤使用者知道改輸入；仍保匹配成功後的正常Rust檢查，不把型別重複當可換另一規則。
Q3：80進fallback自訂訊息，Meters進第一條；再次Meters是E0428。診斷指呼叫或模板時，先查名稱、pattern、生成項目來源／權限／重複的階段。
Q4：兩條規則、80呼叫與五列錯誤表各支持其原因；對005的item唯一性限定追加可見E0428預期，不把所有macro標記都判規則壞了。

檢查點B：manifest與fixture邊界。
Q1：此跨cratefixture自成workspace，依賴本地library；不是普通根工作區的all-targets成員。path／lock基本分工沿7.1。
Q2a（機制）：../..從該Cargo.toml往上兩層，空[workspace]隔離故意失敗targets；--manifest-path選manifest、--example選其同名example，--locked沿已有lock規則。
Q2b（需要／用途）：真正觀察外crate名稱／private邊界，又不讓正常全targets檢查被故意失敗材料阻斷；拒絕原因要是模板契約，不是沒連到library。
Q3：正文給從repo根的check命令選macro_custom_error，預期含自訂訊息；其他預期檔只見名稱與用途，不開其內容。
Q4：TOML節錄、目錄起點和命令選項都有角色，足以理解重現目的；未看完整fixture／Cargo.toml檔或實際diagnostic，未聲稱執行通過。無necessary／optional或未讀10–13必要依賴。

原始SHA-256：b9428f2b860f8fae12bce7ca9ec6907e073e97a5627a191f2d5435e204cc8928；visible SHA-256：b9428f2b860f8fae12bce7ca9ec6907e073e97a5627a191f2d5435e204cc8928。

### 14.2/001-007 — 取得順序與結果順序分開，14.2小節末

Q1（小節末）：macro自身scope、生成item路徑／權限、模板局部與caller輸入來源、輸入求值次數是不同關係，不能由export或hygiene一項代答全部。
Q2a（機制）：本地／路徑scope找macro，$crate固定項目來源仍受pub；mixed-site隔離局部，expression插入次數控制once。練習兩個let依左、右取得reference，再反向組tuple，保原取得顺序。
Q2b（需要／用途）：跨crate可重用模板且不混來源／越過封裝；新增練習要結果(右,左)但取得工作仍左先右後，故不能把有副作用的輸入直接對調。
Q3：最短已教方案：block先存left、right各一次，再用已有公有read_value讀right／left；本地caller用已教library路徑或import。兩來源block先記1再2，得(30,80)，owners仍可用，不需closure／新trait。
Q4：(30,80)加[1,2]同時檢查結果與取得順序，比只看calls=2多一項必要證據；答案closed、外連未開。前面兩名來源圖、public／private與五類錯誤支持各自有限規則，不聲稱Rust實跑。
14.2收束：入口兩問題與四題皆由實讀前文／當段接起，expr已在005補教，原待教紀錄不回填；necessary0／optional0。下節parser／procedural名稱只是預告，尚未當先備。

原始SHA-256：c54b43e8324acef5b22773e0b5c6b9d97351cbc26b5ee30a9233cc9491e434c2；visible SHA-256：980dda61ce320cc012ca8192b32fb4d9c84309b930a6aacbc72fca860d54fc5d。

### 14.3/001-001 — 保留使用者宣告，只增加Reading實作的需要

entry 前置：自訂trait／impl／derive選用／borrowed讀值在實讀4、8章，模板展開在剛讀14.1–2；沒有把未讀10–13當先備，parser與procedural工作尚待本節教。
Q1：原模板從名字生成struct＋impl，現在使用者先寫struct，工具只增加Reading impl；宣告資訊成為新輸入，型別名字／欄位可見性由使用者保留。
Q2a（機制）：當下只交代新增impl的分工，未教parser／函式介面；不自行補procedural API或支持的所有宣告形狀。
Q2b（需要／用途）：使用者想在自己的宣告旁選能力，自己決定可見性與其他derive，同時沿用借讀介面；改輸入來源回應這個不同於固定pub模板的需要。
Q3：保Meters(80)／Seconds(30)交read_value得80／30與原值，後續需學如何讀已寫宣告再增impl；手寫／原模板仍可完成，不說它們沒能力。
Q4：同資料／同行為固定比較目標，文字明示變更的宣告責任；尚無產生結果材料，合理待教，無必要缺口或問題。

原始SHA-256：a8737d1ec638a5959db46353693ff75d8a11b73400ad41e0dc7e0fdc2fd2ccc5；visible SHA-256：a8737d1ec638a5959db46353693ff75d8a11b73400ad41e0dc7e0fdc2fd2ccc5。

### 14.3/001-002 — procedural輸入／交付位置與ReadingValue角色

檢查點A：procedural macro／TokenStream。
Q1：procedural macro編譯時用Rust程式處理語法，entry以proc_macro::TokenStream收／回tokens；不是runtime Meters值或Vec資料。
Q2a（機制）：收到名字／標點／括號等語法，可用已教struct、enum、if、match检查，回新語法再由編譯器處理；macro_rules則模式到樣板。
Q2b（需要／用途）：入口要利用已有宣告資訊決定impl，此分工讓普通Rust邏輯處理語法而非runtime建型別；檢查哪些資訊／怎麼parse尚待後段。
Q3：compiler提供宣告TokenStream到macro entry，entry回產生語法；目前只有型別與角色介紹，不自补建置／parser API。
Q4：TokenStream內容列舉對到14.1的token定義，足以分語法與值；沒有聲稱已查欄位的runtime型別。

檢查點B：derive保留原宣告。
Q1：ReadingValue是工具，Reading是將增加的trait impl；#[derive(ReadingValue)]把該型別宣告交工具，不是傳它的已建立值。
Q2a（機制）：derive保原宣告、後加items，故不回原struct第二次；表的function-like替呼叫、attribute替item是另兩種交付位置。
Q2b（需要／用途）：使用者自己維護struct／可見性／其他derive，只要求補Reading，保持原宣告才能完成入口分工，避免重複型別。
Q3：在自己的型別derive清單選ReadingValue，宏只交impl；另兩類只認本表輸入／替換角色，本輪尚未實作它們。
Q4：三列對照足以支持為何此derive輸出只新增項目，未展示parser／生成骨架，合理待教，無必要前提缺口或問題。

原始SHA-256：e89ff9ede5050542f5669cab126b75c307073ea98e025e58d4fa69e68c399c49；visible SHA-256：e89ff9ede5050542f5669cab126b75c307073ea98e025e58d4fa69e68c399c49。

### 14.3/001-003 — host工具／consumer分工與依賴來源

檢查點A：proc-macro crate。
Q1：原trait library、derive編譯工具、consumer是三個package；新工具必為proc-macro crate，由另一crate使用，entry在根lib.rs為pub fn、TokenStream→TokenStream。
Q2a（機制）：[lib] proc-macro=true選特殊crate；先建可於host執行的工具，consumer編譯時處理語法，consumer執行只用生成impl。host／成品區別已在本人9.2實讀。
Q2b（需要／用途）：先有編譯工具才能處理使用者宣告，又保一般trait／程式與工具責任分開；不用把parser放進最後執行的借讀工作。
Q3：derive目錄設旗標／entry，derive-consumer依賴原library與../derive；路徑起點為該manifest。當段是配置角色，尚未驗實際workspace檔或entry骨架。
Q4：三列package表與兩段TOML明示誰在何時使用誰，能辨認配置位置，不借archive補答案。

檢查點B：syn／quote／compiler來源。
Q1：compiler提供proc_macro crate；syn把tokens解析成可讀欄位／enum的語法樹，quote把帶插入位置的語法產tokens，都是第三方library。
Q2a（機制）：syn關預設，只選derive宣告樹／parsing解析／proc-macro連compiler stream；quote保預設轉換支援，兩版以=固定。feature／精確版本／workspace繼承沿1、7、9已教設定。
Q2b（需要／用途）：為既有宣告檢查與impl產生各選工具，保持同版API而不混版本；只用需要的宣告解析，不要求syn full／印完整樹。
Q3：兩第三方放derive dependencies；proc_macro不向registry下載、不寫成一般dependency，consumer另用path兩項。
Q4：來源句、版本及feature角色列明，語法樹如何查欄位／quote如何插值仍待下一段，當下未要求那些操作。無未讀10–13必要前提或問題。

原始SHA-256：cc622b72d35e652ac1b9eaff7f53eb2ad148b5c159efc0d2e644df9e7b4120b7；visible SHA-256：cc622b72d35e652ac1b9eaff7f53eb2ad148b5c159efc0d2e644df9e7b4120b7。

### 14.3/001-004 — AST宣告資訊、支援外形與Punctuated

檢查點A：AST與檢查分工。
Q1：AST是Abstract Syntax Tree／抽象語法樹，將名字、泛型、欄位形式整理成結構資料，不用自行切原碼文字；syn::DeriveInput是本例模型。
Q2a（機制）：工具只接nongeneric、一欄tuple struct；body固定self.0→i32，名字／外形先查，欄位能否交i32留Rust型別檢查。沿8.9 alias相同型別，不能靠字面拼i32拒alias。
Q2b（需要／用途）：利用已有宣告資訊生成正確有限impl，保使用者的名稱／欄位可見性及合法alias，避免把語法觀察當型別解析。
Q3：讀ident給生成／診斷、params判空、data選Struct、fields選Unnamed，再數恰一欄；其他形狀將診斷，不猜泛型生成方案。
Q4：五列欄位表對到既有Meters宣告的名字／零泛型／一tuple欄，足以建立要讀的資料；沒有聲稱AST能知道runtime值或alias實際型別。

檢查點B：Punctuated與borrowed pattern。
Q1：params／unnamed是語法節點連分隔符的Punctuated，不是Vec；只用當段指定的is_empty／len。
Q2a（機制）：len數節點，不把逗號當欄；match &model.data／&data.fields借模型，Struct／Unnamed分支得shared reference，不move／clone整樹。
Q2b（需要／用途）：正確判有限外形，仍保模型供之後取名字／產生語法；避免把每種len容器都誤當Vec或必須接手所有資料。
Q3：params空且unnamed.len()==1才繼續；原tuple允許多欄的語法已交代，工具的支援範圍是額外選擇。
Q4：表列方法、分隔符限定與borrow示意足夠讀出檢查流程，完整AST／選中enum其他欄位不必另查。無必要未知／問題或未讀10–13依賴。

原始SHA-256：4d1674a6e2eaa5d18eb18328ec1cc6d58fda28629137fb748deb5a438823f896；visible SHA-256：4d1674a6e2eaa5d18eb18328ec1cc6d58fda28629137fb748deb5a438823f896。

### 14.3/001-005 — 解析early return、Span診斷與兩種TokenStream

檢查點A：parse_macro_input。
Q1：syn解析macro接保存compiler TokenStream的input變數，以DeriveInput規則parse；as是這份macro輸入模式，不是數字cast。
Q2a（機制）：成功交model；語法失敗提早從所在函式回compiler compile_error tokens，故函式必回proc_macro::TokenStream，不可放普通i32回傳函式。
Q2b（需要／用途）：把無法形成宣告模型的語法失敗交compiler診斷，後段只對已解析AST檢查有限外形，不把兩階段混合。
Q3：entry用parse_macro_input!(input as DeriveInput)，成功才使用model；所列use把compiler／syn／quote短名稱來源分開。
Q4：成功／失敗契約與return型別已明寫，足以解释骨架控制流，不需自己寫parser或補未知語法解析能力。

檢查點B：reject／Span／型別轉換。
Q1：syn::Error保存自己的形狀診斷，Span是原程式位置；名字ident.span定位使用者型別名，不是runtime檔案資料。
Q2a（機制）：Error::new(span,&str)建診斷→into_compile_error消耗它產proc_macro2 tokens→into以已教From轉compiler tokens；兩種同名TokenStream來源不同。
Q2b（需要／用途）：已parse但不支持的形狀要指出使用者宣告且符合entry交付型別，不panic／不要求新Error trait；原文字訊息已有Display。
Q3：reject借model不消耗AST，取ident位置與message，回compiler TokenStream；本source只用compiler短名稱，無須另寫proc_macro2路徑／直接新增依賴。
Q4：helper三步與兩種stream表可追出每步owner／語法型別；錯誤tokens不是已執行的診斷，compiler後續報出。沒有必要新猜測，未實跑或讀source，無necessary／optional。

原始SHA-256：e07773dc092b73d5727ca0931c0ec260d5ee5e1876e95797c59b0e5abba6ea90；visible SHA-256：e07773dc092b73d5727ca0931c0ec260d5ee5e1876e95797c59b0e5abba6ea90。

### 14.3/001-006 — quote插值、ToTokens與固定外部trait路徑

檢查點A：quote／ToTokens。
Q1：quote!從Rust語法樣板產tokens，#name插入工具Rust變數內容；ToTokens供交語法的契約，Ident已有實作，不是runtime字串format或macro_rules的$name。
Q2a（機制）：let name=&model.ident借AST名字，quote插成Meters identifier，結果proc_macro2 stream再into compiler stream；只產impl，不重宣告struct或建立Meters(80)。
Q2b（需要／用途）：利用已解析名字生成同名impl而保持原宣告與owner分工，不再把宣告文字切成String或另維護名字。
Q3：用既有model取名字，生成reading body self.0→i32，回entry所需tokens；ToTokens只認本段用途，沒寫未知trait method。
Q4（原位圖）：同SVG原生圖1 compiler宣告TS→2 syn AST（Meters／0參數／1tuple欄）→3 quote impl／TS2／#name→into箭頭→4 compiler TS原宣告後加impl→「Rust檢查通過後」→5綠框consumer Meters(80)借讀80。藍1–4是编譯語法交付，綠才user值，條件箭頭不暗示所有形狀必成功。

檢查點B：生成名稱來源。
Q1：procedural輸出在user端解析，不能沿macro_rules $crate；::ch14_macros::Reading從外部crate名字固定trait來源。
Q2a（機制）：caller不必import裸Reading，但必保ch14-macros依賴名／ch14_macros程式路徑，這份工具不自動查rename。
Q2b（需要／用途）：沿14.2的來源問題，使新增impl實作原library契約；有限固定依賴已滿足示範，不虛稱任意改名都支持。
Q3：consumer保原依賴名稱，生成完整trait路徑；沒有把依賴可見性／macro_rules捷徑偷渡進proc輸出。
Q4：quote尾段可見完整path與只加impl，文字限定直接對到consumer manifest來源；未看生成檔或實跑。
圖SHA：`ad2dbe412df2915c13f2b4473f2c709705b98c7d9a3237833f2034e560a8fb9d`；原生PNG：`e62fde597ee3ad43d30fb9f35957f1cd064c7574d7e8906a6035596510252b66`。只驗單圖，無necessary／optional或新前置缺口。

原始SHA-256：ae8db4d9cb54364a8488d45acbe058e7bd264168a450c4d306d0f5b2d5304a1c；visible SHA-256：ae8db4d9cb54364a8488d45acbe058e7bd264168a450c4d306d0f5b2d5304a1c。

### 14.3/001-007 — 註冊derive入口與檢查／生成整合

Q1：#[proc_macro_derive(ReadingValue)]註冊consumer選用的derive名；derive_reading是工具Rust實作函式，consumer不直接呼叫它。
Q2a（機制）：TokenStream先parse，再拒泛型／非struct／非單欄tuple，成功借ident進quote／into；每return reject回診斷tokens，不是runtime Err或panic。所有組件在003–006已實讀。
Q2b（需要／用途）：把既有宣告的有限支援與明確錯誤交compiler，成功只加Reading impl，維持使用者原struct與取值契約。
Q3：先params空，再Struct、Unnamed且len=1；兩次match都借AST，最後name仍可借同model。通過語法形狀不替代後續Rust i32型別檢查。
Q4：完整可見entry的順序與每個return可逐一對前文資料表／reject／quote，沒有突然新增API；未編譯工具或consumer。無必要未知／問題或未讀10–13依賴。

原始SHA-256：2b35cfb875de8b7bd74bf0c8dce7addf0d302b8c6f07005656a22743e9505a8b；visible SHA-256：2b35cfb875de8b7bd74bf0c8dce7addf0d302b8c6f07005656a22743e9505a8b。

### 14.3/001-008 — consumer的實際宣告、alias／private與borrow驗收

Q1：consumer從工具crate引ReadingValue、自己寫Meters／Seconds，原library read_value使用生成Reading；Debug／PartialEq仍是另選的既有derive。
Q2a（機制）：編譯先執行工具，加impl同一型別module；執行才借欄位Copy i32。private欄位可在自己module的生成impl存取，不越過另一library的私有區塊；wrapper沒自動Copy。
Q2b（需要／用途）：回收入口的自寫宣告／保可見性／保借讀，且驗alias語法不同仍同i32型別，不以字面檢查誤拒；零／負值驗沒有自加驗證。
Q3：public Meters／Seconds從example引入得80／30並assert原值；test內PrivateReading(RawReading)可讀80／原欄位，原wrapper仍在。
Q4：可見兩項tests對0／-5、alias＋private這些條件具體支持有限性質；stdout80／30本身不顯示parser發生時點，時點依003／圖／本段流程句。未實跑或直接執行entry，無必要未知／問題。

原始SHA-256：5e519dac2f714fe3db8e089123e1e7526db41925cc29d4d8b2366c3683e45a4d；visible SHA-256：5e519dac2f714fe3db8e089123e1e7526db41925cc29d4d8b2366c3683e45a4d。

### 14.3/001-009 — 合法語法的形狀拒絕與生成後型別錯誤

Q1：五份宣告都可parse，前四為工具不支援外形，Flag(bool)則由生成impl的i32回傳契約拒絕；兩階段不等同parser壞了。
Q2a（機制）：params／Data／Fields／len各走已教reject；bool一欄nongeneric可生成self.0→i32，最後Rust報E0308。diagnostic即使指derive仍需看expected i32／found bool。
Q2b（需要／用途）：知道錯誤是在語法、支援範圍還是trait型別契約，才能修對地方；加泛型能力不自動轉bool，失敗也不能只憑Cargo非零。
Q3：隔離fixture用相對該manifest的../../..與../../../derive連兩依賴，check選macro_derive_bool及--locked；預期目標原因而非缺crate／工具自身编譯錯誤。
Q4：五列與完整Flag程式支持失敗分類，TOML／命令沿14.2已教fixture配置；.expected檔只知角色未開，未實跑。無必要未知／問題。

原始SHA-256：67c1038210d190f2e287a19df834a0297176280549c18f9d1cc55c91c09c4cb2；visible SHA-256：67c1038210d190f2e287a19df834a0297176280549c18f9d1cc55c91c09c4cb2。

### 14.3/001-010 — 第三單位練習與14.3小節末

Q1（小節末）：使用者struct仍是原宣告，compiler tokens→AST資訊→impl語法→compiler tokens是工具分工，ReadingValue是derive工具而Reading／read_value是原取值介面。
Q2a（機制）：model.ident經quote #name生成Millimeters impl；parser只檢查已教外形，alias交Rust型別檢查；i32可負、private欄由同module impl借讀，不增加Copy／驗值。entry在host／consumer編譯時，不是runtime值處理。
Q2b（需要／用途）：原需讓使用者保名字／可見性／其他derive並減impl維護，此工具利用已寫宣告達成；新單位不應迫使改parser或公開欄位，正好檢查該分工。
Q3：local alias i32／private Millimeters派生既有三項，與consumer Meters(80)借讀得(80,-5)，原值再可assert；無須改entry。兩package配置只按BODY，未開archive。
Q4：第三名字、alias、負數、private對照本節模型／型別與借用限定；三種impl來源仍同函式／同取值契約。完整答案closed、外連未開，未實跑。
14.3收束：解析、形狀拒絕、quote、轉換、生成後型別檢查與consumer操作皆有先教契約／當段來源；necessary0／optional0，沒有靠10–13／外部解答補必要前提。

原始SHA-256：3584d074d68863401e273030588bfbb69921142913c30f38b2b3bc022189d2d6；visible SHA-256：82a3f471ab04ec80b7b3d50de0314af2d0709be2e454bf4d5e46b604a44864ce。

### 14.4/001-001 — 搜尋API與維持合法caller的入口需要

entry 前置：本人已實讀7.9的非空片段搜尋／ASCII選擇／保原行、型別與library分工；pub／ownership亦在1–9。未讀10–13不作先備。
Q1：API是application programming interface／應用程式介面，這裡含公有名稱／型別及輸入、結果、失敗、ownership約定；不是只加pub。
Q2a（機制）：pub只令名稱可見，成功意義需另定契約；本段尚未給上限有效性與維護方法，先保存新需，操作合理待後段。
Q2b（需要／用途）：caller選大小寫與最多結果、取原行；library改內檔／加設定時，原合法caller要繼續用且搜尋意義不改。這兩項具體目標先於新API設計。
Q3：另ch14-api／ch14_api，命令repo根；沿原非空連續片段任務，新增有效結果上限，不猜新的搜尋規則。
Q4：沿用搜尋問題而非換資料領域，明示變更的是公有契約與上限；當下無必要缺口，後段須回收穩定caller／搜尋意義兩面。

原始SHA-256：a6bb3b6227a8f6236b8b013f4e2d037e34f1b15eb41e425cdcd0f359d448c45e；visible SHA-256：a6bb3b6227a8f6236b8b013f4e2d037e34f1b15eb41e425cdcd0f359d448c45e。

### 14.4/001-002 — 輸入／成功／失敗／owner的明訂契約

Q1：BatchLimit／Options是caller設定契約，Hit清單是owned文字結果；成功零筆與無效零上限不同，沿7.9已有搜尋／I/O分層。
Q2a（機制）：本例明訂limit1–1000、預設case-sensitive／100；search借query、全文、Options，回最前符合行且最多limit，保原行號／空行計數／大小寫／順序。這是需由後段型別／實作支持的契約，尚未展示Default trait或建構方法。
Q2b（需要／用途）：原caller要選有效上限／保原文且能維護合法用法，先把可依賴的政策寫明；數字範圍是本API選定需求，不推成普遍最佳值／效能保證。
Q3：空query先EmptyQuery、不讀檔；真正讀檔失敗Read保io::Error，caller分類／借原因而不parse診斷文字；忽略case仍只ASCII。ownership可由表直接讀出，不需猜borrow回傳。
Q4：七列條件含0結果／0上限、Ä／ä與錯誤原因，能分輸入政策、正常資料及失敗；private／Default／feature各自未算整體相容性證據。Default目前只名稱，必要操作留後段，無當下缺口。

原始SHA-256：20c074b87350924f64721085ac42a5bdcd2dce0a810f3a1cb274c35c8236e1ce；visible SHA-256：20c074b87350924f64721085ac42a5bdcd2dce0a810f3a1cb274c35c8236e1ce。

### 14.4/001-003 — invariant、non_exhaustive與有效Default

檢查點A：private newtype／invariant。
Q1：invariant是所有有效BatchLimit保持1–1000，不只某次new；newtype由區別身分增加有效數值的契約。
Q2a（機制）：私有usize阻外部直造／修改，new先拒Zero／TooLarge，get借讀；明選Clone／Copy複製有效數字仍有效。crate內仍須自查所有建立入口。
Q2b（需要／用途）：回應002的有效上限政策，讓公開使用限制到可驗證入口，不將pub／private本身當資料永遠正確的證明。
Q3：未知數字走new Result，固定2可expect／get；錯誤保存過大原值。fmt／std Error來源沿8.6，省略imports可由已教來源補，不是新API。
Q4：建構分支、private欄位及文件例直接支持對外有效入口；copy不新增未知值，未實跑doctest。

檢查點B：enum non_exhaustive。
Q1：整體標記預告未來variant，和private不同；目前Zero／TooLarge外部仍可建構。
Q2a（機制）：本crate可窮舉現有variant，外crate分類須留其餘分支，讓新增variant不要求舊caller全改match。
Q2b（需要／用途）：API要增加錯誤種類仍讓合法分類caller持續用，先建立外部不可封死集合的約定，不假稱所有相容性都已保證。
Q3：library Display目前兩分支可行，外部需接已知原因及wildcard；未自行補struct形式的non_exhaustive規則。
Q4：source明分內／外及已知variant建構，足以讀出該attribute的有限作用，實際失敗fixture尚未在此展開。

檢查點C：Default各入口守條件。
Q1：std::default::Default由prelude提供，要求Sized，default()->Self無receiver／input／Result；Default不是數字必零。
Q2a（機制）：derive用欄位預設、不呼叫new，usize=0／bool=false；BatchLimit因而錯。手動Self(100)保有效固定值，新增入口仍需audit invariant。
Q2b（需要／用途）：002已承諾預設100，所有caller入口都應有效；同時保持具体預設意義，100改10即使編譯仍過也會改搜尋結果數量。
Q3：錯設計fixturedefault得0後assert失敗；正常impl回100不需驗未知輸入。ordinary文件///語法沿7.2，此例只知固定2的用法。
Q4：可編譯但runtime失敗的反例與正版本對照揭露new之外的漏洞，不能把default trait名或derive成功當驗值證據。003以前Default的待教角色在此明教，無necessary／optional／10–13前置。

原始SHA-256：74967d01068b52a77311d79609a2703b8a6bf7a07c6a3a99064cdd4d47b91602；visible SHA-256：74967d01068b52a77311d79609a2703b8a6bf7a07c6a3a99064cdd4d47b91602。

### 14.4/001-004 — owned mut self設定鏈與值getter

Q1：Options私有兩欄、公開設定／getter；derive Default使用bool false及已教BatchLimit手動100，符合本例兩項政策。
Q2a（機制）：mut self仍按值接手，只讓內部binding可改；修改後回Self可連鏈，不是&mut self。with_limit只接有效BatchLimit；getter借讀Copy bool／limit，Options自己只有Clone未選Copy。
Q2b（需要／用途）：caller從預設選設定而不用碰欄位／持mut reference；library可增內部狀態不要求caller補struct literal，但仍須保方法型別、預設、成功條件與公開trait能力。
Q3：default後先with_ignore_case，再把所得Self交with_limit；原binding已move，最後值可借search。來源是已教owned receiver加當段mut binding規則，不猜隱式clone。
Q4：四個method簽名／body支持資料交接與invariant；getter沒承諾欄位名／排序／memory layout，private也不是整體相容證明。無必要未知／問題，維護整體驗收仍待後段。

原始SHA-256：1c4068b2a960f74995e9b38cb662acd15b26050e166879d72b8e751b99b372e3；visible SHA-256：1c4068b2a960f74995e9b38cb662acd15b26050e166879d72b8e751b99b372e3。

### 14.4/001-005 — typed錯誤／source、被動Hit與有界搜尋

檢查點A：錯誤分類與原因。
Q1：SearchError non_exhaustive提供EmptyQuery／Read分類，Read擁原io::Error；Display為本層人讀context，source借出原因，沿8.6的外ref借self／原因型別static。
Q2a（機制）：EmptyQuery source None，Read Some(error)；caller匹配已知variant再留其他分支，可沿source報告，不靠固定文字parse。
Q2b（需要／用途）：回應002的可分類失敗／原原因與可維護API，不讓改文案迫使程式判斷壞掉，future variant也預留處理。
Q3：caller保error有效才能借cause；std Error方法／fmt／io等來源沿已教，沒有新錯誤trait或生命期規則。
Q4：Display對Read只說外context而source留原錯誤，實作直接支持分工，未實測。

檢查點B：public結果與搜尋承諾。
Q1：Hit是public line_number／owned String的被動資料，caller可組它；不像BatchLimit保驗證invariant，增加Hit欄會影響外部literal。
Q2a（機制）：沿7.9原index／ASCII／contains，命中存原line，push後恰到valid limit才break。最短推論：limit>=1、hits每次加1，故會到上限而不跳過；owned文字不借已結束輸入。
Q2b（需要／用途）：保最前M筆的原意義／順序／文字，同時允許caller直接讀組結果，代價是public欄位形狀也成相容責任。
Q3：search借三輸入得Vec；read_matches先拒空query再read_to_string，Read保Err，成功借local contents交同search取owned結果。0命中正常，不把0上限當有效。
Q4：可見push／break與兩入口validate順序對上002每條成功／失敗承諾；不宣稱public結果所有手造值都經驗證。無necessary／optional，未看其他source或實跑。

原始SHA-256：0db0913f84263222b60f39efc6bdec048fb26d679cc0865eac9ef57eb2c5edfb；visible SHA-256：0db0913f84263222b60f39efc6bdec048fb26d679cc0865eac9ef57eb2c5edfb。

### 14.4/001-006 — 重導出穩定公有路徑與feature的介面範圍

檢查點A：re-export。
Q1：pub use重導出已有public item，Root Options仍同型別，mod options私有路徑沒有因此公開；不是生成新wrapper／複製定義。
Q2a（機制）：外部用ch14_api::Options等root項目，internal module保私有；搬內檔可更新root來源而保對外名字／契約。
Q2b（需要／用途）：直接回應001的內檔安排可變而合法caller仍用；若舊module已public，舊路徑也是caller依賴，新增root入口不自動使刪舊路徑相容。
Q3：mod options配pub use options::Options；其他types／search／read_matches同一對應規則，名與能力沿前段，未打開實際檔。
Q4：root程式對比private module與public出口，足以追同一item兩條路徑；沒把pub use當行為不變的單独證明。

檢查點B：text feature。
Q1：text預設啟用時加render_text(&[Hit])->String；關閉則函式不在該次API，基本搜尋仍在，沿9.1 compile-time cfg。
Q2a（機制）：text module／root re-export皆cfg(feature="text")，core出口不受它控制；不是runtime選項。
Q2b（需要／用途）：另提供7.9同款文字表示而使caller知道可用功能與建置契約；不把feature開關保證成所有caller相容。
Q3：feature可用才import／呼叫render_text，借Hit排行號:原文加LF；manifest／body明留下一段，当前不判完整配置已驗。
Q4：兩個cfg位置與無条件core列表直接支持介面分工；沒有未讀10–13前置或必要未知，未實跑。

原始SHA-256：1f7601022ececc2d4d36c86b6c05f826cc09d32c5659cf52626f2b7ae86fa1a1；visible SHA-256：1f7601022ececc2d4d36c86b6c05f826cc09d32c5659cf52626f2b7ae86fa1a1。

### 14.4/001-007 — feature矩陣、外部caller與typed原因驗收

檢查點A：可選報告保持基本工作。
Q1：text只增加borrowed Hit→String報告，不改搜尋／預設／owner；render_text按傳入順序格式化，不排序（006的「排」是排版）。
Q2a（機制）：default=[text]、text=[]；root與caller報告block都cfg，關feature仍編譯core。Cargo可合併功能請求，功能宜相加而非互斥／取消原能力。
Q2b（需要／用途）：外caller需要已承諾的基本搜尋與可選表示，改default能力／刪feature名也可能破壞使用者，不只看現在公有函式列表。
Q3：default false／100先顯示，再owned setters到true／2；RUST對原文前兩命中行1／3，行4不收，options仍可讀2，hits只借render。關text前四行相同，重新features text再報告；單feature all等於text。
Q4：外example的root imports、完整TOML與輸出／兩命令對應feature可用性和保原文；只按文字預期核，未實跑或假稱任意dependency graph都被驗。

檢查點B：外部錯誤分類。
Q1：LimitError／SearchError已知variant可construct／match，但外部分類保其餘分支；context與cause typed kind各不同角色。
Q2a（機制）：0／1001分兩limit原因，空queryEmptyQuery；人工Read(PermissionDenied)保原io原因，source借出、match &error借typed原因，兩處仍留未來分支。
Q2b（需要／用途）：穩定可控制材料驗公開原因契約，不依真檔權限／OS文字；程序不需parseDisplay來判動作。
Q3：std::error::Error import使source方法可用，while borrower顯示cause、kind；outer error存活，不當成static引用永遠有效。
Q4：可見三種分類及PermissionDenied構造對上所需條件，外層中文字／原平台措辭不是固定解析格式。無必要未知／問題。

原始SHA-256：cf29c6932ac65250dd4ba95fd1cdeb3e81f623913681c4a2a0c753227ae6cd74；visible SHA-256：cf29c6932ac65250dd4ba95fd1cdeb3e81f623913681c4a2a0c753227ae6cd74。

### 14.4/001-008 — 真正跨crate的三類拒絕

Q1：外部負例檢查API邊界，分別是私有tuple建構、non_exhaustive分類、私有module路徑；不是把library型別重寫到本地再測。
Q2a（機制）：直接BatchLimit(0)預期E0423，外部無其餘分支match預期E0004，options路徑預期E0603；enum在定義crate可窮舉，所以本地複製不能驗外部規則。
Q2b（需要／用途）：已教正確new／wildcard／root import，需要真caller crate才能判公開承諾與拒絕，不能只以錯誤編譯或同crate現象代答。
Q3：fixture用path依真正library，獨立[workspace]／lock／literal欄位；manifest-path及example選目標已在14.2實讀，本文再明起點。三命令要對應目標E，不修為正常綠燈。
Q4：三完整程式、manifest節錄及對應診斷給足操作／原因的對照；「把enum搬進同一main裡，就測不到」明示這項邊界條件。未開fixture實檔／編譯／其他BODY，無必要未知或問題。

原始SHA-256：c334aa6c24285758b4013bc9902c9fcb53c2b91d4ef6343751ef2f7f11133ecf；visible SHA-256：c334aa6c24285758b4013bc9902c9fcb53c2b91d4ef6343751ef2f7f11133ecf。

### 14.4/001-009 — 同caller的兩種相容性、bin路徑與SemVer

檢查點A：source／behavior compatibility。
Q1：來源相容是舊caller不改仍編譯，行為相容是結果仍符合原約定；不同於private一項自動證明全部。
Q2a（機制）：縮小Demo只Default／bool getter；1.1加private limit／getter且保false，原caller無須補literal。default改true仍可編譯但assert失敗。
Q2b（需要／用途）：直接回收001的內部變更與原合法caller／搜尋意義兩面，不同步改caller去適應新版而掩蓋破壞。
Q3：同caller default後assert !ignore_case並印false；兩版源與該結果可依可見body推得，未實跑。
Q4：兩版本、一caller及改true的反例支持這個具體契約，縮小不含完整搜尋／所有trait；public欄新增可能破literal，不能事後隨意藏。

檢查點B：同一實體caller的target。
Q1：[[bin]]指定binary target的name／path，不要求src/main.rs；source明說共用consumer.rs。
Q2a（機制）：兩manifest目錄v1／v1_1各../consumer.rs回同父目錄；同package名保持api_contract_demo library import，version分1.0／1.1。
Q2b（需要／用途）：確保比較使用未改caller，隔離library變更，不以兩份順便改好的consumer作相容證據。
Q3：兩份獨立workspace由manifest-path各run同target；路徑相對manifest，不是repo command目錄。
Q4：TOML與命令的路徑提供最短身分對應；我未親查consumer.rs實檔，這是BODY來源／已教路徑推論，非文件hash／實測證明。

檢查點C：SemVer承諾。
Q1：semantic versioning／語意化版本用major.minor.patch；1.x以後patch修回原契約、minor增相容能力、major處理不相容，Cargo不自動證明。
Q2a（機制）：刪名／必填參數／縮型別可破source；預設100→10可破behavior；預先non_exhaustive減match風險但新失敗意義仍需說明。feature也是公開承諾。
Q2b（需要／用途）：把已判相容性寫成維護／更新承諾，不讓依賴者只憑數字以為所有變更安全。
Q3：修行號可patch、保契約加private欄可minor；本repo0.1.0與1.x示範分開，Cargo通常0.x按左首非零界線，不將0.1／0.2或所有0.0.x當同範圍。
Q4：七列列出具體影響而非只標版本名，支援判斷與限定；不宣稱這一caller證明任意修改相容。無必要未知／問題／10–13依賴。

原始SHA-256：3916dadce417c6e634f2119295468d894e10380fbb5e957b7b75d7b32f478c96；visible SHA-256：3916dadce417c6e634f2119295468d894e10380fbb5e957b7b75d7b32f478c96。

### 14.4/001-010 — 發布前檢查與本機打包

**檢查點 A：外部 caller、features 與工具鏈的檢查範圍。**
- Q1：這段把先前承諾轉成外部 caller 的檢查矩陣；`tests/public_contract.rs`「是真正的外部 crate，只走公有入口」，因此不會用內部可見性掩蓋公有入口問題。
- Q2a（機制／性質）：Cargo 在此 package 根目錄執行 integration tests，故 `Path::new("fixtures/note.txt")` 能指向提供的材料；正常、`--no-default-features`、`--all-features` 分別覆蓋先前 14.4 已教的能力組合。test／clippy／doc 的用途沿已讀 1、7、9，文件建立「不代替 doctests 或 caller 測試」。
- Q2b（原需／改變／用途）：原需是內部改動後仍維持 caller 的輸入、結果及錯誤承諾；加入邊界、default、ASCII、原行資料及錯誤來源等具體檢查，讓維護者能發現契約偏離。最低支援版本也要核對，不能只以自己機器編譯成功代表支援範圍成立。
- Q3：前段 feature 表與三種組合直接對應，檔案案例的工作目錄也明說；沒有需要猜的路徑前提。正文未展開 tests 內容，我只理解應檢查的範圍，沒有驗證其實作或執行結果。
- Q4：必要新前提足夠；integration tests 的 package 工作目錄由當段提供，1–9 的 Cargo 與版本背景可銜接，未用未讀 10–13 補答案。

**檢查點 B：`cargo package` 與打包檔案邊界。**
- Q1：`cargo package` 是本機組裝待發布材料的檢查動作；「組成 `.crate` 壓縮檔，再解開並編譯檢查」，這一步有可核對的產物。
- Q2a（機制／性質）：`--list` 只列預計打包檔案，不建立壓縮檔；`--allow-dirty` 允許未提交變更參與本機打包，並不把變更提交或發布。正常打包預期產生 `target/package/ch14-api-0.1.0.crate`；nested package 不納入這個 package，共用外層 `consumer.rs` 仍會列入。
- Q2b（原需／改變／用途）：原需是發版時 caller 實際拿到足夠且正確的材料；先列清單再組裝、解包編譯，可檢查 src、examples、tests 及 fixtures 的邊界。registry 發版另需公開名稱、授權等 metadata 與 release notes，與網站發布、本機組裝各有用途。
- Q3：本機打包、列清單、允許 dirty 三個動作各有明確差別，與共享 consumer／nested snapshots 的示範關係足夠。打包清單及成功產物是正文預期，未由我開 archive 或實跑確認。
- Q4：無必要缺口；此段給了新命令的動作、旗標、產物與限制。`publish = false` 與本機組裝並列，不必自行猜 registry 上傳流程。

**單位關係與當下判定：** 前段先分清來源／行為相容，這段接到正常測試、features、最低版本及打包材料的維護動作；必要缺口 0，可選改善 0。工具中斷後核對當前仍是第 33 段，未保存成功、未開下一段；此處保存中斷前已形成的上述理解。

原始SHA-256：aca3da144a764c0dd962328fd45bcc49b69b55b219b94f4f9d35f19c5f0a518f；visible SHA-256：aca3da144a764c0dd962328fd45bcc49b69b55b219b94f4f9d35f19c5f0a518f。

### 14.4/001-011 — 上限練習與公開契約小節末

- Q1：練習把有效值建構、Options 的 caller 選擇、搜尋結果契約合在一起；`BatchLimit::new(1)` 得到合法上限後，可從預設 Options 用已教的 `with_limit` 設定，再搜尋 `Rust`。
- Q2a（機制／性質）：已讀搜尋碼先 enumerate 所有原行，符合後才 push，再以命中數達上限時停止，所以這筆仍是原行號 2、原文字「Rust第一行」；上限 1 改命中筆數，不重編原行號。Result 的解包與 assert 沿 3、7 的前文，無新 API 前提。
- Q2b（原需／改變／用途）：caller 原需是只取第一個符合行；新增明選上限 1 的操作能滿足它並保留既有預設 100。引文「前者改未選設定的行為；後者讓原預設與原caller保留」也把 default 變更與相容新增分開。
- Q3：最短推論是第 1 行 Python 不符合、第 2 行 Rust 首次符合→存原索引加一→命中數 1 達上限即停，第 3 行不再加入。只依已讀 BODY 推得，未看完整參考程式或外連、未執行練習。
- Q4：小節必要前提可由 1–9 及本節已教部分串起：私有型別保不變條件，Default 走有效路徑，setter 接合法型別，公有重匯出與 features／錯誤是契約，來源相容與行為相容各有驗證。必要缺口 0，可選改善 0；實際 tests／打包是否成功仍未驗證。

原始SHA-256：d74a2716171a05c628d2ea95b4a38617f8f171b05b2122c761f2dbc9b6dbf416；visible SHA-256：d74a2716171a05c628d2ea95b4a38617f8f171b05b2122c761f2dbc9b6dbf416。

### 14.5/001-001 — 性質測試的原需與必要前文入口

- Q1：property-based testing 是先定義一類合法輸入應滿足的性質，再讓工具生成多份材料逐一檢查；引文「先寫出一類合法材料都應成立的性質」界定了測試形式。
- Q2a（機制／性質）：生成器增加不同長度、重複及鍵組合的案例，但正確答案與合法域仍由人定義；工具「不會從實作自行知道需求」，有限材料也不等於所有輸入已驗證。此段未要求我先懂 proptest 的具體 API。
- Q2b（原需／改變／用途）：4.3 固定 Rust 兩次、Python 一次的案例不能覆蓋許多不同組合；改用自定性質與生成材料，能檢查有無漏算或把次數算到錯鍵，同時避免拿被測實作當正確答案。
- Q3：前提是真正已讀的 4.3 HashMap 按相等鍵累加，以及 i32、Vec 與測試背景；換鍵型別只讓材料容易讀，沒有更換分組規則。本段的「縮減」尚未講機制，但此刻僅是節題預告，未用它作推理前提。
- Q4：本節入口沒有必需的未讀 10–13 知識。我的 4 章背景是 v5 獨立銜接審閱加 v7 局部覆核，非當時 4 的 blind first；此處依實際已讀分組正文即可理解。必要缺口 0，可選改善 0。

原始SHA-256：3ebb7eb3106c59762b4634c7645b027907e2fbfebc2c2553e671df17c605b7d2；visible SHA-256：3ebb7eb3106c59762b4634c7645b027907e2fbfebc2c2553e671df17c605b7d2。

### 14.5/001-002 — 獨立 oracle 與 HashMap 的 values 借讀

**檢查點 A：計數性質與 oracle。**
- Q1：oracle 是測試的判定依據；這裡用原輸入中的相等項目數，而不是再呼叫被測 `frequencies` 當答案。引文「就只是在檢查答案等於自己」說明自我比較的問題。
- Q2a（機制／性質）：正數群組＋總和等於輸入長度可抓漏算、多算或零群組；逐鍵直接從原清單計數可再抓分錯鍵，單看總和沒有這個能力。`[0, 2, 0, -3]` 的四筆對應 0→2、2→1、−3→1，空輸入則空 map／總數 0。
- Q2b（原需／改變／用途）：原需是上一段的「沒有少算、沒有算到別的鍵」；把它分成兩類性質並採原清單作獨立判準，能增加生成材料而保留同一需求，避免被測實作與預期值共有同一錯誤。
- Q3：`get_mut`／`insert`、Eq／Hash／Copy、借用迭代與 Option match 沿 4、6；新鍵 i32 的累加流程可直接對照 4.3。`count_of` 的 None→0 只回報數字，不會插入零群組，所以報告鍵 1 的 0 與 map 裡沒有鍵 1 不矛盾。
- Q4：此處已把工作、固定材料與預期輸出說清楚；生成器尚未出現，不需要猜它才能理解判準。必要缺口 0。

**檢查點 B：`counts.values()` 與總次數。**
- Q1：新方法 `values()` 讓程式巡訪 map 裡所有計數值，供 `total_count` 借讀加總。
- Q2a（機制／性質）：shared 借 map 並回傳 Item=`&usize` 的 iterator，順序未固定；`*count` 複製數字再加到 total，不移走值或改 map。這裡只針對 frequencies 結果，正文限定「這些次數的和不超過原清單可表示的長度」。
- Q2b（原需／改變／用途）：原需是核對所有群組是否合計回原輸入筆數；巡訪 value 就能加總，不需鍵或固定巡訪順序。它補上前一檢查的總量關係。
- Q3：借 map→借每個 value→複製 usize→加總的推論只用已讀所有權／迭代；示範的總數 4 與原長度 4 可手算，未實跑確認。
- Q4：新方法的來源、借用、Item 與順序性質均在當段提供，無必要缺口；可選改善 0。

原始SHA-256：ec9916abd231f4bb0d186e65928c8d732a4851ff8793d8123276724473833727；visible SHA-256：ec9916abd231f4bb0d186e65928c8d732a4851ff8793d8123276724473833727。

### 14.5/001-003 — strategy、proptest 測試語法與 case 設定

**檢查點 A：生成描述與合法域。**
- Q1：strategy 是描述生成型別／合法域並知道如何簡化資料的規則，「不是已經生成的一份資料」。`vec` 在這裡回傳 VecStrategy 描述，不等於直接建 Vec。
- Q2a（機制／性質）：`-3i32..4` 描述 −3 到 3 的 i32；`vec(..., 0usize..32)` 描述長度 0 到 31 的 Vec<i32>。新 literal 後綴直接指定型別，`3_i32` 的底線形式也由當段明說；range 不含上界沿 1.7。
- Q2b（原需／改變／用途）：原需是增加不同長度／重複／鍵組合但保留合法材料；把元素域和長度域分別寫進描述，runner 才能反覆生成本次計數工作適用的清單。最多 31 筆也讓小計數不 overflow。
- Q3：dependency 放 dev-dependencies 的測試角色沿 7.1，std feature 及關閉預設能力／仍可能被其他依賴合併沿 9.1；無須猜 fork 或 timeout 才能理解此例。如何具體縮減尚未展示，當下只知道 strategy 也提供簡化規則。
- Q4：必要型別、域及描述／資料差別均足夠；縮減步驟是合理待教，非此刻阻塞。

**檢查點 B：macro 形成真正測試與 Config。**
- Q1：`proptest!` 把宣告轉成 Cargo 可執行的真正 test；`fn test_name(values in strategy)` 令 values 每次取得生成材料。
- Q2a（機制／性質）：`in` 是 macro 自訂輸入形式，不是一般 fn 型別參數語法，與 14.1–3 已教語法輸入／展開階段可銜接。`Config::with_cases(64)` 只改成功的新 case 目標，其他沿標準設定；block 開頭 `#![proptest_config(config)]` 是這個 macro 接受的設定形式。
- Q2b（原需／改變／用途）：原需是把性質交給工具反覆檢查；macro 連起生成、body 與失敗縮減，Config 指定本次案例目標。64 指輸入案例，不能拿 Cargo 顯示的 test 數量當生成 case 數。
- Q3：提供的名稱表與 explicit use 能逐一對上所需入口；未把整個 proptest prelude 當作已教，也不需開 macro 原碼才能使用它的當段契約。
- Q4：必要輸入形式、設定位置及 case／test 差別足夠；無必要缺口。

**檢查點 C：把 case 失敗交回 runner。**
- Q1：`prop_assert!`／`prop_assert_eq!` 分別檢查條件／相等，失敗交回 runner 以便縮減，最後仍讓真正 test 失敗。
- Q2a（機制／性質）：本節 assertion expressions 取值，只傳 Copy 的 usize；不推定它會保留 owned Vec／String。格式訊息沿已讀 format 規則。
- Q2b（原需／改變／用途）：原需不只有發現錯誤，也要把錯誤材料縮成可讀反例；將 case 失敗交給 runner 才能進入工具提供的縮減流程。
- Q3：此段只給使用契約，尚未列 runner 的具體縮減過程；我未推測內部算法，足以辨認下段測試中的 assert 角色。
- Q4：無必要缺口，可選改善 0；尚未實跑測試。

原始SHA-256：65dc3d6044c68943b6cebccdc7e7e3aeb940ec55a5c35ee42c4fd0808004315e；visible SHA-256：65dc3d6044c68943b6cebccdc7e7e3aeb940ec55a5c35ee42c4fd0808004315e。

### 14.5/001-004 — 兩項性質的分工與 case 數量

- Q1：兩個真正 test 分別核總量／正數群組、逐鍵直接計數；正文「七個鍵涵蓋本策略的全部鍵域」把 −3..4 的迴圈與元素 strategy 對上。
- Q2a（機制／性質）：生成的 values 借 `as_slice()` 給 frequencies；第一 test 只借 values／counts 看正數及總和，第二對每個 key 從原 slice 逐筆比較並增加 expected。兩個 test 各用 64 成功新 case 目標，生成可能重複，重播及失敗縮減不混入此成功 case 數。
- Q2b（原需／改變／用途）：原需既要沒漏算也要沒分錯鍵；重複值被 insert 1 覆寫會讓總和偏小，0 的次數挪到 1 可能保住總和但逐鍵會失敗。兩項性質回答不同需求，單項通過不能代替全部規格。
- Q3：最短已教推論是 strategy 的鍵域只有七種→逐鍵迴圈全涵蓋→直接計數的預期不引用被測答案；配合正數群組與總和可防額外次數藏在域外鍵。`usize` 累加最多 31 與上段限制一致；macro、Config、prop_assert 直接沿上一段。
- Q4：程式與原需、材料域及 assertion 的連結足夠，必要缺口 0，可選改善 0。Cargo「應顯示」兩項通過是正文預期，未由我執行；沒有把有限且可能重複的 64 case 當窮舉。

原始SHA-256：69cb9f3be4956b654b659b5a78ac52fb7d517c4916ffa6cf60009657d16bccc2；visible SHA-256：69cb9f3be4956b654b659b5a78ac52fb7d517c4916ffa6cf60009657d16bccc2。

### 14.5/001-005 — 隔離覆寫負例、重播設定與保持失敗的縮減圖

**檢查點 A：`Just` 與隔離負例。**
- Q1：覆寫版本每次 `insert(*value, 1)`，已有鍵的次數被取代；隔離 fixture 故意讓 test 失敗，正常 library 仍用累加版本。取代規則沿實讀 4.3。
- Q2a（機制／性質）：`Just(0i32)` 是 strategy 建構，要求 Clone／Debug，只生成同一個 0 且不縮小這個值；外層 vec 仍生成 0–31 筆並可刪項。這是重複鍵專用域，不是前段七鍵域。
- Q2b（原需／改變／用途）：原需是看清第二次出現被覆寫的問題；固定元素為 0 讓錯誤焦點落在重複數／長度。自有空 workspace 與正常 package path dependency 沿 14.2／14.4，把預期失敗從一般檢查隔開。
- Q3：兩筆 0→只剩鍵 0 次數 1→總數 1 不等長度 2，可由已讀 insert 規則直接推得，無需猜被測程式答案。fixture 路徑、目標與命令已列，但未開其完整檔案或實跑。
- Q4：負例的材料域、生成值與作用足夠；新名稱的精確 import 位置有下述可選入口問題，不阻塞此處判準理解。

**檢查點 B：固定 seed 與 failure persistence。**
- Q1：seed 是亂數序列的起始資料；`RngSeed::Fixed(20261014)` 接 u64，本負例用來讓生成可重播。failure persistence 則是保存失敗供重播的安排。
- Q2a（機制／性質）：config.rng_seed 欄位選固定起點，同版本／策略／設定才有重播承諾，換版本不保證序列相同；failure_persistence 的 Option 設 None 表示沒有保存器，故這份故意失敗例不生成新的記錄檔。函式中的 mut 欄位指派與最後回傳沿已教 Rust。
- Q2b（原需／改變／用途）：原需是負例能重播又不把教學故意失敗留下新記錄污染一般測試；固定 seed、關保存只套在負例，一般正確 tests 保留隨機 seed 與預設保存行為。
- Q3：種子、保存器及 case 目標的角色各自明說；我未推斷工具內部亂數算法或自行實作保存器，也未驗證固定版實跑輸出。
- Q4：設定含義足夠；`Just`／`RngSeed` 來自本段 proptest 介面，但當段沒有其精確 module 路徑或 use，前段 explicit use 也不含兩者。能理解已提供 fixture 與命令，若自行重建這些片段仍無法只由已讀內容寫出該 import；記可選改善 O1：在首次使用旁補兩個入口的 use／完整路徑。未列為必要阻塞。

**檢查點 C：shrinking 與原位圖。**
- Q1：shrinking 是找到失敗後依 strategy 試較簡單材料、重查同一性質，並只保留仍失敗的候選；不能縮出合法域。
- Q2a（機制／性質）：原位 SVG 的紅框 `[0,0,0,0]` 標總次數 1≠長度 4，箭頭「試更短材料」到紅框 `[0,0]` 的 1≠2；再試一筆到綠框 `[0]` 的 1＝1 通過。底部「保留兩筆[0, 0]作反例」對應一筆已不再呈現錯誤，箭頭表示嘗試而非全部候選被保留。
- Q2b（原需／改變／用途）：原需是不用先分析長清單就能看懂失敗原因；兩筆仍失敗的反例直接顯示相同鍵第二次被覆寫，而不是任意刪到空清單。
- Q3：當段明記「手工示意」，沒有聲稱工具真走 4→2→1；minimal failing input 也不保證唯一或數學最小。本人由同 v3 原位 SVG render 成 PNG 後 view_image，圖框、箭頭、判準與相鄰文字可逐一對應；沒有看整頁／桌面手機實頁。
- Q4：縮減與需求關係足夠，必要缺口 0；先前 strategy 簡化規則的待教部分已在此補清，不回改前段初判。SVG SHA-256 `3504ea9bb7f1ab23579b22fde41b36b9d220c3385bbb86a78d232887ce0b9eb7`；native PNG SHA-256 `1cd65688bc743655e86713d8621d11511e78a113fd5bb2c50258a2397cf9bb1e`。

原始SHA-256：d5281e1ca9686755585aac01e42c7f6c291047ded08d997ac1cecc9aa0bdfd89；visible SHA-256：d5281e1ca9686755585aac01e42c7f6c291047ded08d997ac1cecc9aa0bdfd89。

### 14.5/001-006 — 具體 regression、seed 重播與生成域的界線

- Q1：regression test 保存已知錯誤的具體反例，防同一錯誤日後再出現；此處普通 test 用 `[0, 0]` 核對一組、鍵 0 次數 2、總次數 2。
- Q2a（機制／性質）：proptest 預設保存重建材料的 seed，之後先重播再生成新材料；版本／策略變動仍可能讓同 seed 生成別的材料。直接 literal 固定反例不依賴生成序列，而正常 frequencies 已回到相等鍵累加。
- Q2b（原需／改變／用途）：原需既要保住剛找到的覆寫錯誤，又要繼續探索新材料；具體 regression 和 seed 記錄、新生成材料各有角色。空清單、兩筆 0、−1000／1000 的固定邊界也補上當前小域策略未隨機涵蓋的材料。
- Q3：引文「它們存的是重建材料所需的seed」與上一段固定 seed 的版本界線一致；由此可推出 seed 記錄不能完全代替具體反例。三項普通 regression＋兩項 property＋尚未開的完整練習一項，是正文的六項正常 tests 預告，沒有把負例算成正常通過。
- Q4：必要關係足夠，無新阻塞；沒有從 −3..4／0..32 測試推論所有 i32 或任意長度正確，增加域須另依契約選性質。兩條 Cargo 命令的目標角色沿已教，未實跑、未開下方練習或正常 tests 完整 source；上一段 O1 的 import 入口未知未在本段補清。

原始SHA-256：19c1ab4aeaf14b75e38547ef4e3d30fec4c7a96399ab1cd5c1774d6feffc0031；visible SHA-256：19c1ab4aeaf14b75e38547ef4e3d30fec4c7a96399ab1cd5c1774d6feffc0031。

### 14.5/001-007 — 合併清單的可加性與小節末

- Q1：新性質是合併兩份清單後，每個鍵的次數等於兩邊次數相加；用已教 push 將 Copy 的 i32 逐筆放入 joined，能保留 left／right 供比較。
- Q2a（機制／性質）：left／right 各由既定小域 vec strategy 生成，三份 counts 都借 slice 計算；對七個鍵用 count_of 比較 joined 與 left＋right，最多 62 筆，加法在可表示範圍。沿已教 `name in strategy` 形式宣告兩個材料名，以 fn 宣告的逗號分隔框架延伸，是我的最短語法推論，尚未實跑確認 macro 的雙欄位形式。
- Q2b（原需／改變／用途）：原需是兩次分組工作合在一起仍遵守同一計數規則；可加性補上組合關係。但引文「所有查詢都回0」仍滿足 0＝0＋0，故它不能取代總量與逐鍵直接計數的獨立判準。
- Q3：推論只用已教 Vec、Copy、borrow、push、strategy 與計數 helper，未開完整參考程式或外連。這段先說同域兩份材料、保留原清單及每鍵等式，足以知道要測什麼；我沒有把尚未執行的語法推論記成編譯證據。
- Q4：小節把原需→合法域／oracle→多 case→隔離失敗→保留失敗的 shrinking→固定 regression／重播→組合性質連起來；有限材料與性質能力的界線也具體說明。必要缺口 0；14.5/001-005 的可選 O1 精確 import 入口尚未補清，保留原判。41／41 的最後 BODY 已到此，未開 14.6。

原始SHA-256：b65e8582fcddc24f239e1b78f3353af6c9454a4d7b53405f0e9703013a58ddef；visible SHA-256：b65e8582fcddc24f239e1b78f3353af6c9454a4d7b53405f0e9703013a58ddef。


## Prefix 封存範圍與整體判定

41／41 BODY 原始四問已依序保存，分布 14.1 六段、14.2 七段、14.3 十段、14.4 十一段、14.5 七段；每段有原始／visible SHA-256。own state 的 seen／saved 與 route 全部一致，current=null。每次先過濾 closed 答案才輸出，3 個 Markdown closed fold 留作 marker，0 個 HTML closed fold；結束時 filter stack 空。沒有展開答案、外連、archive 或後章。

必要缺口 0；可選改善 O1 一項（14.5/001-005）：負例首次引用 Just／RngSeed，當段與之前 explicit use 未提供這兩個新入口的精確 module 路徑／use。角色及設定契約足夠理解提供的 fixture 和縮減圖；若自行從片段重建需另定位 import，因此列可選補入口，不是必要阻塞。未受 peer／author／root 技術答案提示，也未用 source 探查補這項未知。

合理待教與最短推論保留當時紀錄：14.2/001-004 的 expr 一般形式到下一段才教清，不回填初段；14.4 導入 Default 到下一段才具體教介面與有效路徑；14.5 strategy 的簡化角色到縮減段補清。14.5/001-007 雙材料宣告的逗號形式是沿已教 macro 測試宣告框架的最短推論，未冒稱編譯驗證。

本人只在原位 hook 後核四張同版本 SVG 的 native PNG：macro-expansion、macro-hygiene、derive-pipeline、property-shrinking，逐關係證據存於各當段；圖與相鄰 BODY 的對應可理解。沒有看整頁或實際桌面／手機版面，沒有執行 Rust、Cargo、tests、package／registry 動作，未核完整 manifests／helpers／fixtures／sources；正文的預期輸出與實跑聲明不等於我的執行證據。native 圖渲染只用指定 frozen SVG，沒有混 live bytes。

入口背景只限本人實際已讀 1–9 與既有局部補讀；FIRST9 original v5 全 27、v6 兩段 local 的封存檔／sidecar 維持原 SHA。第 10–13 未實讀，不以它們作必要知識；各小節可依 1–9 及本輪前文接起，未遇需要停下補讀的必要前置缺口。這是 AI 模擬原始 prefix 首讀，不是整章 14 QA、真人測試或第 5／6 的 primary QA。

本檔封存前只校正本人新紀錄的少數正體字形，沒有改引文、來源指紋、當時理解、未知或分級；先前封存紀錄一律未寫入。工具中斷發生在第 33 段保存之前，恢復後核 current／saved 確認未落盤，保存原已形成理解才開下一段。到此 STOP，14.6、任何 peer14、author／root disposition／答案、source 與其他章正文均未開。
