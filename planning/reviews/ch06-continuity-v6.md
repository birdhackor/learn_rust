# 第 6 章銜接初判：原始凍結 v6

## 身分、獨立性與版本界線

審閱者：`chapter04_traits`；角色是第 6 章的獨立銜接審閱者，沒有參與本章寫作。我曾為第 4、5 章的研究與審閱讀過第 6 章片段，所以這次不是新讀者的盲讀，也不取代已封存的首次閱讀紀錄。

本次只讀 `/workspace/work/tutorial-implementation/frozen/ch06-complete-source-v6`。依 `reading-order.txt` 核對 7 頁、50 個正文單位，再讀 manifest 指定的全部 80 份來源：39 個 example（含 7 份完整解答）、7 份 library source、15 個 compile-fail fixture、15 個 expected、Cargo manifest 與 3 張 SVG。library 中的 21 個 test 也逐段閱讀。初次工具輸出截斷的正文部分已另行補讀，沒有把被截斷的內容算成已讀。

未讀第 6 章首次讀者紀錄、其他角色初判、作者帳冊、作者 source-notes、協調者處置或 live 修稿；沒有修改教材、程式或共享設定。以下 N1、N2、I1 均為本角色由原稿與實際前文產生的初判。

原始 manifest SHA-256：`dacb0f09813d368bcdb9c7bf94b23a618c29cb6b29da1d5f55fb37aa39fdfb0b`。50 個單位與 80 份 source 的位元組指紋已核對，記錄在 `/workspace/work/tutorial-implementation/reviewer-ch06/continuity-v6/source-fingerprints.json`。協調者短訊最初少列一張圖；本次以實際 manifest 的 **3 SVG** 為準，不沿用短訊的 2 張。

審閱依據為專案 `AGENTS.md`、clear-tutorial 的 `SKILL.md`、`references/review-protocol.md` 與 `references/rust-terminology.md`。讀者起點是基本 Python；以好懂好學為優先，正文、失敗範例、test、helper、答案及設定都只能依賴此前實際教過的能力。名詞出現、標題或未來課綱不計為已教。

## 實際前文回讀

前文版本固定為第 1 章 `ch01-complete-source-v8`、第 2 章 `ch02-review-fixed-v5`、第 3 章 `ch03-review-fixed-v6`、第 4 章 `ch04-review-fixed-v7`、第 5 章 `ch05-review-fixed-v7`；各 manifest 指紋已存於上述證據檔。本角色過去讀過這些章節，此次按第 6 章的真正需要回讀以下位置，沒有聲稱本輪重新逐頁讀完所有前章。

| 實際回讀位置 | 本章需要的已教能力與界線 |
| --- | --- |
| 1.4 `004-002`、1.7 `007-002`、1.8 `008-002` | 數字型別與有限範圍、tuple 拆解與欄位、for/range；不把小數字示範誤認為全範圍安全的運算 |
| 2.3 `003-002/003-003`、2.5 `005-002` | `&`/`*`、reference 欄位存取、`Self` 與三種 receiver；支援 closure 的借用、associated type 的 receiver 與 iterator 狀態修改 |
| 3.6 `006-002` | `Vec<T>`、new/push、len、slice view、Option 回傳；能建立空容器，不預設 Vec 已有所有 trait |
| 4.1 `001-006`、4.2 `002-004` | 自訂 trait 的 use、String/&str 的 Debug/PartialEq/Clone、Clone 的名稱來源、Vec 的 Clone 條件；不能由 Clone 推出 Vec 的 Debug |
| 4.3 `003-005`、4.4 `004-007`、4.5 `005-004` | HashMap 借用、Option/數值 reference 的能力、`'_` 與輸入/receiver 的 lifetime 關係；annotation 不延長原值 |
| 5.3 `003-002`、5.4 `004-002`、5.5 `005-007`、5.6 `006-003` | `&T` 的 Display、From/Into/TryFrom 的 prelude 來源、for tuple pattern 與綁住整個值的分支、同型別 From identity 與 `?` 的轉換界線 |

另外搜尋此前正文的 Fn 系列名稱及 Vec 的 Debug 說明，沒有找到早於下列 N1/N2 首次使用的對應能力契約。搜尋不是以所有標準型別名稱已出現為由，把未教過的實作一併算入白名單。

## 正文實際順序與首次需求

下表覆蓋全部 50 個單位。單位編號是凍結稿名稱，表示從正文建立的依賴，不是拿作者課綱補齊缺口。

