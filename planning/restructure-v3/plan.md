# Rust 全書重排與教學改善執行計畫 v1

狀態：待全新 Astra @ max 審查；審查完全同意並無必要待修後，直接實作，不再請使用者授權。基準為 /workspace/learn_rust，HEAD 5827aa2c5966e49bd0d941a2d0f8b001ef5b0cf7。既有未提交 review 檔及 sidecar 全數保留，不納入本次修改或 commit。所有代理英文思考、正體中文交付。

## 一、已授權範圍與不能犧牲的條件

本次完成 15 章／109 節向 18 章的實質遷移、27 筆全書掃描問題裁定與修改、第一章獨立計算機基礎及必要 SVG、async 情境與取消圖解、Python/JS mutability 選讀、每節同頁完整例子與答案、導覽和交叉引用整合及檢查。章數節數服務好懂好學，131 節為上輪建議，不是必須湊滿的指標；必修內容不得因搬移而消失。

讀者只有基本 Python；核心英文術語依 skill。任何展示為可執行或完整的例子，包括 manifest、lib.rs、module、helper、tests、fixtures、答案與 compile-fail 案例，只能用前文實際已教的能力，或本節先解釋清楚再使用的新能力。選讀、未完成章稿、只列出名稱的概念清單均不算先備。圖解不能引入未教的 Box/String/ref 語法；不能把內部維護用設定偽裝成初學者已能理解的完整材料。

寫作節奏：具體問題／材料 → 目前所需概念及操作 → 短例／可觀察狀態 → 解釋結果 → 下一步；最後同頁收成完整版本。不要先列十個抽象 API 契約，再一次貼百行程式。必要解說及圖放正文相鄰，進階部分仍為必修。每節完整範例與答案由 snippets 引用唯一真實 source，可折疊，不能只連 GitHub。

## 二、閱讀章序與已決定的依賴調整

1 Rust 起步、基本語法與計算機基礎（8 節）
2 流程控制：條件、迴圈與可測試程式（8 節）
3 ownership、reference 與 borrowing（8 節）
4 enum 與 pattern matching（7 節）
5 泛型、Option／Result 與可變長度資料（5 節）
6 trait、標準能力與 HashMap（6 節）
7 lifetime annotation：回傳借用與資料模型（4 節）
8 錯誤、格式化與轉換
9 closure 與 Iterator
10 Cargo 生態系、I/O 與第一個 CLI
11 smart pointer、資源與進階型別
12 Cargo 工程設定與檔案處理
13 thread、同步與 atomic
14 async、HTTP 與受控測試
15 進階 lifetime、unsafe 與 FFI
16 Pin、Future 與 executor 內部
17 macro、API 設計、驗證與效能
18 兩個引導式完整工具（A duplicate finder，B URL checker）

章 1：工具／第一程式／Cargo package-crate-workspace／型別運算／mutable-shadowing／函式-expression-block-scope／計算機基礎／tuple-array。range 移 2.3。章 2：bool-comparison／if／for-range／while／loop-break-continue／module-use-pub-library／tests-tools／完整整合。章 3：struct／ownership-move-Copy具體行為／shared-ref／mutable-ref／borrowing-lifetime-scope-reborrow／impl-self／slice／String-str。章 4：enum／match／pattern解構／or-range-guard-@／ownership與binding／if-let-while-let／let-else。章 5 泛型（含泛型 impl）先於 Option/Result，然後 UTF8、Vec操作與借用。章 6 trait、Debug/PartialEq、Clone/Copy、associated type、HashMap查詢與分組；章 7 annotation/elision、來源與 owned 替代、struct borrowed fields、receiver/source。

lifetime 路線保留 3.5 → 第7章 → 11 的 bounds/'static → 13 HRTB → 15 GAT/variance → 16 地址穩定。常數泛型、原生路徑、短讀短寫、錯誤轉換、IntoFuture、atomic Ordering、C/FFI、Loom/Miri、benchmark/profile、最終工具各里程碑全數保留。細節由前輪 astra-restructure/proposed-outline.json 的 old_to_new_mapping 與 preservation rules 建立可追蹤表，不把章名當內容已保全。

## 三、實作前固定的可審查產物（審查通過後才改 repo）

根代理先產生 planning/restructure-v3/{plan.md,sections.json,migration-map.json,issue-dispositions.json,status.md}。sections.json 列全部 canonical 小節的章號、標題、唯一 source、舊節來源與責任作者；migration-map 覆蓋全部109舊節，允許一對多，不遺漏必修。每個作者另外寫短 ledger，指出實際首次需要及先教位置，連到真實正文/source，不能只交名稱清單。

新 canonical 正文統一為 docs/chapters/NN-MM-lesson.md（與現行109頁不撞名）；既有 examples/ 的 chNN 名稱保留為穩定範例識別，不為章號一致而搬動數百檔，正文首次清楚說明它們是儲存庫範例路徑。新材料用語意清楚的新 package 名；所有 nav 與交叉引用以 sections.json 為準，避免全域替換誤改 range、例子序號、歷史審閱。

