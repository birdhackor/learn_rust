# 第7章尾段首次順序閱讀 v1

角色：AI模擬首次讀第7章尾段，目標只懂基本Python。先備只用本人實讀1–4正文及局部修後、5v7全35／6v6全50與6v8兩指定正文、7v9 prefix的7.1–7.5全34單位；前文版本差異已記entry／prefix，不宣稱整章1–6最新版本全讀。第4章曾cross，peer獨有API或說法不作已教前提。

唯一當輪正文為frozen/ch07-complete-prerequisite-v10的006-001起至009末，共28單位（7.6–7.9）。不重讀其prefix，不看author／完整source／其他review／outline／外部解答或第8章。prefix原record SHA-256 292e71f383cad7e5055fa1102eb9dafaa6a50a54159919ca5dc34c0de25c21cb；entry SHA-256 5a09c8f5c308a2a77ad95c688099e0cf27265a09efe080e6709b209c78746c43，均不改。

使用root提供read_visible_unit.py ch07tail gate，一次一單位；closed fold在輸出前遮蔽、只marker。每必要新概念首次及節末保存Q1關係、Q2a機制、Q2b原需→改變→用途、Q3操作、Q4例／圖證據，附原句／未知／最短已教推論與hash；save_ch07tail_record.py append當下理解後gate才放下一單位。後文釐清只追加、不覆蓋初判或prefix分級。

圖片只有當實際正文原位指到該凍結SVG才開，可native渲染並記非整頁。未取完整source／整頁／實跑／外部資料，不把正文預期當實測。完成28單位先獨立封存hash與summary，再通知root停止，不提前找第8章。

首次開006-001時gate原先在tail過濾前assert，因完整order首項001-001而拒絕，沒有輸出正文或生成state。僅將006–009過濾放在assert前修正工具，未開prefix；其餘保存檢查／closed遮蔽不變。

## 原始逐單位紀錄

### 006-001 — clap本節原需與驗收界線

Q1：clap是命令列參數解析library，7.1已設定來源／derive功能；此節要得到owned設定，尚不執行搜尋。
Q2a：先預告由struct欄位建規則／說明，具體macro與API待下一段。Q2b：「自己維護位置、開關與說明之間的對應」原需→交library依同一欄位規則產說明及設定→支援找文字／檔名、忽略大小寫與用法／版本請求。
Q3：cd examples/ch07-cli/clap，後續命令以此cwd；搜尋留7.9，當前不用猜算法。
Q4：「本節只驗收參數如何成為設定，以及顯示請求如何交給呼叫者」界定非所有CLI工作已做完。
未知／影響：新clap介面合理待當節教，無當下必要缺口；未讀其他source／文件。

原始SHA-256：27d1679b4b55b97f9677b8df2b36d5c62fe3c37ecd3a85dbd83f26f7723c7d99；visible SHA-256：27d1679b4b55b97f9677b8df2b36d5c62fe3c37ecd3a85dbd83f26f7723c7d99。

### 006-002 — 位置參數／flag與owned欄位

Q1：positional依順序決定角色，flag按long名稱啟用且不跟值；設定String擁文字、PathBuf擁原生路徑、bool保存選擇，型別沿已讀。
Q2a：Rust第一→query，中文檔名第二→filename，--ignore-case有true／無false，只接受宣告名。Q2b：原需兩資料與可選忽略→區別位置與具名開關→不把檔名混文字要求或flag多吃一值。
Q3：query-settings Rust 中文筆記.txt --ignore-case得相應三欄位；--unknown是用法錯誤，不開檔案、不轉大小寫。
Q4：「解析 filename 不會…保證檔案存在」延續7.5表示／IO邊界；表格給輸入到結果的一一關係。
未知／影響：新CLI分類已足夠，struct宣告／實際parse待下一單位；無必要缺口。

原始SHA-256：ecaf7dd24a8f7d26624cfbedc5cc6f7037722eda2912a9e8e210cb4483015cb0；visible SHA-256：ecaf7dd24a8f7d26624cfbedc5cc6f7037722eda2912a9e8e210cb4483015cb0。

### 006-003 — Parser derive與command／arg attributes首次必要

Q1：clap::Parser是資料→設定的trait，derive(Parser)編譯時依欄位／attributes生成實作；沿4.2 derive與7.1 clap feature／版本／lock契約。
Q2a：command設定顯示名／版本／用途，version提供--version；無short／long的String／PathBuf是宣告順序必要位置；bool arg(long)欄位ignore_case→--ignore-case，help字串產欄位說明，--help不需先給必要位置。Q2b：原需規則與說明一致→同Settings欄位及metadata生成parser／help→成功欄位保存選擇，後續程式再使用。
Q3：use clap::Parser及PathBuf，Debug／Parser derive，依例建立公開三欄位；command name不是尋執行檔，Cargo啟動名可在usage另顯ch07-clap。
Q4：逐項attributes說明在code前，原query／filename／flag表關係全部回收到struct；「並不是…先執行某個 Parser 函式」限編譯生成時點。
未知／影響：生成內部不必要，parse呼叫合理待下一段；無需查macro答案／API文件，沒有必要缺口。