| 單位 | 新需要、首次機制與前置核對 |
| --- | --- |
| `001-001/001-002` | 保留免運門檻以重用計價工作，再介紹 closure/capture、參數/body/回傳與建立、呼叫之別；固定 280/300/320 的例子對應實際需求，也承認普通函式加參數可完成工作 |
| `001-003/001-004` | closure 參數推導會固定而非泛型；`||` 是無參數工作；shared capture 重用 String 的借用規則，讀取長度與原值仍可用各有用途 |
| `001-005/001-006` | mutable capture 先說原變數與直接呼叫 closure 為何各需 mut；move capture 以值保存，Copy 數值/reference 與 owned String 分開；不把 move 當成只能呼叫一次 |
| `001-007/001-008` | 建立時移入、呼叫時移出的兩次 ownership 轉移先交代，再畫圖與看失敗範例；練習只合成已教的 shared 門檻與 mutable 次數，不需 Fn trait |
| `002-001/002-002` | 不同來源的結果型別不同，先列 `Holder<T>` 與 SwitchState，而非先要求讀者理解 associated type 語法 |
| `002-003/002-004` | associated type 是實作選定的型別、不是 runtime 欄位；`Self::Output` 與 self 欄位不同；impl 的 T 先宣告，接手 self 是為了移出 non-Copy 內容 |
| `002-005/002-006` | 泛型 helper 只承諾 S::Output，再用 equality bound 限定 i32；只有後者能做數值乘法，bool fixture 指向實際缺的能力 |
| `002-007/002-008` | test 分別檢查輸出型別/值與 String move；FirstPair 練習不在泛型 impl 中臆測 T 的數值能力，所有答案需求已在當下具備 |
| `003-001/003-002` | 共用函式需要輸入/輸出/呼叫能力，FnOnce receiver 與專用 Output 語法先於第一 bound；名稱不必 use 的來源缺口列 N1，不否定已充分說明的呼叫機制 |
| `003-003/003-004` | FnMut 可透過 mutable receiver 重複、helper 的 task 需 mut；Fn shared 呼叫與能力包含關係分開；Fn 不是純函式/固定結果的承諾 |
| `003-005/003-006` | 小寫 fn 是 function pointer、普通函式及 non-capturing closure 可成為它；帶 capture 不能塞進該型別；sum_with 的 slice、迴圈與 callback 可由當下組合，空輸入回 0 有界線 |
| `004-001/004-002` | factory 保存設定再交回工作，先說 function pointer 無法保存任意環境，再分清參數 impl Trait 與回傳 opaque 型別的選擇方 |
| `004-003/004-004` | 不同 closure 表達式仍是不同具體型別；單一 closure 內分支可解需求；借用 reader 的實際 capture 與 edition 2024 的回傳型別 lifetime capture 分清，未聲稱延長來源壽命 |
| `004-005/004-006` | `use<...>` 先被限定為 precise capture 語法，與路徑 use 不同；可排除未用 lifetime，不能排除真借用；counter 練習接回已教 owned 狀態、FnMut 與 mut 呼叫 |
| `005-001/005-002` | 把索引/推進狀態交給來源，Iterator 的 Item、next、Option 與 &mut self 先於實作；一般 None 後仍可能再有 Some，for 在首次 None 結束 |
| `005-003/005-004` | Countdown 的 next 規則與不公開狀態，Iterator/IntoIterator 的名稱來源，opaque/equality bound 均在使用前；手動 next 的 Some/None 與型別能力逐項可解釋 |
| `005-005/005-006` | IntoIterator 是取得 iterator，非 next；已存在 iterator 交回原進度而非重設。Vec 的三條路線分清 owned/共享/mutable Item，沒有把 reference 移動當作移動整個容器 |
| `005-007` | StepDown 練習由 start=5 手動取 5，再 for 得 3、1，加總 4；start=0 的邊界可由同一規則推出，答案沒有新增索引或 API |
| `006-001/006-002` | 具體需要保留次序、重複值並篩選/縮放；map 的 owned Item→新 Item、惰性及副作用發生時點在第一範例前；未聲稱省下所有配置或必然較快 |
| `006-003/006-004` | filter 借用 Item，所以 slice 的 Item=&i32 導致 callback=&&i32；owned String 的例子不混用這一層。collect 由目標 FromIterator 選建立方式，不自動把 reference 變 owned；失敗 fixture 中 Vec Debug 說明晚了，列 N2 |
| `006-005/006-006/006-007` | fold 交回新累計值，空來源保留初值；Result collect 遇首次 Err 即停、前綴成功值不回傳、空來源成功；沒有偷偷轉 E；parse_total 的 `?` 使用第 5 章已教同 E 契約 |
| `007-001/007-002` | 最終文字統計的順序需求先明確；chars 是 Unicode scalar，lines 的 LF/CRLF/空行/尾端規則與 is_empty 的範圍先於使用；借用來源不需轉成 String 才能走訪 |
| `007-003/007-004` | Entry 的 mutable 借用、兩個 FnOnce callback/回傳型別、先建 key 再 entry 與 default 何時才執行明確；HashMap 三種走訪的 key/value ownership 與不固定順序先交代 |
| `007-005/007-006` | Ordering、PartialOrd/Ord、None≠Equal、NaN 與字串排序範圍先於 cmp/sort；排序回傳 unit、stable 的範圍、callback 次數不承諾；tuple 的 Debug/PartialEq 條件先於整份結果顯示 |
| `007-007/007-008` | count 降序與 name 升序的 cmp 方向各用左右實例支撐；tie-break 才能消除 HashMap 任意次序，stable 單獨不夠。`order => order` 的綁值能力已由 5.5 提供；rare-first 練習只變排序政策，不新增 clone 或借用技巧 |

