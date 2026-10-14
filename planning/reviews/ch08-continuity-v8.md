# 第 8 章 ORIGINAL 獨立銜接初判：v8

## 角色、閱讀路線與隔離

日期：2026-10-14；角色：`chapter02_ownership`，第 8 章非作者的銜接審閱。曾撰寫 9.2–9.4 並使用實際先備，也做過第 7 章銜接審閱；**不是新的 blind 首讀者**。沒有把作者／專家背景當作讀者必須具備的能力。

唯一第 8 章來源：`/workspace/work/tutorial-implementation/frozen/ch08-complete-prerequisite-v8`。manifest SHA256：`79036df9490cd5ad5874be7978062928ff185e1d25abc76adf610c2678eae9c1`。

依 root 續行任務先檢查已保存的 own checkpoint：前八節52個 BODY 的原判斷已在 `work/ch08-continuity-v8/body-notes.md`。從 `009-001` 續讀剩餘14單位，沒有重啟或回寫原52單位；全66 BODY完成後，才讀 manifest 的全部150個 archived sources、必要前文、官方契約及實頁。這是同一次獨立 original 初判的續行，不重標為全章新首讀。

封存前未讀 first8／TECH8／root known issues／disposition／第8章作者 ledger或其他peer，也未讀其他第8章snapshot／live改稿。沒有修改8正文、程式、共享Cargo／lock／nav。讀完才補查的結果在原notes追加，不把後見判斷倒填第一次需要處。

**本初判：必要0項、可選3項。** 在下列實讀範圍，沒有找到需要讀者另補未教能力才能銜接的主要概念／程式缺口。這不是完整發布或全套技術驗收：只實跑四個selected library tests與兩個runtime負例，未跑38個compile-fail／全package checks；版面限八張必要圖與相鄰正文。

## 實際重建的先備與首次需要

| 需要讀者先會什麼 | 已教正文依據／本章首次補的角色 |
| --- | --- |
| workspace、library、assert與不同crate | actual1 v8的1.3 `003-004`明說共同欄位與繼承；1.10 `010-002`明說assert bool／數字與panic。Cargo只繼承已教欄位，pub module／example use沿已有分工，不把「import」算型別定義搬到caller。 |
| 接手值／借用、receiver、Copy與scope | actual2 v5的2.4 `004-003`明說reborrow／最後使用及原mutable reference可恢復，2.5 `005-002`區分self／&self／&mut self。Box move-out、Drop／Ref guard的收尾時間是本章另外教的契約，不能用普通NLL替它們解釋。 |
| 泛型、turbofish、UTF-8 bytes與容器 | actual3 v6的3.3 `003-003`型別argument與值argument不同，3.5 `005-002`的len數bytes；8.1再教const值argument、Sized／?Sized。任何「Rust 教學」的11bytes由已教UTF-8規則核，不當新字元計數API。 |
| trait bound、Debug／PartialEq、outlives | actual4 publication v8的4.1 `001-005`、4.2 `002-003`明說能力與assert_eq條件；4.4 `004-008`真正教`'b: 'a`至少涵蓋，非多活幾秒。8.4再把要求放到T內borrow與`'static`型別bound，沒有偷把owned值存活時刻當lifetime。 |
| Result／Option的expect、parse錯誤、From／Display／`?` | actual5 v7的5.1 `001-004`分清Result要求E:Debug、Option None不需error Debug；5.5 `005-002..003`已教ParseIntError的Debug／Display／PartialEq與typed enum。8.6才加StdError、source與Box Error的selected轉換。完整source沒有對未知Box Error／Box Any強用expect。 |
| associated type、泛型impl、closure與opaque | actual6 v10的6.2 `002-002/004`保存／接手Holder<T>與Output；6.3 `003-004`Fn是shared receiver，非所有間接資料永遠不變；6.4 `004-002`參數／回傳implTrait誰選type與move捕Copybase。8.3／8.5沿此教DerefTarget、dyn與Fn object；不是用後章closure或trait來補早節。 |
| bound組合與既有IO清理界線 | actual7 v13 `006-004`已有`Into<OsString> + Clone`同時要求；7章normal ExitCode／scope資源角色已可回收。8.2教自己的Drop，未將正常scope清理泛化到process::exit等所有停止途徑。 |

上述實際回讀18個必要BODY的hash存 `prerequisite-fingerprints.json`。選定版本：1 v8、2 v5、3 v6、4 publication v8、5 v7、6 v10、7 review-fixed v13；不是全1–7新首讀。其他已讀背景與新增需要由保存的正文判斷定位，不以大綱或術語表當證明。

