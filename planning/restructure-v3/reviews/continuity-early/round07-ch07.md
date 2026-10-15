# 第7章7.1–7.4：獨立銜接與實頁初判

封存時間：2026-10-15T08:44:33.338998+00:00。只以actual1–6作前綴，按7.1→7.4讀正文、全部實際snippet/source、完整設定、答案、tests與fixtures。未讀authorvalidation/ledger、reader/technical結論、known/歷史audit或rootdispositions；封存前也未讀第8章以後。此角色不是盲首讀者、作者或technical，也不是全書通過聲明。

**結論：無新必要或可選finding。** 44份frozen保存（43頁內referenced/global來源另加stage defaultmain metadata），末次hash比對記於sourcehashes；選定正文、Rust、圖、package設定與fixture未變，全域workspace/lock漂移另存end-config，不當後章先備。

## 四題分項正向證據

每個項目分開記身分、機制、獨立需要、操作與例子支持範圍；沒有用「能算出結果」代替用途。以下原位置與JSON的完整四題紀錄相同。

### 單一來源回傳與lifetime elision

首次需要：7.1:7–15；完整same呼叫者26；已教前提：3.5有效借用區間、3.8&str、1.6函式回傳型別。

身分與角色：same交回收到的文字reference，elision是省略標註的推導規則。

機制／性質：只有一個輸入lifetime時，省略輸出連到它；沒有消除lifetime或另建內容。

需要／用途：7.1:3先提出自己回傳借用時，呼叫者要知道哪些來源繼續有效；單來源是兩來源選取前可理解的基礎。

操作與後續：same(source.as_str())保存&str，來源main中有效，最後顯示仍可使用。小prototype前7已說原樣交回借用，精確推導規則15在完整呼叫者前。

例子與界線：single_source顯示Rust、長度4，僅支持借回同來源；不能推出來源結束後仍可用。

依據：7.1:7「原樣交回借用」；7.1:13「回傳值必須連到輸入的借用來源」；7.1:15「只有一個輸入lifetime」。

### 具名lifetime parameter與兩候選來源契約

首次需要：7.1:57–67；caller74；已教前提：5.1泛型宣告、5.3byte len、3.5原資料有效要求。

身分與角色：單引號參數描述有效範圍關係，&'a str連起兩候選與輸出；不指資料型別。

機制／性質：輸出使用區間須兩候選都支持；不要求String同時建立／結束，不延長scope。

需要／用途：原需求7.1:3是從兩段選一段且不另建String，caller需要兩候選來源要求，不是用選到內容猜可保留多久。

操作與後續：first/second依byte長度選、等長第一；a宣告與位置57/59先說，65/67在跨scope呼叫前完整交代來源要求。

例子與界線：交換13/2長度使兩個arm都運作；ab/CD等長異內容能看見第一規則；too_short即使outside較長仍E0597，不能以一次執行內容縮窄簽名。

依據：7.1:39「不是比較lifetime」；7.1:65「同時由兩個候選來源支持」；7.1:109「只檢查長度2無法分辨」。

### 只連第一來源的不同規格

首次需要：7.2:7–15；save-first20；已教前提：7.1兩來源契約、4章_ pattern、block尾端expression。

身分與角色：first_text承諾總回傳first，第二輸入不與輸出a相連。

機制／性質：輸出不再依賴第二來源；兩reference呼叫當下仍有效。

需要／用途：7.2:3/7明示新需要只是借第一段，並非讓任選兩段的longer逃過原要求。

操作與後續：a標first與輸出，_:&str忽略第二；inside結束後saved仍借outer outside。

例子與界線：同樣Rust tutorial可在不同位置用，原因是規格與簽名；23/48公平指出不是印出內容決定，也沒延長inside。

依據：7.2:7「一定回傳first」；7.2:15「兩個參數在呼叫當下仍然都必須…有效」；7.2:23「已經是另一個規格」。

### owned文字結果與來源獨立

首次需要：7.2:50–78；已教前提：3.8String::new/push_str、3.2move/清理、6.3Clone與reference差別。

