# 第12章全章獨立首次閱讀原判

AI輔助讀者模擬，不是實際Python新手研究。延續自己的actual1–10與FIRST11 prefix／middle路線；第12章另加本輪已完整首讀封存的11.8實際正文。舊原判、未知、分級與SHA不回填。只依所交frozen當前BODY，不把目錄或ROOT先檢當前置／本人證據。沿middle已聲明的通用規範額外可見範圍，沒有新的技術／同行／已知問題提示。

每單位CURRENT先在工具輸出前過濾???、details及選讀為CLOSED marker；只原位必要SVG到當段讀／isolated view；新概念逐項四題、Q2a機制／Q2b原需要分存、原句quotes、合理推論／猜測／未知／回讀與rawSHA保存後才NEXT。敘述保存前用OpenCC s2t字體校準，引用、API與ASCII字串保護。沒有Rust執行或實頁驗證，不開source、archive、author、checks、outline、peer、root findings、live、外部答案或未授權後章。第12章全44初判封存前不讀任何選讀，之後如補讀另階段新增證據，不改初判。

路線路徑及reading-order原始SHA：

```json
[
  {
    "snapshot": "ch12-complete-prerequisite-v4",
    "reading_order_sha256": "63eb060b72441eb2f3641fea8b33652bb638fa70cdd403d219b82d06e5c2fd1f",
    "selected_units": [
      "001-001.md",
      "001-002.md",
      "001-003.md",
      "001-004.md",
      "001-005.md",
      "001-006.md",
      "001-007.md",
      "002-001.md",
      "002-002.md",
      "002-003.md",
      "002-004.md",
      "002-005.md",
      "002-006.md",
      "002-007.md",
      "003-001.md",
      "003-002.md",
      "003-003.md",
      "003-004.md",
      "003-005.md",
      "003-006.md",
      "003-007.md",
      "004-001.md",
      "004-002.md",
      "004-003.md",
      "004-004.md",
      "004-005.md",
      "004-006.md",
      "005-001.md",
      "005-002.md",
      "005-003.md",
      "005-004.md",
      "005-005.md",
      "005-006.md",
      "005-007.md",
      "005-008.md",
      "005-009.md",
      "006-001.md",
      "006-002.md",
      "006-003.md",
      "006-004.md",
      "006-005.md",
      "006-006.md",
      "006-007.md",
      "006-008.md"
    ]
  }
]
```

## ch12-complete-prerequisite-v4 / 001-001（順序1）

SHA256 `891bbb45383d202b394398329fac5ee9db3cbb867535e3de78bcc92055db9a33`。

Q1：本節把6.2owned ExtractOutput改為保留來源借視圖，Vecslice與RefCell活躍借guard各自既有規則，GAT名稱尚待教。Q2a：目前只知同trait操作要保不同結果型別，且結果連每call借self的範圍；具體associatedlife語法未教，不用標題當理解。Q2b：原需要不同視圖型別同契約/保借來源，不是普通Vec切片不能做或要通用容器；Vecfnview已能第一需要明給。Q3：想借而非移來源須源在有效範圍，異結果後用待教；不預設dyn統一/clone資料。Q4：開場兩源/兩borrow結果足以提出新類型關係，沒有證GAT唯一解/所有容器實作或效能。引文「保留各自的結果型別與借用規則」「結果仍能連到每次呼叫」。未知GAT機制保留後段追加。

## ch12-complete-prerequisite-v4 / 001-002（順序2）

SHA256 `6cbcf8805f79520f54c6dc2760b399a69f1d121e36f3c2698b26fa65f566db7c`。

Q1 Numbers普通Vec、GuardedNumbers RefCellVec沿8.8，欲return sliceRef與Refguard兩具體視圖；package名字unsafe不代表此節操作unsafe。Q2a 某call借lifea非固定全域，slice只借/Refguard並持活shared runtime狀態至drop，兩者皆不複清單。Q2b 原需要同trait讀同類數字而保各責任，不能把guard暗換成不維借狀態reference；非為省clone產物倒推既定需求。Q3 共同後函數須能借用結果檢數字但retainguard責任，原來源仍owner；Ref來源8.8前文位置此處明給。Q4 表格支持兩結果異型同資料用途、Compilerref與runtimeguard區別，尚未給traitlife關聯做法/不能據表假定a永生。引文「尚不是固定的全域 lifetime」「不應偷偷把 guard 換成…普通 reference」。當段資料/結果與需要足夠，GAT規則仍待。

## ch12-complete-prerequisite-v4 / 001-003（順序3）

SHA256 `d64fa97fb78e1e17122e90af5b6bbddf58916329ecdf1337124ace85ba117175`。

首用GAT/typeView參數/SelfView/requiredwhere：Q1 generic associated type全名與泛型關聯型別，非單一固定Output，而實作選按本次life的View族，沿6.2與generic形式。Q2a typeView<a>宣參數/SelfView<a>帶實際life；method自己<a> &aself→SelfViewa，宣告與method的a是各自名字，使用才建立本call關係而非全call共借。whereSelf:a沿8.4T:a內借用需涵範圍，放GAT宣告；receiver已要求Self:a，而此trait的借出method使Rust要求宣告也明寫以供impl用self借型別，非值永遠活。Q2b 原需要兩種具體結果各保責任又對應每次借Self，把固定associated型變按life選而非為未知通用容器；receiver永久借不是要。Q3 依完整BorrowView宣typewhere與viewmethod，impl後每call可異a；新語法位置/作用明給，不須自己推required規則。Q4 原句/trait關聯支持每call型與borrow關係，尚未證兩impl或共同計算/所有GAT參數類，也不永久延owner。引文「給定這次借用的 lifetime，要用哪個 View 型別」「method 用 Self::View…才把本次receiver…帶進」。001001未知此時GAT/where角色補足，impl待後。

## ch12-complete-prerequisite-v4 / 001-004（順序4）

SHA256 `b5b135264a6399d51359a4b188f6dcd598dea1d83cd45a9f42c696a05bcae19d`。

Q1 兩BorrowView實作選其lifeView：Numbers&slice、GuardedRefVec，保存同一實作但calla變參數，非兩者同型別。Q2a self.values.asSlice借內容／RefCellborrowguard維狀態，impl兩whereSelfa照契約；Self無外借易符a但返回借receiver仍不可越source，非自動static。Q2b 原需要同trait操作各保read/runtimeguard責任不複清單，對每call有效範圍，這兩impl具體滿足而普通Vecalone不用GAT也可。Q3 依impl得到正確borrowresult，用完纔再mutVec新增40及重新view；guard亦須有效來源/清理規則。Q4 SVG SHA256 `35f53e602a2287524f67ecc743309ddf81b4dcdb8bb1e28e759b9b560a3dc7b3`source+390isolated看：藍sourceNumbers持80/120→藍firsta slice借source2→橘借終push40同Vec3→藍nextb slice同source3，箭頭操作序，a/b非固定長短。圖只Numbers不證RefCellguard動態檢查，要用文字/impl保該責任；源同名/Vec對應明確非另複。字可讀未實頁。引文「不能越過來源的有效性」「只標示不同借用」。機制與當下圖關係足夠，共同使用下一段待。

## ch12-complete-prerequisite-v4 / 001-005（順序5）

SHA256 `d5254247beeae6ed174675b4a7d15c7e865413dd96d4ff1cbe55bcf6f340b53a`。

首用共同borrow_view與GAT能力界線：Q1 Sbound供view方法，SView<_>借結果連唯一&source沿4.4，不統一成Vec/不取得S。Q2a 只保view非未知View可索引/Deref/追加；實際S確定才slice與RefGuard讀能力可用。plainfirst最後用200後push40/next3，guardview200顯drop清活借，再短writerpush後nextshared3，GAT未改guarddrop規則。Q2b 原需要同trait轉交各視圖和borrow有效性/責任，而非泛型函式要統一讀任output；兩資料相同讓差別可見不是為兩份新清單。Q3 呼borrowView(&plain或guarded)保源/具體結果，borrowview不須clone；guard若未drop不能獨佔writer，後用len來源仍活。Q4 四輸出配body支持來源保留、先借/後改/再借及兩清理規則，非unknownS所有Viewindex/type同/新規則加速；未run。引文「沒有保證未知的 S::View…一定能索引、Deref 或追加」「没有改變guard的清理規則」（後引原意文字，實際原句「沒有改變 guard 的清理規則」）。001001同trait原需要此時已具體連，無當下必須猜測。

