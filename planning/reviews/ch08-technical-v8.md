# 第8章 TECH ORIGINAL initial：ch08-complete-prerequisite-v8

本輪沒有確認的必要技術錯誤或範例超前依賴；保留一項可選的名稱來源補充 I1。這是非作者的來源、契約與證據審閱，不是首次閱讀或整章發布驗收。已核閱全部66個 BODY 單位、全部150份 archived source，包括原位8張 SVG 的來源；實際桌面／手機頁面尚未驗證。自跑6個選定 library tests、3個正常 example、7個 compile-fail 與2個 runtime-fail，結果符合原稿；未自跑完整49個 example、26個 tests 或全部40個 negatives，也未用 root 舊 QA 代替本輪證據。

## 版本、角色與封存前隔離

- 唯一審閱來源：/workspace/work/tutorial-implementation/frozen/ch08-complete-prerequisite-v8。
- manifest SHA-256：79036df9490cd5ad5874be7978062928ff185e1d25abc76adf610c2678eae9c1。
- manifest 實際包含10頁、66個 chunks、150份 source。逐一核對66／150的原 bytes 與 manifest hash，全部符合。8份 assets 複本也逐一比對 archived SVG bytes。
- source 組成：12份 library Rust、49份 example Rust（39正常示範＋10份練習答案）、38份 compile-fail Rust、2份 runtime-fail Rust、40份 expected、1份 package Cargo.toml、8份 SVG。library 原始碼共有26個 test。
- 審閱者不是第8章作者。先前為第12章寫作已讀第8章8.1–4／8.7的 ownership prerequisite v3、8.5的 dyn prerequisite v4、8.8的 interior prerequisite v5；也具有自己第6／7章技術審閱背景。因此本輪不稱新 blind 或 first reading。
- 本輪 initial 封存前未讀第8章 primary／peer first-reading、technical／continuity reviews、root 問題或處置清單、作者 ledger／source notes、live 修稿或後來 snapshot。任務提供通用查覈範圍，沒有提供第8章已知缺口或答案。
- 遵循 clear-tutorial、review-protocol、rust-terminology 的通用準則；入口仍是懂基本 Python，實際已教 Rust 前文才算先備。下方支持與 I1 均為封存前自行核閱所得。

指紋與執行材料保存於 /workspace/work/tutorial-implementation/technical-ch08-v8/：source-fingerprints.json、archive-check-and-output-comparison.json、run_probes.py、probe-results.json、own-lock-evidence.json。初判副本／SHA sidecar 封存於同一目錄。不是用後來 peer 同意補算 initial 發現。

## 實際先備與首次需要重建

除先前已讀背景外，本輪按需要真正回讀以下凍結 BODY；沒有把大綱、標題或作者 ledger 當已教：

| actual 前文 | 回讀位置／已教能力 | 本章的實際用法 |
| --- | --- | --- |
| ch01-complete-source-v8 | 1.10的010-003：test、cfg(test)、同檔 module、crate::、獨立 assertions | 全部 library tests 不新增測試框架或互相依賴的狀態 |
| ch02-review-fixed-v5 | 2.6的006-003等 slice／索引／assertions；既有 owner、move、shared／mutable borrow | Box借用、slice大小、Chain／SharedChain迴圈、Cell／guard數值操作 |
| ch03-review-fixed-v6 | 001-002／004 enum variant與borrowed match；004-002 String／str reference的角色；005-002 UTF-8與byte len | linked enum、原因分類、String文字、payload String的11 bytes。11是4＋1＋兩個3-byte中文字，不是11個字 |
| ch04-publication-target-fixed-v8 | 4.2 Copy／Clone／Debug／PartialEq；4.4 lifetime省略；005-002／003／004 borrowed struct／placeholder／receiver | Drop與Copy排斥、借用欄位、dyn lifetime、source輸出reference依receiver有效 |
| ch05-review-fixed-v7 | 001-003 panic macro；004-002 From／Into及prelude；Display／fmt、type alias、ToOwned、?與原因enum的既有內容 | 空Error impl、boxing From、錯誤鏈、測試panic、Add／newtype轉換 |
| ch06-review-fixed-v10 | 002-002／003 Holder與associated type；003-002 Fn名稱來源；004-002～005 impl Trait／opaque／capture；005-003 Iterator名稱；007-005比較能力 | Deref Target／Add Output、不同closure的dyn介面；不把Copy內欄位推出wrapper自動Copy |
| ch07-review-fixed-v13 | 004-005 ToOwned prelude；其餘既有CLI／I/O能力不作第8章新方法理由 | errors.rs所用to_owned已具名稱與操作來源；第8章沒有偷用新檔案或thread API |