身分與角色：copy_text是自訂函式，借來源建立新String，再move出函式。

機制／性質：新文字有自己的資源；a不能救local reference，String可在來源結束後使用，也可與原文字分開修改。

需要／用途：7.2:3/52原來源即將結束卻要留內容；70明說代價是另一份內容儲存，非只是複製視圖。

操作與後續：&str輸入→String輸出，內層來源可結束，原String追加也不改copied。

例子與界線：local_output E0515對照新owned可帶出；original加tutorial而copied仍Rust使獨立修改可見。不能從相同顯示反推相同ownership。

依據：7.2:58「a只描述關係」；7.2:70「需要儲存另一份內容」；7.2:94「與來源分開保存、修改」。

### placeholder lifetime

首次需要：7.2:98–106；已教前提：7.1唯一輸入elision。

身分與角色：'_標示此處lifetime交省略規則推導，不是新的具名共用參數。

機制／性質：兩個'_的字面相同不指定同源，唯一輸入規則才建立關係；多輸入仍不能自選。

需要／用途：本小節教明寫位置的性質，沒有承諾新演算法優勢；108明說稍後struct型別位置要用，目前same簡例驗證不改來源。

操作與後續：same_placeholder(&'_str)->&'_str，由輸入取得Rust借用；來源main仍有效。

例子與界線：輸出Rust／長度4只支持既有單來源規則；不能把'_當延長scope或兩輸入輸出關係的替代標註。

依據：7.2:98「依省略規則推導」；7.2:106「不是靠字面相同指定共同來源」；7.2:108「struct的型別位置也會用到」。

### outlives bound與fallback

首次需要：7.2:122–132；已教前提：5.2OptionSome/None、7.1a關係、泛型多參數。

身分與角色：'b: 'a表示b至少涵蓋a，可等長；借用關係不是owner整個scope。

機制／性質：backup標b要能支持輸出a，None才能交回它；不讓資料多活，亦承認可用共同參數寫類似契約。

需要／用途：122先提出可能缺優先文字、缺時要借backup；124給輸出a的需求，b涵蓋a恰能提供該備援，不假造比共同a更有能力。

操作與後續：Option<&astr>＋&bstr→&astr；Some取text、None取backup，呼叫結果內層用完。

例子與界線：presentRust/absentbackup兩路與Some空文字不改選backup可見；缺bound完整fixture在泛型定義不保證b支持a，穩定診斷已提供。

依據：7.2:124「None分支…保證b至少涵蓋a」；7.2:126「不要求嚴格更長」；7.2:132「仍可用一個共同參數…」。

### 借用欄位模型、make_view與owned數字

首次需要：7.3:3/7/15/17/25/35–43；已教前提：3.1struct/pub、5.1泛型impl、7.2placeholder、5.3str.len。

身分與角色：TextView持有reference值，外部source持文字；a寫型別／欄位，impl先宣告再用。

機制／性質：make_view唯一輸入連TextView<'_>欄位，沒有複製內容；byte_len借模型與文字交ownedusize，可獨立保留。

需要／用途：3/76要把已有文字視圖及相關查詢收在一個模型，不重建String；96練習只要數字可在來源結束前算完帶走。

操作與後續：直接source.as_str()或make_view建模，byte_len沿self.text.len；使用欄位時外部來源有效。

例子與界線：實際圖上view.text→下sourceString借用箭頭；成功兩建法同源且owner可讀，E0597否定模型延長來源，答案只帶usize4出內層。

依據：7.3:3「已有文字的視圖，沒有需要重建String」；7.3:15「沒有取得…ownership」；7.3:43「owned的usize」；7.3:76「文字視圖與相關查詢放在一起」。

### receiver elision與欄位原來源輸出

首次需要：7.4:3/7–27；caller34；已教前提：3.6&self、7.1elision、7.3借用欄位、sharedreferenceCopy。

身分與角色：三對象source／view／&self分開；text省略輸出連receiver，original明寫a連欄位原來源。

機制／性質：相同self.text body可Copy共享reference，但簽名對caller給不同要求；output連receiver時不只借到call結束。

需要／用途：3/82原文欲外部String仍在、模型內層結束後還借文字；只有需此用途時明寫原來源，普通text適合借模型時用。

操作與後續：impl<a>中text(&self)->&str與original(&self)->&astr，先立刻用text，再用original取saved出block。

例子與界線：成功saved活過model、失敗改text被指出viewE0597，持續原文字存在不夠；表三對象＋已教圖與程式支持借用層次，無需fmt等內部。

依據：7.4:15「省略的輸出…連到receiver」；7.4:17「不是把&self固定成a」；7.4:25「不一定只到函式呼叫結束」；7.4:82兩種用途分界。

### TextPair兩欄獨立來源

首次需要：7.4:86–94；two_models146；已教前提：7.3structlife、7.4original、泛型多參數、7.2outlives。

身分與角色：TextPair<a,b>各欄各自借來源，methods分別交a或b。

機制／性質：first_source輸出只依賴first，沒有把b或model保活，也不需要b:a因不讓second供first區間。

需要／用途：86希望first外層保留、second內層用完後只留下first；不是任意多加名字。

操作與後續：pair在兩來源都有效時查總bytes與second，最後first_source交外層saved。

例子與界線：13+4=17、secondnote可讀，innerpair與second結束後saved仍Rusttutorial；測試同樣檢查跨model/secondscope。不能推出整個pair也可帶出。

依據：7.4:86「最後希望只保留第一份借用」；7.4:94「輸出只依賴第一來源」；7.4:159「不是把…pair或second保留下來」。

## 來源閉包與實頁

早期intro-lifetimes實際無lib、build.rs、dependency或localCargo設定；各selectedexample只載自己的stagehelper，沒有偷偷載完整後期模型。7.4才改用intro-lifetimes-complete，manifest、lib、source/views兩module、main、兩個example全在同頁內嵌，而且此時全部來源函式與模型都已教完。兩份minimalmanifest都只有package/name/version/edition。stage defaultmain為簡單println，已另讀，未當教材新能力。六份compilefail.rs及.expected全部同頁完整，穩定字串E0106/E0597/E0515/lifetime may not live long enough對應正文先解釋的失敗。stage四test及完整library六test全實讀，沒有執行Cargo/build/Rusttests。

每個fullinclude在actualDOM與來源吻合（僅正規化空白與snippet marker）；局部region同頁都有完整原檔。沒有選讀fold，四頁details皆0，必要新意義都在主線。7.4長頁特別看完整source.rs與views.rs測試尾及相鄰main，沒有只確認頁頂或圖檔存在。文字內容assertion承接6.2；7.4補用不同內容等長ab/CD，能觀察選第一段，沒有把只比長度說成來源選擇證明。

四頁均實開1280×800與390×844；20張截圖全已實看，確切清單在JSON。唯一image是7.3 borrowed-field.svg，兩viewport皆340×360，servedSHA與frozen一致：7a5685938b83980930f20859792f785f453a1cd865709800cc02f124c8479a37。上框view.TextView與view.text:&astr、下框source.String持Rusttutorial，藍箭頭及旁字明說借用／沒有另建內容；相鄰正文指認外部owner與欄位。19/21px主字及16px來源有效句都可讀。7.1scope程式與來源表、7.4三對象表及兩種method契約，加已教3.5及7.3圖，足夠建立當下關係，未見缺必要圖。

無raw**；主要table手機可完整讀。長signature與testname有局部code水平捲動，所有程式仍在頁。兩張原mobile test尾截圖的locator會自動水平pan至長函式名，另拍明確resetleft版本並實看，沒有誤報成預設頁面裁切。actualURL／時點／HTMLhash／viewport／fullinline結果／SVGhash及尺寸見round07-ch07-visual/metadata.json。

本輪僅銜接、來源覆蓋及必要圖文實頁檢查，不代替technical執行，不聲稱所有長頁每個pixel或最終strictsite驗收。原初判封存後按授權接第8章，再封存才接第9章；未先用10/11作知識前提。
