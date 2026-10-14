# 第8章首次順序閱讀 v1

角色：AI模擬第8章首次順序讀者，目標基本Python。本人沒有讀過第8章正文、source或作者／peer摘要；不把先前他人寫作依賴的source precheck當本人的首讀證據。

先備限本人實讀1–4正文與指定局部修後、5v7全35、6v6全50加6v8兩段及6v10的006-002／006-004、7v9 prefix34與7v10 tail28再局部7v11的005-006。前文最新版本並非每章全讀；相應原record／hash維持。進入本章前已先封存ch07-first-reading-followup.md（SHA-256 31cf5b50b8d8e5d2e00bedb9dc6d5e9791374e88d313a1ba13f18322550c5722）與ch08-entry-late-fixes.md（SHA-256 b4039a61b9286771c73939a4df4f18bb319d401d595e8e706b30e8a070770f18）。第4章peer獨有API／技術答案不作已教前提。

唯一當輪正文為frozen/ch08-complete-prerequisite-v8全66單位。只從reading-order逐unit開BODY，不讀作者概念清單／outline／source／archive／技術答案／peer／review／外部／整頁或未讀正文。read_visible_unit.py ch08順序gate先遮蔽closed答案只marker；save_visible_record.py ch08 append原四題／引文／必要未知／hash後才放下一unit。

每必要新概念首次及節末分存Q1關係、Q2a機制與Q2b已存需要→改變→用途、Q3操作、Q4例／圖證據；沿前文概念短定位，合理推論保存最短前提，不以本來懂Rust補答案。必要／可選／合理待教／未驗分開，後文釐清只追加不改當下判。

SVG只有实际原位BODY指到才開／native渲染，逐關係核對，不把局部圖當完整頁面；不實跑／取完整source或外部資料，正文測試與輸出只核理解。全66初判完成封存後停止，等待root另授權，未提前讀第9章。

## 原始逐單位紀錄

### 001-001 — 有限連結資料與型別空間原需

Q1：連結存數字及下一連結，最後結束標記；資料長度有限但型別要容不同長度，與已讀Vec表示不同分工。
Q2a：當前未教Sized／Box機制，先給「型別裡又包含同一個型別」的空間問題。Q2b：原需連結形狀可表示不同長度→須使編譯器知道每值空間→用有限數字鏈觀察，不添加樹／算式規則。
Q3：可沿已讀enum／struct表形狀，具體定義與解法待下一段，不先猜Box。
Q4：「這份資料也可以放進 Vec」限定例子為學空間關係而非必須換容器。
未知／影響：新概念合理待當節教，目前無必要缺口。

原始SHA-256：246a8de6775a4f04d2d2faf189f58c6a313ee78803d00df5351ade9b354e2326；visible SHA-256：246a8de6775a4f04d2d2faf189f58c6a313ee78803d00df5351ade9b354e2326。

### 001-002 — Sized／DST與兩種大小查詢

Q1：Sized表示型別大小編譯時可定，compiler判非自寫impl／method；DST如str／[i32]大小隨內容，借用它們的reference自身仍固定，沿2.6／3.4位置＋長度。
Q2a：i32=4bytes、[i32;3]=12與內容無關；size_of::<T>()需Sized不接值，size_of_val(&T)可DST量被借資料而非ref自身，usize bytes。Q2b：原需知每型別所占表示→分type大小與data大小→能核固定借用資訊容不同長度來源，為內含同型別問題備空間概念。
Q3：std::mem安全查大小、turbofish沿3.3；當前不讀raw address，也不用自己設定Sized大小。
Q4：「固定的是型別大小，不是資料內容」與array／slice對照、兩API表給足必要關係。
未知／影響：一般局部／按值需已知表示已說；實際reference bytes平台值待例子／未驗，無必要缺口。

原始SHA-256：c1af48a3ccd69efb367ac79f585a5b79ba7f48de4c04f2e33b4f85abc3878f08；visible SHA-256：c1af48a3ccd69efb367ac79f585a5b79ba7f48de4c04f2e33b4f85abc3878f08。

### 001-003 — 泛型預設Sized與?Sized放寬

Q1：<T>通常預設Sized，T:?Sized移除預設可Sized／DST，非強制DST／略型別檢查；&T自身固定仍可借DST。
Q2a：word:&str傳&T時T=str，只有&不改bound；borrowed_bytes用size_of_val target資料，fixed_bytes型別查需Sized。Q2b：原需借不同長度資料→放寬T但以固定reference傳遞→取資料大小而不取得owner。
Q3：part兩i32資料8bytes、word Rust! 5；本64-bit ref各16但不跨平台保證。無?Sized fixture E0277，改bound才合法；package／Cargo沿既教。
Q4：「T 是 str，不是 &str」直接解釋第一必要推導，數值例明分資料／reference；不自行用Rust知識補bound。
未知／推論：範例直接用Sized，本名可在此scope使用是最短讀法，沿此前常用trait預匯規則；其std module定位未提供且目前操作不需另猜。無必要缺口，未實跑。

原始SHA-256：69ec9e4062657c887bfb4de6a422f695c8a64f2b246f51df79c573344682f09e；visible SHA-256：69ec9e4062657c887bfb4de6a422f695c8a64f2b246f51df79c573344682f09e。

### 001-004 — const泛型保array長度

Q1：T是型別參數，const N:usize是編譯時值參數；[T;N]保完整array形狀，&[T;N]借它而非改slice。
Q2a：N由array type推導或turbofish第二值明選，body N回usize不用讀內容；參數_沿3.1忽略pattern仍檢型別。Q2b：原需同一fn接不同型別／固定長度且保簽名形狀→讓長度成泛型值→每個具體array仍Sized，不靠runtime數元素。
Q3：counts T=i32/N3、switches bool/N2、明選i32/3同得3，原array可讀；選3傳長2 E0308非runtime調整。
Q4：表格區分N四位置與「執行時取得、顯示」避免把編譯選值誤作fn必編譯時執行。
未知／影響：type／value兩argument已有足夠依據，無必要缺口；不以未教const fn或其他泛型規則補答，未實跑。

原始SHA-256：726aaa3e13ac055bfd30434e270b0305c0fd4b604d5089779af4786986d6bbd1；visible SHA-256：726aaa3e13ac055bfd30434e270b0305c0fd4b604d5089779af4786986d6bbd1。

### 001-005 — Box owned間接表示、借用與move出T

Q1：Box<T>標準型別owns另一處T／清理，非零值new取得value並安排heap；Sized T時Box自身固定單pointer，Box自己可在struct／heap，非必stack。
Q2a：*讀i32 Copy；&*／&mut*借T守原borrow規則；Box非Copy，即使T i32仍move整Box。持有Box可*move非Copy T出，原Box失效，非shared ref／所有pointer通則。Q2b：原需固定連結表示→把pointer與T另存→不把完整T嵌入／轉交時不用再複製T；String／Vec本已管理heap不用另套Box。
Q3：先shared120最後用，再mutable+20，最後Box交second140；Amount無自動Copy，taken:*boxed取得owned80；再讀first E0382。
Q4：四操作表在code前，明說「不把這項Box契約推成所有pointer」，使用先不需未知Deref trait。
未知／影響：來源／owner／表示與各operation已教，Deref機制合理待8.3；清理細節待後節，無當下必要缺口，未實跑。

原始SHA-256：99123c9948044d4314244f1d66445e88ee04336e80553f4d70aba00b2eaa1d23；visible SHA-256：99123c9948044d4314244f1d66445e88ee04336e80553f4d70aba00b2eaa1d23。

### 001-006 — 遞迴型別尺寸循環與Box鏈原位圖

Q1：遞迴型別由欄位回自身，直接Chain next使完整表示無法收束，有限實際值仍須同型別固定表示；Box<Chain>owned handle中斷直接內嵌。
Q2a：Link只存固定Box表示，下一Chain另配置；into_total按值match得value／Box，*next取下一owned Chain賦回，再loop至End，非遞迴呼叫。Q2b：原需同型別表示不同有限長度→間接存放→大小可定且原加總規則不改；不宣稱快於Vec。
Q3：80→120→End得200，End空鏈0；source被消耗，想保原鏈需另借用介面，短鏈小整數限定明說。
Q4：首次原位SVG/native390×570：上chain Link80的next Box→中heap Link120的next Box→下heap End；兩實線是owned管理非額外shared borrow，圖不按layout比例／不標enum padding bytes。SVG SHA-256 dc4a7ca6b5116da4b4479d99c277404b45d5fa52078afd5cca036a4b426a3679；PNG SHA-256 d3e971757a88a4d39ee5600e010b16e7c305108a57f58937e1819842eac4e1f0。
未知／影響：E0072及解法有前提，無必要缺口；未實跑／看整頁／開描述的三tests source。

原始SHA-256：4212d83116acaca15f796b3a4a37ffd3d728e9c3e5914665b2cc6959a4dc351b；visible SHA-256：4212d83116acaca15f796b3a4a37ffd3d728e9c3e5914665b2cc6959a4dc351b。

### 001-007 — 8.1節末三鏈與固定表示

Q1：同Chain型別容有限三項，Box handle與各next owned內容分開；Sized型別／DST內容／固定reference／const N角色不混。
Q2a：從End外包每Box一份Chain，into_total消耗逐項處理；只增加鏈長，不改函式／型別。Q2b：原需自含型別大小可定→固定間接表示→仍可不同長度且加總規則相同。
Q3：40→80→120→End得240；三Box分別管理80段／120段／End，最短由兩項例延伸，不開參考程式。
Q4：「仍是同一個型別」與從內往外組的指示足讀ownership，不需layout／遞迴函式答案。
節末：必要新概念／原位圖已有完整前提，無必要初判問題；Drop只預告待下一節，未實跑或開外連。

原始SHA-256：6d5e08a261fcb2f347b301f7d3f80520b14778f7b162395c714bf590d26dc151；visible SHA-256：6d5e08a261fcb2f347b301f7d3f80520b14778f7b162395c714bf590d26dc151。

### 002-001 — 自訂收尾的原需

Q1：scope結束值與依型別清理沿2.2／8.1 Box；現在自訂資料要在清理前做工作，不是借用最後使用規則。
Q2a：此段只提出收尾顯id，Drop方法契約尚待教。Q2b：「看清楚這項工作何時發生」→數字id追蹤→能觀察收尾順序而不先管理檔／網路。
Q3：後續資料各id作觀察標記，不能把println示例當完整釋放記憶體實作。
Q4：「不是模擬真正釋放記憶體的完整實作」限定本節追蹤證據。
未知／影響：自訂Drop機制合理待下一單位，無當下必要缺口。