## ch12-complete-prerequisite-v4 / 001-006（順序6）

SHA256 `a74216d035b0271e09b41214979356fc324c078e4c225d0c1ba737327c668995`。

Q1 兩負fixture分GAT宣告bound不足與來源生命期不足，沿本節trait規則/4.4，不新增另一種借用能力。Q2a 第一隻移typewhere而method不變→missingrequiredView，需Selfa非static；第二GAT足但sourceinnerblockdrop、slice不ownVec→E0597，保源或owned資料才各滿需要，GAT不延長source。Q2b 原需要辨同樣編譯失敗的不同原因，避免把新life語法當所有存活修方，補契約不能代保資料。Q3 可依固定移一條件／移來源scope預測各診斷角色；不開fixture完整source或執行。Q4 兩完整BODY程式與先教規則能解因果，診斷是正文宣稱非自己核編譯；沒有證明everyGATallbounds相同。原句「只移除 View 上必要的 bound，其餘契約保持原樣」「GAT 本身沒有延長資料存活」。已有前提可直接推，當下足夠不靠後文補。

## ch12-complete-prerequisite-v4 / 001-007（順序7）

SHA256 `f8155bf6518198ce94c34ce100c383b79eae2bf5e7ed1bb6b9631ee2f326d479`。

12.1末首用LendMut變化／Resultis_err：Q1 LendMut同GATView族改receiveramut/SNumbersViewamutSlice，沿已教排他/VecmutSlice，不另變owner；Resultiserr共享variantbool非拿內值。Q2a as_mut_slice從mutVec借已有元素全段不改len，改first80→100寫原Vec，短block借終push40再view3/總260；view不足條件仍同requiredSelfa，method借關係沿前原理。iserr只Errtrue/Okfalse供guard活時trywriter拒assert，借檢不消結果。Q2b 原需要讓不同call可更新來源且後續保修改／回使用權，不要把source改與複清單混；三tests原slice/guard權限/empty明對前關係。Q3 用原80/120改100/120後push40，可不看完整答案link依既有語法實作，未開referenceprogram或外連/未跑。Q4 有限mutable變化支持GAT結果借源與借終再改/再借，非通用lendingiterator或所有View操作；tests是本文範圍宣告非PASS。原句「先指出修改落在原 Vec」「它只檢查 variant，不取得內部值」。原開場同trait各保視圖/每call範圍已連，無必要缺口；下一題替換合法性尚待。

## ch12-complete-prerequisite-v4 / 002-001（順序8）

SHA256 `5ce90f1858516b5d10601562d10a525c731c3bada28ab59339cade7e12ef4610`。

Q1 合法替換是使用端需要型別時另一型別能滿全部契約，承8.4 staticstr滿短借，但Cell/函式pointer新情境待教。Q2a 替換改可依保證，不改源存活/非數值轉型；方向規則未知。Q2b 原需要不同外包型仍否可同方向替換，不能見static長就普遍縮，也不能由新變異分類倒推用途。Q3 使用端所需契約先固定，再判所給型，具體操作材料後文。Q4 開場給問題/範圍，不足以判Cell或fn方向合法；不先用外部型理論補。原句「能否滿足全部契約」「不是數值轉型」。未知保存。

## ch12-complete-prerequisite-v4 / 002-002（順序9）

SHA256 `c53e4b2a55c63e64b25593c6d9cd800a16c692cc526012b5c782f58b8e1b3fce`。

首用subtyping／mutable外借縮短：Q1 子型別能滿另一型要求，非Python繼承；long:short已有關係使長ref可短用。Q2a helper genericbody原ref直接返，sharedstr與mutablei32外借期可縮而內i32不變，不clone/alloc/延短資料。Q2b 原需要使用端short讀寫保證得到滿足，長已有有效性足，不以數字100當新life規則理由。Q3 依兩helperbound適用任符合範圍，muttemporary改原80→100，scope借終再讀amount；所給來源須活。Q4 泛型body型檢才核所有bound方向（正文聲明未自己編）；outputs僅演用途/同一amount而非life長短數據。原句「對所有符合這項 bound 的 lifetime，這個方向成立」「不以數字相同當作 lifetime 規則的證明」。當段機制/用途夠，內部T變化尚待。

## ch12-complete-prerequisite-v4 / 002-003（順序10）

SHA256 `e7599e9a10122e2a576541308fa3ae47a6af693ed177f487611778d104969451`。

首用variance／co-contra-invariance：Q1 型變問參數合法替換被外型包住後方向，須指哪參數非一整型標籤。Q2a 假long涵short，co保方向、contra反、invariance不可只原關係改參通常同；&amutT外a協變可shorten但內T不變，非mutable全不變。Q2b 原需要解同長借移入Cell/function是否仍方向成立，先分外借期間與內借型別保證，不從三新名稱倒推「學分類」。Q3 判每參數與包裝分關係，再套bound；contra機制與T限制例未教留下一段。Q4 表與前mutablehelper足以辨外a，而不證內T任變；此處只定方向角色，不提前假懂Cell/函式所有替換。原句「必須先指明正在看哪個參數」「它對被借型別 T 則是 invariant」。當前導覽足夠，理由/具體操作待後。

## ch12-complete-prerequisite-v4 / 002-004（順序11）

SHA256 `ac3002cea627438601601e878f3a9e95b3c9843fc32a95b3827990a634ed3c99`。

Q1 target借reference變數saved非文字mutable內容，&mutT內T=&staticstr，兩層對象須分。Q2a replace寫另一同T；若內T可從static縮成short，就能把localstr寫回宣稱static的saved，local終後失保證，因此T不變而E0597拒，不是所有replaceunsafe。Q2b 原需要可依使用端既定T契約而不被寫入破壞，這正是內T不能套外a縮借的理由，非為invariance名稱分類。Q3 若需short資料如實設計saved使用範圍，不能先static再寫short；body合法用途同T替換仍保。Q4 只改內層life的反證與完整負例說明破壞哪承諾，編譯拒所以未實行失效ref；自己未compile，不能說所有&mutlife不縮。原句「target 沒有直接借到文字的 mutable 內容」「寫入能力可能破壞原 T 的契約」。內T理由待教此時補足，無新必須猜測。

## ch12-complete-prerequisite-v4 / 002-005（順序12）

SHA256 `7aed2b3882a4d0fe6f40e392fac2f8a69deea50acbeeee150f7b5a430e453c88`。

Q1 CellT內部型別亦不變，shared介面仍set寫入，沿8.8非只有mutref能破契約。Q2a 同Cell另一alias若當Cellshort可setlocalref，原Cellstatic的get卻仍承static；故容器T不能只long:short直接換，ownedhelper只轉交也不改全typevariance規則。取出ref再short用非禁止。Q2b 原需要一方依原型別讀時不被另一寫入欺騙，保所有該型可依保證而非只當前body恰無寫。Q3 shortenCell fixture移容器T仍拒，想短借可get再按reference規則，不能把shared當readonly。Q4 SVG SHA256 `3d2662d35ef3cce5ab8956f77851d9efdd7dc64ba0b897dc33f578a3d6d42609`source+390已看：頂藍同Cellstatic/getstatic，中紅假設另一&Cellshort，底橘set短ref；兩向上虛線是若允改型的寫迴路徑，禁止圈在中頂之間指出替換假設被阻，不是實際失效存取/時間方向。字可讀未實頁。原句「不能因為操作時只有 shared reference 就當成只讀」「禁止的是把原容器的 T 契約直接改換」。Cell新問題機制理由足夠。

## ch12-complete-prerequisite-v4 / 002-006（順序13）

SHA256 `9321ca9e92954e7c0bcc5604241a8c061f4dcd8b562c4051173875ed047c553b`。