## 四題與主要通過關係

下列先保留原需，第二題分機制／性質與需要／用途，第四題另核例子實際發揮的作用，不從「程式會跑」反推理由。

### 8.1／001-001–001-007：型別大小與owned間接表示

1. **身分／關係：已足夠。** Sized是型別編譯期大小性質；str／slice資料可不同長，reference自身有固定表示。size_of／size_of_val各自測哪個對象、usize bytes／64-bit示例限定在首次使用前。
2. **機制：已足夠。** 泛型預設Sized，?Sized解除限制；const N是編譯時值。Box在本節非零資料取得owned heap值，Sized T的handle與T內容分開，非每種Box都在stack。**需要：已足夠。** const段原句是「一份函式處理不同元素型別與不同固定長度，同時在簽名保留完整array型別」；Box回收自含Chain大小不能收束的原問題，間接next使同型別可以保存任意有限長度，改存放關係不改加總規則。
3. **使用：已足夠。** i32讀出Copy、借用、修改、整Box move、Box特殊owned move-out各有先教契約；未把此特例泛成任意Deref都能移出T。
4. **例子：已足夠。** array／slice／str區分型別與實際資料大小；明選N3卻傳長度2得到E0308。Chain80／120／End與240練習實際運用相同型別遞歸表示，圖箭頭owned管理而非borrow。沒有宣稱比Vec快或以圖示作精確ABI布局。

### 8.2／002-001–002-006：清理責任與觀察時刻

1. Drop由Rust觸發的destructor、`drop(&mut self)`與接手值的mem::drop分清；已教receiver和scope現在用於資源收尾。
2. **機制：足夠。** 局部反宣告順序／struct先自身Drop再定義序欄位；move不在舊名失用時清T，reference最後使用亦不是owner收尾。**需要：足夠。** 原需自訂收尾與知道何時發生，id1／2／3、內外塊和Pair兩欄位讓各項時刻可觀察，而非只多幾個println。
3. **操作：足夠。** 提早結束交mem::drop；活borrow阻擋E0505、直接Drop method E0040、Copy+Drop E0184、move欄位E0509各先講規則再fixture。無ManuallyDrop／replace隱藏先備。
4. **證據：足夠。** 先move再清一次、先Pair Drop後10／20欄位、練習先清id1仍讀id2，確有必要對照。只支持正常收尾路徑，未把scope語義說成任何進程結束都執行。

### 8.3／003-001–003-006：借用內部內容，權限和ownership仍保留

1. Deref的Target／deref，DerefMut的前提／同Target，在impl前交代；Wrapper一般struct不額外Box／heap，先知道現在包裝什麼。
2. **機制：足夠。** Wrapper→String→str借用轉換、Vec→slice及mutable路徑保持同資料，shared不會憑空獲得mutable權。**需要：足夠。** 開場明說as_str本來就能完成“不交owner、不複製文字”，新介面減少每個呼叫寫欄位路徑、讓已有函式／method共用；沒有把原法贬成無法借用。
3. **操作：足夠。** 顯式deref／&*、需要&str的參數自動借入、mutable數字80→100→101，receiver來源關係沿4章；into_inner接手無Drop的Wrapper才交T。
4. **證據：足夠。** 顯式5／自動5／owner仍可讀、Vec2與數字權限對照實際發作用。E0507區別Box特殊move-out，E0277區別Target Copy與Wrapper Copy；不把可method查找誤推任意trait bound自動成立。

### 8.4／004-001–004-005：型別內borrow與值何時清理是兩回事

1. T:'a、'b:'a、&'static str、T:'static分別約束何者，Borrowed欄位只持reference，原owner未轉移。
2. **機制：足夠。** ?Sized與lifetime要求同時成立；&'aT的特定implied bound不泛化所有trait。**需要：足夠。** 回應“外面有owned Box能否忽略裡面的borrow”，因此static owned String仍在local塊Drop，對比Box<&str>仍依外部資料，不是只記標注語法。
3. **操作：足夠。** String／str／&str三個T及nested第二層reference，literal填一般短回傳與require_static都先給角色；+首次歷史稱謂有O1，但組合機制本身清楚。
4. **證據：足夠。** literal越過reference變數scope、局部String不能標注延壽、Message正常Drop後block結束、Box借用不滿足static，各驗證不同關係；不要求先教Box::leak或unsafe。

### 8.5／005-001–005-008：保持共同amount工作，讓caller擴充型別