原始SHA-256：b3ae59fcdf1032e8895167d0d11b986e05b778dfbb5c0a87a1ffcf758dd76605；visible SHA-256：b3ae59fcdf1032e8895167d0d11b986e05b778dfbb5c0a87a1ffcf758dd76605。

### 002-002 — Drop／destructor與scope反序、borrow區別

Q1：destructor是值結束清理；Rust自動清owned欄位，Drop為額外自訂工作，fn drop(&mut self)->unit借即將清理值，非新owner。Drop在標準名稱範圍，可直接impl不derive／外依賴。
Q2a：觸發由Rust管理非隨意method call；正常block局部宣告反序，內3再2、外first到main末1。reference末次使用只結束borrow，不即清owner。Q2b：原需看自訂收尾何時發生→id trace＋scope／borrow對照→不把借用範圍當owned收尾點。
Q3：view讀first後仍印main準備結束，才清1；Drop方法讀self.id，已有struct／trait／&mut與println前提。
Q4：「view最後讀取後…first才隨main結束清理」及輸出次序給當下直接證據，限定正常離scope。
未知／影響：compiler觸發契約已明說，無必要缺口；未把示例輸出當實跑或一般手動drop呼叫。

原始SHA-256：51f767f6d226045acb3ff01c8bd577088712719e897488f64ad893e8f3d22fe1；visible SHA-256：51f767f6d226045acb3ff01c8bd577088712719e897488f64ad893e8f3d22fe1。

### 002-003 — std::mem::drop按值提早清理與禁手動method

Q1：std::mem::drop普通fn接值回unit、返回前正常清理；Drop::drop只借receiver由Rust觸發，兩者非同呼叫。
Q2a：first move到second不是清理，drop(second)才交出非Copy值，main不再清同一份；i32／shared ref傳Copy副本不清target。value.drop() E0040防手動清後仍owned／再自動清；borrow還未用完drop(value) E0505。Q2b：原需工作完先收尾→移交owned值給fn→清完繼續且保責任單一，不靠drop(&value)或繞過borrow。
Q3：id1只清一次後續工作；將view最後讀放drop前才合法，原borrow活到使用要求未改。
Q4：三對照含實際預期trace、直接method與未完borrow fixtures，明說編譯拒絕非清後panic。
未知／影響：新fn／Drop差別、Copy及borrow邊界都足够，無必要缺口；未實跑／以warning或probe補答案。

原始SHA-256：87cc610c2c3040c885a9eda635397a0cceadf28b2c380d74f54e7272da4ac20c；visible SHA-256：87cc610c2c3040c885a9eda635397a0cceadf28b2c380d74f54e7272da4ac20c。

### 002-004 — Pair自訂Drop後欄位依定義清理

Q1：Pair owns兩Tracked欄位，first／second此處是field非先前變數；Pair自訂Drop與owned欄位cleanup不同層。
Q2a：先Pair hook、後欄位定義序first10→second20；局部反序规则不套field。String／Vec／Box欄位本自管，無需重複手動清。Q2b：原需額外收尾且責任清楚→自訂工作先完成，再由Rust清欄位→不把Drop當需自行拆光resources的完整實作。
Q3：Pair hook仍能讀10／20，scope後依預期兩field清理；現有struct／trait／field借讀足用。
Q4：「和前面局部變數的反序不同」與四行輸出直接支撐新次序；明限正常scope／stdmemdrop非所有退出。
未知／影響：無必要未知或缺口；未實跑，也不推未教Vec元素或所有容器清理次序。

原始SHA-256：72cb7bb3ca3d87ce51cec0cc145bd362f9ccc5bcd23bbbd29d01a0d384cbd5ac；visible SHA-256：72cb7bb3ca3d87ce51cec0cc145bd362f9ccc5bcd23bbbd29d01a0d384cbd5ac。

### 002-005 — Copy／Drop互斥與非Copy欄位不能先move

Q1：同型別不能Copy與Drop，即使字段全i32；有Drop整值仍可move，但不能直接取走非Copy欄位留不完整值。
Q2a：derive Clone/Copy＋Drop E0184，owned Labelled取value.label E0509；讀id是Copy不移原field。Q2b：原需自訂收尾能用完整值→排除隱含Copy及先拿走ownedfield→保持cleanup責任與hook可讀資料。
Q3：需要文字借label.as_str，需交資料move整Labelled；6.2 Holder可extract因沒有Drop，不把其做法盲套這裡。
Q4：「main即使已擁有整個Labelled」＋具體String／id對照正面給限制，而非只錯誤碼。
未知／推論：8.1 Box取另一處T的特定契約與此直接field移出不同，由Box表示／.label操作可區分，不擅自取消前文；無必要缺口、未實跑fixtures。

原始SHA-256：bb7e95ab20f0947d3e5ec696d87e680b824e1c149dce8980b0fae025d491a869；visible SHA-256：bb7e95ab20f0947d3e5ec696d87e680b824e1c149dce8980b0fae025d491a869。

### 002-006 — 8.2節末兩值不同收尾點

Q1：兩Tracked各owner，顯式drop first與second自然scope清理分開，並非drop借用就清target。
Q2a：first按值交出立即清，second仍owned可讀到main末；局部反序只對尚持有值，field序另規則。Q2b：原需提早一份收尾而繼續其他工作→owned移交fn→不錯drop兩份／reference或直接method。
Q3：預期id1清、讀id2、main準備結束、id2清；最短用本節已教操作即可，不開參考程式。
Q4：練習明給順序與兩個錯誤替代，支持責任及時點理解。
節末：Drop触發／自動field／早drop／Copy互斥與保持完整值皆足夠，無必要初判問題；Deref尚預告，未提前用／實跑。

原始SHA-256：a47c213bfabe12eac64e99cf5d80aee486a9ac8a0c246fbf5323ba5a00276ed6；visible SHA-256：a47c213bfabe12eac64e99cf5d80aee486a9ac8a0c246fbf5323ba5a00276ed6。

### 003-001 — 外層模型借內String的原需

Q1：自訂struct owns String，既有fn只需&str；&*Box已讀，直接field.as_str亦足做，現在需外層借用介面。
Q2a：機制尚待教，當前只定借不交owner／不複製。Q2b：「不必…重複寫出欄位路徑」→外層說可借到哪種內容→既有fn／method用同份資料。
Q3：已有String／str與borrow前提，尚不用未知Deref去猜API或轉換owned型別。
Q4：與直接field.as_str對照給原需，無宣稱原方法不能工作。
未知／影響：Deref介面合理待下一單位，無必要缺口。

原始SHA-256：c69e49cf414a7a257f30584c7510f98ad25bb0dfc3bc3fdd4b317cf1bbdb2cc3；visible SHA-256：c69e49cf414a7a257f30584c7510f98ad25bb0dfc3bc3fdd4b317cf1bbdb2cc3。

### 003-002 — Deref借用Target與Wrapper實作

Q1：Wrapper<T> owns value欄位、不另給T配置heap，String自管；無自訂Drop可依Holder整體接手取field。Deref在std::ops，associated Target是此實作選的借出型別。
Q2a：deref(&self)->&Self::Target只shared借，回ref連receiver lifetime；std Target可?Sized如str／slice，非新建短命資料或延存。Wrapper impl<T>選Target=T，&self.value原欄位不clone／move。Q2b：原需外模型可借內容供既有介面→提供Target及shared借法→保持原owner又免每caller重寫field路徑。
Q3：use Deref／impl<T>語法沿4.1／6.2，Self::Target角色同Output；此impl T仍8.1預設Sized，不因trait Target允DST就自行改Wrapper參數。
Q4：「仍由原Wrapper持有…String」與body直接證實所有权／heap和借用介面分工。
未知／影響：新契約與必要前提足够，語法*如何接上待下一段；無必要缺口。

原始SHA-256：3e3aa81c2f7a949d04c21325b75039444ee9e9fe9a0cb1fbe2e91317b7b07052；visible SHA-256：3e3aa81c2f7a949d04c21325b75039444ee9e9fe9a0cb1fbe2e91317b7b07052。

### 003-003 — *借用對應與DerefMut同Target

Q1：*Wrapper透過Deref先得&T再存取，&*借欄位、i32讀可Copy，不是星號一律交owner；DerefMut在std::ops，須已有Deref並用同Target。
Q2a：deref_mut(&mut self)->&mut Target借原field，需mut外層、ref連receiver／遵守shared衝突；直接trait method use依4.1。Q2b：原需既能借讀亦能改原內容→shared／mutable兩介面→同field透過外層操作而不另造容器。
Q3：兩讀80；最後shared用完才mutable+20，直接field100，&mut self.value顯示修改仍在原data。
Q4：「理解操作的對應，不表示每個星號…owned T」與三輸出限定borrow／Copy／修改的各種效果。
未知／影響：非Copy所有權尚需后段檢查，不提早猜；現有新契約足够，無必要缺口，未實跑。

原始SHA-256：59b731caf4b1916d37a57081534bc18ea8e94f3993e90bebc2848437e6c3c5c9；visible SHA-256：59b731caf4b1916d37a57081534bc18ea8e94f3993e90bebc2848437e6c3c5c9。

### 003-004 — deref coercion／method查找與原位圖

Q1：coercion依所需reference型別繼續借Target；String Deref Target=str、Vec<T> Target=[T]，Wrapper外層可串借。method查找也沿Deref並依receiver借，不交owner／新建文字。
Q2a：&Wrapper<String>→&String→&str；mutable經DerefMut可給mutable或shared，shared不增權成mut。Q2b：原需既有&str／slice／mut fn讀改同data→多層borrow轉換→免重寫field路徑仍保原owner；是否暴露所有內method是介面選擇非每struct必用。
Q3：明as_str及&wrapped皆5，原Rust!可讀；&numbers slice len2，*number+20再add_one(&mut number)得101；as_str／push_str沿既有String API。
Q4：首次原位SVG/native390×625：上&Wrapper<String>借外層→Wrapper Deref Target String→中&String借value→String Deref Target str→下&str借同UTF-8；兩箭頭皆borrow，不是move／clone／每層造文字。SVG SHA-256 d74fb8a3d77009bc84ecaa486cfa4bf83a002e172ac595ab72ee781c2cdb6599；PNG SHA-256 a95c921cc4835f4463b19c9b9b07e0a138f7828e7a83535a79d1ea5eccc7ce26。
未知／影響：圖與新查找契約足够，無必要缺口；局部非整頁、未實跑。

