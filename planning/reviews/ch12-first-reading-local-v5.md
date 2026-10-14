# 第12章v5指定六單位修後局部複查

AI輔助讀者模擬的有提示LOCAL，不是新盲讀或獨立問題重現。沿本人actual FIRST12 whole44原稿與其他已讀actual1–11／14路線；舊original的字節、分級、未知、時序保持，不回填新增理解。

FIRST12 original SHA `acebae2a5843d0a64275b88b76e57487bdae506e0d8879b235018f14723e0a7c`；剛封LOCAL11 SHA `326832fb540116510151ea0ebfac9ec2178c852899865cb34f12a03ba6ffaa24`。

唯一ch12-review-fixed-v5六BODY：001-001→003-002→003-005→005-002→005-005→006-006。只CURRENT在工具輸出前closed-filter；原位必要圖到當段才讀SVG／孤立view，每必要新概念四問、Q2a機制／Q2b原需要分存、原句、rawSHA、未知／合理推論／回讀保存後才NEXT。不讀其他新版BODY、source／archive／manifest、作者、同行、ROOT處置、outline、live、外部答案或13／15。不以修訂提示作通過理由；答案維持CLOSED，不執行Rust／C／Miri／benchmark或驗實頁，未見完整source不聲稱驗收。沿既有已聲明通用規範可見範圍，不再查技術參考。保存前OpenCC s2t保護引用／API／ASCII，初判保存後只追加。

路線／reading-order SHA：

```json
[
  {
    "snapshot": "ch12-review-fixed-v5",
    "reading_order_sha256": "63eb060b72441eb2f3641fea8b33652bb638fa70cdd403d219b82d06e5c2fd1f",
    "selected_units": [
      "001-001.md",
      "003-002.md",
      "003-005.md",
      "005-002.md",
      "005-005.md",
      "006-006.md"
    ]
  }
]
```

## ch12-review-fixed-v5 / 001-001（順序1）

SHA256 `ba212f7dcddb83f396c251795638dfef9799ec55ba7b45088119cee88a9b7c58`。

Q1：這段再次界定GAT節的工作角色：沿6.2 Extract接手source／固定Output，改成保來源只借view；Vec與RefCell分別slice／活躍借用guard，同一trait操作仍保兩種結果與規則。GAT具體泛型associated type機制在本人原FIRST12 12.1已實讀，這個CURRENT本身只提出問題，不能假稱它已完整教syntax。Q2a：普通Vec可直接fn view(&self)->&[i32]完成借slice；本節要把不同view型別以及本次self借用有效期一同放trait契約。未在這段新生成guard／容器，也沒有把RefCell改成Vec。Q2b：原需要是在保來源的共同行為下，保留各自result類型／活躍借用規則與每次borrow範圍，並非僅因已有GAT產物才造新需要。

Q3：對只用Vec的caller，普通viewfn已夠；不同來源共用trait時，後續應有與每次借self相連的結果，不能先假共同結果必Index／Deref或都是slice。本人後面的原已讀型別族知識可用，當下沒有取新版後續BODY作例。Q4：原句「只替普通 Vec 寫 `fn view(&self) -> &[i32]` 已經可以完成第一種需要。」與「把不同視圖型別放進同一份 trait 契約」分清已有解法／新關係；「不需要建立通用容器或重新實作 RefCell。」限定本節範圍。

LOCAL：具體新問題在引入做法前已說，局部方法需要比單純“統一”更可判；沒有新增必要／可選負擔。原FIRST12的無問題原判與當時未知不回寫成曾見這句。無圖／fold、未執行／看source；首用新語法此段沒有增加，背景GAT機制不重做全章驗收。

## ch12-review-fixed-v5 / 003-002（順序2）

SHA256 `48ca80da5b0bd13bf9c77c80a3572c6980f1db20faa053fc56a387949af58a94`。