1. &dyn Charge是借共同能力的DST視圖，Box dyn持own資料；unsizing不等Deref逐層借欄位。static dispatch／monomorphization、dynamic dispatch／vtable是實現選擇，不是動態語言隨意接受i32。
2. **機制：足夠。** 同handle型別保存不同資料／method對應；defaultobject static與+'a／+'_'、dyncompat的receiver／輸出／泛型條件先教。**需要：足夠。** 原需求80+120=200、caller另加Shipping20=220；enum能算已知型別，關鍵是caller擴充而不改library variants／match。正文及真正獨立example定義Shipping，支持該需求而非單看library test名。
3. **操作：足夠。** Box／shared slice只借查amount，三factory concrete／opaque／dyn不同契約；Fn object捕owned Copy base。whereSelfSized排除個別method、supertrait及associated Output需選定各有具體用法。
4. **證據：足夠。** enum和dyn同200，move現有Vec再追加220、closure20／40、borrowed200原Vec仍2，讓開放型別／借用／呼叫各真的發生。優化可能／no ABI示意限定清楚；圖對應可辨但label與箭線相交另列O3，不算程式碼或機制錯誤。

### 8.6／006-001–006-006：共同報告界面不取消具體錯誤政策

1. StdError要求Debug／Display，source借下一層；空impl用None默認，Result本來不要求Error，先區別新介面目的。
2. **機制：足夠。** outer Read／Parse／leaf保存具體原原因，每層Display說自己工作，source回傳borrow而不轉移。**需要：足夠。** 開場報器“不知道每module variant，卻要顯示工作再沿下層原因”，一份report走int／range的有限鏈，免各enum重寫分類；不是強迫所有library改Box Error。
3. **操作：足夠。** From／?仍沿5章，BoxErrorselected From(Error+'static)在兩?前；source的內部static與外borrow分明，BorrowedParseError可借label、cause卻own滿足static。
4. **證據：足夠。** abc／-1不同深層理由、一次各層文字；source逃owner失敗不與type static混淆。report邊界少outerinput context有明說，enum仍適合穩定分類恢復。完整tests以match接BoxError，不偷用未教Box Debug條件。

### 8.7／007-001–007-007：共享同一份尾段與僅觀察的连結

1. Rc是多個owned handles共享T，不是&T borrow數；clone同allocation不需T:Clone，ptr_eq非內容Equal；Weak不保活T、upgrade可回傳新Rc。
2. **機制：足夠。** strong0清T、Weak可留控制配置；move不增strong，borrow total不clone。**需要：足夠。** 原句“尾端120是同一份資料；其中一條結束另一條仍能讀”獨立於新方法，不以相同總額當同一對象；重建／借用可算但各有另外owner要求，Rc使兩頭各自負責。
3. **操作：足夠。** tail／first.next／second.next3個handles→清first變2→清tail後second仍160。Weak臨時升成owner亦計數；無DerefMut和NonCopy move-out分別失敗。
4. **證據：足夠。** 共尾圖明三條線到同allocation、Tracked無Clone仍Rc clone有效。cycle圖明假設關係非當前可構建code；內strong互保活與依責任選Weak的因果已給，不預用RefCell/new_cyclic或說任意Weak可解所有cycle。

### 8.8／008-001–008-007：保活和修改權限是兩份工作

1. Cell用值交接、RefCell用執行時guard借入；都不是新增shared ownership。Ref／RefMut借原T且攜活借狀態，最後讀不是guard Drop。
2. **機制：足夠。** Cell get僅Copy／set替換／replace交舊owner；RefCell多shared或一mutable，try把衝突改Result。**需要：足夠。** 80／120輪流記錄、shared receiver避免長期獨占整模型，是原明確選擇；普通&mut self/Vec仍可用，正文不聲稱所有&T都能任意改。
3. **操作：足夠。** 兩shared guards全結束再改、try失敗不改80、Drop後100／120；回傳Ref把解除責任交caller。Rc保活與RefCell借寫分開，Weak parent用此時才教會的後填能力，沒在8.7先偷偷構建cycle。
4. **證據：足夠。** 同Vec2項200、釋放view再錄第三40、branch/leaf counts及parent None、guard runtime負例均支持責任與界線。tests用match，不藏Result predicates或額外Error Debug；單thread限定不要求先學同步工具。

### 8.9／009-001–009-007：單位身分、有限介面與impl責任

1. alias還是i32，newtype是另一struct；tuple struct的constructor／pub欄位／.0先教；Add右型別默認Self、Output固定於impl，owned兩輸入會move。
2. **機制：足夠。** Meters與Seconds同80仍不同，Add保結果單位／跨單位先換算。orphan按定義crate、coherence防重疊。**需要：足夠。** 原需秒不能誤給路程、結果數字仍有單位；MeterBatch只公開報告/計算防caller依賴所有Vec操作，localtrait／外部Vec與localwrapper／外部Display各對不同責任。
3. **操作：足夠。** 80+120=200、500mm+2m=2500mm，反方向未實現；私有集合只讀不clone，Wrapped From交owner、into_inner合法回T。uncovered T出現順序與fundamental Box限定只教當下診斷，不拿名字當完整Rust規則。
4. **證據：足夠。** 單位／move／orphan／重疊各獨立負例，TimeBatch30/60=90且Display後仍讀總值。public欄位不驗證非負與i32範圍限定已說。example“本crate”輸出有O2，附近正文已說明library與caller，不泛報orphan關係未教。

### 8.10／010-001–010-007：確切型別檢查與own失敗結果

1. payload隨id事件保存caller資料，Any標準static型別能力；本次&dynAny的borrow期限仍依owner。downcast不是數字轉換或任意trait反射。
2. **機制：足夠。** ref／mut／owned三個helper結果責任不同，owned失敗交原Box；TypeId查receiver自身或明確&*內容。**需要：足夠。** caller可擴充payload、傳遞層不加enum且無共同業務method，真正處理者才取理解的T；固定集合／共同Charge仍各有直接替代，未誇大Any用途。
3. **操作：足夠。** 同80 i32／usize只有i32更新100；String誤請求i32後仍取String追加，Box特例move-out再接T。helper泛型是dynAny額外方法而非不兼容的泛型traitmethod表，static要求不會被+'_'取消。
4. **證據：足夠。** borrowed不丟owner，owned Err保原String，TypeId Box≠i32執行斷言失敗，ExtraInfo2→3驗證caller資料。TypeId非跨建置持久編號，stdAny內部值不需解碼；tests以match處理BoxAny，額外Debug候選不成立。

## 自己提出的可選項

### O1：`+`組合符號已出現，不宜稱本節新增

- 位置：8.4 `004-002`：「本節新增的bound組合符號 **`+`** 表示各項要求一起成立」。必要前文7.6 `006-004`已實際說明`Into<OsString> + Clone`是同時兩項能力。
- 已教／影響：本節?Sized與'a的組合解釋充分，讀者不會因此用錯；只是新符號的歷史定位不準確，讓複習者誤以為不同語法。**可選：名稱／前後引用改善，非主要背景缺失。**
- 最小建議：稱「沿用 `+` 的同時要求，這次組合大小與lifetime要求」，保留現有T:'a／implied bound說明。不要求加一節、不修改程式碼。自己BODY提出後實回讀前文確認，未受peer提示。

### O2：example輸出的「本crate」與trait真正定義處相鄰說明不一致

- 位置：8.9 `009-005`／archive newtype_interface.rs，binary印出「本crate的trait能實作在Vec上」，但BatchSize定義／impl在library；example只use然後呼叫。
- 已教：正文馬上說「第三行使用的是library已提供的本地trait實作，example只import BatchSize並呼叫它」，所以local責任已可理解。不是因為兩個80相同就推型別相同，也不是source非法。
- **可選：主詞明確。** “本crate”在binary句子可讓讀者回找主體，和剛教import不搬定義的規則混用。最小把輸出稱「library的trait…」；近旁已有足夠解釋，勿升主要orphan阻礙或擴寫新例。自己讀BODY提出，archive確認位置，未實際執行此未變例。

### O3：dyn圖兩條右箭線穿過“method表”label

- 位置：8.5 dyn-dispatch.svg，圖在`005-004`；兩條右側path x290，label文字從x200開始。真正1280／390原位畫面都看到直線穿過最後的“表”，不是只由bbox推測。
- 已教／影響：Order表／Bundle表框名、資料左欄與正文仍明確，基本關係可辨，圖不是資料布局錯誤。**可選：圖解呈現改善。** 讀者識別method一側時線字不應交碰；最小讓label靠箭線一側留足間距或分行，不改箭頭含義／vtable抽象／兩對象關係。
- `dyn-dispatch-1280.png`與`dyn-dispatch-390.png`保存原證據；原SVG SHA見150-source scope。沒有修圖，沒有把此小負擔寫成字無法辨認或主線失敗。

這些判斷都在互看之前形成。where“新clause”候選經actual6無此語法而解除；短Add/fmt片段省use經完整module首兩行與已有import規則而解除；Box Error／Any Debug候選經完整match tests而解除。保留原候選到追加驗證的過程，不把猜測報成缺陷。

## 來源、執行與視覺證據

### 全部來源與配置

manifest全部150 keys實讀：101個Rust檔（12個src含根lib、49個examples含十個完整答案、38個compile-fail與2個runtime-fail）、40個相鄰.expected、1個Cargo、8個SVG。library內所有tests／helper同樣全讀，不重複增加總數。沒有跨到作者notes取解答。

根lib只有11個pub module宣告；成員Cargo只繼承actual1已教欄位，沒有外部dependencies。source指紋與66BODY／8原位asset副本／manifest等全部226個snapshot文件，讀前與封存前相同；`reviewed-source-scope.json`存150-source mapping，不拿已知root precheck當獨立QA。

### 官方與相稱執行

在全66 BODY和150-source之後才查selected官方段落：RefCell borrow_mut／try_borrow_mut的guard持續到scope退出；Weak upgrade的Some Rc保活／None；Any type_id／is／downcast ref與mut；Reference dyn compatibility的generic type／Self／where Sized／opaque規則。只把本章選用契約作為佐證，不把參考網頁出現的unsafe、Arc、Pin等提前變成學生先備。

HTML與URL／anchor／hash在 `official-selected-api.json`。初次Any required method的anchor誤用method.type_id而工具斷言失敗，改tymethod.type_id取到真實段落；這不是教材錯誤，不宣稱未讀網頁已讀。

使用Rust1.99.0／edition2024直接編frozen src/lib.rs成test binary，實際只跑4項：不同同值Any型別更新、owned downcast失敗保String、upgrade Weak成額外owner、全部Ref guards結束後才mutable。各1通過／25 filtered，**不是全26 tests已執行**。

另直接編兩個原standalone runtime負例：guard_used_then_conflict、any_box_type_id；編譯成功，執行均101，stderr分別含.expected的`already borrowed`與``assertion `left == right` failed``。保存原始commands／stdout／stderr／status於 `selected-probes.json`；沒有跑全部compile negatives／Cargo／Clippy／fmt，無共享根設置修改，也不以stdout未看就誤報panic失敗。

### 原位圖與未驗範圍

完成閱讀後由原66BODY重組相同theme的獨立preview：只去除chunk重複H1、改本地figure URL，原鄰接順序及圖bytes保留。strict=false、build No issues found僅支持此局部preview，不是發布build。沒打開live第8章，也沒借root實頁證明。

1280×1000與390×1000各八張Playwright screenshot，全16張經view_image實看；一次transport中斷後補看未輸出圖。圖desktop intrinsic390，mobile約343寬，24px字體縮後約21px；必要理解不需放大。除O3的字線交碰，必要label／箭頭與正文在以下範圍清楚：

| 圖 | 親看支持的具體關係 |
| --- | --- |
| boxed-chain | 80／120／End三個Chain，next Box各管理另一owned值；不是borrow、同一塊stack或按比例ABI圖。 |
| deref-coercion | &Wrapper→&String欄位→&str同內容，兩條借用箭頭／Target不同，沒有生成副本文字。 |
| dyn-dispatch | values0 Order80×1與values1 Bundle120各資料／method表，型別相同資訊指不同實現；沒有move／clone；O3保留。 |
| error-chain | ReadCountError abc→ParseCountError Integer→ParseIntError，source借原因直到None，每層Display只說自身。 |
| shared-tail | tail、first.next、second.next三條線匯同120 allocation strong3，再next到End；三份owner不是三份120。 |
| strong-cycle | 明標假設與內A→B／B→A，外部移除後各1保活；沒有在未教RefCell前偷偷給可執行構建例。 |
| guard-lifetime | 建view→最後讀仍活→tryMut Err→Drop→Ok，箭頭是順序而非資料搬動；區分ordinary reference最後使用。 |
| owned-downcast | String Rust→請求i32得Err(original)→再請求String得OkBoxString，失敗沒吞掉資料或把它轉數字。 |

沒有必要圖缺席的已確立問題；8.2清理順序／8.4資料與borrow／8.9單位則現有有限順序例與文字已足定位，不為圖數量硬加裝飾。未點擊全站導航／每個外部link／所有fold或source下載；這些範圍不宣稱通過。

本初判留下的unknown是未執行的38編譯負例／全部正常examples與完整QA、不同平台／pointer寬度、完整發布與真人目標讀者反饋。它們不自動算教材缺陷，也不借root舊checks填成自己的pass。

## 封存與停止

原逐段判斷留work；本文件、SHA sidecars、讀入／source／圖／selected執行指紋另有immutable initial副本。封存後立即通知root並停止，等待交叉許可；不因他人問題提示改寫original，不自行寫修法或啟動第9章primary／新章作者工作。