原始SHA-256：9c5514254e38a26a46bfbc4175bec73f5d4ad25b30e7f4436f8a291004013d9a；visible SHA-256：9c5514254e38a26a46bfbc4175bec73f5d4ad25b30e7f4436f8a291004013d9a。

### 003-005 — Deref不供move-out／Target trait不自動傳外層

Q1：一般Deref回ref，Box *move非Copy是特例；Wrapper沒有Drop也不能*取String，並非前節field Drop限制。
Q2a：*wrapped String E0507從borrow移出；into_inner(self)->T整體接手後取field，Wrapper無Drop才合法。Deref Target i32 Copy不給Wrapper Copy，泛型copy_value要求傳入自身T:Copy，E0277。Q2b：原需借讀或owned交出不同工作→借介面與self consuming介面分開→不把星號／method可找誤作外層所有權或trait承諾。
Q3：要String用into_inner、旧Wrapper不再owned；要數字可*Copy i32再交copy_value，不需Wrapper自己Copy。
Q4：兩完整fixtures分辨E0507／E0277與Drop原因，正面给合法替代，確認本人前段Box／field最短區分而不改初判。
未知／影響：無必要缺口；借轉／trait bound範圍足够，未實跑或另造答案。

原始SHA-256：932cbfb5034392d63095e1992ff71793112e769dd7222c7622b553daee86e180；visible SHA-256：932cbfb5034392d63095e1992ff71793112e769dd7222c7622b553daee86e180。

### 003-006 — 8.3節末先借用後取回owned

Q1：Wrapper own原String，push_str／as_str沿Deref method查找借同內容，into_inner接整Wrapper回owned T。
Q2a：mutable Wrapper供push_str追加、shared as_str核全文，末次borrow後self consuming合法；不是用*移出非Copy。Q2b：原需免field路徑又可交出資料→兩種borrow與owned出口分工→借用不拿走owner，改仍落原String。
Q3：Rust交Wrapper、追加 教學、借核Rust 教學、取回owned顯示；只需已教操作，不開參考程式。
Q4：「不把輸出正常解讀為所有自訂Deref…相同設計」限定本Wrapper回原field的證據。
節末：Target／lifetime／DerefMut／coercion／method／owned限制各足够，無必要初判問題；未實跑tests／外連。

原始SHA-256：46274118204da185cf50392096c37854ca20b6e347a895efbc7b6120d346b658；visible SHA-256：46274118204da185cf50392096c37854ca20b6e347a895efbc7b6120d346b658。

### 004-001 — Box內仍可有借用與type lifetime原需

Q1：Box<T> owns T，但Box<&str>只owns reference值，文字owner仍來源；沿6.1 move reference／8.1位置與內容分工。
Q2a：外heap不延原文字存活，T可能含borrow需判有效範圍；'static兩寫法尚待教。Q2b：「泛型T裡面的借用…滿足…有效範圍？」→型別borrow條件→不能因外層owned就判內部全owned。
Q3：先依4.4／4.5來源推Box內ref仍需target有效，不猜static為永不Drop。
Q4：Box<&str>具體反例與兩static預告給所需區別。
未知／影響：bound機制合理待下一單位，無當下必要缺口。

原始SHA-256：2fc4427b60bb4c6fb478d672fb24362d06ec36ad33adac90e4c3a5a970face57；visible SHA-256：2fc4427b60bb4c6fb478d672fb24362d06ec36ad33adac90e4c3a5a970face57。

### 004-002 — T:'a內部borrow／+與implied bound

Q1：'b:'a是lifetime涵蓋，T:'a限T內borrow不短於a，非T值強制存活；i32／String無外借，&'b str／Box<&'b str>仍須b涵a。
Q2a：T:?Sized+'a兩要求同成立，Borrowed field &'aT只借；此field已要求T:'a可implied，所以省+'a仍合法，非所有trait自動推得。Q2b：原需判泛型內借用是否合有效範圍→看實際T及其來源→不把Box owned儲存當消除borrow，model使用仍要T值／內來源有效。
Q3：Borrowed T=String借&text、T=str借as_str、T=&str再借&reference成&&str；*nested.value Copy內ref不取得String，三例來源均活着。
Q4：表格與「不要把所有省略的bound…缺漏」给必要與可省明確界線，符合4.5／8.1而不猜泛型规则。
未知／影響：新type lifetime／隱含條件首次說清，無必要缺口；未實跑。

原始SHA-256：7cdba550947bd24192d68ce58a1f20915cf75dd27f3adf8be67859c33ab9b0f3；visible SHA-256：7cdba550947bd24192d68ce58a1f20915cf75dd27f3adf8be67859c33ab9b0f3。

### 004-003 — &'static str資料有效性與短契約

Q1：'static是預留lifetime不用宣告，&'static str說target可在剩餘程式期間有效；literal如此，持ref的局部scope不等literal資料存活。
Q2a：長有效性可合較短borrow，choose_label依candidate返回契約但else literal可供；短String.as_str不能annotation成static，E0597。Q2b：原需分內借用與存活範圍→literal來源／ref值分開→可保存literal ref到外層，不誤延inside或限制全部fn只static。
Q3：內inside ref值交外saved仍Rust；choose_label false亦Rust但仍依簽名一般來源契約，不因該分支就擅改結果static；要保存短文字交owned String。
Q4：「改寫reference型別沒有改變它的owner」與两个inside對照给清楚機制／限界。
未知／影響：literal特殊有效性及方向足够，無必要缺口；未實跑或補global／raw pointer概念。

原始SHA-256：95f228ebf14e10878c0a53a0c00db1cc3ef5c84c9f2a5e5e7fdd7ea345d99315；visible SHA-256：95f228ebf14e10878c0a53a0c00db1cc3ef5c84c9f2a5e5e7fdd7ea345d99315。

### 004-004 — T:'static資料依賴與普通Drop

Q1：T:'static限無短命外借依賴，String／i32／&'static str符合，局部String借出的ref不符；非值永不Drop。
Q2a：require_static按值收回同T不clone／儲存／延存；Message owned String＋id可通bound仍block末Drop，自訂Drop不影響此型別條件。Box<&local str> owned外層仍E0597因內借短。Q2b：原需接收者可保值而不被外源先結束破壞→檢查整T依賴→保留或即Drop可由接收者選，非普通fn皆加static。
Q3：80／literal／Message合法，Message收尾在block已結束訊息前；需Box獨立則交String，能借則保實際lifetime。
Q4：「說 owned值都符合static…漏掉」與TextView／Box兩例反證，讓bound與scope明分。
未知／影響：新static type／reference關係足够，無必要缺口，未實跑或擅定接收者保存到程式末。

原始SHA-256：8997466f52b8cf227558fa8fda31e76fd08cea65a17af0da1e5762fb56facbd7；visible SHA-256：8997466f52b8cf227558fa8fda31e76fd08cea65a17af0da1e5762fb56facbd7。

### 004-005 — 8.4節末owned模型通static仍可改

Q1：Wrapper<String> owns文字且T無外借，Wrapper<&str>仍內借；外模型owned不一律static。
Q2a：require_static只檢型別並原樣交回，mut Wrapper可經DerefMut push_str；static不改原owner／Drop／借權限。Q2b：原需泛型內借用能否合範圍→選真正owned內容／保明確來源→判資料依賴而非讓value永久存在。
Q3：Rust String交Wrapper→require_static→追加 教學；若傳text.as_str需短borrow無法同bound；Box<String>可通後*取得owned修改依8.1特例。
Q4：練習的兩種T對比及library測試敘述對應本節機制，不開參考程式或tests source。
節末：type lifetime／?Sized＋／implied／static兩角色已有必要前提，無必要初判問題；未實跑。

原始SHA-256：09c3b8dc7db29ac2f46a44f9d14354698583f6f30ef7765a70be036082b8dadd；visible SHA-256：09c3b8dc7db29ac2f46a44f9d14354698583f6f30ef7765a70be036082b8dadd。

### 005-001 — 異型費用清單的原需

Q1：4.1 Order／Bundle共同amount但欄位形狀不同，Vec只存同T沿3.6；新清單還可接caller運費型別。
Q2a：當段未教trait object機制，先定每項只要amount能力。Q2b：「放進同一份費用清單」且可外加型別→共同能力表示→加總不需各全部field，enum／泛型／impl Trait差別待後段。
Q3：訂單80＋組合120=200，加運費20=220，既教小i32加總規則。
Q4：具體三種類／答案給當下需求，不以原來懂dyn猜物件表示。
未知／影響：異型如何同清單合理待教，無當下必要缺口。

原始SHA-256：e3e59fb0fe44cb27cd4e66ecda7c6ad2a8b2fc6256c436db65eadec410763fce；visible SHA-256：e3e59fb0fe44cb27cd4e66ecda7c6ad2a8b2fc6256c436db65eadec410763fce。

### 005-002 — 同款Charge／enum固定種類與fixture範圍

Q1：本module重新定義Order／Bundle／Charge，無外依賴，amount乘或固定價沿4.1；KnownCharge包兩型別令Vec元素同型，borrowed match沿3.1。
Q2a：兩impl Charge不改Vec同T要求，異型混放E0308；enum已知variants可完同加總。Q2b：新需caller自加型別而不改library variants→接下來共同能力介面→固定enum與開放型別的需要不同。
Q3：KnownCharge match借order／bundle呼叫原amount，不取得owner；目前尚未用dyn，無未知API。
Q4：前段兩Charge impl＋已教Vec同型足推規則；獨立「完整fixture」只定Order／Bundle未含Charge，直接展示的僅異型不能同Vec。
原判：可選O1（例子證據範圍）：正文「兩種型別有共同trait」緊接fixture，但該獨立code無此trait實作；必要關係已可從前文推出，不阻讀，因此optional。最小改窄歸因說fixture只核異型同Vec、共同trait另由前段契約知，不需補新機制或實跑。

原始SHA-256：3873e92abc365d56e167aea1a74182dc665794fa2187e5b56d1f244b5160e2bd；visible SHA-256：3873e92abc365d56e167aea1a74182dc665794fa2187e5b56d1f244b5160e2bd。

### 005-003 — &dyn Charge借用視圖與unsizing

Q1：dyn Charge是共同能力trait object／DST，&dyn是shared視圖，資料仍外owner；不是動態型別語言或必須Box配置。
Q2a：需要&dyn時Charge的&Order／&Bundle可unsizing成DST視圖，compiler核trait，保資料及method查找資訊；非Deref逐field、不clone，object lifetime跟外ref要求且來源仍活。Q2b：原需不同形狀只用amount→同共同視圖介面→使用處不写具體T仍可borrow各型別；generic_amount保T則另一設計。
Q3：兩介面80、dyn Bundle120，原Order count1可讀；最短從「method資訊」推同dyn fn能找到各實作，不另補vtable細節。
Q4：兩gettercode／原owner後讀支撐借視圖，尚不當owned異型清單已驗。
未知／影響：新DST／轉換與前文Sized、lifetime足够，無必要缺口；後段清單合理待教，O1初判不改。