## 各節教學任務的銜接判斷

| 節 | 主問題與機制為何在此需要 | 最強的例子用途與材料界線 |
| --- | --- | --- |
| 6.1 | 計價保留設定，接著觀察三種 capture 及一次移出；採熟悉的計價、String 與次數避免新資料模型搶注意力 | 280/300/320 說明計價；讀取、兩次修改、String 交出各驗證一種關係。它們不證明 closure 的內部 layout 或所有型別的 Copy/Clone |
| 6.2 | 共用提取操作但輸出不同，associated type 再接到泛型 helper 的能力限制 | i32/bool/String 實際差異與 equality-bound fixture 比單純列語法更能說明用處；沒有承諾 caller 可任意改一個 impl 的 Output |
| 6.3 | 共用函式接手工作，必須承諾怎麼呼叫而非知道匿名 closure 型別 | take、tick、shared add 分別觀察 receiver/移出/重複；function pointer 對照只支持此型別不能攜帶任意 capture，不代表所有 callable 都沒有狀態 |
| 6.4 | 能接工作之後才提出交回工作的 factory，opaque 回傳解決無法直接命名的具體型別 | branch fixture 與單一 closure 解法、reader 短來源 fixture、independent `use<>` 對照有分工。capture 承諾與實際存了何物不同，不需要先學 Box/dyn 才能理解本例 |
| 6.5 | Iterator 把逐筆推進放回來源；associated type、opaque 型別與 Fn 前文此時成為可用工具 | Countdown 的剩餘量與回傳分開，Vec 路線顯示借用/move，StepDown 測原進度。選定來源的有限進度不替全體 Iterator 證明永久結束或精確長度 |
| 6.6 | 先有一筆一筆來源，再將共同篩選與轉換表示成工作；collect/fold 分別回答保存與累計 | Item 型別表、callback log、Result 的 visits=2 互補，能看出值/時點/停止，不是只背鏈式語法。結果順序來自來源與具體實作，不是效能比較 |
| 6.7 | 完成文字整理、計數、排序的真實小任務；新 API 依解析→entry→走訪→比較→排序逐步引入 | 固定輸入報表與不同插入順序測 deterministic 結果；Unicode/line ending 與輸出獨立來源 test 覆蓋重要邊界。這不是 locale 字典排序或 trim、正規化後的同名政策 |

整體次序把 associated type 放在 Fn 的 Output、Iterator 的 Item 之前，再把 opaque 回傳放在 iterator factory 之前；collect 與 callback 的 reference 層次也已有獨立入口。沒有發現範例借用未教的 Box/dyn、closure Entry 黑箱、完整 `'static` 規則或未解釋的第三方 crate。最主要待補的是下列兩個小範圍首次使用關係，無須重新安排七節。

## 必要修正 N1：Fn 系列的名稱來源