這裡的回讀是技術判斷的背景覆核，不表示第8章讀者先讀了第12章或本輪正式 API。Rust 新契約必須能在原 BODY 取得；官方來源只用於確認其正確性。

### 8.1：001-001～007

原問題是用有限資料鏈呈現遞迴型別的表示，並分開型別大小與一段內容的大小。001-002在查詢以前先教 Sized／DST與固定大小reference；size_of取得型別bytes，size_of_val取得被借範圍bytes。001-003在helper首次使用處教預設 T: Sized 與 ?Sized解除限制；T:?Sized不是要求T必須DST。64-bit示範限定reference的16 bytes，不推成所有平臺。

001-004教const N的值參數、[T;N]型別位置、body中的N與turbofish型別／值順序；固定長度從型別選定，不是在函式裡數元素。001-005在使用前教Box接手、借用、whole move與Box特殊owned move-out，並明確先用具體Box契約，後續8.3才解釋Deref；沒有把自訂Deref偷偷當作第一份Box程式的先備。Box不是必定在stack，非零Sized資料的Box自身才是單一pointer表示。

001-006的End／Link／Box next使自身表示有限，into_total每次接手next，再取出下一份Chain；沒有Drop impl，故不觸發後續的Drop型別欄位move限制。示範與answer總額200／0／240只是選定小型有限鏈，沒有對所有長度的清理成本或效能作證明。4份negative分別查Sized bound、const長度、Box move與直接遞迴layout；沒有互相代答。boxed-chain來源中的箭頭明定owned管理關係，不作padding或ABI證據。

### 8.2：002-001～006

原需要是知道owner何時收尾、如何提早交接收尾；不是隻印任意訊息。首次Drop簽名與prelude名稱來源在002-002交代，正常scope輸出3→2→1支持局部變數逆宣告順序；最後borrow使用不推出owner立即drop。002-003的std::mem::drop按值接手，與不允許直接呼叫Drop method區分；Copy／shared reference不會因drop副本而清理原referent。

002-004直接比較Pair自己先Drop、欄位依定義順序10→20，與locals逆序不同；自跑drop_fields的stdout逐byte符合BODY。002-005的Copy＋Drop與有Drop型別的non-Copy欄位move-out各有獨立fixture。move_field_with_drop自跑得到E0509，問題是整體仍需Drop而欄位已被移出，不是普通field從來不能move。002-006答案沒有新增API或以最後使用取代真正收尾。正文限正常控制流程，不承諾process任意退出都會收尾。

### 8.3：003-001～006

原需要是讓Wrapper<String>接上既有借用介面而不clone／接手文字；直接讀公有value.as_str同樣可做，沒有貶低它。003-002先教Wrapper擁有inline欄位、Deref Target／receiver與明確std::ops import；003-003再教DerefMut使用同Target且需mutable receiver。星號借用取得T的方法不等於交回owned non-Copy T，與8.1的Box特殊能力分開。

003-004的&Wrapper<String>→&String→&str與Vec→slice都有同一資料對應；mutable轉換回到原i32使80→101，並不clone。圖中兩段Deref箭頭與文字字段相符。003-005拒絕從Deref借用移出String，並以into_inner(self)在無Drop wrapper整體接手後移出欄位；另外的Copy bound negative排除「Target是Copy所以Wrapper也Copy」。003-006答案沿既有push_str與into_inner，未新增解構技巧。

### 8.4：004-001～005

004-002第一次使用T:'a以前教清楚內部borrow依賴至少符合'a、不是強迫value活到'a；與?Sized的+組合也在當下說明。Borrowed的&'aT確實含implied T:'a，原稿明記顯式bound只是教學呈現，沒有宣稱永遠必須手寫。String、str、nested &str三個T角色不同；*nested.value只是Copy reference資訊。