原始SHA-256：989fadeb1d7f26e1201a77045d8f7b18cc3c0074559ea88a6e30fd88cc067a4c；visible SHA-256：989fadeb1d7f26e1201a77045d8f7b18cc3c0074559ea88a6e30fd88cc067a4c。

### 005-004 — Box dyn同型清單／dispatch／object lifetime及原位圖

Q1：Box<Order>/Box<Bundle>可在所需位置轉Box<dyn Charge>，仍owns原具體資料；Vec元素同此handle型別。fn簽名／struct field無Charge lifetime bound時object預設static，限內依賴非Box永存。
Q2a：Box Deref讓amount method查找借內dyn；dynamic dispatch用handle資料／vtable method入口，generic具體T可monomorphization選static實作。Charge能力轉換時編譯核，i32無impl E0277。Q2b：原需caller新增型別而不改library→同handle共同能力／runtime找實作→保原計算且total只borrow清單。
Q3：enum與dyn同80+120=200；Vec move到extended、Box Shipping20 push後220，total無新增分支；不證enum不能算／dyn較快。
Q4：首次SVG/native390×690：values0同Box dyn handle左箭頭找到Order80×1、右找到Order表amount；values1左Bundle price120、右Bundle表amount，所得80／120；箭頭非move／clone，非ABI布局。SVG SHA-256 064f1dbb82d01c2a81498690e308cced5d61c1663ea4bd6c36241e1f83b46d47；PNG SHA-256 0a0cf04f68cda72024537a8de3586a2e18daf8c40db8eaca5b325a5457f8417b。
原判：必要關係足够、無缺口。可選O2圖可讀性：兩條右箭頭x290垂直經y178／487的「method表」字標，native實見交字；仍能由右框Order／Bundle表辨關係，故optional，最小移標籤到箭頭旁／繞字。局部圖非全頁；O1前unit初判不改。

原始SHA-256：aff6d000a8df4d72630621447e7cdb1c51d0067baf34fbc2228b2f99b5a46b00；visible SHA-256：aff6d000a8df4d72630621447e7cdb1c51d0067baf34fbc2228b2f99b5a46b00。

### 005-005 — 具體／opaque／dyn工廠與boxed Fn

Q1：回Bundle露公開field，impl Charge只承諾單具體T，Box dyn可每分支不同T但共同handle；泛型／參數impl由caller選，enum固定已知種類。
Q2a：Order／Bundle皆Charge仍opaque E0308；Box各分支轉共同介面合法。dyn Fn(i32)->i32保存兩closure，work(value)按同Fn能力call；move Copy base無短借符合static。Q2b：原需不同資料形狀／caller增實作→選dyn owned介面→不像只隱藏一個T；同shape選行為仍可6.4 opaque，不需處處Box。
Q3：具體price120、opaque amount120、dyn工廠80／120、兩closure30→20／40；新完整opaque fixture含Charge實作，另直接支持同trait≠同T，005-002 O1原範圍不改。
Q4：比較表給每選項用途與「可能消除間接呼叫」限定成本，數字輸出不作效能排名。
未知／影響：所用Box轉型／Fn／capture／lifetime都有前提，無必要缺口；未跑／補vtable ABI細節。

原始SHA-256：b9dd4b6a46915d3069bb7fa02f14fdb40813b03373a0200895b220e85d7ae38d；visible SHA-256：b9dd4b6a46915d3069bb7fa02f14fdb40813b03373a0200895b220e85d7ae38d。

### 005-006 — dyn object含borrow與外ref兩種lifetime

Q1：BorrowedCharge含prices slice borrow，owned Box不消來源；dyn Charge+'a限內型別borrow，外&'a lifetime是借視圖期間，兩位置分開。
Q2a：return Box dyn+'_按單輸入elision連prices，impl BorrowedCharge<'_>適用相應來源非固定static；默认Box簽名若static會lifetime不足。&'a(dyn Charge+static)外借仍短，內不依短源；local省略亦可能推導不一律static。Q2b：原需owned共同能力仍讀既有prices→明object bound跟來源→不為了Box重建清單或延長原Vec。
Q3：charge amount200、&*charge dyn_amount200、原Vec len2；原Vec結束不能續讀。純owned Order可內static但外borrow短。
Q4：完整失敗fixture与四位置表說清同shape不同有效性契約，修正+'_直接對應問題。
未知／影響：placeholder／括號／object lifetime在必要處有說明，無必要缺口；未實跑／用前文ABI猜答案，O1／O2原判不改。

原始SHA-256：9712e07ac22e90508520005635c6e0386d789f2e1b9f0c4fe3b17f6526d14f6a；visible SHA-256：9712e07ac22e90508520005635c6e0386d789f2e1b9f0c4fe3b17f6526d14f6a。

### 005-007 — dyn compatibility、supertrait／where與固定Output檢查點

A／dyn可用介面：Q1：dyn compatibility（旧稱object safety）讓隱藏Self後receiver／輸入／結果仍可明確。Q2a：整trait Self:Sized與dyn DST不合；generic method任意T不供全部vtable入口；返未知Self／直接使用Self／opaque回值不作動態method。Q2b：原需共同amount→介面在未知具體Self仍能呼叫→限制表與返回契約而非禁止普通具體method。Q3：三完整fixtures E0038，Charge &self→i32合適。Q4：「呼叫處不必再知道Self是Order還是Bundle」给規則需要／機制。
B／限定部分method：Q1：trait :Charge是supertrait要求，須已有Charge；where clause放簽名尾、body／分號前，where Self:Sized只限該method。Q2a：rebuild不供dyn，但其餘amount仍可；不同於整trait :Sized。Q2b：原需具體型別便利重建又保共同能力→個別排除動態呼叫→不犧牲整trait object。Q3：Bundle可rebuild120、Box dyn仍amount120，硬call rebuild訊息拒絕；不是Clone／Copy實作。Q4：完整成功與method拒例对应兩層bound。
C／associated Output：Q1：普通associated type可用，dyn Measure<Output=i32>先定結果。Q2a：沿6.2 equality bound固定Output，未定E0191；不要求知道Self布局。Q2b：原需保不同實作能力而caller知回值→介面選定Output→動態method可交確定i32。Q3：Bundle80 measure80。Q4：「associated type不是全部禁止」及合法／失敗例足分from generic method。
未知／影響：各新必要條件首次與節末已區分，非完整進階規則清單；無必要缺口，未實跑。O1／O2不改。

原始SHA-256：d8d1a1633a95e7405e2d49ee56611199c98351e283baeb846f65d2ead3249a5b；visible SHA-256：d8d1a1633a95e7405e2d49ee56611199c98351e283baeb846f65d2ead3249a5b。

### 005-008 — 8.5節末caller加ExtraFee

Q1：三不同concreteType同Box dyn Charge元素，caller ExtraFee owns數字40，Charge amount共同能力。
Q2a：impl Charge後Box::new轉介面，Vec同T／total既有borrow流程；一般實作能力先編譯核、dyn由handle選method。Q2b：原需不改library加費用→caller own型別實作共同契約→不用新增KnownCharge variant／total分支。
Q3：80＋120＋40=240，最短由Shipping例改數字，既有trait／Box／Vec足用；參考程式未開。
Q4：具體練習与測試目的對上開放種類、borrow／factory／Output關係，非效能證明。
節末：shared／owned dyn、DST轉換、兩lifetime、dispatch、compatibility與介面選擇足够，無必要缺口。可選O1 fixture歸因／O2圖標箭頭交字原判保留；未實跑。

原始SHA-256：e69783e33eb667cd77aa0f932695f8964e23aadcde590ddd102bf5594539254e；visible SHA-256：e69783e33eb667cd77aa0f932695f8964e23aadcde590ddd102bf5594539254e。

### 006-001 — Error共同顯示／原因借用的原需

Q1：具體enum分類／caller政策沿5.5–5.6，Result對E不要求Error；std::error::Error另給通用報告介面。
Q2a：當前只提出顯示與下層原因borrow，methods／supertrait待教。Q2b：「不知道每個module的variant名稱」仍需工作失敗與原因鏈→共同標準介面→報告器不用為每enum重寫match。
Q3：原enum仍保可分類資料，不因新Error介面改变是否繼續／停止或Result容器。
Q4：兩種工作的界線明說，所需dyn／borrow已前段實讀。
未知／影響：新Error契約合理待下一單位，無必要缺口。

原始SHA-256：55783c59cf1750457cd80e177dc96c39a562c7a278ef543860284bed318caec6；visible SHA-256：55783c59cf1750457cd80e177dc96c39a562c7a278ef543860284bed318caec6。

### 006-002 — Error supertraits／source借用與method預設body

Q1：Error:Debug+Display两supertrait同需；source(&self)->Option<&(dyn Error+static)>借下一原因，None終端／未提供、Some下一Error，不取owner。
Q2a：內object static限原因型別內borrow，ref仍連self非永久；trait可供預設method body、source默认None，未覆寫沿用，故CountError空impl合法。Q2b：原需通用顯示与原因借讀→已有顯示兩能力＋可選原因介面→葉錯誤不憑空造原因、不用重寫None。
Q3：同0..100規則，Negative-1 Debug檢查／Display含數字、source None；少Debug即使Display／空impl亦E0277，source caller需use stdError沿4.1。
Q4：full缺Debug fixture與leaf輸出分別支撐前置／預設，Source signature把8.4–5兩lifetime位置對上。
未知／影響：新default method語意與標準Error條件明說，無必要缺口；所需fmt／Error imports由已讀use規則及當段完整fixture可推，未實跑。

原始SHA-256：9a62a1563e98858460792529941f77e77a71579c44c9bc1640750405d2763df3；visible SHA-256：9a62a1563e98858460792529941f77e77a71579c44c9bc1640750405d2763df3。

### 006-003 — owned原因／shared source與有限通用report原位圖