原始SHA-256：6be948e58244a004bd0afd07bc33868aef90bd7b165cf386f8602758bfc910da；visible SHA-256：6be948e58244a004bd0afd07bc33868aef90bd7b165cf386f8602758bfc910da。

### 006-004 — try_parse_from輸入契約與Err顯示請求

Q1：Parser trait方法try_parse_from回Result<Settings,clap::Error>；Err未產設定，但help／version亦是正常顯示請求，非一律用法失敗。
Q2a：輸入IntoIterator、Item同時Into<OsString>與Clone，owned array／args_os符合；完整序列含程式名直接交，不先略。Error shared use_stderr→bool、exit_code→i32，此版診斷2／顯示0；parse不印／退出。Q2b：原需owned設定與顯示政策→先留Result與出口／狀態判別→caller可保正常help而非所有Err返失敗。
Q3：四tests核flag off／on、缺位置／unknown stderr2、help／version stdout0；只比欄位／方法結果，不比完整clap::Error或排版。
Q4：「還沒證明真實 CLI 訊息寫到正確出口」限定library tests；From／Into／Clone／associated Item各已讀，+兩能力由文字直接說清。
未知／影響：error實際印法待下一單位，當前輸入／回傳／分類足夠，無必要缺口；未執行四tests。

原始SHA-256：a5fa883c969b1e2431efc698412e5b4d9851c016c1ece4458281cc6e794baf6b；visible SHA-256：a5fa883c969b1e2431efc698412e5b4d9851c016c1ece4458281cc6e794baf6b。

### 006-005 — Error.print的IO結果與設定工具main

Q1：error.print shared借clap::Error，按角色寫stdout／stderr，回io::Result<()>；不退出。成功Settings只供設定檢查，非搜尋結果。
Q2a：保exit_code後print，成功才回0／2，print Err返標準FAILURE；show_settings借Settings用writeln!與?，ExitCode及stdout沿7.5。Q2b：原需library可測且caller決定顯示／清理→try_parse_from與print分開→不由Parser::parse／error.exit立即終止。
Q3：完整args_os交parser；正常三欄位有／無flag true／false，檔案不必存在；help／version stdout0，缺或unknown stderr2，真正IO失敗FAILURE。
Q4：「三件不同的工作」與兩層match足以追每種結果；Cargo --與工具--ignore-case已在command位置區分。
未知／追加：新print機制與用途足夠、無必要缺口。先前prefix O1的Stdout完成位置未於show_settings補說，仍保optional原判；7.8真行程測試合理待教，未實跑。

原始SHA-256：08c4930979978bb4d541e2e0acee106d829e98e9c2a0ed9ba0bec89ccbdae9f8；visible SHA-256：08c4930979978bb4d541e2e0acee106d829e98e9c2a0ed9ba0bec89ccbdae9f8。

### 006-006 — 7.6節末owned設定與新增flag

Q1：line_numbers新增bool設定，--line-numbers是新long開關，仍不產生搜尋／行號輸出。
Q2a：依已教arg(long)欄位underscore→hyphen規則，省略false／提供true，parser保存其他既定欄位。Q2b：原需多位置／開關／說明對應→同模型生成規則與owned設定→可加選擇且純測參數，不先混執行行為。
Q3：新增欄位／help與兩情況tests即可最短推得，核query／filename與ignore_case不變；closed答案只marker，外連未開。
Q4：「只解析與核對設定，先不輸出行號」清楚隔離當下驗收；既教try_parse_from／owned資料／Result足完成。
節末：Parser生成、位置／flag、Clone＋Into輸入、Err顯示與IO／ExitCode全有前提；無必要初判問題，O1僅舊可選界線未解，未實跑。

原始SHA-256：5efe1cc24a15ffa2b5f3a67cae8b6cae63f7c2952227f8f3f5ce11a267c616da；visible SHA-256：d7f747b3146aa895a30e2d005ff56dc6d5ef1b455f018bb11ee06568c2c5adf6。

### 007-001 — JSON資料模型的原需

Q1：JSON是資料交換文字格式，不需執行JavaScript；本節讀完整UTF-8後要轉已教Rust型別。
Q2a：目前先說資料形狀，未教解析／轉換API。Q2b：「還要保存標題、數值清單與是否封存，讓下一次…讀回這些角色」→結構化文字→恢復有角色的owned模型，而非一串未分類內容。
Q3：cd examples/ch07-cli/json，fixtures依此cwd，沿7.3完整文字IO。
Q4：標題／清單／封存三角色足以界定後續欄位，尚不猜JSON或Serde語法。
未知／影響：資料格式與Serde介面合理待教，無當下必要缺口。

原始SHA-256：1c20c399b705062f1ddc54fc1bffa41a3c6a68838ef3f5e8456f50506f8c46a3；visible SHA-256：1c20c399b705062f1ddc54fc1bffa41a3c6a68838ef3f5e8456f50506f8c46a3。