早期另建 examples/intro-workspace，只有初學材料與明寫 name/version/edition 的完整 member manifests；minimal [workspace]/members/resolver 在1.3前教完，Cargo.lock提交。根維護workspace排除該 nested workspace，check.sh另外測它；將這項初學材料的特例明記 CONTRIBUTING/AGENTS，原範例仍由根workspace驗證。完整根workspace inheritance、features、profiles、build等到12章教；10章教 crates.io/dependencies/features/cargo add。早期命令使用 --manifest-path/指定workspace；不要求全書 --workspace，不因早期示範載入後章lib.rs。

## 四、實際工作的先後順序與 subagent 分工

階段 A（根代理，先做）：保存baseline，固定全部小節與舊去向表；將27項逐筆記採用／合併／保留及具體理解收益（不是全刪「模型」「契約」）。維護 Cargo/nav/checks 的修改只有根代理，避免衝突。開始前對原版跑 scripts/check.sh 留baseline，先處理實際環境故障，不能把舊故障算新改動。

階段 B（基礎順序鏈）：章1→章2→章3→章4→章5，依真實已完成內容逐章交接；每章都保留獨立可理解／可執行的階段快照，完整檔不 export 後章模組。其他代理可以同時做外部教材研究、圖片規劃、舊文盤點及27項定位，不提前寫依賴未完成能力的完整教學例子。根代理負責及早審查先備與編譯，通過後才能放行下一章。

階段 C（可並行）：前5章成立後，第6 trait線、第7 lifetime無trait部分可以並行；若lifetime例子用 &str 比較/derive Debug，必須等6.2已教。Cargo/doc工具原文研究可先做，正文也須等自身先備。

階段 D：章6/7交接後，第8錯誤與第9 closure/Iterator基礎可並行；Result collect等章8，dyn等impl Trait，Error等Display。章11的Box/Drop/Deref只在實際先備成立後寫，其餘後續再整合。章10依I/O/paths/clap/Serde/fixture的真實邊依序寫，不能因是一章就全平行。

階段 E：章10/11成立後，章12工程檔案、章13同步、章15 GAT/variance/raw-pointer、章17 macro可依各自先備平行；FFI build接線等build.rs；unsafe Send/Sync等同步。不得引用未完成的章14/16能力。

階段 F：章13完成後章14 async（章內依序）；與章15餘部、章17 API/property/performance可並行。Loom等atomic，Miri等unsafe。章16等async/unsafe/synchronization全部必要契約完成。

階段 G：A/B最終專案分派不同作者，僅在各自file/CLI/JSON/error、async/HTTP契約已成立後改寫；維持約1000核心行、不硬湊；先給可動手的起點／驗收／分層提示，答案同頁依檔名內嵌。保留現在真實工具及tests，除非具體依賴問題需要改碼。

每次作者開寫前，先讀 AGENTS、clear-tutorial/SKILL、writing-prompt、rust-terminology，以及至少一個知名英文與中文對應章節；實際來源/實讀範圍/借鑑及未採用理由保存 source-notes，不能只引書名。章1必讀 Brown與Rust Book/Comprehensive 的對應來源，章14讀 FastAPI EN/zh-hant。研究與寫作是兩階段；不得用後文補前文。一次最多6名子代理，留root整合；檔案所有權互斥。空檔可獨立做checks/研究/審閱，不追求讓每人一直產文。

交接包含：凍結正文/source/manifests hashes、必要前綴與實際能力位置、代表性正常/失敗用法、未教能力、局部編譯結果；根代理核對真實內容後標 ready。若前文契約改動，重驗受影響下游，不機械重寫全書。

## 五、圖解與選讀的具體交付

章1.7至少四組原創 SVG，手機可用直式排列：A 儲存檔→編譯→CPU/RAM；B 值/位置/bytes（120不是地址）；C 沿用subtotal(120,3)呼叫前/中/後；D 純概念heap配置／位置資訊與內容／最後釋放（明標數字程式未實際配置heap）。不出現未教String/Box/ref語法，不暗示locals一定在stack、固定大小不能heap、heap必比函式活久。之後3.8 String回收真實例子。原SVG以程式手寫，屬技術圖解；每個圖有title/desc、箭頭與時序意義、穩定物件名稱及相鄰說明。

章14先用固定任務／等待／可繼續情境建立動機，借FastAPI逐格狀態教法，不把cashier硬等同executor；保留立即加法的lazy例與既有gate等待實驗，各說支持範圍。畫Future/task/runtime關係與取消時JoinSet S1/task T0/等待future W1的不同物件；drop W1不等於task被取消。async不保證平行、await不保證每次讓出。圖不代替真實機制驗證。