A／原因封裝：Q1：ParseCountError own Integer／Range原錯，ReadCountError own輸入String＋cause；成功簡化為已驗i32而非Count模型。Q2a：&self match得field refs，unsizing回&dynError，內type static、外ref借self；Display只本層，From／?流程沿5.5–6。Q2b：未知variant報告器需同介面讀原因→封裝保資料、借source→不複製錯誤或重复人讀原因。Q3：3成功、abc Integer、-1 Range；read_count只讀傳入文字非FileIO。Q4：原code保field而頂層Display分工明确。
B／通用報告：Q1：report借&dynError，format／push_str建owned文字。Q2a：Option source循环Some顯層再借下一、None結束；本型別有限向下不循環。Q2b：原需不為每enum重寫分類→common Display／source→同report走兩失敗路，caller for仍續。Q3：每層一行、含原abc／-1及葉訊息，未以文字猜variant。
Q4圖：首次SVG/native390×650，上ReadCountError→借cause ParseCountError::Integer→借原ParseIntError→None；每框本層Display，箭頭shared非move／clone。圖&error指原payload關係，code borrowed match error已ref故Some(error)，不再借局部ref值。SVG SHA-256 1a77580f6cc6f481b9689d20ce8b157acb23f005dcddc17dc1cc4038759ef146；PNG SHA-256 e02e81916f1b993fbf43e82015000340b4778ad58abd268dfcfd62e6a19863c6。
未知／影響：兩關係都有前提，無必要缺口；局部圖非全頁、未實跑，O1／O2不改。

原始SHA-256：2927ce7d9bd6c1bc95df9eac3435a3c2f89a1f64b12d4bc837209a3b367bc3da；visible SHA-256：2927ce7d9bd6c1bc95df9eac3435a3c2f89a1f64b12d4bc837209a3b367bc3da。

### 006-004 — source ref連self、外Error可borrow而cause須static

Q1：Option<&(dynError+static)>內限原因type，外ref仍借self；Error整體只Debug／Display，外BorrowedParseError可借label並own ParseIntError cause。
Q2a：Cause owned數字符合static仍不能借過Outer结束，E0597；含短borrow的BorrowedLeaf能Error但作source cause不合static，lifetime不足。Q2b：原需保持context／原cause／借用各角色→只對下一原因type用標準bound→不把外Error強制static或返回ref永久化。
Q3：with_context label lifetime進returnedError，text僅parse不做0..100；report可借label有效的error，顯輸入欄位A／標準cause，之後label仍可讀。要保報告越error用owned String。
Q4：兩完整fixtures＋合法borrow外層對照把前段lifetime約定實際拆開，「不是說所有borrowed錯誤都非法」界線明確。
未知／影響：既教receiver／object bound足推全部角色，無必要缺口；未實跑或open source。

原始SHA-256：2ebee4f92baee94a7175bbbb13daf91191004509364c2da28c9b1b75816ab1d9；visible SHA-256：2ebee4f92baee94a7175bbbb13daf91191004509364c2da28c9b1b75816ab1d9。

### 006-005 — Box dyn Error標準From與報告邊界

Q1：需要match分類用具體enum，只統一報告可owned Box dyn Error；原資料／Display仍各來源不同，context對照此例刻意不保輸入。
Q2a：此Box返回默认object static，Std From把Error+static具體錯誤owned包Box，?沿5.6轉ParseIntError／CountError，无需各自重寫boxing；report &*error只借內dyn。
Q2b：原需不同來源共用報告→std owned共同介面→callers不列所有variant，但明確回復分類需求仍用enum。
Q3：3成功，abc標準解析訊息，101件數超限訊息；Err分支持Box至結束、report不拿owner。
Q4：三輸出顯同handle不等同錯誤文字；body先給From／static前置，沒有從?可用倒猜實作。
未知／影響：無必要缺口；只採已教穩定source、不取Rustdoc完整清單或實跑。

原始SHA-256：fcd86118f995eda1f002b5775b2317aeb1979c71ffdc9160d0c379495c35bc63；visible SHA-256：fcd86118f995eda1f002b5775b2317aeb1979c71ffdc9160d0c379495c35bc63。

### 006-006 — 8.6節末OrderError自己的context

Q1：OrderError own id7與CountError101 cause，Debug／Display／Error共同契約，report未知此struct亦可borrow。
Q2a：source Some(&cause)對&self receiver、cause type static但ref不永久；父Display僅訂單工作、子葉說原件數。Q2b：原需自訂context與下原因同報告→每層保資料／分顯示→不重複或讓報告器學variant。
Q3：依已示ReadCountError改id／cause即可兩行：訂單7檢查失敗＋件數101超限，source不move；不開完整參考程式。
Q4：練習／測試目的对應單次各層與typed分類保留，未把Box要求通用套所有介面。
節末：Error條件／預設source／兩lifetime／借鏈／From Box／報告用途足够，無必要缺口；未實跑／取tests source，O1／O2不改。

原始SHA-256：778660ee93fc7b6ee3ec7c7d65b4c954eb04696633d6ea81c0235bd5e1b0bf8c；visible SHA-256：778660ee93fc7b6ee3ec7c7d65b4c954eb04696633d6ea81c0235bd5e1b0bf8c。

### 007-001 — 共用尾段且不固定最長owner的原需

Q1：兩鏈80→120與40→120要同一份尾120；重建兩份只是數值相同，借用則需外owner有效，沿已讀ownership／borrow。
Q2a：當前提出shared owner，Rc／Weak機制尚待教。Q2b：「不先指定哪一條必須活得最久」→各鏈持共享owner→其中一鏈結束不妨礙另一讀同尾。
Q3：先保同data關係，不把Box move交兩次或複製120冒充共享。
Q4：兩種可算同值但資料關係不同的對照，給新介面真正需要。
未知／影響：計數／觀察机制合理待教，無當下必要缺口。

原始SHA-256：9418fe6c4d66721556435fb934baa8b7d63f52db30ca5cc7c11417837ed920ac；visible SHA-256：9418fe6c4d66721556435fb934baa8b7d63f52db30ca5cc7c11417837ed920ac。

### 007-002 — Rc strong handles、clone／move與同配置

Q1：std::rc Rc=Reference Counted，strong handles共同保活同heap T；每handle own普通值／scope，計數非所有&T，Rc供shared Deref含field查找。
Q2a：new接T strong1；Rc::clone借handle新同配置strong+1非Tclone／不需T Clone；strong_count usize目前數，ptr_eq bool同配置非內容。move換handle名不+1；末strong0才T清。Q2b：原需無固定最長鏈owner→多handle共管理→一份結束另一仍讀同T，不copy尾資料。
Q3：Tracked無Clone/Copy仍Rcclone成功，1→2／ptrEq真，drop first後id1及strong1，drop second才一次T Drop，main再印末訊息。
Q4：「數的是Rc handle，不是…&T」及trace明分borrow／owner；不推清T即所有控制配置已釋放。
未知／影響：單thread限制已給，thread細節留並行章、現單流程不需操作；必要契約足够，無缺口，未實跑。

原始SHA-256：ffc66b002c0ffb5f075f0fbc65ba7f3afdbc17936df657880564fd205ba8e0a8；visible SHA-256：ffc66b002c0ffb5f075f0fbc65ba7f3afdbc17936df657880564fd205ba8e0a8。

### 007-003 — SharedChain借total與三strong尾段原位圖

Q1：SharedChain next是Rc shared owner，固定表示仍靠間接存放；total借&Chain，mut chain僅換ref不改原資料。
Q2a：borrowed match value:&i32 Copy、next:&Rc；chain=next依Deref coercion繼續借，不Rcclone／move。Q2b：原需兩開頭同尾且各可先結束→head next各有strong owner→不需外tail一定最長，原數字加總不變。
Q3：tail＋兩next count3，sum200／160；drop first fieldnext→2，drop tail仍second.next保活sum160；clone／清理需管理計數非零成本。
Q4：首次SVG/native390×640，tail handle及first80.next／second40.next三實線同指heap Link120 strong3；此Link own另一Rc next→End，非三份120或三份End.next。SVG SHA-256 eccb2e83c9873c06ca513418653cb3dcf1546151066dc618611065ad575f5663；PNG SHA-256 e7e54c17e9ad51574b6c8d6cdd210e1d0ce97973ef7661facd8d4878c75878ed。
未知／影響：圖与code清楚給共享／borrow兩角色，無必要缺口；未實跑／看整頁。

原始SHA-256：3adb1fbb2faa93ef3b53f06c09986bf394dc59b7407664a5b2ab2eaa04ea5fe0；visible SHA-256：3adb1fbb2faa93ef3b53f06c09986bf394dc59b7407664a5b2ab2eaa04ea5fe0。

### 007-004 — Rc沒有DerefMut／不提供Box move-out

Q1：Rc供shared Deref無DerefMut；mut handle binding不是獨占修改T，*非Copy仍borrow不能moveout，與Box特例不同。
Q2a：*owner+=20 E0594，*owner String移出E0507；外多handles可能仍需同T，不能各以獨占target修改。Q2b：原需共用尾只讀→保持shared介面→不讓mut變數繞共享資料權限；可修改狀態另介面後節教。
Q3：只讀sum仍足；可重新交handle和Copy數字讀，不推直接mut／所有pointer都moveT。
Q4：兩完整fixtures正面說原因與已有Deref/Mut機制，而非只列碼。
未知／影響：新的Rc限制足够，修改API合理待教，無必要缺口；未實跑。

原始SHA-256：ccf23771cb79ab8e4c09851fe13ac896d67cd401605815504562f16d663aa3a7；visible SHA-256：ccf23771cb79ab8e4c09851fe13ac896d67cd401605815504562f16d663aa3a7。

### 007-005 — Weak不保T、upgrade新strong與配置收尾

Q1：std::rc Weak觀察同配置不保活T，非普通&T；T可已清理Weak仍在，讀前upgrade獲owned Rc或None。
Q2a：downgrade借Rc不减owner／不加strong；Weaknew空無配置；upgrade借Weak、alive Some新strong+1，empty／T末None不復活。最後strong清T，Weak留只保管理配置，末Weak才不再因它保配置。Q2b：原需記位置但不延T→觀察與保活owner分開→每次Option判可用，成功guard自己又保活。
Q3：空None；owner1加weak仍1，temporary升2、match末回1；drop owner T Drop一次，observer upgradeNone，再drop observer。若先升級Rc留外層，原owner結束仍可Some需算新strong。
Q4：「T清理與整份配置釋放分開」在操作前說明，trace各scope直接支撐，沒有借用就自動增count。
未知／影響：新的weak／strong／配置三角色足够，無必要缺口；未實跑或猜原T raw位元值。

原始SHA-256：52e07618103717d9ba10443c54e82ff3591b6c1650487571d312674184100e7b；visible SHA-256：52e07618103717d9ba10443c54e82ff3591b6c1650487571d312674184100e7b。

### 007-006 — strong循環／Weak責任與假設圖