### 007-002 — JSON object／array／escape與欄位形狀

Q1：JSON object名稱→值、array依序值；對應Rust String標題／Vec<i32>清單／bool封存，非Rust struct建立式。
Q2a：名稱雙引號、冒號接值、逗號隔項，不容註解／末項後逗號；文字LF寫\\n、引號\\\"作escape區別內容邊界。"120"是文字，超i32整數也不符模型。Q2b：原需保存三角色→明示名稱與資料形狀→可讀回有型別模型，不因JSON接受更多形式就假定Rust欄位全收。
Q3：notebook.json三欄位應恢復筆記、[80,120]、false；保數字順序，不能把有引號數字自動當i32。
Q4：表格給每JSON位置與owned欄位，無須執行JavaScript或查JSON答案。
未知／影響：JSON其餘null等合理不需完整教，當前模型範圍與escape足夠；Serde如何轉換待下一單位，無必要缺口。

原始SHA-256：cb296d4c1a209030e1ccb84fb5e83b714e8b9995fe9fea05079f25b58d172085；visible SHA-256：cb296d4c1a209030e1ccb84fb5e83b714e8b9995fe9fea05079f25b58d172085。

### 007-003 — Serialize／Deserialize能力與serde_json格式

Q1：序列化資料→交換格式，反序列化格式→資料；Serde提供Serialize／Deserialize<'a>能力，serde_json選JSON格式使用它們。Deserialize lifetime連本次輸入，但模型可owned或borrowed。
Q2a：serde derive feature編譯時按欄位生成兩trait，String／Vec<i32>／bool已有能力；deny_unknown_fields拒未知欄位，預設會忽略。Notebook無reference，標題／清單屬自己，PartialEq比全欄位與Vec位置。Q2b：原需同欄位可編碼／讀回並抓拼錯→模型能力＋JSON實作＋明選拒未知→獨立owned資料不依原String存活。
Q3：兩dependencies按7.1版本範圍／lock、use兩trait，derive與serde attribute；成功結果可Debug及完整相等，不把Serialize當只顯示文字。
Q4：「不是執行時呼叫的函式」限定attribute工作；「欄位中沒有 reference」足推資料owner／來源關係，沿3.4／4.5。
未知／影響：反序列化內部與borrowed模型未需，具體from／to API合理待下段；無必要缺口。

原始SHA-256：dddad3bb28c1c54dac550c37e6ae25517208f34891fa64b30d6e45f19f98dae2；visible SHA-256：dddad3bb28c1c54dac550c37e6ae25517208f34891fa64b30d6e45f19f98dae2。

### 007-004 — JSON泛型API、owned往返與原位圖

Q1：serde_json::from_str::<Notebook>(&str)依Deserialize回Result<Notebook,Error>，非需模型FromStr；to_string_pretty借Serialize模型回Result<String,Error>。Error Debug／Display保存格式工作失敗，非IO。
Q2a：Deserialize<'a>連輸入來源但owned Notebook不存reference，可讓原String結束；語法／缺欄／型別／未知／超範圍皆可Err。編碼產新含縮排文字，保資料不保原排版。Q2b：原需JSON恢復角色與重存→明模型decode／encode→獨立資料可用且格式實作處理escape，分讀檔與JSON失敗。
Q3：codec兩helper依API直接轉交；caller先read match，再decode核標題／項數，再encode，values排版變多行而數值順序相同。此示例有標題訊息、不是純JSON CLI。
Q4：首次原位SVG與native340×462實看：上借&str文字→from_str→中Notebook owner，title String／values Vec／archived bool分欄；中→to_string_pretty借模型→下新owned String，兩箭頭表表示轉換，不是結果仍借原文。底明排版可不同。SVG SHA-256 d614744ae58f91f324a1be6dc4eb47debe1149b4116b8f70d2ea8a82ec445448；PNG SHA-256 5c0c94dcf54508fb7ae36d0aa4034660b60f5ecbaf19b0316365b73e5549f5a8。
未知／影響：API條件與結果足夠、無必要缺口；只局部圖、未實跑或看整頁。

原始SHA-256：6a328c0182b4c7398611666592208a5ab6bd588620c636875de3799fe9d10f2d；visible SHA-256：6a328c0182b4c7398611666592208a5ab6bd588620c636875de3799fe9d10f2d。

### 007-005 — NotebookError與兩步?來源保留

Q1：NotebookError Io／Json各owned原錯誤，來源封裝／Display／From沿5.3–5.6，不derive全錯誤相等。
Q2a：read?錯封Io早退，不decode；decode?錯封Json，成功owned Notebook離helper仍可用。Q2b：原需讀模型可能兩來源失敗→具體variants＋From→helper簡化且caller按variant判來源不猜文字。
Q3：合法標題、missing Io/NotFound、broken語法Json、非法UTF-8先Io/InvalidData；for逐筆match繼續。
Q4：「文字已讀到但 JSON 語法錯」配四材料支撐階段差別；code所需borrowed Display與?條件皆前文已教。
未知／影響：無必要新機制或缺口；fixture實際bytes未取、command未實跑，僅核正文預期與路徑。