- **原位置與短引文：** `003-002.md:7` 說「`FnOnce` 是 Rust 的可呼叫 trait」，第 9 行充分交代 `FnOnce() -> T` 的專用語法與 Output，但第 14 行首次寫 `pub fn run_once<T, F: FnOnce() -> T>` 時，尚未說明 FnOnce 的名稱如何可用。完整 `src/callbacks.rs` 也直接使用 FnOnce/FnMut/Fn，沒有 use 或本地定義。
- **此前已教：** 4.1 說自訂 Charge 的 method 呼叫端要 use；4.2 指定 Clone 是預先提供的名稱；5.4 指定 From/Into/TryFrom 由 prelude 提供。6.3 已教 Fn 的 receiver、Output 及呼叫能力，這些不是此項缺失。
- **缺少的關係：** 知道 prelude 的概念及先前三個具體成員，不能推出所有標準 trait 均可直接用。FnOnce/FnMut/Fn 也是已預先匯入的名稱，這一點沒有被交代。後面 `005-003` 對 Iterator/IntoIterator、`007-005` 對 Ord/PartialOrd 有相同補充，不能倒算到 6.3。
- **實際影響與必要性：** 讀者依前文的名稱規則重建 library helper，會多出一個「為什麼這裡省略 use、是特殊語法還是標準 trait？」的未解關係，後續答案與 opaque Fn 回傳又沿用它。Rust 程式本身合法；必要性來自本專案首次使用必須可由既有教學解釋的標準。
- **最小修法：** 在第一次 bound 前加一句 FnOnce、FnMut、Fn 和 Clone 一樣由 prelude 提供，所以本節不用額外 use。保留現有 Fn 機制與專用語法，無須列全 prelude，也不展開標準庫的內部 call method。
- **定位指紋：** `003-002.md` 為 `288b3e49c1d956aa3f0e84f09248317a26300030948bcf8e7555b1430db88c69`；`src/callbacks.rs` 為 `789d54dc7907c2f9e3745e65daaedef798b24110a3fc8651768bceaa45e6a4bb`。

## 必要修正 N2：失敗範例先用了整份 Vec 的 Debug

- **原位置與短引文：** `006-004.md:32` 起以 E0277 示範無法把 `&i32` collect 成 `Vec<i32>`，第 38 行卻先使用 `println!("{owned:?}");`。標準 `Vec<T>` 在元素有 Debug 時也有 Debug 的說明，要到第 61 行才出現。
- **此前已教：** `{:?}`/Debug 的意義、String/&str 的 Debug、Vec 的建立/len/Clone，以及本單位前半的 collect/FromIterator 已教。這些沒有提供整份 Vec 的 Debug；Vec Clone 的條件也不是 Debug 的契約。
- **缺少的關係：** 這個先出現的輸出操作需要容器的條件式 trait 能力，正文已準備了說明，位置卻晚於第一次使用。正常 `adapter_collect` 的顯示與比較位於該說明後，沒有相同缺口。
- **實際影響與必要性：** fixture 想隔離的是 Item=&i32 與目標 `Vec<i32>` 的建立能力不匹配，未教的整份 Vec 顯示讓讀者還需猜第二項能力。雖然這段故意不能編譯，使用者的「每個 example 只用已教過的東西」仍適用；不能把與預定錯誤無關的新 API 隱藏在失敗範例中。
- **最小修法：** 將現有第 61 行的 Vec Debug/PartialEq 段移到該 fixture 前，不增教學內容或測試。也可把這個 fixture 的輸出改為已教過的 len，但前移既有契約即可保留完整 source。
- **定位指紋：** `006-004.md` 為 `287695109243795af643231d6e95cd027c6ad192fdae2af8fd3da63359ee250f`；`fixtures/compile-fail/collect_reference_items.rs` 為 `e1a87d9d567c90c0033b57f3b86d812a3b108570488f7178cdf4c3b02f611560`。

## 可選改善 I1：把 Vec 的 FromIterator 契約先寫成一般形式

`006-004.md:7` 以「`Vec<i32>` 可以從 i32 元素建立，並依取得順序保存」教第一個目標；6.7 接著收集 `Vec<char>`、`Vec<&str>` 與 `Vec<(String, usize)>`。第 3 章已教 `Vec<T>` 的元素參數，6.6 也教 `FromIterator<A>` 與 Item，6.7 在每個範例前明說實際 Item，因此這些後續可以由泛型容器與當下型別流向組合理解，不列為必要缺口。

若希望減少讀者對「只有 i32 還是其他 T 也行」的短暫疑問，可把原句改為「標準 `Vec<T>` 能從 T 元素依取得順序建立，例如 `Vec<i32>` 接收 i32」。只需改一個已有句子，無須增加所有標準庫實作或替各種 T 寫重複例子。

## 完整來源、練習與失敗範例