Q1：reference cycle沿連結回原位置；若兩內Rc互保活，外handles都無仍A／B strong1，T未清所以內Rc也未清，成memory leak。
Q2a：Rc要0才T清，安全borrow不自動收循環；若B→A僅觀察改Weak不增A strong，upgrade接受None；其他圖須查真正strong循環非隨改一field。Q2b：原需owner與觀察選擇→依連結責任設計→不無意讓已無使用者資料永遠保活。
Q3：此圖假設非本節可構造程序，按Rc/Drop已教即可推各1；不填未教修改API。
Q4：首次SVG/native390×490，上A strong1來B、下B strong1來A；左Rc實線A→B、右Rc實線B→A，無外handles，兩向回圈無0。SVG SHA-256 1d5a085fb785ab0636284d6031d1cf280e9b491398a3d643f15561afb46994d0；PNG SHA-256 340b0c15206ffb8e37647218f7606bde59e8c036f86076d152cb11a3763b3144。
未知／影響：假設範圍与機制明說，無必要缺口；未實跑／看全頁或把所有相互連結都算泄漏。

原始SHA-256：19dd996b8c079a3910a406205bd2b474019b6261ec784505326d7b7b30cf4660；visible SHA-256：19dd996b8c079a3910a406205bd2b474019b6261ec784505326d7b7b30cf4660。

### 007-007 — 8.7節末三owner與observer

Q1：三Rc保活同120，Weak只觀察；upgrade的Some是新owner要算，不是普通borrow。
Q2a：clone兩次strong3，drop前兩剩1；短match升級暫2後回1，末owner結束upgradeNone。T清與配置保留沿前段，不因observer而保T。Q2b：原需owner任意先收尾但可觀察狀態→分strong與Weak→檢查可用性不漏算暫時Rc。
Q3：用match／assert／count即可最短推驗證；末tests None記0／Some至少1僅辨存在，若數count也含Some當時handle，不當探測前數。
Q4：練習先要求temporary短scope，直接支撐None前提，不需未教is_some API或參考程式。
節末：Rc clone／move／Deref限權、共享圖、Weak／兩cleanup層與假設循環足够，無必要缺口，未實跑；O1／O2不改。

原始SHA-256：62dfc5140661c69d632ed16f288c3e5f2beedc9ebb96a4225f818d481466934a；visible SHA-256：62dfc5140661c69d632ed16f288c3e5f2beedc9ebb96a4225f818d481466934a。

恢復註記：環境重連後，本人state有46筆至008-001、四題record45筆至007-007。以已保存自己的8.2–8.7紀錄恢復前文上下文，未讀peer／作者／未讀BODY；當前008-001斷前已開未存，僅重取同unit filtered BODY，原visible SHA一致，不advance state。下項四题在此恢復點保存，誠實記有同段回讀，沒有聲稱重現斷前內心初判；O1／O2及舊45項原文不修改。

### 008-001 — shared記錄器原需與interior mutability

Q1：Record record(&self,i32)->unit是shared介面，普通Vec.push需mutable所以fixture E0596；內部可變性只由特定安全API在shared下修改，非所有&T任改。
Q2a：當前先定Cell交接／Copy與RefCell runtime借用兩路，具體API待教。Q2b：「寫入輪流發生…shared receiver」→模型內部短讀寫權限→各持有者不需長期獨占整模型；若能改&mut self則普通Vec足夠。
Q3：示例self.values.push(80)不能借mutable，不能以Rc shared Deref或mut handle繞限制，沿8.7實讀；目前不猜RefCell方法。
Q4：完整fixture與&mut替代先說清需要，非只有錯碼／聲稱普通Vec不能記錄。
未知／回讀：此是重連後同當前unit重取，未見下一unit或peer，不回填既有判；新具體規則合理待教，無當下必要缺口。

原始SHA-256：360caa0db9c3dbd5ad58fa8b2dc6d19c5e260db9d4ce49cd32c50b2dec5dcc68；visible SHA-256：360caa0db9c3dbd5ad58fa8b2dc6d19c5e260db9d4ce49cd32c50b2dec5dcc68。

### 008-002 — Cell值交接／Copy讀取／shared更新

Q1：std::cell::Cell<T>存T，用shared操作受控更新，不交供本例長期持有的&mut T；get副本與owned交接不同。
Q2a：new接T；get需T:Copy留原；set(&self)換新並清舊回unit；replace換新返舊owned不需Copy；into_inner接整Cell返owned T。Q2b：原需shared輪流記錄→按值讀／換而不曝光可變borrow→Counter無mut仍tick，非Copy文字亦可換／取。
Q3：0兩tick得1／2；String replace舊Rust、新Rust 教學再into_inner，Cell已交出；Cell<String>.get E0599不是Cell不能存String。
Q4：五操作表先明receiver／結果／Copy界線，後trace支撐同data更新與新舊owner。
推論／未知：Counter片段未另列import，但本段給std::cell來源及main use，依1.9可在library模組補use std::cell::Cell，無需猜API。必要關係足夠、無缺口；未實跑。

原始SHA-256：a2cf0125039db123d9eb2336f2b3ac5de4723c3f068a472e7eed4e623a423656；visible SHA-256：a2cf0125039db123d9eb2336f2b3ac5de4723c3f068a472e7eed4e623a423656。

### 008-003 — RefCell runtime借用與Ref／RefMut guards

Q1：RefCell owns T并保存活躍borrow狀態，非共享owner；Ref／RefMut是guard值，Deref／Mut借T，lifetime連容器非延長它。
Q2a：shared多份或mutable一份互斥，borrow／borrow_mut receiver均&self，衝突當次panic；guard正常清理才解狀態。mutable guard binding mut才能DerefMut，不需RefCell binding mut。Q2b：原需shared介面push留原Vec→容器檢短權限、guard供內容borrow→不關掉ordinary references編譯檢查或clone另一Vec。
Q3：兩shared读80／80，drop兩者後mutable push120再drop，再shared得len2／200；stdmemdrop與Deref查找前文已教。
Q4：兩guard表／三API表先交代能力及時點，code顯式drop對上讀寫互斥。
未知／影響：state解除與普通borrow不同已有依據，無必要缺口；失敗／可處理API合理待後段，未實跑。

原始SHA-256：1306c2bbe27ba7646a71fe7bb947cd04274bd9bad53d01c9977cda5a63fc57fe；visible SHA-256：1306c2bbe27ba7646a71fe7bb947cd04274bd9bad53d01c9977cda5a63fc57fe。

### 008-004 — guard清理時點、try方法與原位圖

Q1：Ref guard是owned借用管理值，最後讀不解除runtime狀態；try_borrow／try_borrow_mut回Result，成功仍原guard，錯誤BorrowError／BorrowMutError本節不需讀fields。
Q2a：shared未drop即borrow_mut可編譯但panic already borrowed；try_mut Err不建新guard／不改80，drop view才成功；mutable活躍時try_shared Err。Q2b：原需可能衝突而正常繼續→由try取得結果並match→明確guard清理後再試，不把ordinary last-use套runtime。
Q3：80→第一次try Err→drop shared→100，match末清mutable，再writer+20活躍拒shared，drop後120；drop guard不清內i32。
Q4：首次SVG/native390×595，藍建立view shared活躍→橙println末讀仍未drop／try_mut Err→綠drop view／try_mut Ok；兩箭頭是本例程序順序，非資料move。SVG SHA-256 1b5fbebf9a2a4b41704ab4816739135318890361fb26a2b11ad71ae60c9e79b6；PNG SHA-256 c5ef02e638bde3eaf695ac45f5bae92c62a1b9e33694f2de82073f4d8117b77f。
未知／影響：兩種失敗政策與時點有原文／code依据，無必要缺口；局部非全頁、未實跑或讀panic外部解答。

原始SHA-256：68c6d3fade6e5767420fff4146bdcc6397e5d68986958f9b8464b3e1c1142495；visible SHA-256：68c6d3fade6e5767420fff4146bdcc6397e5d68986958f9b8464b3e1c1142495。

### 008-005 — Recorder shared寫入與回傳guard責任

Q1：Recorder owns RefCell<Vec<i32>>，Record仍shared；empty建立空值，view借self回Ref<'_,Vec>，不是snapshot，caller保來源及guard時點。
Q2a：record內mutable guard push、fn末清理；view '_連receiver，留view再record會borrow_mut衝突。局部RefCell回Ref即使任意'a亦E0515，沿4.4借回局部限制。Q2b：原需各程式部分輪流記／讀→內部短guard與caller借view分責→shared介面真保存同Vec而不長期獨占模型。
Q3：record80／120，view2項總200；drop view後40第三項；trait method需Record進scope沿4.1。
Q4：「view不是免費的snapshot副本」與顯式drop、新view兩段直接核借用責任，完整new_view fixture給來源有效限制。
未知／推論：library片段的Ref／RefCell imports可由本段fixture std::cell及已有use規則最短補，不需外部解答；無必要缺口，未實跑。

原始SHA-256：bbbd37fe28af8b698f1811b25d67446d1826783b5b61015c99b86d8badbf2190；visible SHA-256：bbbd37fe28af8b698f1811b25d67446d1826783b5b61015c99b86d8badbf2190。

### 008-006 — Rc保活／RefCell權限及實際Weak父連結

Q1：Rc<RefCell<T>>外層shared owner保容器、內層guard管借T，兩計數不同；Rc Deref找到RefCell，其methods &self所以不用Rc DerefMut。
Q2a：Rcclone不borrow_mut、borrow guard不增strong；writer block末清後另一Rc讀同Vec。Node children ordinaryVec<Rc>保子，parent RefCell<Weak>只觀察，短guard補backlink不增父strong。Q2b：原需不同持有者輪流修改／共享且不強循環→分保活與讀寫／觀察責任→不因mut handle或Weak延T。
Q3：两Rc同Vec80／120 count2，drop view與first後second仍len2；父臨時upgrade Rc末清，branch1／leaf2，branch末Drop2後leaf1、父upgradeNone、最後Drop1。
Q4：完整兩code把8.7假設轉可構造例；「children…不修改…普通Vec即可」明說僅parent需RefCell，trace對應Drop hook後字段清理。
未知／影響：所有接合操作已有前提，无必要缺口；單流程範圍明說，thread同步合理待並行章，不補未教工具或實跑。

原始SHA-256：e7722d3b1c5c122b2d0aa16d0e577a237c9af84c8b64519e85fb6170dcf4a066；visible SHA-256：e7722d3b1c5c122b2d0aa16d0e577a237c9af84c8b64519e85fb6170dcf4a066。