原始SHA-256：dc6bf87816f970a8b40ff744045f7bb7c71992092425c00cfd1ec754de6ca577；visible SHA-256：dc6bf87816f970a8b40ff744045f7bb7c71992092425c00cfd1ec754de6ca577。

### 007-006 — owned存活／全欄位往返／拒絕tests

Q1：codec三tests＋files來源一test各核模型／政策；assert全Notebook相等沿已教derive，不要求JSON排版相等。
Q2a：內層text結束後Notebook仍用；encode→decode比較全欄。拒絕材料用if let Ok抓意外成功，escape材料往返不用手寫規則。Q2b：原需可恢復有角色資料→測owner獨立、欄位保真與拒絕選擇→不只看解析成功或漂亮文字。
Q3：核筆記、兩值80／120、false；五種語法／模型不符應Err，換行引號往返同資料；四tests command未執行。
Q4：「不是證明任何 JSON 都符合任意 Rust 型別」限本Notebook；fixture實體內容尚未取，本輪不把名稱／敘述當實測。
未知／影響：tests無新必要API或缺口；quoted-title的actual escape bytes未驗，但已有JSON逃脫規則足讀操作。

原始SHA-256：bf128cc6e830b067d427539980f22a847a507efd6fbda4ddb0d91724bd6af807；visible SHA-256：bf128cc6e830b067d427539980f22a847a507efd6fbda4ddb0d91724bd6af807。

### 007-007 — 7.7節末只改欄位再編碼

Q1：archive讀owned Notebook、改archived再借編码交owned String；NotebookError仍保存IO／JSON來源，非新容器。
Q2a：read_notebook?後mut binding改bool，encode?由已教From封Json，成功Ok文字。Q2b：原需下一次可讀角色→資料模型變更再編碼→只改封存而保標題／順序值，以decode查資料非找文字true。
Q3：已教mut field／?／encode足最短推得；結果decode應筆記、80／120、true，closed完整答案只marker。
Q4：「不只看輸出文字是否出現 true」對準欄位保真，往返JSON排版差異不算資料變更。
節末：JSON形狀／Serde能力／兩API／owned圖／具體錯誤／測試前提足夠，無必要初判問題；外部process僅預告，未先補答案或實跑。

原始SHA-256：56117352c2c52aeb707db099aaa52f5267acd97a2b0e6ac7500d6bd671c643e7；visible SHA-256：d9b04a3cbe028117bdeee5aec8451777bc4dcc5b6a864b7d69e8488417184489。

### 008-001 — 可重複CLI fixture的原需

Q1：fixture是特定檢查已知內容／用途的材料，此節由每測試自己建立獨立目錄／檔案，非別人筆記。
Q2a：先描述每項各有目錄，不共用固定會覆寫檔名；子行程cwd將另指定。Q2b：「日後調整 main 也能發現出口或狀態被改壞」→可重複真CLI測試→保存原內容／診斷／狀態，而非只有library結果。
Q3：cd examples/ch07-cli/fixtures後Cargo跑測試，資料與子程式起點待後段API指定。
Q4：「每項測試使用自己的目錄」回應材料可控與彼此覆寫的需要，未假定現存檔案或權限。
未知／影響：暫存目錄與啟動／capture方法合理待當節教，無當下必要缺口。

原始SHA-256：86405313a8d3eb29e67f97d7b9092fe1f6b4602f147f9d2a9eac37d2dd268ab1；visible SHA-256：86405313a8d3eb29e67f97d7b9092fe1f6b4602f147f9d2a9eac37d2dd268ab1。

### 008-002 — 受測binary契約與依賴重用

Q1：本package binary重用path-cli library parser，測的是自己的main；tempfile是測試用暫存library，dev-dependencies沿7.1。
Q2a：main所有API沿7.5，原生filename讀UTF-8、stdout原文、stderr診斷、正常ExitCode。Q2b：原需真CLI出口／狀態→先固定四類材料與結果→測試新增工具不改原筆記需求。
Q3：中文空白檔名合法原文stdout／stderr空0；missing與255內容stdout空／診斷1；缺檔名診斷2，path ../path-cli依成員manifest起點。
Q4：表格明列stdout精確與stderr／status的驗收，code對應已讀流程；「不是…執行檔路徑猜測」指出接下來需本package target定位。
未知／影響：暫存建立／binary定位仍待教；本段無新必要概念或缺口，未執行四種情況。

原始SHA-256：df2d18c554b445c427c315fe8ac8ae25e44f337484b8f3f64ad5d597312d2a28；visible SHA-256：df2d18c554b445c427c315fe8ac8ae25e44f337484b8f3f64ad5d597312d2a28。

### 008-003 — TempDir資源owner／明確close及fixture IO