Python/JS選讀放3.8後：binding重新指派／物件內容／借用權限三軸；1.5可短連結但本身必須教足Rust。不可說Python immutable物件等同Rust immutable binding；JS const不是deep freeze；不提前加入Cell/RefCell程式，該例外於11章正式教。

## 六、27筆問題的處理與引用遷移

以 /workspace/work/tutorial-reassessment-20261015/merged-audit.json 作原始唯讀發現，不回寫原分級。逐項保存舊位置／引文／新位置／採用或保留理由／修後證據。所有明確錯誤修正；可選逐筆權衡：採用不必要模型、名稱對不齊、修改歷史殘字、普通詞的多餘英文、冗餘作者自我驗證。保留真有技術意義的模型／契約；觀察guard以timeout命名改善，不捏造為未教Guard API。缺圖項與1.7整合，不只刪「照圖」。

章節內容重寫時直接吸收這些修正，不先花力氣改將被拆掉的舊頁。Canonical內容完備後才批次將舊109頁替換為相容入口：解釋內容重排並列出所有新去向；保留原頁URL，不直接redirect一對多到錯誤小節；原headings產生的anchors以明確HTML id保留，舊深連結仍能找到轉向說明。舊頁不保留另一套過時教學。全部目前公開頁保有可達入口。

依mapping改canonical正文及docs/index/readme/CONTRIBUTING中章號、相對連結、下一步、前節指稱、命令工作目錄。原examples package名、Rust行號、range、測試資料、歷史planning reviews不全域改數字。維護source notes/ledger和新導覽索引，保留舊研究及review為歷史不冒稱重跑。

## 七、驗收、返工與發布

自查及獨立審閱分開。每波freeze後由非作者代理做技術／先備銜接檢查；重大新寫章1、章2、章4、章7、章14另用沒有作者歷史的新代理做逐段正文首讀：只給基本Python背景與實際先備正文，分段釋出當前必要圖及文字，先保存四題理解及引文才給下一段，後補選讀；不給作者ledger、掃描缺陷或預期答案。不把整頁review叫盲首讀。技術者／銜接者不得是作者；同一凍結稿各自先封存獨立初判，才交叉比對，root裁定全部必要及可選建議並核對受影響轉場。必要理解缺口修正後再查，不用通過數掩蓋。

所有131左右新節至少有逐節先備/完整範例/交叉引用覆核；保留內容與全新教學分別記錄驗證範圍。root親看未報問題的圖文樣本，不只看別人指出的錯。完整site build之後用可用瀏覽器（可安裝Playwright於work工具，勿加教學依賴）實看新圖和代表長頁1280x800與390x844：字體/箭頭/滾動/折疊/程式與圖位置；保留screenshots/實際尺寸。不能以SVG存在或HTTP200代替視覺驗證。若環境真的無法取得browser，標未驗證並保持尚未發布，不捏造通過。

自動檢查：嚴格Zensical build、缺snippet必失敗；所有canonical和legacy mapping完整、相對連結與anchors可解析、無錯誤舊節號、每節full sample/answer同頁；早期nested workspace的fmt/clippy/tests/doc與根workspace分別執行；scripts/check.sh全套（fmt/clippy/tests/doc/compile-fail/runtime-fail/C例），受影響的Miri使用固定nightly，故意失效pointer只由Miri，不普通執行。只新增必要結構檢查，不替低風險措辭改動寫鏡像測試。固定工具鏈及lock；不為避失敗升版。

先修局部再跑該部檢查，最後一次全套；成功後不反覆無目的跑。27項全部有處置；109舊節全部有去向；所有新節實際先教後用；必要審閱問題關閉；未驗項誠實列出。最終diff不混入baseline未提交review；只stage本次明確檔案。

現有網站在全部整合通過之前不更新。全部通過後整理可審查commit；用現有GitHub權限與先前發布授權更新網站（若本輪環境無有效寫入身份，保留完成且已驗證的本地成果，明確報告唯一的push/deploy阻擋，不要求使用者重做已完成設定、不印token、不假稱已部署）。若需remote提交，先核對remote最新HEAD，合併非衝突更新、不覆寫。CONTRIBUTING的「導覽公開釋出另行確認」由本輪「完全同意後就開工」與前次發布授權判斷可做既定教材更新，不額外停工要批准；沒有可用身分不繞過認證。

## 八、Astra 審查方式

請全新 Astra @ max 獨立檢查本計畫與實際repo/上輪mapping/掃描證據，特別抓可執行依賴、章間順序、全書改動覆蓋、可平行邊界與驗收漏洞。可不同意章序細節，提出最小充分修正；不以「都同意」作預設。只有 verdict=APPROVE 且 required_changes=[] 才放行實作；有必要修改則root修訂新版本，再派另一個全新Astra @ max，只給最新計畫與來源、不給前位review結論。改善建議若仍影響必要理解就屬required，不能改稱optional放過。保存每輪原稿、review與root disposition，保留可審查歷史。Reviewer可完全同意計畫但不能宣稱還沒寫的教材已通過。