### 008-007 — 8.8節末兩Rc共享Recorder與guard

Q1：兩Rc保同Recorder活著，view Ref只管同內Vec的shared借用，不是第三Rc／資料副本。
Q2a：Rc Deref讓Record shared method找到Recorder，view仍在即阻mutable內借，必須drop才追加；Cell值交接／RefCell guard讀寫兩不同介面。Q2b：原需各部分輪流記讀→分shared owner與短guard→保資料與順序，又不長期獨占模型。
Q3：80／120 view總200、drop後第二record40，再view三項240；Rc、Record import与trait method規則已教，最短延Recorder例，不開答案。
Q4：「哪份Rc維持…哪個guard阻止」正對兩責任，不靠輸出數字猜ownership。
節末：Cell五API／guard兩kind、runtime互斥、try結果、清理時點／圖、回guard來源及Rc接合足够，無新增必要缺口；O1／O2不改，未實跑或取外連。

原始SHA-256：ebbba998ba033b3d6d902adb00f572107031efd661253f3b654b42d34ea52a67；visible SHA-256：ebbba998ba033b3d6d902adb00f572107031efd661253f3b654b42d34ea52a67。

### 009-001 — 公尺與秒的型別邊界原需

Q1：路程80公尺／等待80秒同i32數值，alias沿5.2只另取名、不成新型別邊界。
Q2a：此段未給新型別／trait機制，先要求錯誤單位在編譯發現。Q2b：「函式只接受路程」→需不同型別表示角色→避免把秒安靜當公尺，後續加法／報告需保單位。
Q3：既有alias不能保該邊界，具體struct／operator／實作可放crate規則待後段。
Q4：兩80相同值但不同角色，使問題不是數值算錯或需要外部量測。
未知／影響：新介面合理待本節教，無當下必要缺口；不提前猜coherence完整規則。

原始SHA-256：4f750202d0292050ec7ae76b7004a6888a5bc3234ae2f6845fec5d619db4f24e；visible SHA-256：4f750202d0292050ec7ae76b7004a6888a5bc3234ae2f6845fec5d619db4f24e。

### 009-002 — alias不能隔單位、可縮Box dyn Fn名稱

Q1：MeterAlias／SecondAlias都i32，PriceFn就是已教Box<dyn Fn(i32)->i32>，名稱不成新型別／新能力。
Q2a：alias_distance仍收elapsed80；PriceFn可move成完整型別不轉換／重新Box／clone，Fn與object lifetime原契約不改。Q2b：原需單位邊界alias不足，但長型別閱讀→取短名→簽名聚焦角色仍由實型別決互傳。
Q3：減10 closure price_fn(true)，同owner原完整型別80→70；两分支Box dyn能力沿8.5，非opaque兩closure同T。
Q4：兩行預期分別展示錯用未擋與適用縮名，不拿「編譯正常」當單位保護成功。
未知／影響：無必要新前提或缺口；owned句柄／Fn介面／static背景已有實讀，未實跑。

原始SHA-256：18dd18f7ea75d7641fe24e7f10c238787276a0409587abf3cefa988321c10f42；visible SHA-256：18dd18f7ea75d7641fe24e7f10c238787276a0409587abf3cefa988321c10f42。

### 009-003 — newtype／tuple struct／公有欄位與單位邊界

Q1：newtype是用新struct包舊資料的pattern非keyword；tuple struct字段有順序無名，Meters／Seconds／i32三個不同型別。
Q2a：外pub公開type，內pub公開field／建構；Meters(80)與.0依新語法／既有tuple編號。derive Debug／PartialEq不含Copy，整體move、借field i32可Copy。Q2b：原需秒不誤傳公尺→distance_value只收&Meters→在同數值80時仍編譯隔型別，不靠名稱alias。
Q3：借距離讀80，原仍Debug Meters80；&Seconds E0308。公有i32容任意值不驗非負，需要驗證另選5.4private建構。
Q4：完整成功／錯型fixture與「沒有驗證數值是否非負」分清身分、ownership与有效資料政策。
未知／影響：tuple struct／兩pub必要語法首次前已教，無必要缺口；不假定newtype自動Copy／heap／驗證，未實跑。

原始SHA-256：fc90b8f7a62f0b2caca52edcdd7d42ca4c013e1a56b31150d41abdf6e6b4961e；visible SHA-256：fc90b8f7a62f0b2caca52edcdd7d42ca4c013e1a56b31150d41abdf6e6b4961e。

### 009-004 — Add預設Rhs／Output／owned運算與單位方向

Q1：std::ops::Add是+介面，Rhs=Self為預設型別參數非method body；Output由impl選，沿6.2，與Rhs角色分開。
Q2a：add(self,rhs)接左右非Copy Meters，Self(...)本tuple struct constructor；add_same要求Add<Output=T>才同型輸出。Add<Meters> for Millimeters明右Mètres、乘1000返毫米，不自動反向／其他operator。Q2b：原需路段相加保單位→同單位或明方向換算的Add→不把500毫米+2公尺當502／丟單位。
Q3：80+120→Meters200，40+60→100，500mm+2m→2500mm；再讀left E0382，反向Meters+Millimeters E0308；小i32限定不溢位／無限精度。
Q4：三角色表、兩完整fixtures、三結果支撐型別／ownership／方向，非只給加法數字。
推論／未知：Add import由明示std::ops及fixture use可依舊規則補；必要關係足夠，無缺口，未實跑或推所有Add同結果／借用方式。

原始SHA-256：f7dd3c40f156867bcd7cabc7ea0fa6fa4436a4747bcecfb7e96cb33181b951fc；visible SHA-256：f7dd3c40f156867bcd7cabc7ea0fa6fa4436a4747bcecfb7e96cb33181b951fc。

### 009-005 — orphan本地性、有限newtype介面與泛型covered順序

A／非泛型impl：Q1：local依定義在寫impl的crate，import／alias不搬定義；外Display＋外Vec不許。Q2a：本節無impl型別參數時trait或Self須local，alias CountList E0117。Q2b：原需自訂路程顯示→由自己type／trait負責→避免兩library替外同一對提供冲突實作。Q3：MeterBatch local可Display；library local BatchSize可impl Vec<i32>，example只import呼叫。Q4：正文把「本crate」明指提供impl之library，非import使trait變example本地。
B／有限介面：Q1：公有MeterBatch但private Vec欄位，接原Vec不另Box／clone；newtype不自動繼承method。Q2a：from_values接owned、count／total借數值，fmt用既有Formatter；無Deref所以route.push E0599。Q2b：原需建立後只計算／報告→僅公開所需操作→caller不依賴全部Vec方法。Q3：2段／200再借仍200、Vec batch_size2，沿本節小數限制；數字Copy不移非Copy Meters。Q4：合法兩方向及push fixture分開local規則／介面選擇。
C／泛型impl：Q1：uncovered是直接T未被一般型別包，檢Self先再trait args，第一local前不可有這種T。Q2a：From<T> for Wrapped<T>先local合法；From<Wrapped<T>> for T先裸T E0210，非所有foreign Self＋local argument都禁。Q2b：原需泛型包／取回→合法From與自身into_inner→不替所有T插入外trait。Q3：80.into成Wrapped／String From後self取回，無自訂Drop由8.2–3可移field。Q4：兩完整實作對照；Box fundamental例外（Box<Local>可local／Box<T>不coverT）只是此段界線，不用來另猜新impl。
未知／影響：每個首必要條件有正例／反例，無必要缺口；完整orphan其餘规则不假裝已教／實跑，O1／O2保留。

原始SHA-256：c4a86c84982f43c33f3fc57a2ff1b423df553804893ba9a805fbab8a56dcc0ec；visible SHA-256：c4a86c84982f43c33f3fc57a2ff1b423df553804893ba9a805fbab8a56dcc0ec。

### 009-006 — coherence與blanket impl重疊

Q1：coherence同需orphan合法與impl不重疊；blanket泛型impl涵一類型別，非更精確可覆寫的預設版本。
Q2a：impl<T> Label for T已涵Meters，再impl Label forMeters兩份匹配E0119；本地trait不免唯一性。Q2b：原需責任與適用實作唯一→限制重疊→不同標籤應各不重疊，別先承諾所有T同規則。
Q3：fixture自己的Label可通local責任但因兩份impl拒，正常選各具体type實作；不需要執行才發現不同標籤。
Q4：完整fixture與「兩項都要成立」讓crate責任／同型匹配分清，不混8.6 trait method預設body。
推論／未知：<T>已有Sized預設，這裡「每個T」按該參數契約理解，Meters確實涵蓋；無必要缺口，未跑或推進階specialization。

原始SHA-256：87b211774c2d039b279a6816529af3243d41404e5c01ed844ad1c627612601ab；visible SHA-256：87b211774c2d039b279a6816529af3243d41404e5c01ed844ad1c627612601ab。

### 009-007 — 8.9節末TimeBatch仍保單位及有限介面

Q1：TimeBatch local新type owns private Vec<Seconds>，total借數字返新Seconds，非Meters／alias。
Q2a：沿MeterBatch from_values／while／Display只借，不自動公開Vec方法；local Self可impl標準Display，仍不能餵distance_value。Q2b：原需等待報告保自身單位→新邊界＋有限操作→顯示後資料仍可借且不誤成路程。
Q3：Seconds30／60共90秒、Display兩段，再total；最短改已教MeterBatch模板，不開參考程式／額外probe。
Q4：「單位邊界仍要保留」與各失敗情況描述對上目的，不把本地實作或新的名字當驗證所有資料。
節末：alias／tuple newtype／pub、Add預設與方向、有限method、orphan順序／fundamental邊界、coherence均有必要前提；無必要缺口，O1／O2不改；未驗完整tests／編譯結果。

原始SHA-256：b4af2eb2617b935b0fb54aa9cd3c861915182ab881c21595d8d8660742317e87；visible SHA-256：b4af2eb2617b935b0fb54aa9cd3c861915182ab881c21595d8d8660742317e87。

### 010-001 — caller擴充事件payload與確切型別原需

Q1：payload是事件id之外附加內容，傳遞層只保存，處理者取回原i32／String／自定struct；Any是標準exact型別辨識trait，非Charge共同amount能力。
Q2a：此段尚未教辨識／取回methods，先區分已知enum種類与caller擴充。Q2b：「沒有一項amount那樣的共同業務操作」→需保原型別後辨識取回→不每次改傳遞層variants。
Q3：已知種類仍可enum，需要共同操作仍先用trait；具體Any條件／操作待下一單位，不先猜轉換語意。
Q4：對照8.5清單用途給新需要，不只導覽新名字。
未知／影響：Any介面合理待教，無當下必要缺口；未讀外部型別答案。