Q1：tempdir建新系統臨時目錄回Result<TempDir>，library取名；TempDir管理目錄資源、path借&Path，PathBuf只owned位置，非接目錄存續。
Q2a：scope結束嘗試刪目錄內容且忽略清理錯；close消耗owner並回io::Result<()>可核，之後不能借原owner。fs::write建／替換完整內容，&str以UTF-8、&[u8]原bytes，AsRef<[u8]>沿5.5；to_path_buf複製位置，try_exists回Result<bool>區別無法判定。Q2b：原需獨立已知材料且可清理→持有TempDir到IO／子行程結束→不因保存路徑誤使檔案消失，明確close檢查收尾。
Q3：tests/directory.rs建目錄／中文空白檔、readback核全文、exist true，close成功後saved_path仍有表示但exist false；fswrite不建上層，這裡tempdir已建。
Q4：「材料準備失敗…不能假裝工具已被驗收」限定expect為測試條件；try_exists只當下不保障後IO，未新增先查才讀政策。
未知／影響：Drop trait留8.2合理待教，當前資源契約足用，無必要缺口；未建立／清理實體目錄或實跑。

原始SHA-256：c28788a468609a4bb05cca7187afd52803a2e01d98023f1df56bc42a2eed5ef5；visible SHA-256：c28788a468609a4bb05cca7187afd52803a2e01d98023f1df56bc42a2eed5ef5。

### 008-004 — Command設定／parent-child／Output與status

Q1：Command是尚未啟動的設定，parent測試／child工具各有cwd／狀態；Output是已結束child的status ExitStatus及stdout／stderr Vec<u8>，觀察者status與main交出的ExitCode不同。
Q2a：new借程式表示AsRef<OsStr>；current_dir／arg mutable改設定回&mut Command，arg原生一筆不經shell拆空白；output啟動、等結束、收兩出口回io::Result<Output>。success零正常成功才true，code Option<i32>有數字Some／Unix signal可None。Q2b：原需真CLI三面驗收→固定child起點／一筆檔名／捕捉bytes與狀態→不靠終端混顯或把缺檔當啟動失敗。
Q3：child回1仍Ok Output但status失敗；外層Err才啟動／捕捉／等待失敗；有限小輸出才用等待全收，不拿無限服務。
Q4：「Ok 不等於工具工作成功」與code Option契約給兩層判斷依據；引號／>不是argument操作，parent cwd不改。
未知／影響：所有新操作receiver／結果與完成時點已教，signal只需理解None，不需OS細節；無必要缺口。

原始SHA-256：fe7da3918cd1416ccdf66d6ceb9dfab15c9e79e6b73b88c0d8b68532679f0339；visible SHA-256：fe7da3918cd1416ccdf66d6ceb9dfab15c9e79e6b73b88c0d8b68532679f0339。

### 008-005 — 編譯期env!執行檔位置與借bytes轉文字

Q1：環境變數是環境具名資料，env!編譯時取出&str、缺值編譯失敗；integration Cargo供本package CARGO_BIN_EXE_<binary target>絕對路徑並建binary，非dependency或通用main變數。
Q2a：target連字號保留。std::str::from_utf8借&[u8]驗UTF-8→Result<&str,Utf8Error>，Debug／Display已有；Output仍owned bytes，異於String::from_utf8接Vec。Q2b：原需啟動實際本工具→Cargo給可靠位置，設定child cwd／native argument→不猜target path；捕捉資料需可借檢查文字而不消耗Output。
Q3：run_note(directory:&Path,filename:&Path)建Command env!位置、current_dir、arg filename.as_os_str、output，只回IO結果不先解讀status。
Q4：「不是每份 main／unit test 都有」及「不…dependency binary」限定新macro可用範圍；helper三步與前段Command契約一一對應。
未知／影響：borrow文字來源沿4.4單來源和本段view說明可推，無必要缺口；未查環境或實啟動。

原始SHA-256：8bbbec3525b5a3d5c5cb5a11066b7e1783406877dcda988c0a9b173b626327d9；visible SHA-256：8bbbec3525b5a3d5c5cb5a11066b7e1783406877dcda988c0a9b173b626327d9。

### 008-006 — 真CLI三結果tests與fixture原位順序圖

Q1：真child Output三欄分核，TempDir保到child／assert完成才close；新str.starts_with shared借文字判前綴bool，Output已有Debug供意外成功panic。
Q2a：正常完整stdout／空stderr／success及Some0；missing／255是Ok Output的status1、缺參數2，不存在執行檔才外IO NotFound且無Output。前綴只核本工具診斷，不寫死OS細節。Q2b：原需日後main改壞可察→可控制檔案與真正binary→捕捉完整介面而非library結果／權限偶然失敗。
Q3：中文空白名一argument，非法byte借slice寫檔；missing不建，absent program用自有位置；--test cli四主文tests，另空檔答案使cli5＋directory1=6，但本輪未開答案／實跑。
Q4：首次原位SVG與native340×520實看：藍外框TempDir owner包建立fixture→output啟動／等待收兩出口→檢查stdout／stderr／status；箭頭離框才到close接owner刪目錄核結果。底PathBuf只位置，時間順序非owner箭頭互借；對應上段directory與本test。SVG SHA-256 d300657daac7ba35c30454a180509d275ace4856455141c694d7e8475e61f539；PNG SHA-256 366f0791033d0f27d08cac4c05b60eef3b9d34430be6a4b1ba5cffaa9d516faa。
未知／影響：新前綴與Debug在使用前教，關係足夠，無必要缺口；只局部圖／未實測六項或全頁。