全部 7 份解答依當節已教能力可寫出：closure 的 captures、FirstPair 的 T、callback 的累加、counter factory、StepDown 的原進度、parse_total 的既有 E、rare-first 的排序方向。答案沒有新增未教的 helper/API，沒有拿後節新增能力回填前節 model。N1 影響 Fn 名稱沿用，N2 的相關 fixture 已對到同一完整來源，而非只修讀者看到的摘錄即可。

21 個 test 的主要能力可由本文重建：associated type 的值/型別、Fn 接手及重複、opaque 型別與來源、iterator 推進/空輸入/Vec 路線、adaptor 順序/副作用/早停、文字 line endings/大小寫與空白/不同插入次序/輸出脫離來源。test 中既有數字/Option/reference 的顯示與比較沒有被一概當成 Vec 的已教能力；整份 Vec/tuple 的比較條件仍依本章引入點檢查。

15 個負例及 15 份 expected 都已讀：closure 型別固定、capture 借用衝突、移出後再呼叫、FnMut 需求、function pointer capture、Output equality bound、兩種 opaque 回傳/來源過短/implicit capture、Vec iterator move/mutable overlap、filter reference 層次、collect Item 不符、Entry 再借用、sort callback 回 bool。除 N2 顯示先備之外，沒有找到無關的新能力混入預定失敗機制。這裡核的是 source/已教機制與 expected error code 的對應，**沒有本輪重新編譯**來驗證診斷。

Cargo 的 package/edition/library 規則沿用前文；`[lints.clippy]`、TOML `#`、`unnecessary_fold` 與 `"allow"` 的角色及保留教學累計狀態的理由在設定旁有註解。此前第 3 章 v6 已提供 lint 設定入口，現在 selected flag 的目的也明確；沒有把未教 sum 方法當作完成這個例子的必要操作。

沒有以 package 中其他尚未在正文教到的 module 檔案存在，宣稱前節已教其 API；也沒有把工具跑得過替代每次首次使用的依賴判斷。數值例子及答案處於文中與前文限定的小範圍，沒有承諾加減乘在所有 i32 邊界都不 overflow。

## 三張 SVG 的原位角色與驗證界線

| 原位／source | 接在何處、幫助哪個關係 | 本輪可作的判斷 |
| --- | --- | --- |
| `001-007` / `closure-transfer.svg` | text→建立 take 保存 owned String→呼叫 take 移出→saved；兩個不同時點承接緊鄰正文與錯誤 fixture | 同一條文字的去向由箭頭與標籤追蹤，沒有把相同內容當作 clone 證據；不是 String layout 圖 |
| `005-003` / `iterator-next.svg` | remaining 3→2→1→0→0 與每次回 Some(3/2/1)/None 分開；讀者尚未手動 next 前可見狀態與結果不同 | 節點符合這個來源的推進，for 的首次 None 結束提示沒有改成所有 iterator 永遠結束 |
| `006-004` / `adaptor-pipeline.svg` | iter 的 &i32、filter 的 &&i32、map 的 &i32→i32、collect 的 owned Vec；固定小例得到 [0,30,30] | filter 借用 Item 與 map 接手 Item 的層次不同，正文補足 collect 驅動與逐筆工作，沒有誤畫完整中間 Vec |

三圖均讀了凍結 SVG 原始文字、箭頭/節點與正文位置，並核對 `assets/` 與 archived source 位元組一致；intrinsic width 為 400，字級為 24。來源指紋分別為 `4f54b08bae8312a004d862850983245108f10df1a6077a5e1420ebf8f85039e4`、`f2512c81d7e13a13c933e05b228aeb7a4291b4fd76895000c7e0e0821df69749`、`9f76ea92d5a61b0c93cf91d4ee6f4db71714631afd93497fffd23b99eecbca4a`。

本輪**未渲染 SVG、未親看 PNG、未做實際桌面/手機頁面或 DOM 驗證**，所以不宣稱已確認字的可讀性、碰撞、縮放或資產在站點實際顯示。亦未執行 Rust、整套 check 或新實驗；需要解釋的關係可由凍結正文/source 直接判斷。沒有發現正文理解必須依賴卻未提供的圖，但這項判斷不替代網站呈現 QA。

## 初判結論

第 6 章主要依賴順序、例子用途、練習組合、ownership 與 lifetime 的界線可以從本稿及指定前文重建。N1、N2 是待補的首次使用關係，I1 是可選的泛型契約措辭；不需要新增章節或擴講目前不用的內部機制。在必要修正完成並局部複查前，不宣稱原始 v6 整章通過。這份初判封存後停止，交叉閱讀及修後複查另存文件，不回寫本紀錄。