raw型別與存取：Q1 *constT／*mutT是指向T的位置資訊型別，expression *pointer才deref，與reference的借用／保活不同。Q2a *const不能由此type直接write，*mut可表達read/write但仍須權限，type mut不要求bindingmut；raw可null／dangling／invalidT，無lifetime替來源追蹤，scope不自動free。Q2b 原需要是在操作指定位置前分清位置與可用資料／權限，不以星號或mut字樣把所有讀寫當有效。Q3 pointer須連回來源allocation，array全元素同範圍／local與Box各自己的，合法access不只合理地址。Q4 原句「真正寫入仍必須有合法權限。」／「型別中的 mut 也不等於 pointer 變數本身的 binding 必須是 mutable」支援語法與存取責任區分。

新增明示raw Copy：Q1 *constT與*mutT的pointer值皆Copy、不要求T Copy；複製的是位置資訊。Q2a 不複製T、不延長source生命、不增ownership／freepermission；mutable raw可Copy也不放寬有效性／衝突存取，&mutreference不能據此當Copy。Q2b 這條性質服務既有“原來源與合法存取責任仍在”的區分，並未另說本節必須多複製pointer做新工作；不由可Copy反推被指T或owner要複製。Q3 即使留多份資訊，每次deref仍核來源range／生存／權限，一份info scope退出不意味free；本段僅契約沒有需要試deref無效指向。Q4 原句「不要求 T 有 `Copy`。」與「不能把它套成 mutable reference 也有 `Copy`。」直接限定所支持type及錯誤推廣。

LOCAL：新Copy條款足以分pointerinfo／T／owner責任，用已讀4.2Copy與actual12來源有效性即可接，沒有新最低語法要猜。原初判有raw不保活與來源責任，但沒有以這項新增明確條款另分級，本次只追加確知，不回寫舊。無圖／fold／實跑／完整source，無新增確認必要或optional洞。

## ch12-review-fixed-v5 / 003-005（順序3）

SHA256 `1e1583e15b3fb7f93edf07087fc6d32277ecd2a15efc377de297e55cd5dd37c9`。

array→slice coercion：Q1 語言提供array reference轉借同元素的slice reference，非8.3 Deref coercion；Q2a 預期&mut[i32]時&mutvalues從&mut[i32;3]轉slice，shared&values也可轉&[i32]，長來自array，不複製、不延生命，&mutvalues[..]仍可。Q2b 原需要是先取得三元素的borrowview再pointer改第二項，現new寫法須交代如何滿足slice預期而非默認所有pointer/容器轉換相同。Q3 caller仍擁有原array，slice保borrow規則；raw結束才能原index核。Q4 原句「這是語言提供的 array→slice coercion，與8.3的 Deref coercion 是不同的轉換規則」支持取得方式與區別，先於typedlet使用，足以接例。

i32 array能力：Q1 本例array有Debug／PartialEq兩種指定能力，非i32有就一概容器有；Q2a Debug列元素，與同長度i32 array逐位置皆相等才相等，因此assert_eq全array核80/140/40。Q2b 原需要是不只第二項變140也核其他兩項仍保留／整份結果；不能把原T能力當容器能力未教推論。Q3 不需只print總260推所有元素正確，assert核全array；Q4 原句「每個位置的值都相等，兩份 array 才相等。」在assert之前說明比較意義，不支持任意container皆同契約。

raw相關已教：Q1 as_mut_ptr回首*mut／sharedas_ptr回*const不接owner，add(n)以T元素位移；Q2a 本例add1在源allocation第二項、addlen onepast只is_null不deref，byte位移<=isizeMAX/無addressoverflow，全部init/aligned/sourcealive、raw時不從view或其他ref存取。Q2b 需只改第二項、保持原allocation與權限，不能把endpoint作額外資料。Q3 before120/after140/endnullfalse/原總260，onepast不配置第四項；正常值輸出並非一般unsafe有效性證明。Q4 原句「可以依 add 的契約形成，卻不能當成下一項 dereference」界定形成／access，沿原FIRST12 12.3已理解。

原位SVG SHA1e3fe4be89e831721566567d512691379bf1d99587b7fdb738f2f24ff8bdaa9c已source／Inkscape390isolatedview：三相連框index0藍80、1橘120→140、2藍40；start向上到第一起點，add1到第二起點（非中心值），add3到右端x410虛線且無第四框。三線是pointer位置非操作時間／owner移動，底可形成不能dereference；單圖字可辨。SVG內font-size24等style可見，未讀站點CSS／實頁，未稱手機頁面passed。無fold／執行／source實檔。