004-003的&'static str是程式保存literal有效，不是變數本身不會結束；choose_label回傳literal仍受其輸入／輸出借用契約，未宣稱caller一定得到static。004-004的T:'static只是型別不依賴短borrow，Message仍在block結束Drop；Box接手reference不接手referent，短borrow fixture仍失敗。004-005用owned Wrapper<String>串回Deref，不新增靜態儲存配置或延長生命的helper。

### 8.5：005-001～008

原需要保持80＋120及新增Shipping20的同一計價工作，讓caller新增具體型別而不用改library KnownCharge variants。005-002先示範不同型別不能直接同Vec，沿既有enum可以完成固定種類；005-003再教dyn DST、&dyn借用視圖與Box<dyn> owned handle的unsizing，沒有把它混同Deref。signature／field中無其他bound的Box object預設static在首次相關使用處交代，並明記local推導不同。

005-004對照泛型monomorphization與dyn透過資料／vtable入口查找，diagram明定不是ABI佈局或效能測試。known與dyn各算200，新增caller Shipping纔算220；效果是開放實作種類，沒有說enum原來算不出200。005-005比較concrete、同一opaque hidden Bundle與dyn不同分支；Box<dyn Fn>的move capture／shared呼叫先備在第6章，資料只有Copy整數。

005-006將內部prices borrow接到Box<dyn Charge+'_>，先教impl BorrowedCharge<'_>與object bound的兩層要求，再示範default-static negative。005-007以完整fixture分開trait Sized／method type generics／returns Self的不相容，where Self:Sized在用前介紹排除該method的dyn dispatch，supertrait仍保留amount。自跑dyn_rebuild_call正因Sized限制拒絕呼叫；普通associated Output在dyn Measure<Output=i32>已固定則可用，和GAT不是同項規則。這節不宣稱列完全部dyn規則，也未提前實作進階GAT或thread auto traits。

9份negative與normal／答案分別支持同型別集合、trait能力、opaque、object lifetime與compatibility問題。an_external_model_joins_the_same_heterogeneous_list自跑1 test通過：空集合0、caller模型加入後220；不把這個結果當作動態呼叫一定比較快的證據。

### 8.6：006-001～006

原需要是共同Display／source報告器，原Result／具體enum已可保存錯誤，不是為了讓Result開始能保存E。006-002先教Error: Debug＋Display、source的Option／reference／object-static關係與trait method預設body，所以空impl有明確來源。標準ParseIntError能借成Error與?的From控制流程在006-003接回原解析／檢查工作。

ReadCountError、ParseCountError、leaf逐層保存自己的原因；每層Display只描述本層，report再遍歷source，不重複拼接同一cause。error-chain.svg與abc三層、borrow箭頭、最終None相符。report使用的是本節有限且朝下的原因鏈；文字明定這些source最後None，不把任意自訂Error鏈都不會循環當成標準保證。

006-004將外層borrowed label、owned static cause與source回傳reference借自error分開。with_context的text只解析，label才保存於錯誤；receiver省略規則使source reference不能逃出局部error。自跑a_borrowed_context_can_expose_an_owned_static_cause通過，同時error_source_escape得到E0597、error_non_static_source得到lifetime診斷，兩種失敗是不同契約，不能用一個static說法包辦。

006-005的Box<dyn Error>在signature採static，標準From<E>在此object bound下boxing原Error，?只在當前函式回傳；caller仍擁有Err內Box，&*error只借其內容。具體enum分類與共同報告邊界理由有分工，不要求所有library都用dyn。a_report_includes_each_context_and_leaf_once自跑通過，完整答案的OrderError仍借原CountError，3份negative各有不同約束。沒有沿用deprecated cause／description，未把nightly API放進範例。

### 8.7：007-001～007

原需要是兩份清單共同持有同一120尾段，任一開頭可以先結束。007-002先教Rc名稱、單thread範圍、strong計數、clone handle而非clone T、ptr_eq配置身分；把strong與ordinary &T借用分開。007-003 total借用SharedChain，next是&Rc，Deref coercion前進而不clone；圖中的tail、first.next、second.next三份strong確實合流到同一份120，其next另保活End。