原始SHA-256：dbbfec3210ab26218a2571cfccca055bd8636979a774a67a7d962fe44ae9ebbb；visible SHA-256：dbbfec3210ab26218a2571cfccca055bd8636979a774a67a7d962fe44ae9ebbb。

### 008-007 — 7.8節末空成功與完整介面

Q1：empty檔合法UTF-8空文字，兩出口零bytes仍成功Some0，異於missing的空stdout／診斷／1。
Q2a：沿tempdir、fswrite空str、run_note、Output三項assert與close，無新API。Q2b：原需完整CLI可重複驗收→每測試獨立owner／材料＋等待child→三項一起避免把無輸出當失敗。
Q3：以既有正常test改空材料，核success／code0及兩len0可最短推得；closed答案只marker。
Q4：「差別在 status 與 stderr，所以三項一起檢查」解釋空成功test的必要觀察，不靠外部工具或書後程式。
節末：TempDir資源／清理、fixture寫／查、Command設定／等待、env!目標、借bytes解碼、Output兩層結果及圖全部先教；無必要初判缺口。未實跑／建fixture／看整頁或預讀7.9。

原始SHA-256：19a6b1229a7ce2c772608c3abefa2de18d60ba92889f76292e6455b8f1f95f3e；visible SHA-256：65944ac7041471bc3817b9175710842ab6c4315dfdde5a96cdd97cb85c0371aa。

### 009-001 — 完整搜尋CLI的原需與分層

Q1：完整工具接原生檔名／parser／文字讀取／JSON／child tests，owned結果保存符合整行及原行號。
Q2a：仍限定完整小文字檔，先定材料答案、純文字工作，再接IO／process，非新串流架構。Q2b：「工具交出符合的整行及原行號，也能改用 JSON 報告」→同資料結果加兩表示→caller可選人工文字或結構化資料。
Q3：cd examples/ch07-cli/search，後續Cargo由此；純搜尋新method待當节教，不以原clap flag誤當已實作搜尋。
Q4：「每一層都有自己的可觀察結果」限定接合驗收，尚不跳到後章。
未知／影響：匹配政策／行號method合理待下一段；當下無必要缺口。

原始SHA-256：ab74dbced0dd5a23780bc3602a5ced53755f2c29b10de58def5ba20b68698503；visible SHA-256：ab74dbced0dd5a23780bc3602a5ced53755f2c29b10de58def5ba20b68698503。

### 009-002 — 搜尋／ASCII忽略／行號／出口原始契約

Q1：匹配連續片段，一行符合多次仍一筆，原內容／順序／1起行號保留；JSON保存query／選擇／matches。ASCII涵英數基本符號，此工具只忽略A–Z／a–z，Ä與ä不同。
Q2a：lines沿6.7，空行占原號、末行尾不加行；文字每結果加LF不恢復原CRLF。空query先拒不IO；零符合正常，JSON空array；stdout失敗可有部分bytes。Q2b：原需可用文字／JSON結果→先固定材料與小範圍政策→純搜尋／序列化／process各可核，不暗示完整語言大小寫匹配。
Q3：Rust預設1／4；RUST忽略1／3／4且保原大小寫。正常／help0，檔／編码／輸出1，用法／空query2，資料stdout、診斷stderr。
Q4：四行材料與表格給規則推來的答案，非待測輸出反推；「尚未寫出資料」僅讀取／用法失败，與Write部分成功分開。
未知／影響：實作matching／ASCII轉換method待下一單位，契約足夠，無必要缺口。

原始SHA-256：300795160572851b7e2ddb75b9f2ba59b782db8f537a30feee07c07ae16cbf2c；visible SHA-256：300795160572851b7e2ddb75b9f2ba59b782db8f537a30feee07c07ae16cbf2c。

### 009-003 — Settings／Match／Report與分來源AppError

Q1：Settings保存原生filename與query／選擇，Match owned原行號／text，Report owned query／ignore_case／Vec<Match>；檔名不塞報告。AppError EmptyQuery／Read／Encode／Output保存不同階段。
Q2a：新增json long flag沿7.6；Serde usize整數受平台範圍、Vec<T>需T能力，Match derive再供Report元素；deny未知及全欄位相等沿7.7。僅JSON Error有From，IO需當層明選Read／Output；status shared借返u8 1／2不退出。Q2b：原需owned報告與不同失敗→分設定／模型／原因／狀態→同IO型別不被自動From混失敗來源。
Q3：四dependencies已教，EmptyQuery2、其他1；model解碼型別限制非任意usize，modules／tests位置沿1.9、7.2。
Q4：「報告已完成但出口失敗」與檔案樹指出各helper責任，下一段run仍待新搜尋前提教完。
未知／影響：通用Serialize元素能力本段已說，無新必要缺口；未開完整source或據檔案樹提前讀helper。