首用fn輸入逆變/輸出協變與adapter：Q1 沿6.3functionpointer把參數/return改ref，adapter回可呼叫型非現在usize/ref結果。Q2a 使用端每次給long，只需short函式可辦→fn(short)->usize滿fn(long)，反不行，護caller所有合法input；輸出保long可滿只需short，逆向不可。泛型body只原pointer轉交不重建/複料，reverse_input缺方向所以拒；加short:long變彼此涵蓋不是同原契約。Q2b 原需要所選callback接受caller可給的全部範圍並交保證的結果，非literal現在剛好可用就決定型變；兩方向回應原fn包ref問題。Q3 adaptInput(byteLen)→需static位置pointer再callRust4，adaptOutput(fixedLabel)→一般借輸出Rust；需按input範圍選而非照錯提示改domain。Q4 literal只演使用，genericbody才一般life方向依據（未自己compile），不能以4/字相同代替variance理由。原句「它保護的是呼叫者依介面合法交來的所有輸入」「改變原契約」。contra/輸出問題此時補足，無新未知。

## ch12-complete-prerequisite-v4 / 002-007（順序14）

SHA256 `4ca1c9d38b474639d40e4f42501b8e157239882d7c523f0dd6776c430a61bbdb`。

12.2末Q1 copiedCell新容器≠直接改原Cell型，保持子型/型變區別。Q2a getCopy sharedref long→short允，Cellnew另容器，localnote可寫新Cell但原staticRust不變，沒有複文字，Cell仍T不變。Q2b 原需要保原容器承諾又使用短借來源，另建改可寫的容器便不欺另一alias；不是證直接Cell變協變。Q3 genericboundlong涵short可copiedCell(&original)，新Cell使用需local源仍活，reference copying非ownership文字；答案/延伸連結未開。Q4 兩value原Rust/新note對照有機制理由，不因同cellliteral作一般證；整節refs外life可縮/內可寫T不變/Cellshared寫/fninput反output保各判所需契約已接。原句「兩個 Cell 現在是不同的值」「不能由此宣稱 Cell 變成 covariant」。無必要未知，未違法存取測crash，rawpointer下一段才讀。

## ch12-complete-prerequisite-v4 / 003-001（順序15）

SHA256 `1afe4cad2a5e4019b16fcbb7d2982dfa8fa1d2ff2a5fe78482ca84e1f44fd417`。

Q1 rawpointer較低層保存位置，與2.3reference把存取/borrow有效性入型、8.1Boxowned清理不同。Q2a 不由型別完成同等有效性檢查，建立/檢查/讀寫動作待分教；具體語法/條件未知。Q2b 原需要理解後安全抽象/外部介面責任，不是局部數值修改ref不能完成或raw必快；有限單thread已知來源避免不相關工作混。Q3 本節先local數/array作材料，持源責任須說明後用；不預先假定pointer有有效life/owner。Q4 開場支持導覽選擇目的與表示角色，尚不能據存位置認可以存取/隨意unsafe。原句「直接使用 reference 已能完成這些數值修改」「建立、檢查、讀寫三種動作」。未知待正文。

## ch12-complete-prerequisite-v4 / 003-002（順序16）

SHA256 `4955e82ad5b52d213b70f12935a7d22e36bb6267469d022ba21b09911fc0bc30`。