007-004限制Rc不提供DerefMut、不能任意move出non-Copy T；沒有說mut binding給handle就取得獨佔內部T。007-005先教Weak new／downgrade／upgrade契約；T清理與Weak仍保留allocation分開，成功upgrade多出Rc。正文限定「本節正常建立的配置」，未涵蓋尚未完整建構的特殊API。

007-006循環圖明定假設資料關係，沒有在RefCell未教前用可執行雙向修改程式。strong各1確實互相保活；Weak改的是不該保活的方向，不聲稱任意改一欄位即可解所有圖循環。007-007答案的Some temporary在match分支結束，最後第三份Rc才結束；an_upgraded_weak_is_an_additional_strong_owner自跑通過，暫時upgrade真能接替原owner，直到它結束Weak才None。2份compile-fail不需要未教的thread API。

### 8.8：008-001～007

原問題保留record(&self)與共享caller；ordinary Vec需要&mut self的替代也明確承認。008-001在shared Vec失敗後介紹interior mutability，不把它說成全部shared reference無限制可改。008-002的Cell get需Copy、set清理舊值、replace交回owned舊值、into_inner消耗Cell，在程序前已有契約；Cell<String>不是不能保存String，只是不能get複製。

008-003在borrow／borrow_mut首次使用前先教Ref／RefMut guard、Deref／DerefMut、receiver與panic條件，RefCell擁有T不等於共享owner。008-004將普通reference最後使用與guard Drop解除runtime borrow分開。自跑guard_used_then_conflict先印80，再在borrow_mut panic status101，符合expected already borrowed；讀完guard沒有解除borrow。try方法只回Err，不建立衝突guard或修改內容。every_shared_guard_must_end_before_a_mutable_guard_starts自跑通過，兩份shared必須都結束才取得writer；圖的流程箭頭與Err→drop→Ok相符。

008-005把view回傳guard責任連回Recorder有效性，沒有clone Vec或把局部RefCell的guard帶出去。refcell_guard_escape自跑E0515。008-006 Rc與RefCell各管保活／借用狀態，clone Rc不取得guard，borrow不clone Rc；父子例子只parent欄位需RefCell，children保持普通Vec。自跑weak_parent全部stdout逐byte符合BODY：temporary父owner結束後branch strong1／leaf strong2，Drop父id2後leaf strong1、Weak None、最終Drop leaf id1。008-007答案使用到的API都已教，4組negative含1runtime，沒有將Cell／RefCell偷當跨thread同步。

### 8.9：009-001～007

原問題是i32同數字而不同單位不能互傳；alias縮名仍是同型別，與PriceFn合法回到原型別的用途公平分開。009-003在首次tuple struct code以前教宣告、公有欄位、建構與.0，Meters與Seconds獨立身分；也明記公有欄位不驗證非負，newtype不自動完成驗證模型。

009-004首次Add用前教Rhs預設型別參數、Output與owned receiver；Self(...) tuple struct建構形式也在使用前交代。Meters不是Copy，兩邊接手後不能再用原left；mixed Add明定毫米＋公尺先換算→2500毫米，不自動產生反向實作或無限精度數學。

009-005 orphan以定義所在crate判斷local，不因import／alias把Vec變local。對非泛型Display self，至少trait／self一方本地；newtype MeterBatch的Display只借原Vec，有本地BatchSize為外部Vec實作的另一方向。限制公開method、不自動繼承push的理由是保持選定報告介面，不強迫newtype模仿內部型別。

泛型From後再教self先於trait arguments與第一local型別前uncovered T限制；Wrapped<T>作self與反向self=T確實不同。自跑newtype_uncovered_from得到E0210且指出相同檢查順序。Box fundamental限定有當下orphan用途、正文已教Box／泛型；只作規則提醒，未新增此種impl或要求一般框架。009-006 coherence分開orphan與overlap，blanket Label for T已涵蓋Meters，精確impl不能覆寫它。7份negative分別支持單位、move、方向、alias、method、orphan與重疊；009-007 TimeBatch答案仍沿既有borrow／Display／.0。

### 8.10：010-001～007

原需要是caller可擴充payload，傳遞層保存而處理者要取回確切型別；fixed enum與共同業務trait先前仍有用，沒有為Any貶低它們。010-002先教Any:'static與標準blanket impl，不要求使用者自己impl；局部owned String可借成&dyn Any，這次reference不static。