LOCAL：新增coercion／arraytraits兩關係在用之前有正文具體教，能沿已讀borrow/Copy/Debug/PartialEq接；沒有新增確認必要或可選負擔，不回填原acebae判斷為曾學本v5句。

## ch12-review-fixed-v5 / 005-002（順序4）

SHA256 `e95f42fb1d6eea77ad65858871ae743b53435228af090d25d7ac6750aab3e8f5`。

Q1：C宣告／標準型別／header／sizeof沿本人原FIRST12 12.5已實讀；目前重新連array長度從哪來，以及傳函式後能否再取得。C type在名字前、local預設mutable、初始化再讀，無Rustlet/mut。Q2a：int32_t／uint8_t由stdint.h在相符平臺提供，size_t由stddef.h表示sizes/count非固定unsignedint；char一Cbyte非Unicodechar、sizeofchar1，本hostbyte8bits非所有platform；include preprocessor納header內容不同use。Q2b：原需要是讀懂FFI C一側輸入／長度／表示，不能假所有Cint32或byte平臺等同，未初始化local也無可讀預設。

sizeof／array傳遞關係：Q1 sizeof是Csizequery返size_t，非實際讀value，在此wholearray時知道配置bytes／itembytes。Q2a values[3]明確array，sizeofvalues/sizeofvalues[0]兩個byte數相除得3項；一般fnparameter傳遞arrayexpression則變指首項pointer，不連arraylength一併傳，pointer端sizeof只是該pointer自己的size不能復原原項數。Q2b：此介面需處理原數列完整合法範圍，所以另外傳length；這需要來自sourcearray資訊失去的關係，非任意因看到ptr+len就猜長度來源。Q3：在array仍是array的此scope可算len3送接口，接收端不能拿sizeofptr除itemsize猜domain；型別／sizeof大小非boundscheck或有效pointer驗證。Q4 原句「這兩個數的單位都是 byte，商才是項數。」與「接收端若只有 pointer，sizeof 查到的就只是 pointer 本身的大小，無法取回原 array 的項數。」支持unit/資料流界線。

LOCAL：新的明示接收端sizeof限制可接前例兩行與後原已讀C數列契約，不需猜pointer帶length；尚未開新版後段／source。原原判已懂arrayparams第一pointer不帶length，此處只追加更具體sizeof判斷，不把新句回填當初或另作新盲讀。無圖／fold／C執行，新條款未加未教syntax／能力負擔，無確認必要或可選洞。

## ch12-review-fixed-v5 / 005-005（順序5）

SHA256 `b1ab3ffabc1108f3aecee4129dd47578a62bca618af37fa49ea31f89693352b5`。

NUL／buffer規則：Q1 Cstring爲有效連續char以數值零NUL終止，不同字元'0'／NULLpointer；普通buffer的零是資料。Q2a literal末尾零，charword[]=Rust局部array五char副本可改但literal儲存不可寫；strlen(string.h)掃有效NUL序列回第一零前size_tbytes，不含零／容量／Unicode字數；count_nonzero按顯式len步進，0元素不結束。Q2b 原需要是正確選擇文字終止約定或有限buffer範圍，不能把len3材料因爲中0丟掉20，不能用strlen替一般byte_len。Q3 word strlen4而sizeofarray5；bytes10/0/20中非零2，len0允許NULL只是此helper無訪問，非空仍validsamearray／aligned/init／no衝突write、不save/free來源。Q4 原句「沒有合法終止零的資料不能交給 strlen。」與「一般 buffer 根本沒有字串的終止約定」限定strlen前提／用途，沿原FIRST12已知。

圖／實際讀取關係新增明說：Q1 上箭頭是word被計入返回長度的R/u/s/t四項，下箭頭則buffer實際len3走訪三項，兩箭頭角色不同。Q2a strlen爲判停止還須讀第五個有效NUL，雖計數不含第五；buffer3項包括中0。Q2b 原需要是分清‘長度不計入’不等‘沒讀取’，把合法終止零與圖的範圍對應，而不是見四箭頭便以爲讀4後無須NUL。Q3 五個有效char與len4可同時成立；沒有多配下一字，NUL位置必須在readableallocation內。Q4 原句「判斷字串在哪裡結束，仍須讀到有效的終止零，長度不計入它。」在圖前直接教這關係，足以對箭頭作正確結論。