原始SHA-256：c2409de1421286122ae446089dd3524182129be293ff1967d4be2346d82eec09；visible SHA-256：c2409de1421286122ae446089dd3524182129be293ff1967d4be2346d82eec09。

### 010-002 — Any自動實作與內型別static／外借用區別

Q1：Any在std::any，要求實作type static；標準已供本節數字／String／無borrow struct，不另手寫impl。
Q2a：String type不依短借，可局部Drop，&dyn Any只借原String期間；存as_str結果则T含短borrow不能靠外owner變static，沿8.4两位置。Q2b：原需確切資料可保存／取回→先限型別依賴→不把事件內容指向先結束的local來源，又不強迫每次view永久。
Q3：以原String作data可Any，以局部借用reference值作payload不符，失敗fixture待後段；不自己加入blanket／重疊Any impl。
Q4：「reference仍只在原String有效期間」把bound與borrow正面拆開，已有前文足推。
未知／影響：新static前置足够，具體操作待下一段，無必要缺口。

原始SHA-256：f4b3eedde9e548bca4263f735878e525b8a71cd8cd0959e778f1ce3406ac35d9；visible SHA-256：f4b3eedde9e548bca4263f735878e525b8a71cd8cd0959e778f1ce3406ac35d9。

### 010-003 — exact downcast與dyn Any泛型helpers

Q1：downcast從共同view試確切T，caller編譯選T、runtime核原type，非值轉換；is bool辨識、downcast_ref Option<&T>成功借原，依外borrow有效性。
Q2a：Any供辨識，std另為dyn Any提供generic helpers，非vtable中generic trait method，故不抵8.5規則；失配None不改payload。Q2b：原需取回caller定型資料→先exact check再typed借讀→不因數值可容80就改usize成i32。
Q3：amount i32 true／Some80、count usize false／None，原兩數仍可用；局部String type static、短&dyn view取&String得Rust。
Q4：表与「helper的T由caller選定」在code前，equal value／different type對照支撐機制；不是查任意trait／文字field名。
未知／影響：所用T及helper條件足够，無必要缺口；不推完整進階型別查詢／內部TypeId ABI，未實跑。

原始SHA-256：8117b9fbca13c7cf839dcde41989c4dec1695e5ded35eb319b8c44e9c76c14e5；visible SHA-256：8117b9fbca13c7cf839dcde41989c4dec1695e5ded35eb319b8c44e9c76c14e5。

### 010-004 — Event owned Any與downcast_mut／來源限制

Q1：Event owns id與Box dynAny，event payload:implAny由caller選T，按值入Box非clone；payload_number借Event→&dynAny→Option<&i32>來源連Event。
Q2a：downcast_mut需&mut dynAny，回Option<&mutT>只成功借原T、不建新T；&mut *Box沿8.1。i32輸入依Copy、String依move；static內type與外shared／mutableview分開。Q2b：原需傳caller資料並由處理者改已知type→owned公共handle＋typed借用→未知String保持原，非shared任改。
Q3：numeric80+20為100／true，StringRust更新數字false仍Rust；局部as_str reference入BoxAny E0597，+'_不能消Any本身static；需独立文字交String。
Q4：正例三helpers在操作前給契約，完整錯例對上8.4外Box不own原String資料的關係。
推論／未知：note.payload的downcast_ref可依8.3 Box Deref到dynAny helpers；Any來源已明std::any可補library import。無必要缺口，未實跑或推任意trait反射。

原始SHA-256：37a0b6b7c85ea7d474ddda75fa51e8baf8888e12debaa85a8fd714d29c3e9696；visible SHA-256：37a0b6b7c85ea7d474ddda75fa51e8baf8888e12debaa85a8fd714d29c3e9696。

### 010-005 — owned downcast Result交接與原位圖

Q1：Box dynAny.downcast<T>消耗handle，回Result<Box<T>,Box<dynAny>>，Ok known owner／Err原owner，不是Option借讀。
Q2a：成功或失敗原boxed皆move；String後Box DerefMut push_str，再8.1特殊*move出String；失配不改T／clone。Q2b：原需接收者要ownership又可試另一T→Result保失敗資料→不因查i32失敗丟掉String或假轉數字。
Q3：Rust請i32 Err(original)，再String Ok改Rust 教學並taken；owned試後讀舊boxed E0382，即使T是Copy i32 Box亦非Copy。
Q4：首次SVG/native390×560，上BoxDynAny StringRust→downcasti32／不符→Err仍owned String→再String→Ok BoxString；兩箭頭交接handle／Result非資料轉數字，與兩層match一致。SVG SHA-256 c726615dd61789b855dd3f28ee1d4617ef784110bb724bb4cd5dd3f548087be8；PNG SHA-256 59a450de12445810121f582ea51a242c860f8ee63cc519d214a281cab3e8e3ab。
未知／影響：新增owned失敗責任與借讀分辨足夠，無必要缺口；局部非全頁、未實跑或假定所有pointer支持此動作。

原始SHA-256：71abcc5ddbe78e99c08a776338011c7449c4447552c7b436b1f3856e101413ac；visible SHA-256：71abcc5ddbe78e99c08a776338011c7449c4447552c7b436b1f3856e101413ac。

### 010-006 — TypeId與Box自身／內payload receiver

Q1：std::any::TypeId是型別身分值，of<T>對staticT查，PartialEq／Debug可比；非事件id／data值，內表示不用解碼。
Q2a：Any.type_id借receiver；dyn內容回隱藏T，但BoxDynAny也自有Any，boxed.type_id優先Box能力；&*boxed成dynview才查i32，concrete_id(&boxed) T明Box。Q2b：原需確切辨識→明查哪層receiver→不把外handle身分誤認payload或Rust忘型別。
Q3：view ID==i32真，generic BoxID==BoxDynAny真；誤boxedID==i32 fixture可編但assert失敗left==right，非借用或解析失敗。
Q4：「Box本身也是一種型別」在兩receiver例前說清，正常predicate與錯例各給所測層的證據。
未知／影響：比較／lookup及static條件足夠，無必要缺口；跨建置不作保存格式／外號碼，未實跑／解碼ID或補RTTI實作細節。

原始SHA-256：fc4703eac6a7ed10a96650fcc3a7b245cb16a0d8a415e715d0359613b782af61；visible SHA-256：fc4703eac6a7ed10a96650fcc3a7b245cb16a0d8a415e715d0359613b782af61。

### 010-007 — 8.10節末caller ExtraInfo owned取回

Q1：自定ExtraInfo count2無短borrow，可用已供Any，Event id3與payload owner分開；確切type檢查不靠共同amount。
Q2a：owned payload.downcast ExtraInfo回Ok Box或Err原Box；成功mut count+1，再Box特例*移出whole struct，原payload名稱已move。Q2b：原需種類由caller擴充、傳遞層不增variant→通用owned handle再typed處理→可取回自有資料，保失配owner。
Q3：event(3,ExtraInfo)後請求同T、加count至3／taken確認；所需AutoAny、mutableBox／Result／field／move已教，不開參考程式。
Q4：「失敗分支要接手原payload」直接對應責任；段末並列enum／共同trait／Any需要，避免一概改成runtime辨型。
節末：static與outerborrow、standard泛型helper、ref／mut／owned downcast、失配Result／圖、TypeId兩receiver與保存界線足夠，無必要缺口；O1／O2原判不改。本章66unit到此完成，未實跑或讀外連／後章。

原始SHA-256：1e5e011be3c2284f458eaf7949a11318930df3855de3b9244e7c2e1fe8897f9f；visible SHA-256：1e5e011be3c2284f458eaf7949a11318930df3855de3b9244e7c2e1fe8897f9f。

## 第8章原始首讀封存

全66單位依唯一frozen v8順序完成；每unit保存自己的四題／依據／未知／hash後才下一。8個SVG都有原位文字相遇後的本體／native局部渲染記錄與逐關係指認；沒有完整desktop／mobile頁面。closed fold數0，參考程式與外部連結都只見連結文字，未開答案／archives／完整source／peer／author／outline／live或第9章。

必要初判缺口0；可選初判O1／O2共2項，保持原位置與分級。O1在005-002：獨立fixture只證異型同Vec失配、未含共同Charge實作，正文共同trait關係已由前段教，最小窄化fixture歸因。005-005後續完整trait fixture另補證據，不覆蓋初判。O2在005-004：dyn圖兩右箭頭穿method表字標，仍可辨資料／表角色，最小移標或繞線；只依原位native所見，不宣稱整頁通過。8.8–8.10未新增必要或可選問題。

已核理解範圍：固定表示／DST／const長度與Box、Drop責任／次序、Deref借用與move邊界、type lifetime／static、共同dyn介面與相容条件、Error原因借鏈、Rc／Weak兩清理層、Cell／RefCell受控修改與guard、newtype／Add／coherence，以及Any確切辨型／三種downcast／TypeId receiver。新必要條件、原需與操作相接的證據在逐單位記錄，不從本來懂Rust或peer4獨有說法補答案。

恢復限制：重連時state已開46單位但僅45項已存；先恢復本人舊record上下文，再只重取當前008-001的filtered BODY、核相同visible hash後記四題。該項有同段回讀，不能宣稱還原斷前未存的內心答案；其餘舊45項原bytes由guard保護未改，也沒有越看008-002後文再補它。記錄拼字註：009-004筆記的Mètres指正文Meters，屬拼字、不新增型別或改初判；此處追加澄清而不改原行。

未驗範圍：Cargo／編譯與runtime fixtures／完整tests與sources／跨平台輸出／全頁與外部文件。正文預期輸出與錯誤码、局部SVG只能支持本輪理解，不當實測通過或真人試讀。

獨立frozen scope（order＋66 units＋8已讀SVG，共75項）保存work/ch08-v8-raw-scope.sha256，該檔SHA-256：dd5fa1638fa66b5efbc3c4f649d182e2f55e109847c40175e6458ec026d7630c。visible ledger SHA-256：c33b49480e981295ae5f4562274eb0cf530b5125f57780f4c95ed80d1ba9f186。重連前45項prefix guard SHA-256：c786524cd8fe80068ec889c39a7dd60c22fed6e7eaaeae1c82315717267da2a5；所保護原bytes前綴SHA-256 4316bfb4db0b3596511d2463d069e38e0f94d919c1d3061d7c5b129d5be0ec2b 保持一致。

原始初判封存至此。停止讀第8章後續或cross；7v13受影響局部followup另等root給正確slice後獨立記錄，不提前開稿，不回填本initial或舊第7章。