010-003 is／downcast_ref比對的是確切T，不是數字相等或轉型；i32／usize同80是必要的辨識對照。泛型helper屬dyn Any的inherent介面，和8.5「trait內的泛型method不能放進dispatch表」不同，正文在首次使用時已說明，不用後文替合法性補洞。

010-004 event參數impl Any沿6.4，Box擁有原payload；downcast_mut先需要&mut dyn Any，不是shared下任意改。add_to_number只改原i32，String保持原型別；有短borrow的資料不會因owned Box或+ '_取消Any要求。010-005先教owned downcast接手Box與Result<成功Box<T>,失敗原Box>，失敗仍可再請String，success沿8.1特殊move-out取得owned值。a_failed_owned_downcast_returns_the_original_string自跑通過，UTF-8 len11有前文依據，不把成功borrow誤寫成owned交接。

010-006明確查詢對象，Box自身type_id與&*Box內容type_id分開；自跑any_type_id兩個true的stdout逐byte符合BODY，而any_box_type_id編譯成功執行status101、assertion left == right失敗。TypeId不作跨版本外部保存身分，這項限制回應事件id／資料值與TypeId的角色差別。010-007 ExtraInfo答案只用既有事件、downcast與Box move-out；3組negative含1runtime，各自證明move、static要求與查錯receiver。

## 可選項 I1：Sized與Box的bare名稱來源可以一致地補一句

- 原位置：8.1／001-002定義Sized，001-003開始寫T:?Sized；001-005首次Box::new與Box<i32>。
- 原引文：「Sized是Rust標準函式庫中的trait」；「Box<T>是標準函式庫提供的型別」。Sized／DST／Box的角色、契約和教學理由當下都已教足。
- 已教能力：5.4／004-002解釋prelude預先匯入常用標準名稱；6.3的Fn與6.7的Ord、8.2的Drop也有自己的名稱來源說明；其他標準名稱Rc／Deref則明寫use。
- 未明寫的最小關係：這兩個名稱分別來自std::marker::Sized／std::boxed::Box，也由prelude提供，因而當前程式不用新增use。官方prelude確認兩項均可直接用。
- 實際影響：讀者操作與理解不受阻，程式也不存在缺import；對照不同標準API時能更快判斷何時需use。這是名稱／來源一致性補充，不是Sized機制、Box ownership或?Sized先備未教，也不以過去其他章同類問題自動升級本項。
- 分級：選讀／編輯改善，可保留原稿。建議只在兩個首次定義處各補短句，不擴成一節prelude清單。無須為文句重跑Rust。
- 發現者／時點：TECH8獨立initial，封存前自行比較actual正文與前文、查官方，未受peer8或root問題清單提示。未修改教材；必要性由root後續裁定。

## 正式來源與查覈範圍

本輪真正開啟或find讀到下列官方selected條文；不是隻列連結。std頁面顯示1.99.0、b940084d7 2026-09-28，與本機工具一致。沒有要求正文照搬官方完整API清單；只確認當前使用與限定。