原位SVG SHA da375b3c849136eff7ae922089b1251b1273ef0c47e11a3970367dca02030627已source/Inkscape390孤立view：上4藍框R/u/s/t＋橘0，第上橫箭僅四藍長度區域，旁strlen4／讀到終止零長不計零／sizeof5bytes明確；下3綠框10/0/20，橫箭len3走訪全三、零仍資料非零2。兩條橫線非owner／時間或共同strlen scan範圍；文字把第五NUL讀取與上圖計長範圍補分開。SVGfont24/28等style已見，未看網站CSS／實頁，獨圖可辨不稱pagepassed。

字符傳輸覈對：工具JSON轉義顯示可能看似多slash，爲避免據顯示猜錯誤，僅檢查同CURRENT可見inline codepoints：NUL寫法[92,48]、char[39,92,48,39]，確爲一反斜線；不讀外部答案或Csource，也不報告並不存在的Cescape問題。無fold／運行。

LOCAL：原已讀區分字串zero與bufferdata；這裏對箭頭/實際讀有效NUL的條款比只見長4更具體，不回填原grade與圖初判。無確認新增必要／optional負擔，未作全Cstring安全證明。

## ch12-review-fixed-v5 / 006-006（順序6）

SHA256 `a78531f478ccb1c97b251ccb42e9e561efe0757d144d614013d4a42cc9499f7a`。

Greeting owner／Drop：Q1 C工廠新malloc五char的唯一cleanup責任由專用RustGreeting承接，非rawbits本身是owner或Box/CString任意接heap。Q2a C完全init含NUL後return新data非localword，NULL失敗；privatepointer僅由工廠成功值、無公開改寫/重設/free任意構造，Greetingmove無Copy/Clone；正常Drop交指定Cfree原始成功pointer一次。Q2b 原需要是Rust可用C所配內容並配對該allocator cleanup，保單owner不重free，不能從heap外觀推Rustallocator合法。Q3 try_new Option保NULL/成功，借&self時不能drop來源；正常cleanup後只ownedcopy可用。Q4 原句「正常清理時交回對應C函式一次。」及「看起來都指向heap」沒有證明allocator相符，界定正常Drop流程與來源契約，不把scope例擴成任何異常程序終止都保證執行cleanup。rawpointer Copy不等Greeting Copy/可free權限，接003-002本次已明教。

CStr from_ptr／mutable→const：Q1 unsafefromptr接*constc_char，交借view不接owner；Q2a *mut在預期*const處語言隱式只讀type轉，同資料不增資格/生命；nonnull、整段含NULinit/readable單alloc、總<isizeMAX，borrow時不得write/free，固定工廠＋private／無writeAPI＋&self回傳共同保證，非只nonnull。Q2b 原需要是從Cowner借文字還保sourcevalid／只能相同釋放契約，不能因爲viewread-only便當raw任意合法。Q3 as_c_str的返回life連self非static、不復制內容；無論pointer信息能Copy，view後續用仍阻結束owner。Q4 原句「from_ptr不接手配置，也不自己查明其真正owner。」限定API效果/調用責任，最小參數/前提/結果皆先教，沒查完整source。

UTF-8／普通副本：Q1 CStr格式valid不等UTF8；to_str借self返Result<&str,Utf8Error>，成功仍borrow，錯誤Debug足expect。Q2a 本固定ASCII才expect，任意Cbytes API需Err處理；to_owned沿7.4建獨立String，view最後用過才dropGreeting再printsaved。Q2b 原需要是顯示可用Rust文字並在Cfree後保內容，所以獨立ownercopy有用途，不能把CStrview冒稱會活過free。Q3 sourceclear後只savedRustString可用，output核內容關係非配對free/borrow安全證明；後者靠Cbody和Rust接口契約。Q4 原句「&CStr借的仍是第一步C配置；第三步另建的Rust String才是一份可在free後繼續用的內容副本」支持該關係，未跑example。