首用const/mut rawtypes、dereference、allocation：Q1 *constT/*mutT型別星與expr*pointer存位置動作不同，allocation是可識別配置範圍含local/整array/Boxheap。Q2a const不直接寫、mut可表讀寫但真權限另需；pointer值binding不必mut才能改所指料。raw可null/源終了/非T，不帶life追源、不替ownerdrop釋放配置，合法access須連回範圍非地址貌似合理。Q2b 原需要分保存位置的能力與有效存取/清理責任，避免沿ref/Box保證套raw；可寫語法不是有效寫權。Q3 日後用*pointer須檢所指來源/alloc/權限，當前尚未列齊unsafe條件；持pointScope非持owner。Q4 table/來源例足以分型/變數/alloc角色，不能只*mut就忽略只讀材料或null可讀。原句「真正寫入仍必須有合法權限」「而不是只找到一個看似合理的地址」。各新名當段足夠，如何建/unsafe待教。

## ch12-complete-prerequisite-v4 / 003-003（順序17）

SHA256 `192ca3f48244aa996de5fc67c983d727aa94901832788b89f4499d7b3f0f5d07`。

首用raw borrow/null/is_null/align_of/alignment：Q1 &rawconst/mutplace直接建相應rawtype，非先&reference讓borrow失效；alignment正常存取地址倍數要求不同sizebytes佔用。Q2a safe建指仍活已初始化locali32，nullconstT無配置、isnull只bool，alignofSizedT→usizebytes只型別要求；alignment4需起始4倍非nonnull即可。Q2b 原需要先觀察建立/最低檢查不混成讀值資格，保原sourceowner/查type平臺要求，非新i32副本或安全存取完成。Q3 建兩raw指同amount與獨null，再querybool/alignment，不dereference；起始合法來源已給，false不能代完整檢查。Q4 範例前三bool及作者環境i32align4僅此建/型查關係，自己未run／不證每平臺4／指值aligned或全部權限。原句「它們都不讀取被指向的 T」「false 不代表可以 dereference」。當下接口/意義足夠，存取條件下一段待。

## ch12-complete-prerequisite-v4 / 003-004（順序18）

SHA256 `34de7409254893151003e8d1d717b126546b54f116de9e163ed83862e459e1f9`。

首用unsafeblock／UB／aliasing／SAFETY理由：Q1 unsafe允特定低層op責任標記非關borrowchecker/自動驗證；UB違語言/API基本要求無正常語意結果保證，非必crash。alias多路徑同資料非僅兩指值即UB，要看權限/時間實access。Q2a 本locali32需nonnull/全值alivealloc/align/init有效bits/合法寫權無衝突/單thread無race，表逐來源80及mutable/不移不釋放、一指順序供理由；before/afterCopytuple後外println避免中途另borrow。Q2b 原需要把rawdereference作者責任說明清，不null或cast/加unsafe/正確輸出不能修資格；理解後安全抽象如何承擔。Q3 先source契約再unsafe讀80寫100讀100，raw不用後從amount讀；SAFETY註解列理由非因字存在才safe，省unsafeE0133只是標記缺。Q4 正例輸出80/100僅值變化，安全由來源/範圍/權限；負fixture診斷正文聲明未自己跑，不危險執行試crash/不推所有T檢查只此表。原句「各路徑在何時、以哪些權限實際存取」「加上 unsafe 只滿足語法上的責任標記」。newconcept機制/責任足夠。

## ch12-complete-prerequisite-v4 / 003-005（順序19）

SHA256 `ac8259a3c4106a07d92130c0a0b3f9fabff240d9a4dd707e5d1f6fa55ed1a032`。

首用sliceas_mutptr/as_ptr、add、onepast/isize：Q1 slice取first rawptr不own配置/改slice長，add元素n非bytes；onepast端點無新元素/非null。Q2a add形成本身unsafe，須同alloc內或exactend、byte位移<=isizeMAX有號usize對應型、不addr溢；三項i32已init/align/活，1可讀寫、len3只能形成/檢null。raw期間不從view等其他路徑access，後Copy結果外顯/原array再讀。Q2b 原需要只改array第二項且易追同配置，不手算整數addr轉ptr；分pointer形成合法與dereference資格。Q3 startasmutptr→secondadd1→120改140，endaddlen不讀，原80/140/40總260；sharedslice只能constptr按來源權限。Q4 SVG SHA256 `1e3fe4be89e831721566567d512691379bf1d99587b7fdb738f2f24ff8bdaa9c`source+390已看：三相鄰i32索引0/1/2，first/third藍80/40中橘120→140；start箭頭落第一起點、add1第二起點、add3右虛線端無第4框，箭頭定位非程序序，文字可形成不可dereference。字可讀未實頁，輸出260非安全證/未run。原句「n 是元素數量，不是 byte 數」「add 本身就是 unsafe API」。新API條件/用途足夠。

## ch12-complete-prerequisite-v4 / 003-006（順序20）

SHA256 `396a07ddc546f9b2aa904ad8edbb6d0c364000cac1a3d62f44ebcd69b2d20cb4`。

首用provenance／raw轉ref：Q1 provenance來源alloc與允範圍關係，地址僅在哪；&*ptr/&mut*ptr是建立reference非只是方便語法。Q2a 不同local即相鄰仍alloc各別，不能跨；freed舊ptr不能因新配置同addr沿用，array能位移因源涵整array。建ref時須aligned/validinitT與ref權限，非等firstuse才補，returnedlife須真正來源非static註記延local。Q2b 原需要存取理由看來源資格而非nonnull/addr/type，後safe抽象須保前置與輸出關係。Q3 路徑來自已知涵完整配置且活的源，轉ref依既有shared/mut限制；不製整數addr/失效demo。Q4 兩有意義反例adjacent/differentgeneration說明地址相同不足，非完整alias模型已定；本段表明仍未定部分，有限例理由仍清晰。原句「這條路徑從哪個有效配置衍生、能存取哪個範圍」「建立時就必須有」。概念足夠當下，wrapper後教，不假稱已安全封裝。

## ch12-complete-prerequisite-v4 / 003-007（順序21）

SHA256 `a5546c40f373988dace97a4483c705e02b19f322203e7be4b5178f0721d4f0e8`。

12.3末Q1 換第三項材料復用rawtype/來源alloc/add/unsafe，不新接口。Q2a wholemutSliceasptr源涵三項，add2第三init30→50，無其他access/raw結束才普通index；add3是end非third，byte/addr條件三i32來源已滿。Q2b 原需要驗讀寫責任理解後safe抽象/FFI，不單算80；需指出source/alignment/init/provenance/權限/時間，源80總值不能代安全理由。Q3 正常有限材料10/20/30→10/20/50、sum80，將SAFETY理由先寫，不開完整refprogram/外連，未run。Q4 移index小變化足以沿原規則判第三資格與端不可讀，assert有限已定義input結果非anyraw程式安全證。原句「raw 操作期間還有哪些存取路徑」「不代表由有限測試證明任何 raw pointer 程式都安全」。整節三動作與來源責任接齊，無必要未知；一般alias模型未定界線照留。

## ch12-complete-prerequisite-v4 / 004-001（順序22）

SHA256 `a4aa258857fa71b139c04ab2d5841c44adab23160c7c90b85756e28afd948cf4`。

Q1 可重用介面接12.3每次raw責任，問caller與型別/函數檢查誰負條件，另只讀跨thread需life及SendSync兩軸。Q2a 目前只方案問題，unsafefn/抽象/跨thread細則未教。Q2b 原需要把安全責任安排到接口/實作便審，非i32讀reference不能或split無標準safeAPI；手寫有學責任的有限目的。Q3 保i32有限模型，從輸入型別/檢查能保啥判public是否safe；不提前猜unsafe讓checker無作用。Q4 開場為導覽/需理由不證手寫更好/實作safe/所有genericT可跨，後文才判。原句「哪些可以由函式的輸入型別與檢查接手」「切分也已有標準safe API」。未知保存，原線尚未收束。

## ch12-complete-prerequisite-v4 / 004-002（順序23）

SHA256 `e99998a18ff3124dcb30444a54546573254d916e445a84d78f2981296c020753`。

首用unsafefn／Safety文件／bodyunsafe lint：Q1 unsafefn宣型別不能單獨驗的caller前置，bodyblock標操作，兩責任不同；Rustdoc/// item前#Safety契約與//SAFETY本次理由分。Q2a readconsti32→Copy副本不own/返ref，nonnull/align/全i32有效init/來源讀資格/不移釋放/無別名衝突寫等caller需先確認；作者保此前提成立body合法。2024unsafeop省內block預設lint警告非標fn就全免，site省callblockE0133非直接deref缺。Q2b 原需要重用讀取時分哪些caller仍負、author憑何op可讀，不能source值已好就跳責任標記，也非文件字自動驗證。Q3 明來源amount→rawptr、callerSAFETY/unsafecall→observedi3280與source仍80，內block憑contract；違caller條件不由unsafe語法修。Q4 正常輸出只副本關係、表資格理由另，負fixture只缺callack而非pointer必invalid；未compile。原句「在這些前置條件成立時，body的操作也必須合法」「前者說需要什麼，後者說這次操作如何滿足需要」。新契約角色/操作足夠。

## ch12-complete-prerequisite-v4 / 004-003（順序24）

SHA256 `e0389df61f9c5ec66910afcdb6445cd5d9dac585a66911566c27b7f5d6d2c787`。

首用safe abstraction/soundness/unsound：Q1 安全抽象由impl接hiddenunsafe條件，caller守type/borrow但不另證raw；健全要求合法safeclient不可由此觸UB。Q2a 若合法safeinput致impl違mem契約則unsound，非Err/crash纔算，正常tests不洗UB；單刪readfnunsafe而要求rawcaller自行保有效未接責任，rawtype仍允null/失效。Q2b 原需要可重用safe介面真正移責任到作者，不只是省callerunsafe語法，後需輸入保有效/獨佔以逐理由。Q3 判publicconstructor/type和檢查能保前置，不能只方法安全名或正常呼結果。Q4 此段基本定義/反例支持責任分配標準，下一實作未讀不可先PASS；與ordinaryErr區分已沿12.3。原句「不能因此觸發UB」「原來的安全條件仍在」。概念足夠目前，safeinput/wrapper證據合理待下一段。

## ch12-complete-prerequisite-v4 / 004-004（順序25）

SHA256 `a8a8d2ce509df1ad9078d5a6a9309379985b5828943c39980a7720e56826f860`。

首用safe split／標準checked／fromRawPartsMut／空range條件：Q1 手寫safe函式借同slice拆兩不重疊段，不newVec；標準split_at_mut_checked已可做，直兩mutindices形式E0499不表示不同元素不可獨佔。rawPartsMut unsafe(data*mutT,lenusize)建mutSlice不配置/clone/驗證。Q2a 先mid<=len防越界形成/len-mid減法，源有效mutSlice供nonnull/align/init/alloc與byte界，data源資格，兩range0..mid/mid..len無共元素供排他；4.4唯一輸入連兩outputlife，回期間不可另原slice/ptr訪。空仍nonnullalign但無可derefT要求，emptydataadd0/非空midlen端+len0合法只形成。Q2b 原需要兩段各改、原數列保改與責任由interface管理，midcheck不是所有安全證，手寫只為學審責任非標準API做不到。Q3 caller不用unsafe，mid2得到2/2改left0+20/right0+5→100/120/45/60，借終再原讀；mid5None無pointer/修改。Q4 SVG SHA256 `7b6aeb6e536ea758c2a32429c30698a60a1a9c4cdbc12b26eb1ae0865f1af6c3`source+390已看：頂四原格索引0..3藍80/120綠40/60，兩下箭頭借到left0..2與right2..4非copy，left0原idx0/right0原idx2，底同源100/120/45/60借終讀；字可讀未實頁。原句「只有mid檢查並不足以證明全部安全」「原slice在兩段被使用期間不能另行讀寫」。當下責任來源/空與端new規則均給足，未run。

## 傳輸／上下文恢復紀錄（第26單位保存前）

本次恢復先核自己的gate：saved25/44、CURRENT004-005、raw SHA256 `82988d027ebd74508641e68e3be5f3735b040836eab6954eab2b25f7ee83b24b`，record最後單位004-004。恢復前record SHA256 `603fcc73ccfd67662def20e0a3b4313328d0b39bb27be87d0485b3c752dcac08`。僅重讀自己已保存25段理解以恢復背景，沒有重開其frozen BODY、改判或提前取NEXT。CURRENT此前是否已成功送入中斷前的閱讀上下文無法由state單獨確知；此時只重取同一pending的過濾BODY以續存，不能冒稱它是一份新的盲讀。11.8尾段saved12/currentnull與原SHA `c8f1ef0667df646d8607f096867a456ee55ee9271378c3ec93305a8640f24373`保持。沒有新同行／技術／作者提示，目前無寫入阻塞。

## ch12-complete-prerequisite-v4 / 004-005（順序26）

SHA256 `82988d027ebd74508641e68e3be5f3735b040836eab6954eab2b25f7ee83b24b`。

恢復後同pending理解，非另一次新盲讀。首用unsafe trait/unsafe impl：Q1 SendSync沿10.1但手寫時屬unsafe trait額外安全契約，作者責任非加鎖/atomic/storage生命期。Q2a rawpointer無SendSync，requireSend即使僅isnull仍E0277按整type契約；unsafeimpl空markerbody由理由支持，不把const當read-only來源證。Q2b 原需要跨thread接口可靠移交/共享而非省診斷，compiler不能完整核對來源、權限、其他寫路徑，故仍由作者確認。Q3 每實作分Send（移資訊、不移referent、無thread專屬清理）與Sync（共享method只讀i32、無管理寫）證；新增寫入/任配pointer需重審。Q4 data race至少一寫一non-atomic未同步衝突是UB，兩thread全讀普通i32無此衝突，非atomic旗標才安全/unsafeimpl升RcCell能力。
首用ReadOnlyI32安全借wrapper：Q1 source &a i32保源life、rawptr同源存取兩表示，非owned配置；日常ref足夠，教private不變條件。Q2a new從sharedsource rawconst*source只指同一有效alignedinit i32；private兩欄caller不可自行錯配，read遵read_i32契約Copy值、original回來源a，i32無interiormutability且全部接口只shared。私有本身不足，new/read/內部改欄仍須核；E0597源結束仍拒。Q2b 原需要安全抽象接源責任再跨thread，讓caller合法使用不因hiddenraw成UB，不是兩個80/160證安全。Q3 scope保amount，move第一wrapper用Send，借第二&wrapper用Sync→refSend，兩join都先完成再判，source後仍80；placeholder impllife不要求static。Q4 完整body與不變條件/兩契約表支持有限i32全接口，不外推任意T/有寫容器；輸出只示用途未執行。原句「這一份i32型別與已列出的全部介面」「private本身不是安全證明」。各新機制/需要有據，無當下必要猜測。保存腳本上一回因字串引號語法解析失敗，尚未調用checkpoint或開NEXT；這回只重新保存本pending，未改早期紀錄。

## ch12-complete-prerequisite-v4 / 004-006（順序27）

SHA256 `52ac50ab8a713e84e08237ce482a025635dc53b7066f0465248ef3e29505a22f`。

12.4末四題連原問題：Q1 練mid3/len復用safe split與source生命期/跨threadwrapper，不新API；14.6Miri/Loom只是未授權工具預告，不當現在已會查證。Q2a mid3由lencheck、同alivealloc、兩不重疊range與來源mut借有效性支持，left2原index2/right0原index3各+20；borrow終才原讀80/120/60/80；midlen右空不能index。Q2b 原需要把安全責任可重用地分caller契約/輸入型別檢查/private不變條件，不是六個結果答對就證全部soundness，constructor安全碼也可破原raw來源理由。Q3 先寫SAFETY必要前提再做有限結果檢查；六tests覆幾個已知正常/邊界而非全部輸入/源狀態/排程，外連完整答案/source不開，Rust不執行。Q4 小變化可沿已教推改哪源元素/空段/拒mid不改，安全依全部前提與介面，不能因unsafe block字未變仍宣理由有效/六PASS全安全。原句「也要看safe程式」「SAFETY註解記錄理由，不能代替真正成立的關係」。004001原caller/author責任及有限只讀SendSync依據已收束；原生C/外部配置尚未知待後，不回填早判。

## ch12-complete-prerequisite-v4 / 005-001（順序28）

SHA256 `a76564302a094e49a052836c759025c652ee8a79c15045a21a7a76a2a71449d6`。

Q1 FFI全名foreign function interface/外部函式介面，跨語言call，接12.4caller/impl安全責任但來源現在C；C11是2011標準版，cc環境GCC/Clang。Q2a 當前只知須讀C資料表示/存取/清理契約，並先獨立C程式教操作，尚未教C語法或Rustcall綁定，不以名字補先備。Q2b 原需要Rust能call C且包安全API，不能不懂對方資格就承caller普通Rust輸入安全；先獨立C數值/數列/字串buffer/副本釋放把契約材料清楚。Q3 無需先懂C，單thread範例/compiler命令當正文材料，下一節才Rust接口；本輪不編譯/執行C或查source。Q4 作者LinuxGCC14.2查證是正文宣稱非本人結果，不能據此所有平臺/ABI同一；全單thread不假設data race測。原句「讀懂對方的資料表示、存取條件與清理責任」「下節才加入 Rust 的呼叫程式」。C各規則未知保留後文逐首用。

## ch12-complete-prerequisite-v4 / 005-002（順序29）

SHA256 `eb6de28228be066b55d22b2b40c2dab908e0c9a2b295f97b230c206f9f04a006`。

首用C宣告/各型別/header/preprocessor：Q1 型別在名前、區域預設可改、=初始化/指派、;敘述與{}block，非Rustlet/mut。int32_t exact32signed同i32range、uint8_t8unsigned0..255、size_t物件大小/項數unsigned但寬由C平臺定、char是Cbyte非Unicodechar、int主返回狀態不假32、void return無值/獨參數表無參。Q2a stdint/stddef header由#include讓型別可用，preprocessor編譯前納header非Rustuse；相符exacttype此host有，sizeofchar1Cbyte、本host8bits非所有C。Q2b 原需要讀懂C接口數值/長度/字符與ABI表示不混Rust，未init局部不可先讀，明初始化保有效值。Q3 按header+宣告80再指派100，型別用途契約明但跨語言具體Ctype對應後教；不拿int猜i32。
首用array/sizeof/參數轉pointer：Q1 宣Tname[n]、花括序初始化與0索引，sizeof大小operator→size_t，固定array自身有長度。Q2a values3個、wholebyte/itembyte比3項，fixedsizeof不實讀item；一般參數arrayexpr轉firstpointer不伴len，sizeofpointer只能pointer大小。Q2b 原需要數列interface能know有效項數，故另傳len，不用pointer大小反推未知array，也區分byte與items。Q3 此處main array用sizeof算len，接收pointer者靠callerlen，不靠相除還原；後函數語法/有效存取待教。Q4 host限與兩行支持大小/單位/轉交關係，非所有C平臺byte8/int32普存、未初始化值可讀或pointerowner保源。原句「不把整個 array 和長度一起傳入」「讀取前必須初始化」。機制/需由正文可接，未compileC。

## ch12-complete-prerequisite-v4 / 005-003（順序30）

SHA256 `f5151774465e9ec4f3cbc10ea23200b0f2a227e3230a6e1b6231c9e2d6c3541a`。

首用C函式/pointer/&/NULL/set_value：Q1 return型別fnname(params)body、return值; 結束/void無結果，intmain(void)無參狀態0成功；T*name型指與expr*存取分開，&amount是Cpointer非Rustref。Q2a ordinaryinteger參數數值副本、pointer值亦複但仍同物件，無borrowcheck/life/owner；NULL無可存取來源，nonnull不保有效，dereference需活來源/alignment/讀寫資格。set_valuecaller保有效aligned可寫i32與無衝突，body寫*target不保存/配置/釋放源。Q2b 原需要改caller原amount，按值改local參數不足，pointer副本的同源關係才直接回寫；FFI後需caller契約，不能因C無checker當自由存取。Q3 amount80活且可改→pointer&amount→setvalue100，source仍在caller，離blockstorage不可用，不讓指逃。Q4 兩片段與原資格支持80→100寫的是原物件非副本，不能由非NULL或正常100認全部pointer合法/源被延長。原句「pointer 值也會被複製，但仍指向同一個原物件」「函式不保存 pointer」。當下介面責任已足夠，未compile/run。

## ch12-complete-prerequisite-v4 / 005-004（順序31）

SHA256 `ebb2717163c5f2ba45be1bdcfa8fc07a9c6bb9174e125c919b011ce4d3e835c9`。

首用C constpointer／for loop／sum契約／signedoverflow：Q1 constT*限制透此路寫非原data永immutable，pointer+len介面與Carray參數轉ptr相接，for三段語法非Rustforiterator。Q2a init→條件truebody→更新再條件，index0<len逐total+=元素；values[index]=*(values+index)以元素位移，同array/onepast資格且僅真項讀，不自boundscheck。caller保len有效連續alignedinit/源存續無寫衝突/每次中間i32範圍；len0不arithmetic不讀所以此helperNULL合法，Rustslice0仍nonnullalign差別。Q2b 原需要函數知道讀幾項與保讀資格/數值有效，const或最後總240都不能代來源/長度/中間overflow理由；C signedoverflowUB非Python無限/ Rustdebugpanic。Q3 原3項80/120/40→80/200/240，len2→200，len0→0；不可三項配4，不跑違約示範。Q4 部分長度/空變化由loop與契約支持，非所有CAPI允NULL/const同步或具lengthcheck，有限正例不能證一般加總全i32安全。原句「每次累加的中間結果」「這份特定介面因而允許 NULL」。當段規則/用途足夠，未自行編譯。

## ch12-complete-prerequisite-v4 / 005-005（順序32）

SHA256 `69981877b0592cdacc647f99ec83264ad1b76ce5aa93b5354ea893d4a542d9e4`。

首用C string/NUL/literal/strlen與buffer/if：Q1 Cstring連續char以值0NUL終，字元0不同、NULLpointer另角色；buffer零可普通元素，不能泛作字串。Q2a word[]literalRust編譯得5項含終零，局部array副本可改而literalstorage不可寫；strlen<string.h>需有效可讀範圍NUL，size_t回終零前byte4非容量5/Unicode字數。countNonzero同len有效array/alignment/init/無衝突契約，len0此helperNULL合法，for不因元素0停；if條件0false/nonzerotrue/!=比較0或1。Q2b 原需要分字串掃描界線與任意buffer的指定長度，數10/0/20仍需看第三項20，不能把byteLen改strlen；為後FFI資料契約而非算2而已。Q3 保字串終零／bufferlen，count3項index0/2+=1→2，函數不save/free源，缺NUL不能strlen。Q4 SVG SHA256 `43b877900235ccf2a0d540b20453b0c4354cf52c2532390cccc37f73777db14a`source+390isolated已看：上五格四藍Rust/橘0，橫箭頭到第四項末表示strlen走4、whole5；下三綠10/0/20箭頭全3、countnonzero2，箭頭走訪非步驟先後。字可讀未實頁/compile；工具JSON對反斜線顯示曾需辨字樣，只檢本currentBODY inline碼點確認NUL記法一個反斜線加0，不據序列化重複escape報缺。原句「一般 buffer 根本沒有字串的終止約定」「不是 Unicode 字數或 array 容量」。新機制/需要足夠。

## ch12-complete-prerequisite-v4 / 005-006（順序33）

SHA256 `a10d810e7254258ff122aa8003654f8258ecb7430066fc5f67c7fac050eeba5b`。

首用C allocator/malloc/free/voidpointer／exit常數：Q1 明確配置儲存與前arrayscope清理不同，stdlib提供malloc/free及EXIT_FAILURE/SUCCESS，void*無指定元素的objectptr不是void無回值。Q2a malloc(bytes)成功原起點aligned足本int32但未init，失敗NULL；Cvoid*與T*互轉只改pointer表示非已有效值。free須原malloc未free起點唯一一次，NULL允但局部array/內元素不行；pointer變數scope終不auto free。sizeof*owned按fixed型不dereference，len3乘itembytes可表示，一般大小先查溢位。Q2b 原需要C程式自己分配/初始化/釋放責任，pointer副本不多一份owner，為跨語言誰配置/誰釋放／borrow何時終先清楚。Q3 先NULLcheck失敗returnEXITFAIL、逐values copy寫新alloc，再sum讀已init，free後ownedtotal數值Copy仍用而所有pointer不再用；readhelper不save/free。Q4 真逐項copy另一alloc與ptr別名對照支持資料副本/owner責任差，不能非NULL=init或把T*guessfree策略。原句「一般 pointer 變數離開 scope 不會自動 free」「沒有新增資料副本或一份可另行 free 的 ownership」。機制/需要/合法步驟足夠，未執行違約malloc/free。

## ch12-complete-prerequisite-v4 / 005-007（順序34）

SHA256 `6a4d1c109217f2896dace4e03607abb879df50ba91c8cab3a30fdd35ee696f8b`。

首用printf/格式/PRId32 macro/prototype/檔案分工：Q1 stdio的printf函數非Rustformatmacro，NUL格式+按位置參數，%s須Cstring/%zu size_t/%+PRId32 i32精確型別；inttypes宏由preprocessor替成平臺格式literal。Q2a 相鄰字串literal合成完整format，非假int32=int；格式型不符可UB，returnint非負count/負輸出錯，但demo未處理輸出/最終flush全責任。prototype是fnreturn/params後;無body供call前型知，counts.h宣告、counts.c已教body並同header核一致、main.c資料driver，//註解；雙引header先source目錄找、角標準。Q2b 原需要可讀/核對C介面型別與固定結果，跨平臺i32格式片段減少錯配，而不是看到printf出值就安全/完整CLI；不同source保同prototype免呼叫未知型。Q3 main資料已init且小範圍，freeowned後只Copy total可印；helperptr不保存/free，%s word終零，buffercount不當strlen。Q4 完整driver+header支持報100/240/0/stringlen4/storage5/buffer3/nonzero2/owned240與原配置關係，未編譯/未讀counts.c外檔；printf輸出本身不代合法存取或flush完成。原句「格式或型別不匹配可能是 UB」「核對實作與 prototype 的型別一致」。現在C接口與材料來源足夠，編譯連結步驟待後。

## ch12-complete-prerequisite-v4 / 005-008（順序35）

SHA256 `6df66fcd3a5cf30b6259219c7c00b0ce405b9044cb5f7a439714748035374092`。

首用cc driver/object/linker/選項：Q1 cc hostCdriver協調preprocess納header、compile成指令表示、assemble object.o，再linker把所需body連到call得demo，承9.2建置/執行分。Q2a cc-c僅.o未link，mainprototype知fn型而body在counts.o，hostedlink亦納printf/malloc標準實作；只main.o可compile但link缺3body。stdC11明語言、Wall/Wextra兩套warn非全部、Werror已開warn成error、pedantic部分standard診斷非無UB證；-o輸出、mkdir-p可既存、./才run。Q2b 原需要把C介面/定義source做真程式並辨缺型/缺body與記憶體資格，prototype不能代implementation/安全caller前提。Q3 指令兩source各-c再兩.o連demo，少sumlen負例在prototype檢參數數量，不能幫驗ptrlen有效或afterfree，primary不compile/run/不安裝compiler。Q4 SVG SHA256 `c59f53b87498b661601d7b0a462ca22f387e54753ac178317a489199b2fe1212`source+390已看：橘header向兩側下箭頭納main藍/counts綠，各cc-c向其同色.o，兩.o兩匯線入橘link+stdlib實作→藍demo。上箭頭納宣告非body，下流程產物非時間，沒有把header變.o。五輸出支持已說資料關係而不能任caller安全，編譯成功非內存契約。原句「header 宣告沒有代替 counts.c 的 body」「合法編譯不等於完成記憶體契約」。圖可讀未實頁，當下操作/理由足夠。

## ch12-complete-prerequisite-v4 / 005-009（順序36）

SHA256 `d501da1c7b8b33f3a5deb9e360c615892df5d42a503e9c92674156974b02f968`。

12.5末四題：Q1 checks與copyexercise復用C函式/資格/allocator/編譯link規則，非C自動owner或新NULL契約。Q2a 固定{-80,120,-40}中間-80/40/0與len2=40，buffer{0,10,0,20}len4count2/len1count0，NULLlen0兩helper0，&amount寫-20；cc無-c兩source→executable，檢查mainif失敗EXITFAIL。另malloc新三項copy後first100→260、原240，free一次後僅保存total用。Q2b 原需要能讀C接口原資料/長度/有效期間/唯一釋放並分真資料copy與ptralias，為Rust safe wrapper承caller責任，不是隻算260；有條件案例可核行爲不證所有caller資格。Q3 先寫來源/有效期，再copy/init/修改/數值副本/釋放順序；closed參考答案不展開，未編譯checks/非source/archive讀。Q4 案例含空、抵消、prefix、內零、寫原/獨立copy的有意變化，可沿既定規則推但沒有模擬malloc失敗/總安全證，不能不輸出就猜進程已驗。原句「free 之後只使用已保存的整數副本」「誰負責釋放」。005001新C前置所需材料已正文具體給足，RustFFIABI/綁定/跨語言配置尚未教，留下一節。

## ch12-complete-prerequisite-v4 / 006-001（順序37）

SHA256 `9abc5a85304f925282eee9f9c986ef7552fe5fa09647f8a2c69f3a2559afde64`。

Q1 FFI把C契約接普通Rustsafe介面，承12.5source/len/allocator與12.4責任分工；本地C函式庫與三資料關係尚待實作。Q2a 先版本fn確認link，再一般bytebuffer借、Rust-ownedCstring借、C配置固定字串owner；同步C不保存input/不callbackRust，綁定/ABI/cleanup具體規則未教。Q2b 原需要Rust使用C而普通caller不自己管raw資格，三有限材料使type/call/borrow/清理逐核，純Rust原可做並非必C/更快。Q3 Csource隨package但本輪只讀BODY列片段不開source；環境Linuxx86_64/GCC14.2/Rust1.99edition2024/C11是限定聲明，非己驗。Q4 開場角色/用途足以說明先後，但不能以「safe接口」宣實作健全或跨所有平臺；source存在不等ABI相符。原句「讓普通 caller 不必自行處理 raw pointer」「不是宣稱必須使用 C 或會更快」。新綁定/借字串/外配置不推後文，未知保持待教。

## ch12-complete-prerequisite-v4 / 006-002（順序38）

SHA256 `542eef49e99a09d37fe8751ee74154ff708db526f3d96284648469758540ed28`。

首用uint32_t／ABI／unsafe externC／sys safe wrapper：Q1 ABI全名ApplicationBinaryInterface約參數/回值與layout，externC選RusttargetC約定非嵌Csource/全球同ABI；uint32exactunsigned32此target有→u32。Q2a unsafeexternblock只name/Rustsignature沒body，edition2024標unsafe是author聲明匹配C，Rust不readheader替核；block內fn未safe預設unsafecall，當callblock另責任。privateSys mod有pub供父但library外只apiVersion，無input固定u32此wrapperunsafe實作接契約。Q2b 原需要先無pointerfn驗body能link且兩語言聲明/實作/target一致，不因同名/各編過即可，讓普通caller不做raw呼叫責任。Q3 Cprototypevoid→uint32return1，Rustdeclaration()->u32/apiVersioncall→值；safeexample須下段建置備body再run。Q4 source已在BODY完整給的固定fn可支持無額外input前提的safe包裝，但unsafe字本身不證ABI/symbol存在、targetmatch或allforeignfn安全。原句「名稱一樣、兩邊程式各自能編譯，仍不足以保證」「block前的unsafe承擔宣告正確性」。當下角色/來源/需要足夠，object如何提供待教，未實compile/link。

## ch12-complete-prerequisite-v4 / 006-003（順序39）

SHA256 `d4d886856abe8560711d8512ba15ca0655efa3a8f0bac342383e26a6dec5ef5a`。

首用cc crate／staticlibrary/archive/symbol／linkmetadata：Q1 cc是host buildscript的Rustlibrary協調targetCcompiler非compiler本身，build-dependency供script；staticlib包objects成archive，link取所需code到程式，symbol是linker識別函數入口。Q2a Buildnew設定，file相對packageCsource/stdC11/warnings/warningsIntoErrors組態串接，compile(name無lib/path/.a)真正編譯archive+Cargo link資訊失敗script不能完成；確切前五receiver返回型未列，我不套reqwest按值假說，當前temporary鏈設定角色夠用。rerunbody/header，OUT_DIRobjects/libtutorial_c.a這GCC環境，linksearch目錄/linklibstatic名字解外body位置非signature。Q2b 原需要Cbody提供給Rust正確target且可重建/連結，只有extern名型不夠，手寫unsafe不修缺symbol或C語法錯；cc metadata故不另#[link]。Q3 buildrs在host而Cobject匹配Rusttarget，同host可同，cross須targettoolchain不可拿hostobject硬配；safeapiVersion1來自Cbody，primary不cargo/cc實跑。Q4 BODYbuildscript與關係說明支持不同階段/filename平臺限定，不驗所有ABI或自動Cheader匹配；C語法建階段錯vs缺bodylink錯分。原句「extern宣告、C實作與連結設定各自有工作」「C object卻必須符合Rust target」。006002如何提供body未知此時補，無需外部資料。

## ch12-complete-prerequisite-v4 / 006-004（順序40）

SHA256 `d4c8191efdfda2551777105213cd449c5a72ead82cce4eea57bcfca082a55dc8`。

首用Cstruct/else與RustreprC/padding/countwrapper：Q1 CCounts兩具名fields是按值result非bufferowner，RustCounts reprC保C相容表示，CountErrorRustonly不跨ABI。Q2a CstructT型名/定義;/{0,0}依序/點欄、else假branch，zero或nonzero每index一次；uint32len最大counter<=len，最後MAX−1→MAX條件false不再加。reprC宣序+各align/必要padding到欄/總size，defaultRust不保，field自身u32/uint32同target仍需合，StringVec不自可C。Q2b 原需要buffer零是資料且同時返回兩計數，slice安全interface由實作接len/範圍/C結果layout責任而非caller「不要太長」；長度不能截斷少處理。Q3 checkedLen tryfromusize→u32/TooLong?先Err不callC；bytesborrow供同有效aliveinitallocation、asptr不own，C同步只讀不save/free，空len0完全不讀；result兩u32不借input/無heapfree。Q4 code/table與MAX邊界推論支持沒overflow/兩字段相合/三項含零(1,2)empty(0,0)，非真配置MAX資料或所有Rusttype reprC皆相容/任rawcall安全；example10u8suffix沿已學。原句「欄位本身仍要各自相合」「public safe介面只接受本身帶有有效性的slice」。各新表示/契約/用途足夠，未run。

## ch12-complete-prerequisite-v4 / 006-005（順序41）

SHA256 `641258391f8403d86050691eb75d14b1e0e7236a62bd1591cc6d92053eaeca07`。

首用CString/CStr/NulError/c_char c_int/strcmp：Q1 CStringRust-ownedNUL字串，CStr借view不own/不保UTF8，NulErrorRust建格式拒非C錯；stdffi Cchar/int alias對target非Unicodechar或一律i32。Q2a CStringnew本例借str→自己的initbytes、拒任何zero補末NUL→Result；asCStr借owner、asptr給constCchar原資料不帶life。strcmp兩有效NUL只read比較0相同/其他int，C==0returnint0/1，Rustexternreturnc_int後answer==1→bool非直接boolABI。Q2b 原需要完全判Rust四byte而普通str有len/UTF8但無終零，不能直接傳strptr或內零Ru被當截短；一般含零資料須ptr+len不強當Cstring。Q3 保存namedword至syncC返回，C不save/write/free；?NulErrorcall前返，Rust caller仍有原str，CString最後Rust清理；C改保存/free會令原wrapper理由失效。Q4 Rust/Python前兩由C比較、RuNULst是Rust拒且不call的有意變化，型別/owner/來源契約才安全非輸出true/false代資格；不從rawptr推temporary自保活。原句「第三項是Rust端格式檢查」「CString只有shared借給C」。全部新API必要角色/條件/用途可讀，未執行。

## ch12-complete-prerequisite-v4 / 006-006（順序42）

SHA256 `d334d235534c236cf994d6e72d254d80b0c3b3845b6132afcf12ba1c11734cda`。

首用C-ownedGreeting／C工廠配對free：Q1 Cmalloc新五char RustNUL配置交唯一cleanup責任，非局部wordptr/非RustCString或Boxallocator；專用owner把C來源契約接safeAPI。Q2a Ccopy含NUL/init完return或NULL，privateptr只新工廠起點不可任rawconstructor/寫/重設/free，tryNewOption排NULL，move無CopyClone、Drop一次本Cfree，borrow未用才終源。Q2b 原需要跨語言取得owned內容並正確釋放/不要求普通caller驗raw來源；heap字樣或nonnull不足證allocator/整資料資格。Q3 newNone處理失敗、Someowner持原pointer，不能交內元素/別allocator/已free；Rust正常Drop回指定函式非Rustalloc。
首用CStrfromPtr／mut→const／toStr：Q1 unsafegetter建借view非接owner、C格式非UTF8。Q2a ptr要求nonnull/含NUL全部init可讀單alloc/<isizeMAX、borrow中不mut/free；*mut到*const可隱轉但不添資格/延life，&self→&CStr有效範圍連owner；toStr檢查→Result<&str,Utf8Error>仍借非複，Debugexpect本固定ASCII有據。Q2b 原需要看C資料而保管理/有效期，若free後還用須to_owned另RustString，不靠CStr或UTF8檢查延源。Q3 view讀Rust/另savedString→view最後用→dropGreeting→只saved，原五byte終；其他API任意bytes須處理UTF8Err。Q4 SVG SHA256 `6694a5d79c7bfed0c3ab14408700ca3b996066af9ff8e69ab6d8baa93de63ba4`source+390已看：綠C五char/Greetingowner→藍view借同sourceSelf→藍另RustString並終借→橘dropCfree原配置終→綠獨Rustcopy仍用，四箭頭順序非耗時，不把CStr當文字副本。輸出只內容對照，安全從Cbody/Rust介面資格與匹配free，未實頁/執行。原句「不只靠NULL檢查」「CStr格式正確仍不等於UTF-8」。當段所有新規則/需已可解。

## ch12-complete-prerequisite-v4 / 006-007（順序43）

SHA256 `4e7b2363bbf76d62be9ded2e731516d7634d4d091945d4887ee8ebc00d0bc5fb`。

首用improper_ctypes/deny內attribute/usizeBITS MAX：Q1 lint對外宣告型別品質，不是ABI錯配運行實驗；#!deny檔首將此warn升err，consts詢整數平臺bit與最大值非資料量測。Q2a 缺unsafeextern與缺callunsafe各編譯前責任/ E0133，缺reprC普通Counts外宣lint，deny使fixture確定失敗；CStringborrow逃owner E0597只Rustowned，Greetinggetter同selfborrow規則但非驗Cfree配對。usizeBITS>32才usizeMAX trycheckedLen→TooLong，無巨大alloc，isErr沿12.1共享variant檢查。Q2b 原需要辨宣告/操作/表示/借用不同責任，不執行ABI錯配/失效ptr/doublefree才示例；變成error用於固定負fixture，不把lint當全部safety。Q3 publicsafewrapper日常caller免直接unsafecall，但author匹配義務仍在；正常六test覆subslice/empty/len不截/內NUL拒/move後copy/drop/兩C配置獨立，ErrorDebug已給，primary不compile/tests。Q4 六normalcase不模擬mallocfail/u32MAX資料，字串負例沒C配置不可free證，編譯accepted+testpassed都不代完整soundness。原句「不是在執行時試出layout有沒有剛好相同」「兩個分支的安全責任仍須由實作核對」。當下新規則用途明確，未另外查archive/source。

## ch12-complete-prerequisite-v4 / 006-008（順序44）

SHA256 `cee9f5d95f869277e1ac91bba08e60ad9dd264ce94a65e08f855f851d518b03e`。

12.6末首用CStr toBytes/toBytesWithNul：Q1 兩getter借原owner[u8]view，不取ownership，分不含終零/含終零範圍；承C字串與buffer不同零契約。Q2a GreetingasCStr→toBytes四資料=count(0,4)，withNul五含zero=count(1,4)，兩種合法；toStrUTF8check再isRustLabel另Rust-ownedCString借給C比較，sourceCowner仍原同資料。Q2b 原需要同一C文字不同介面各做工作，buffer以len數所有項/字符串以NUL約定，比較不混條件且free後獨立RustString才續用，非改Ctypeowner換allocator。Q3 tryNewNone停止，Someborrow期間完成count/label，toOwned保存String，borrow終dropGreeting指定Cfree後僅ownedcounts/bool/String。Q4 末尾零小變化可沿已教推不同計數且保同內容，closed參考答案未展，有限同步單向Rust→C不含callback/unwind協定；更換library要重新核ABI/layout/每APIowner而非套五byte條件。原句「兩種計數都合法，只是處理範圍不同」「不能把此工廠的五-byte不變條件搬到任意pointer」。006001借buffer/借RustCstring/接C配置三關係及安全caller目標此時連齊；所有新機制/原需要各自有據，未用輸出代soundness或執行查證。

## 封存

PRIMARY FIRST12 WHOLE ORIGINAL正文完成封存：唯一ch12-complete-prerequisite-v4，六頁44/44單位（001-001至006-008，依reading-order），每CURRENT預先closed-filter再讀BODY，原位SVG讀/view後保存四題、Q2a機制/Q2b原需要、原句、推論/未知/rawSHA才NEXT。saved44/currentnull；STOP未授權13／14章，不與同行交叉。沿自身actual1–10與已封存FIRST11 prefix36＋middle32＋tail12為實讀背景，既有unknown/分級/SHA不回填。

本輪主要理解：12.1 GAT將來源/結果族連每call borrow，slice與RefCellguard各保責任，不是Vec借view不能做；12.2 subtyping/variance按參數/使用端全部契約看外life縮、可寫內T不變、Cell共享寫與fn輸入反向/輸出同向，數值literal非life規則證；12.3 raw建立/檢查/存取分、unsafe/UB與alive allocation/init/alignment/provenance/alias權限，one-past只形成不讀；12.4 caller unsafefn與author safe abstraction/soundness、split型別/檢查/不重疊範圍與ReadOnlyI32全接口SendSync理由分；12.5 C宣告/pointer+len、字串NUL與buffer零、malloc初始化/free唯一起點及prototype/compile/link責任；12.6 ABI/宣告/Cbody/link配置、reprC欄位、長度不截、RustCString只借與C-ownedGreeting匹配free/self借用/UTF8檢查三材料相接。各節局部操作與原方法需要分開有據，沒有從新產物反推需要或把正例結果當安全理由。

未確認阻擋正文操作/原需要連結的必要缺口，也沒有已確認需要另補教材的可選問題。保留局部完整cc設定method receiver／返回型未列、不用把它猜成reqwest按值契約；本段僅temporary鏈設定至compile副作用，當前操作用途足夠，沒有利用未知簽名推後步。一般raw aliasing完整模型的未定界線與14.6工具只是合理後學，未將範例說成所有alias允許/全soundness證。編譯/語言/ABI與allocator實際正確性為技術輪未驗，本文fixtures診斷、作者環境/正常tests數目或source宣稱不是自己passed。

中斷恢復按ownstate核saved25/CURRENT004-005，自己已存25段筆記復原背景後只重取同pending（不能確知中斷前工具正文是否送達），沒有重開未存NEXT；恢復範圍與當時SHA另追加原文，未改原25初判。其保存腳本一次字串引號SyntaxError在Python解析階段未調checkpoint/開NEXT，已僅重存同pending後前進，無持續阻塞。

全44初判期間closed答案／details／選讀均遮蔽，完整答案/外部參考程式/延伸鏈結/source/archive未開。圖只有七個實遇SVG：001-004、002-005、003-005、004-004、005-005、005-008、006-006，sourceSHA與具體框/線/標記/名稱對應見各單位；Inkscape390獨立view，不是真實頁面／Chromium／手機桌面版面驗收。未run Rust、C、cc、Cargo、HTTP、危險pointer/ABI錯配/doublefree示範，不因正常輸出/建置或測試通過認全部安全。

本檔44段保存後seal前僅OpenCC s2t敘述字體校準2字；引用/程式碼/API/ASCII數字/hash序列exact不變，沒有改理解、分級、時序。校準前byte另存 /workspace/work/ch12-first-reader-04-before-font-normalization.md，SHA256 `8cbff4e20e1970767e7df7e76b035ee34e1ba8769e6fe94e643b6f6f926f28d9`；字體sidecar /workspace/work/ch12-first-reader-04-font-state.json。原FIRST10、入口、11prefix、11middle、11tail保持既有sealedSHA。

此處為INITIAL seal，任何後續選讀只可另記補讀新增證據，不能回寫44原判。