原始SHA-256：ac1e903eb48789077262db2480ab265a694da05e9563096a52d54fb7900ded87；visible SHA-256：ac1e903eb48789077262db2480ab265a694da05e9563096a52d54fb7900ded87。

### 009-004 — contains／ASCII比較表示／enumerate與純搜尋

Q1：contains借兩文字查連續片段非完整單字；to_ascii_lowercase借原str建新String只改ASCII；enumerate接iterator新Item=(usize,原Item)從0，lines仍借原文字。
Q2a：query比較表示建一次、candidate每行建，兩邊都轉才忽略；符合保存原line.to_owned，index+1數所有原行，結果不需HashMap排序。validate_query兩入口需共用，AppError?同型From沿5.6。Q2b：原需原行號／大小寫原文→判斷表示與保存內容分開→匹配正確而不把輸出改小寫；非最少配置／大串流承諾。
Q3：純search先驗query，不IO；五tests核預設1／4、忽略1／3／4、Ä≠ä、空行CRLF原號3、owned存活／原拼法、空queryErr。test名稱filter新命令在本段明說選此路徑。
Q4：表格在code前給三method輸入／返回；Trust me包含rust展示片段，兩邊轉表示與保存原文的對照有實際作用。
未知／影響：新方法／索引／配置界線皆已教；Rust literal的\\\"可按基本Python逃脫引號及已讀JSON概念作最短理解，不另猜API。無必要缺口、未實跑五tests。

原始SHA-256：a8bcebe9092c5cbb12cf3e4a25c123da26493bf84436a39024f5d18bf8072e27；visible SHA-256：a8bcebe9092c5cbb12cf3e4a25c123da26493bf84436a39024f5d18bf8072e27。

### 009-005 — String.push及render／read／write／run接合

Q1：render借Report，產文字或JSON owned String；read_report檢查／IO後回owned Report，write_report借W:Write，run接兩步，所有AppError沿既教。
Q2a：新String.push(char) mutable追加该char UTF-8回unit，'\\n'單LF異於&str；文字按slice原序format／push_str，JSON pretty?後加LF。read_report先validate，IO錯明Read；render Encode?，write_all錯明Output。Q2b：原需兩表示與各層可測→先完整模型、再render／出口→純搜尋不模擬檔案、排版不啟process，空query優先用法錯。
Q3：空matches文字空String，JSON仍完整報告；empty query＋missing回2，不先IO；各helper借輸入但結果owned，不依已結束contents。
Q4：兩種文字來源與三helpercode對應前述資料／錯誤分工，新增push在使用前給完整契約。
未知／追加：無必要新前提缺口；for slice shared元素由7.4已實讀例及此code可最短推得。write_report只述write_all完成，舊O1的標準出口／flush完成責任仍未說，不提高原optional；未跑／開兩render tests source。

原始SHA-256：32e4b76529a24b921060893875f03176bd69b0e8f590a71d0f0cde84e210c950；visible SHA-256：32e4b76529a24b921060893875f03176bd69b0e8f590a71d0f0cde84e210c950。

### 009-006 — main行程邊界與原位搜尋圖

Q1：main用已教parser／run／Stdout與report_error；clap Error顯示請求不進run，AppError診斷／status另處理，非同一錯誤型別。
Q2a：parse Err先print後回0／2或FAILURE；run Ok SUCCESS，Err借to_string與status報stderr；所有scope正常返回。Q2b：原需下一程式可解析純JSON→資料只由write_report到stdout，main只掌診斷／狀態→不混標題／進度。
Q3：三commands預設行1／4、ASCII忽略1／3／4、JSON全stdout報告；redirect及緊接status沿7.5，本文預期0／1／2非本輪實跑。
Q4：首次原位SVG及native340×650實看：實線args_os/Parser→owned Settings→read_report UTF-8/owned Report→render借Report建String→write_all借bytes到stdout；虛線由read／render／write匯AppError回main→stderr／ExitCode。parser help/version另底註stdout0，不接檔案鏈；正常0／作業1／用法2與code相符。SVG SHA-256 a5473c1a270b6fd203a78fd3e6c264de247c3eb192308f1ab163cb9d97230df7；PNG SHA-256 324bd3c2fc0dc28d51c8d8a24c6714fc7e0cc5088734bfbd7f015c117dd4fba3。
未知／影響：無新必要API或圖文缺口；舊O1未獲stdout完成位置新說明，維持optional，未看整頁／執行。