| 官方來源 | 實際核對的最小契約 |
| --- | --- |
| [Sized](https://doc.rust-lang.org/std/marker/trait.Sized.html)／[prelude](https://doc.rust-lang.org/std/prelude/v1/index.html) | 大小、預設generic Sized、?Sized、Self例外；名稱提供 |
| [boxed](https://doc.rust-lang.org/std/boxed/index.html) | nonzero heap配置、Sized Box單一pointer、遞迴資料；不把這段的raw／FFI例子當讀者先備 |
| [Reference destructors](https://doc.rust-lang.org/reference/destructors.html)／[Drop](https://doc.rust-lang.org/std/ops/trait.Drop.html) | Drop先執行、struct fields定義順序、locals逆序；Copy／Drop互斥 |
| [Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html) | Target可?Sized、返回borrow、coercion與method查找、公開Deref的介面責任 |
| [trait objects](https://doc.rust-lang.org/reference/types/trait-object.html)／[lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html)／[dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility) | DST與資料／vtable、object default在expression／signature差異、supertrait、where Self:Sized、泛型method、associated type |
| [Error](https://doc.rust-lang.org/std/error/trait.Error.html) | supertraits、source與default、Display／source避免重複原因；From<E>對Box<dyn Error+'a>實際要求E:Error+'a，當前'a=static |
| [Weak](https://doc.rust-lang.org/std/rc/struct.Weak.html)／[Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) | Weak保留配置不保活T、upgrade新增Rc、空Weak；clone handle不clone T |
| [Cell](https://doc.rust-lang.org/std/cell/struct.Cell.html)／[RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html) | Copy get、replace／into_inner接手、guard有效到清理、borrow衝突panic／try Result |
| [Add](https://doc.rust-lang.org/std/ops/trait.Add.html)／[Reference implementations](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence) | Rhs default、Output、owned receiver；orphan順序、uncovered限制、Box fundamental、overlap |
| [Any](https://doc.rust-lang.org/std/any/trait.Any.html)／[Box downcast](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.downcast)／[std::any](https://doc.rust-lang.org/std/any/index.html)／[TypeId](https://doc.rust-lang.org/std/any/struct.TypeId.html) | Any static blanket、inherent helper、確切T借用／owned取回、Box receiver type_id、跨版本不依賴表示 |

## 自跑probe、輸出與限制

執行只使用archive的隔離package，所有142份package source複本逐bytes核對original manifest，沒有讀取live改稿。隔離工作區由本輪建立Cargo.toml、rust-toolchain.toml與自己的Cargo.lock，不複用root lock。自己的lock只有ch08-smart-pointers 0.1.0，SHA-256為3315475cf952643ac73d9e2ffcbabba2e8e40a75bc54c45c0718773ce343a90c。

實際環境是rustc 1.99.0 b940084d7、host x86_64-unknown-linux-gnu。新shell先載入scripts/env.sh，並指定本輪CARGO_TARGET_DIR及RUSTUP_TOOLCHAIN=1.99.0。完整command array／cwd／exit／stdout／stderr／expected保存於probe-results.json；run_probes.py可重做，但本輪沒有無故重複全套測試。

選樣各有當前問題：

- 6個tests各自執行exact filter，均1 passed／0 failed，另外25項filtered；驗caller模型擴充、Error label／cause分工、一次報告每層、upgrade接替owner、兩份shared guards都必須結束、failed owned downcast保留String。
- 3個normal：drop_fields、weak_parent、any_type_id均status0，實際stdout分別逐byte比對002-004、008-006、010-006的text fence，完全相符。不是隻看cargo正常退出。
- 7個compile-fail：move_field_with_drop E0509；dyn_rebuild_call固定method診斷；error_non_static_source lifetime診斷；error_source_escape E0597；newtype_uncovered_from E0210；any_non_static_borrow E0597；refcell_guard_escape E0515。全status1，stderr包含各自原expected，並讀實際錯誤位置，避免別的失敗碰巧含代碼。
- 2個runtime-fail都先編譯status0，再執行status101；guard_used_then_conflict先stdout80、在borrow_mut already borrowed；any_box_type_id在該assertion失敗。沒有把它們說成compile-fail，也不依賴TypeId的具體hex bytes。

其他31份compile-fail／其expected、其餘normal／answers／tests均已全文核閱來源與第一次需要關係，沒有全部執行。本輪未作全workspace check、clippy／fmt、站點建置、跨平臺執行、桌面／手機實頁視覺或點擊跳轉驗證。8SVG已核原位BODY／source／箭頭含義、物件／數字對應、XML與width／height／viewBox；這只證明來源一致與所表關係，不能代替字體、換行、裁切與手機實際可讀的驗證。

## initial判定

在已實讀範圍，核心方法有原需要→新契約→可觀察關係的銜接，常見混淆與各自失敗對象有區分；技術事實與限定由selected正式來源支持。範例、helper、test、答案、fixture、expected與Cargo使用沒有確認的未教必要能力。I1只作名稱來源補充；未發現證據不足而需放寬入口背景或改教學範圍的項目。

此判定只保存獨立TECH initial。頁面視覺、其他角色首次理解與根據同行初判追加的檢查仍各有範圍，不以本輪來源審閱代替；封存後停下，待root另行開交叉覆核。第12.4仍等待actual10.8，本輪沒有寫ahead。