原位SVG SHAe493ca38f3061d00e643a0a216fa79e116288eb5e64f17169c985a184523d300 source/Inkscape390孤立view：頂綠C配R/u/s/t/0＋Greeting清理owner，下藍&CStr借原配置/borrow連&self，下藍另RustString/用完view，下橘dropGreetingCfree原5byte結束，底綠String仍可用。四下箭線只正常操作先後，不把view看成新C配置或把free原allocation移給String；雙藍框文字區分borrow/獨立copy。SVGfont24等style可見，未讀站CSS／實際頁，字在獨圖可辨但不稱page驗收。無fold/原source/執行。

LOCAL：此原有限factory/owner/borrow機制已沿FIRST12讀過，新正常Drop及圖/dataflow條款具體且不用外部補；當前最低權限、來源、成功/失敗、後續用途都可分別說清，無確認新增必要/optional缺口。保留原acebae與LOCAL11／14所有初判bytes/等級，不把本複查叫獨立再現或完整FFI soundness/source驗收。第6段保存後停止。

## 封存

唯一ch12-review-fixed-v5六個指定BODY已依序完成：001-001→003-002→003-005→005-002→005-005→006-006。6/6、CURRENT=null；每CURRENT先closed-filter、必要新關係各四問與Q2a機制／Q2b原需要分判、引文／rawSHA／未知先存後NEXT。沒有取本v5其他BODY、source／archive／manifest、作者、peer12、ROOT處置、outline、live、外部答案或13／15；六段本身沒有fold，未另開C／FFI任何答案。這是有提示LOCAL，非新盲讀／獨立問題重現，原FIRST12 acebae…及剛封LOCAL11 326832fb…保持。

當下可判斷：001-001先給普通Vec view已能做的事，再明定同trait不同view型別、連每次self借用範圍的新增需要，不從GAT產物反推用途。003-002 rawpointer的Copy只複製資訊、無T Copy要求，也不增來源生命／owner或free權限，不能據此把&mutreference當Copy。003-005在arrayreference直接交slice前說語言coercion與Derefcoercion不同，預期型別／共享mutable兩形式／不複製不延生命明確；又先說具體i32 array Debug/PartialEq比較，再assert全array，不憑元素能力推任意容器。005-002 sizeof在源array知道整個bytes/itembytes，接收端array→首pointer後sizeof只pointer自身size，不復原原項數，因此另傳len。005-005 strlen計長不含NUL，但為判停止仍讀有效終止零；圖上四項是計入長度，下buffer三項是len走訪，不把箭頭當同一讀取範圍。006-006正常Drop一次配對Cfree，borrow&CStr仍原C配置，只有後來String獨立copy可free後使用；正常流程／ownership與值輸出關係不同證據，不由結果正確代完整FFI契約證明。

原位三圖均到CURRENT才讀SVG／SHA並Inkscape390孤立view，具體框／線／名稱／關係和界線當段保存；只見SVG自身style，未讀網站CSS／實頁／完整Chromium截圖，不稱頁面passed。圖不當時間/owner箭頭混猜；raw-array線定位、Cstring計長線與終止零、FFI正常順序各分別核。工具JSON中NUL字樣轉義以同CURRENT inlinecodepoints核得單反斜線，不錯判成C語法問題。沒有Rust／C／Miri／benchmark或工具實跑，也沒載入完整source。

分級與未驗：沒有確認新增必要或可選缺口。新語法／能力／用途和已讀actual前文可連，不需技術答案補；current局部可懂與原方法需要已各自核對。所有unsafe來源有效性／整體soundness、任意C平臺型別／allocator、任意client或普通/異常終止流程仍非本LOCAL技術驗收。未教全集不當本例必要洞，未跑也不拿ROOT採用或正文PASS作自身證據。沒有必要回讀其他oldBODY，這輪只沿本人原已讀背景，不改學生檔或舊判。

封存前只按bytes核對舊FIRST11 prefix/middle/tail、FIRST12 original、FIRST14入口/tail/LOCAL及LOCAL11皆同已封SHA，完整SHA保存在本sidecar；源單位SHA coverage只核本已讀六段。此處封LOCAL/newstate／coverage／sidecar並STOP，13／15 FIRST gate仍未開。