原始SHA-256：f75dc8c8f145eb9f4029486a603ac144b259a8accd747e3e0893d7449fa16116；visible SHA-256：f75dc8c8f145eb9f4029486a603ac144b259a8accd747e3e0893d7449fa16116。

### 009-007 — 控制writer及完整JSON child驗收

Q1：RefusingWriter只模擬輸出PermissionDenied，不是Read錯；execute／assert_failure沿7.8工具，接受僅真正arguments的OsString slice，與parser完整含程式名序列不同。
Q2a：Command啟child時已安排argv第一筆，不在arg再加名；nonempty report保write_all真呼叫，called核它；writer無中間buffer／flush Ok。完整stdout借UTF-8再decode Report核全角色。Q2b：原需出口來源不混、JSON不摻進度→控制writer與真child雙檢→不因空輸出或main短就認已測。
Q3：writer核Output variant／kind／called；JSON child核status0、stderr空、query Rust、ignore false、行2及原引號文字，再close。預期7 library＋1 writer＋5 CLI，未讀其餘完整source／執行。
Q4：「不包含程式名稱」在helper前明說；「解析整份 stdout」使進度前綴可被拒，依已教模型／API即可判斷。
未知／追加：無必要新前提缺口；無buffer控制writer不釐清標準出口O1，仍原optional。未宣稱全部13項已看source或實測。

原始SHA-256：6c45230f53fda192b936c8646dfd599230d32e1ac2c71042eae79e45c99f091c；visible SHA-256：6c45230f53fda192b936c8646dfd599230d32e1ac2c71042eae79e45c99f091c。

### 009-008 — 7.9節末按行計數與尾段結束

Q1：數Report.matches每筆Match，並非每片段出現次數；iter借元素、fold累計state沿6.5–6.6。
Q2a：先search?得Report，slice iter fold從0每Item加1即可；state可選usize而Item是&Match，6.6已教兩角色不需同型。Q2b：原需搜尋整行→重用固定模型資料再統計→Rust Rust那行仍只一筆，不改query／文本匹配政策。
Q3：Rust／Python／Rust Rust三原行僅兩Match，得到2；最短推論不用closed完整答案或外部程式。
Q4：「它數符合的行，不數片段總共出現幾次」與材料明定答案；後章smart pointer等只預告，不當已教前提。
節末：設定／模型／具體原因、三搜尋API、render新push、IO與W:Write接合、main／三層tests及原位圖都有實讀前提；無必要初判缺口。舊prefix O1保持optional，未讀v10 prefix所以不判其最新修後；本尾段28單位到此結束。

原始SHA-256：8dd2549080041e39dec17ecb35ba229ce7854dc0ba63d7e4f36bb8206bae076c；visible SHA-256：daf33631a97bfe6caea597ade5a32f7bb7b79c470d1ef582ac3592729568eb06。

## 尾段原始初判封存

依tail gate完成7.6–7.9全28單位，每單位保存四題／必要未知／hash才續讀。006-006／007-007／008-007／009-008四個closed答案只marker，沒有開完整答案；三個圖均實際遇原位後才讀凍結SVG與native局部渲染，逐箭頭／owner／表示關係記在該單位。沒有誤見v10 prefix正文、第8章、author／source／其他review／outline／外部解答。gate初次order修正已在header誠實記錄，未繞過先保存限制。

新增必要初判問題0，新增可選問題0。既有prefix O1不回填或改級：本人已讀7v9 prefix的Stdout／Stderr完成位置與flush界線屬optional；此tail的show_settings／write_report／main仍未補該關係，控制writer明說無中間buffer而非檢查標準出口。尾段可理解所有結果分支，故不升必要；未讀v10 prefix，不宣稱最新整章已解決或未解決。

已教關係核對範圍：clap欄位規則／生成／owned解析與正常顯示，JSON形狀／Serde能力／借文字與owned模型／具體來源錯誤，fixture資源owner／清理與child三結果，以及ASCII搜尋／原行號／表示／Error來源／各helper接合。各必要新API的receiver、結果、ownership與原需均可用此前實讀正文或當段短推論說明；沒有用本來懂Rust或peer4獨有答案補未知。

未驗範圍：完整source／archives／實體fixtures／未顯示tests、Cargo命令與真child、環境與跨平台狀態、完整desktop／mobile頁面及後章。正文測試數與輸出只核其可理解性，不當實際通過。這是獨立AI首次順序理解，不是真人測試或技術實測。

獨立tail範圍指紋保存work/ch07-tail-v10-raw-scope.sha256：凍結reading-order（只有名稱metadata）＋tail28原始units＋實讀3SVG，共32項；該檔SHA-256：455a94a2771c47726f9916a171ff9c58c836472ba0ea45f073c98084857021e1。原始unit hash機器涵被遮蔽bytes，但未向reader輸出答案。visible ledger SHA-256：8d599aa75a45a5cbc199c286cc6436853126273e3e90e8dd9667844298459b61。

全部尾段初判先封存至此，停止等待root另授權cross／followup；不修改prefix／entry／舊ch04紀錄，也不提前讀第8章。
