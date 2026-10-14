# 第7章 prefix 首次順序閱讀 v1

角色：AI模擬首次讀第7章；目標只懂基本Python，但已實讀前1–4章正文及局部修後、另補讀5v7全35／6v6全50與6v8的003-002、004-002。補讀背景獨立封存於ch07-entry-reading.md，SHA-256 5a09c8f5c308a2a77ad95c688099e0cf27265a09efe080e6709b209c78746c43。沒有參與第7章寫作或看它的peer／作者答案。第4章曾cross；其peer獨有技術不作第7章已教前提。

本輪只讀frozen/ch07-prefix-five-reading-v9/reading-order.txt的34單位（7.1–7.5），不尋live／7.6以後／作者notes／outline／sources／其他review或外部解答。closed fold於輸出前遮蔽，僅見marker。每次一單位，必要新概念首次及節末記Q1關係、Q2a機制／Q2b原需→改變→用途、Q3操作、Q4證據；保存後才下一單位。既教只定位前文，合理短推論與猜測分開；後文釐清另追加不覆蓋初判。

只於正文原位指圖才讀該凍結SVG並可native渲染；局部圖不等於整頁。未執行程式／Cargo或看完整desktop／mobile頁面，不作這些範圍通過宣告。hash與遮蔽metadata另記work/ch07-visible-units.jsonl，本文逐單位亦存原始及visible SHA。

## 原始逐單位紀錄

### 001-001 — 本節原需與工具範圍

Q1：沿1.9自寫library與std，現在要由Cargo準備別人維護的library；本頁package是ch07-tooling。
Q2a：尚只說明需要設定library／功能／版本，未教新機制。Q2b：原需「如何讓 Cargo 知道使用哪個程式庫、哪些功能，以及哪一組版本？」；設定先備好才接檔案／CLI。
Q3：本節在repo根目錄操作examples/ch07-cli/tooling，延用數字計價；目前不需呼叫未知CLI API。
Q4：「命令列參數解析的 Rust 介面會在 7.6 教清楚後才呼叫」把當頁範圍說明清楚。
未知／影響：外部依賴具體設定合理待本節教，CLI Rust介面合理待7.6；尚無必要缺口，不猜後文。

原始SHA-256：00ff9e07f42902887f557910032448a43acaf56aaa6cc178c9670767943b86dd；visible SHA-256：00ff9e07f42902887f557910032448a43acaf56aaa6cc178c9670767943b86dd。

### 001-002 — dependency來源、registry／API與本機path

Q1：dependency是所需另一package，Cargo建其library crate供Rust使用；registry存package／版本，API是公開介面，docs.rs供其文件，兩網站角色不同。std由toolchain提供。
Q2a：「先找到它的來源與合適版本，再建置需要的 library crate」；加入不等於執行。path依宣告dependency的成員manifest資料夾，../../先上兩層。Q2b：為取得別人功能需定位來源／版本，再依library公開路徑使用；本機ch01-basics則不從registry找。
Q3：clap供CLI解析、tempfile供後續檔案測試，目前只設定；manifest名稱與Rust crate路徑分開，ch01-basics→ch01_basics沿1.9，網站URL不能作use。
Q4：路徑「從 examples/ch07-cli/tooling 先往上兩層，再走到 ch01-basics」足以推出../../ch01-basics，無需未讀source或網站。
未知／影響：clap／tempfile API合理待教，當前不用；來源與名稱關係足夠，未開外連。

原始SHA-256：47ef0ee15581960848980fa868a21fbf82bc67defded21aa6fe72ca21c8a5717；visible SHA-256：47ef0ee15581960848980fa868a21fbf82bc67defded21aa6fe72ca21c8a5717。

### 001-003 — manifest功能與版本要求首次使用

Q1：TOML行內表格集中一dependency欄位；dependencies供一般library／binary、dev-dependencies只供測試／範例等，workspace管理不自動產生相依。feature是該library具名開關，clap derive非本package同名功能。
Q2a：path指定本機，僅version預設registry；features加derive且保留clap預設，default-features=false可關預設但須依各library規則。major>0的"4.6.7"允許≥4.6.7且<5.0.0，"=4.6.7"才精確，實用版本待lockfile。Q2b：原需library／功能／版本→三項設定明示來源、開所需derive、訂可接受版本；同workspace仍須dependency才跨package用library。
Q3：依例設定本機ch01與clap一般依賴、tempfile開發依賴，main以已教use呼叫計價100+140=240；不需clap API。
Q4：「版本約定，並非編譯器…保證」限定major/minor/patch契約；main的跨package呼叫展示path依賴用途，不展示derive效果。docs外連未開。
未知／影響：lockfile如何保存確切版本合理待下一段；derive Rust語法待7.6不必要於此。沒有需猜設定的必要缺口。

原始SHA-256：310cd75a3583364e66912aa5707b4fa75e8e982467246d1570198d649416d503；visible SHA-256：310cd75a3583364e66912aa5707b4fa75e8e982467246d1570198d649416d503。

### 001-004 — cargo add與修改後的locked檢查

Q1：cargo add是改manifest與所需版本解析紀錄的命令，-p沿第1章選package；check／run沿既教用途。
Q2a：@給版本要求、--features加功能、--dev選開發區，現有項目會更新而非重複另建。Q2b：為在練習副本重建前段設定而刻意改dependency，故先允許更新解析，再--locked確認一致。
Q3：兩add之後check、run -p ch07-tooling --locked，可能先下載編譯，最後預期240。
Q4：「還沒有驗證 clap 的參數解析行為」正確限建置／計價觀察；不用把尚未使用的derive當已驗證。
未知／影響：版本解析紀錄細節待下段，目前命令目的／先後足夠；未實跑，無必要缺口。

原始SHA-256：0339ad3091bfb1c61392c62036b6ac2e83900c4554315159c595d47804b87202；visible SHA-256：0339ad3091bfb1c61392c62036b6ac2e83900c4554315159c595d47804b87202。

### 001-005 — 實際dependency組合與刻意更新

Q1：Cargo.lock存完整解析組合，包含依賴的依賴；manifest是允許要求，lock是這次實際選擇。
Q2a：不改設定正常建置沿用lock，--locked禁止更動；cargo update clap重新選合manifest版本，也可能改相關依賴。Q2b：可重用同組版本；想更新時明選dependency並核diff／重驗，不刪lock硬過關。
Q3：在練習副本update clap，和run -p選可執行package不同；更新後比manifest／lock差異再跑程式／測試。
Q4：「不保存作業系統與所有編譯環境」排除把lock視為跨平台全同結果；先前版本範圍／add角色於此閉合。
未知／影響：無必要未知；未實跑update或查看完整lock，僅核正文已教的命令意義。

原始SHA-256：d5cfb824620986ca13c28bc4867824739f5316722d44a1a3074f2a43a8230510；visible SHA-256：d5cfb824620986ca13c28bc4867824739f5316722d44a1a3074f2a43a8230510。

### 001-006 — 7.1節末關係與練習

Q1：manifest答允許版本／功能，lock答實際選定組合，tempfile開發區服務測試；來源package與Rust library路徑已分開。
Q2a：clap範圍4.6.7起<5、derive開關；lock個別version需讀副本真值。Q2b：原需讓Cargo知library／功能／版本→dependencies、features與lock各記不同事，工具準備好才使用API。
Q3：可在練習副本找三項，第三不能抄manifest；本輪禁止完整source／未執行，故不猜實際lock版本。
Q4：「第三項要讀你副本中對應 clap 項目的 version」適當指出操作證據；前兩直接有正文答案。
節末：來源／能力開關／允許與實際版本機制均足夠，無必要初判問題；未驗副本實讀及Cargo，CLI待7.6。

原始SHA-256：51128571ef196feb70928c711f4f1c710bca43393b9f38377668ebcc7377d669；visible SHA-256：51128571ef196feb70928c711f4f1c710bca43393b9f38377668ebcc7377d669。

### 002-001 — 外部公開使用與文件正確的原需

Q1：沿1.10內部測試，本節增加別的crate使用library與文件例子兩個觀察面；仍用ch07-tooling。
Q2a：尚未教新測試入口，兩訂單各依已教規則計含運再相加。Q2b：「別的 crate 能否照公開的使用方式完成工作，文件中的例子是否仍然正確？」要求外部與文件檢查，而非只內部數字。
Q3：80／120各100／140合240；150／150各170合340，分兩訂單不可先合300免運。
Q4：分開訂單對照具體排除錯誤合併算法，原需清楚。
未知／影響：外部test／文件可執行方式合理待當節教；此段無當下缺口。

原始SHA-256：0b7341a7eb5445efd1ffa2e856dd97d81b676f4215e9c2f1d69e6371b943b2ce；visible SHA-256：0b7341a7eb5445efd1ffa2e856dd97d81b676f4215e9c2f1d69e6371b943b2ce。

### 002-002 — 文件註解、doc test與cargo doc首次必要

Q1：//供人閱讀到行末，///附下一項供rustdoc成API文件；其中Rust fenced例子是外部caller的doc test，非library內部位置。
Q2a：cargo test讓rustdoc取例子編譯執行、補簡單main外框，assert_eq真比较；cargo doc只生成文件不執行，--no-deps限選定package、--open請開瀏覽器。Q2b：原需文件例子保持正確→函式旁同寫用途／限制／公開使用例→既能給人查也能實際核行為。
Q3：公開pair_total兩次計價相加，例以ch07_tooling::pair_total取得240；doc命令或target/doc/ch07_tooling/index.html可查，test才驗證。
Q4：「不是單看編譯是否成功」及assert240呈現測行為；非負小值／i32不溢位沿既有限定，沒有新增全面輸入驗證。
未知／影響：rustdoc外框不需自行補未教語法；新註解／Markdown／command已足夠。未生成文件或實跑。

原始SHA-256：1d0a3006ab163528cd86e9e830e313309d0e1152a5efa0d700aad84d96613e9a；visible SHA-256：1d0a3006ab163528cd86e9e830e313309d0e1152a5efa0d700aad84d96613e9a。

### 002-003 — integration test的crate邊界

Q1：tests/該層每.rs是獨立測試crate，從外部用公開library；crate::依所在crate，test中不指向library。
Q2a：Cargo只在測試工作編tests目標，因此直接#[test]不用同檔cfg/mod外框；library同檔則沿1.10。Q2b：原需別crate能公開使用→tests/public_orders.rs以ch07_tooling路徑呼叫→同時驗公開可見性與訂單行為。
Q3：外部tests核80／120→240及150／150→340；內部核300／120→440。改成合併判門檻會破壞第二測試。
Q4：檔案樹、兩種use與「預期答案來自訂單規則」支撐邊界／規則，無需猜測輸出當答案。
未知／影響：已有module／test／公開性足用；本新目標位置與crate指向已明說，無必要缺口。未實跑。

原始SHA-256：164ae3c22c83a14baec4e4d3f8e6657f7c70b2554d3fe9bcb023c692dbbbd432；visible SHA-256：164ae3c22c83a14baec4e4d3f8e6657f7c70b2554d3fe9bcb023c692dbbbd432。

### 002-004 — 選test目標與驗證強度

Q1：同檔／integration／doc是三位置角度，完整test含三者；--test public_orders選獨立目標、--doc選library文件tests。
Q2a：目標不帶.rs，命令選擇已有三機制；main零測試不取消其餘通過，名稱輸出順序無規則承諾。Q2b：分開觀察可知道哪種公開／文件檢查有效，也需assert才實際比行為。
Q3：三條repo根command預期1 library＋2 integration＋1 doc；把文件240改200，doc可生成但test失敗，之後改回。
Q4：「刪掉…assertion…卻沒有比較總額」具體展示執行成功與有意義驗證的差別。
未知／影響：無必要未知；未實際改副本、生成頁面或跑命令。

原始SHA-256：4f50e954d0a1ab714a601b1915f9e9c93aaa1b1121ff585968674023dbe6ed97；visible SHA-256：4f50e954d0a1ab714a601b1915f9e9c93aaa1b1121ff585968674023dbe6ed97。

### 002-005 — 7.2節末公開例子與三種驗證

Q1：第二文件例子仍公開pair_total，不需新語法；doc／integration／同檔各有位置與觀察角度。
Q2a：///中use／fence／assert沿本節首例，--doc取出執行。Q2b：原需外部使用及文件不過時→外部目標與doc test共同核公開結果，而非只生成可看的文件。
Q3：pair_total(300,120)保存結果assert440後跑指定--doc；第一免運、第二加20可由已教規則直接推。
Q4：「use 與 fence 的放法和第一個例子相同」足以安排第二例；預期440由規則提供，非未見答案。
節末：三位置、生成／執行／assert用途均有足夠依據；無必要／可選初判問題，尚未實作或跑。

原始SHA-256：cd8822be4d7a815d7fb10271b0b6437c87fa0497715e6c1928a9669713a856f7；visible SHA-256：cd8822be4d7a815d7fb10271b0b6437c87fa0497715e6c1928a9669713a856f7。

### 003-001 — 檔案讀取的原需

Q1：文字來源改檔案，仍沿5章Result保存具體失敗、caller決定政策；UTF-8文字與bytes沿3.5。
Q2a：未介紹讀檔API，先界定成功需保留原中文／換行。Q2b：「檔案不存在，或內容不是 UTF-8…可處理的失敗」要求把來源失敗交caller而非panic或假文字。
Q3：要讀一份筆記並分類成功／失敗，具體path／方法待下一單位。
Q4：原需同時涵蓋成功保真與兩失敗，不需先知道IO內部。
未知／影響：讀檔與路徑介面合理待教，當下未需猜。

原始SHA-256：8db426225c5b8048d95b01809dbde1593fd4def0a2dbfa2f55f64206a28bdb0a；visible SHA-256：8db426225c5b8048d95b01809dbde1593fd4def0a2dbfa2f55f64206a28bdb0a。

### 003-002 — 相對檔名起點與四種真材料

Q1：ch07-io-text資料夾是本節所有command的cwd，相對檔名依這起點，非Rust原始碼位置；尚不用Path API。
Q2a：note是整份UTF-8含末尾LF，Rust四ASCII＋LF=5、中三bytes＋LF=4合9；empty是成功空String，255 byte非法UTF-8，missing不建才是不存在。Q2b：固定cwd／材料→讀同一明確對象→成功保真與失敗可核，不混空內容和讀失敗。
Q3：先cd examples/ch07-cli/io-text，讀fixtures/note.txt等四名；不為missing建立檔。
Q4：表格與「完整內容…\"Rust\\n中\\n\"」對照完整文字／9bytes，沿3.5與6.7 LF定義可推，尚不預分lines。
未知／影響：相對路徑起點已足夠，Path／cwd API合理待7.5；未親讀或執行實體fixtures。

原始SHA-256：e509102619655dd62b87f8cc1ae7f2e0ad3256f2abe87fdc2b7647e3b5ed787f；visible SHA-256：e509102619655dd62b87f8cc1ae7f2e0ad3256f2abe87fdc2b7647e3b5ed787f。

### 003-003 — 整檔文字讀取與可判斷的IO錯誤

Q1：std::fs供filesystem讀檔，std::io供I/O共同結果；read_to_string取檔名&str回owned String或io::Error。io::Result<String>只是Result<String,io::Error> alias，沿5.2。
Q2a：開並讀完整檔、成功必合法UTF-8；error.kind借錯誤回ErrorKind分類，NotFound缺檔／路徑、InvalidData不符資料需求，其餘_；Debug／Display供檢查／人讀而非猜分類。Q2b：原需保中文換行且可處理失敗→UTF-8 owned結果＋具體分類→caller逐筆報告繼續。
Q3：show_file match成功印9bytes／Debug換行，缺與非法各報原因，main最後仍顯三結果已處理；empty應Ok空String。
Q4：3輸出與原材料吻合，「Err 沒有自動停止程式」沿5章政策，未知OS文字不作分類依據。
未知／影響：kind剩餘variants不用窮知，_已處理；API／結果／用途足夠，無必要缺口，未實跑。

原始SHA-256：e10322624b7e0ffe28d464065a4088c077be18bea0926fae5142a1e7fa07c5c0；visible SHA-256：e10322624b7e0ffe28d464065a4088c077be18bea0926fae5142a1e7fa07c5c0。

### 003-004 — File資源／Read追加／讀取位置首次必要

Q1：File是開啟檔案的資源操作對象，open唯讀回Result<File>非完整文字，scope結束關資源；Read是File已實作的讀取trait，use才dot method。
Q2a：read_to_string(&mut text) mutable借File從目前位置到尾、追加合法UTF-8到String，回本次bytes；invalid UTF-8 String不變，其餘IO錯誤可能部分讀入。Q2b：「把讀出的文字接在既有前綴之後」→分開開啟／位置與目的String狀態→保留前綴並核新增量。
Q3：mut file、mut text，open與read各?沿5.6同型錯誤傳遞，Ok回(String,usize)；新增9、前綴10合19，失敗不把局部文字交完整成功。
Q4：assert全文與9bytes、輸出19明分兩量；「Err…不…所有外部工作從未發生」限制回傳失敗與外部效果。
未知／影響：讀取trait／mut兩狀態／scope資源已明說，無必要缺口；File OS內部合理不需。未實跑。

原始SHA-256：e892f98f15f9cf3343922ed69a5ae81d0be2d12e155a80cee7afe1200051211c；visible SHA-256：e892f98f15f9cf3343922ed69a5ae81d0be2d12e155a80cee7afe1200051211c。

### 003-005 — File建立清空、Write出口與讀回確認

Q1：File::create取得可寫File，不存在建、已存在清空；Write是出口trait須use。write!形式沿5.3，File出口錯誤是io::Result<()>，非Formatter的fmt::Result。
Q2a：mut file完成寫入，文字自帶LF、macro不加換行；helper return後File結束，另fs::read_to_string重開。Q2b：要保存同筆記再核保真→專用note-output不動輸入→Write後獨立讀回assert全文。
Q3：create?、write!?、Ok(())，caller先match寫結果再讀結果；讀回可單獨失敗，即使寫已成功。
Q4：「已存在就清空原內容」與專用輸出名先於code，新增效果界線清楚；readback文字完整對照。
未知／影響：Write借用細節未展開但已教mut與寫出口足用，不需另猜新macro；無必要缺口，未實跑或產出檔。

原始SHA-256：4f5a846e705e16b3bab826a795e110ee12a91a397def84af8ef6a01279893c08；visible SHA-256：4f5a846e705e16b3bab826a795e110ee12a91a397def84af8ef6a01279893c08。

### 003-006 — tests工作目錄／比較能力／example目標

Q1：Cargo package測試cwd是package根，fixtures指同材料；ErrorKind標準Debug／PartialEq可比，io::Error不具相等，完整IO Result不能直接assert_eq。
Q2a：先match成功／錯誤再kind比較；--all-targets執行各target tests，普通test預設不執行example中的tests。Q2b：要核完整文字、原因及新增量→固定只讀fixtures＋明選例子測試→不因位置漏掉Read追加檢查。
Q3：兩段同檔tests核全文9、NotFound／InvalidData、prefix全文19／新增9，指定all-targets；expect只對已知教材材料，外部保留match／?。
Q4：「單純的 cargo test 預設不會執行 example 裡的 tests」解釋新flag真正需要；新增比較限制在測試code前已說。
未知／影響：沒有必要缺口；各code位置只由相鄰說明知example target，本輪不開source，未實跑測試。

原始SHA-256：b6f78be862db9f270b0ba7a65bc5c63bf4d467cf633c6a646c537fcdff0dea3a；visible SHA-256：b6f78be862db9f270b0ba7a65bc5c63bf4d467cf633c6a646c537fcdff0dea3a。

### 003-007 — 7.3節末兩次讀取與錯誤保留

Q1：兩份文字owned結果各從fs::read_to_string取得，io::Result<usize>只在全成功交總bytes；沿5.6 ?同型錯誤傳遞。
Q2a：依序read?，第二失敗整helper Err，不交第一份部分長度；讀取已發生不因此撤回。Q2b：原需文字檔成功保真／失敗可處理→Result具體原因及caller政策→不能把錯誤改空String偽造成功。
Q3：note＋empty=9；note＋非法UTF-8第二次Err。各String as_str len＋Ok依已教操作可構成，最短推論不需closed答案。
Q4：「不要把失敗文字替換成空 String」支撐空成功與失敗區分；完整答案只marker，外連未開。
節末：整檔／File追加／Write／三測試新前提已教，無必要問題；buffer僅下節預告，未當已教／未猜。

原始SHA-256：81deee53fef3fa7c2ca724010be6cb5efd7a6d11d44ea17639129331fe8843e1；visible SHA-256：b3f7a8c1e123f2a415d047daf7442a28f7dea3c733b32ae0957f2b5175bad3a9。

### 004-001 — 分批bytes的原需與工作起點

Q1：文字全檔String與分批bytes不同；本節package ch07-io-bytes，cwd仍沿7.3相對起點。
Q2a：尚未教buffer/read具體操作，先要求以回傳數量定有效新資料，不能假設填滿。Q2b：「分批讀取…或處理不一定是文字」→要分新讀資料與舊buffer內容，不混成結果。
Q3：repo根cd examples/ch07-cli/io-bytes後，../io-text/fixtures/note.txt先上一層再進兄弟目錄，沿7.1的..可推。
Q4：「不能把 buffer 原本留下的內容混入新結果」清楚給下節問題；buffer概念目前合理待教。
未知／影響：分批API／何為有效段待本節正文，未需猜內部，無當下問題。

原始SHA-256：d0217c4201cd3ef2f9b8b5a91ecdf889b32a7d21aafbef009f682d76ee2d72e4；visible SHA-256：d0217c4201cd3ef2f9b8b5a91ecdf889b32a7d21aafbef009f682d76ee2d72e4。

### 004-002 — u8與buffer重複值array語法

Q1：u8是8-bit unsigned、0..255可存一byte，Copy／Debug／數值比較已供；buffer是暫時放讀取資料區，非自動有全檔。
Q2a：array／slice／Vec形狀沿1–3，改元素u8；值位置[0;4]把同Copy值初始化四格，型別[u8;4]指定形狀，mut允改。Q2b：分批讀取需要固定暫存區與可增長累計→四格buffer及Vec<u8>分别接本次／保存跨批資料。
Q3：let mut buffer:[u8;4]=[0;4]建立四零；借&[u8]讀、&mut[u8]寫，Vec new／push／as_slice沿3.6。
Q4：「分別指定初始值與資料形狀」排除把兩方相似語法混淆；u8能力也在讀取使用前交代。
未知／影響：新元素型別與重複array值足夠，Read將如何填／回量合理待下一單位，無必要缺口。

原始SHA-256：6b81a1777e03398b7b4fa551ec7628cba623881493033a6a10c698c81a91ce22；visible SHA-256：6b81a1777e03398b7b4fa551ec7628cba623881493033a6a10c698c81a91ce22。

### 004-003 — read有效前段、短讀／EOF／重試與原位圖

Q1：Read::read借可變來源及byte空間，io::Result<usize>成功n是本次新bytes，只有buffer[0..n]有效；n≤buffer長度。短讀是成功非滿，EOF是檔案結尾，Interrupted是一類可重試錯誤。
Q2a：n>0處理前段再讀；有限來源／非空buffer時Ok0結束本輪，零長buffer不可據0判EOF，檔日後追加也可能再有資料。Interrupted重試，其他Err傳caller，本次無成功bytes可處理。Q2b：原需分批且不混舊內容→回傳n限定有效前段→累計各批真資料，短讀不誤停、Err不偽造全成功。
Q3：假設ABC n3後DE n2，第二僅收DE不能DEC；持續處理而非短讀就停，最後Ok0才break，具體helper待後段。
Q4：首次原位SVG及native340×452實看：索引0–3第一列藍ABC／灰0，第二藍DE／灰舊C與0；同buffer重用，底ABC＋DE=ABCDE對應每次[..n]，不是累計內容保存在buffer。圖假設分批，不主張真File必交3／2。SVG SHA-256 3d08f577d1cee9743b01de0955a6d7ae301481560f3e51fd9ca6da3a637cd738；PNG SHA-256 4848ed1047a6a78adcac8dc1dc8c10f9d96bb467acc3745ef7353f41b6b2c6d6。
未知／影響：結果契約／讀取界線在操作前足夠，無必要缺口。局部圖非完整頁／未實跑，EOF不擴為所有來源永久結束。

原始SHA-256：78085784bccd8e8b119603a072aea8c558acf4bf14e1ac6c86d7b3d6f16c848d；visible SHA-256：78085784bccd8e8b119603a072aea8c558acf4bf14e1ac6c86d7b3d6f16c848d。

### 004-004 — read_all共用流程與完整UTF-8轉換

Q1：read_all<R:Read>(&mut R)借來源，結果owned Vec<u8>；四格buffer暫存、Vec累計不同。String::from_utf8接Vec驗完整UTF-8，Result錯誤FromUtf8Error非io::Error，Debug／Display已供。
Q2a：每次match n／Interrupted continue／其餘Err return，n0才Ok全Vec；for &buffer[..n]此處明說byte=&u8，*Copy後push，沿6.5 for及2.3 deref。from_utf8須完整資料，scalar可能跨批。Q2b：原需不混舊資料與不誤判分批中文→Read bound共用來源、先合bytes再驗文字→分開開啟／讀入／格式失敗。
Q3：File::open後Ok(mut file)pattern為binding加mut，read_all(&mut file)，成功9bytes再from_utf8得全文；255 bytes讀可成功但轉文字失敗。
Q4：「總記憶體仍隨輸入增長」限定buffer固定不等全流程固定記憶體；中三bytes可跨批給出先收後驗的用途。
未知／影響：for借slice的Item本段直接交代，最短推論是該來源提供shared逐筆能力，不需查新trait實作。無必要缺口、未實跑。

原始SHA-256：22107b14b10c2051bdf73a6c921e9b2b79496a220f838f2137034927809463c4；visible SHA-256：22107b14b10c2051bdf73a6c921e9b2b79496a220f838f2137034927809463c4。

### 004-005 — 可預測來源、新byte視圖與ErrorKind建錯誤

Q1：ShortReader owned data／下一position／正數max，Read impl依契約填buffer；as_bytes只借str UTF-8 &[u8]，ToOwned沿5.5複製bytes成Vec。另一來源step是事件次序非byte位置，PermissionDenied是操作不許的kind。
Q2a：while三界限保n≤buffer／上限／剩data，每筆同增copied、position，空／尾回0；io::Error::from(kind)標準From建只有分類的Error，slice is_empty測len0先避索引。Err無成功bytes仍可更新事件狀態。Q2b：真File批量不定→自訂最多3及中斷／後失敗來源→必然核短讀接合、重試及保原因，不改真權限。
Q3：ABCDE bytes view→owned data，read_all依3、2、0得完整5bytes／文字；事件先Interrupted、再1byte、再PermissionDenied，caller核kind與step3。
Q4：三界限code與前圖n3／2吻合；「沒有改動真實檔案權限」限定模擬證據。字面82對R未另計算，但是示例資料標籤，不影響流程理解。
未知／影響：新API的來源／借或owned／結果皆首次前已說，無必要未知。這是正文推演，未實跑以宣稱實際輸出。

原始SHA-256：612fc559277101ef64940d971cf7055c12e4cd5c3a1579ef000271f521e8bdef；visible SHA-256：612fc559277101ef64940d971cf7055c12e4cd5c3a1579ef000271f521e8bdef。

### 004-006 — Write部分成功、write_all與flush完成位置

Q1：Write::write接&[u8]回已寫n，可短寫；write_all回unit結果、負責全寫；flush是另必要method推中間緩衝到出口，非永久儲存保證。
Q2a：write_all重複write並重試Interrupted，其他Err返回、非空write0為WriteZero避免無進展重試，已寫不自動撤回。ShortWriter只實作write及flush、繼承既有write_all。Q2b：原需全bytes不漏／不重→一次成功只處理前n，再把後段交write_all→核全部完成，而非成功一次就停。
Q3：ABCDE先write2、input[written..]剩3再全寫、flush；本memory出口無緩衝直接Ok，owned Vec轉文字assertABCDE。
Q4：表格與「尚未寫入的後段」解釋slice從n起而非重給全input；示例是記憶體非新檔案，讀回文字只核不漏重。
未知／影響：新增trait必須提供／已提供methods在code前明說；flush完成界線足夠，無必要缺口，未實跑或驗耐久性。

原始SHA-256：98a3d55ca510d00dc57a2b35b6f33ffe54ecf0d5aed81fa2705964a82a9e109a；visible SHA-256：98a3d55ca510d00dc57a2b35b6f33ffe54ecf0d5aed81fa2705964a82a9e109a。

### 004-007 — 短讀寫契約的控制材料tests

Q1：三ShortReader／兩ShortWriter／一error_reader tests各檢查已教契約，同檔及all-targets沿7.3。
Q2a：手動read3／2／0核舊C；不同max含8仍受四格buffer限制，read_all後才from_utf8；空成功、全寫不漏重、零進展WriteZero。Q2b：可控制來源／出口避免真File批量與使用者權限偶然差異，讓邊界必然被測。
Q3：已教struct construction、for、match、expect／assert足讀全部tests；command預期六項，未實跑。
Q4：「不依真實檔案…或目前登入者權限」限定材料用途；buffer[2]=67具體保存圖中舊C，不算新資料。
未知／影響：無必要新概念／缺口，error_reader一測試只由正文敘明未另開source；未宣稱六項實際通過。

原始SHA-256：31161bee5db853be1de8ad494e8e85a71ed7182767b492d84251f50696c075ca；visible SHA-256：31161bee5db853be1de8ad494e8e85a71ed7182767b492d84251f50696c075ca。

### 004-008 — 7.4節末只統計的關係

Q1：count_bytes借Read來源與四格buffer，只累計本次n，沒有保存全內容Vec；bytes成功不等UTF-8文字成功。
Q2a：沿read_all n／Interrupted／其他Err政策，n0才Ok總量，改push為加總即可；此處限定已有小fixture驗證不需新語法。Q2b：只要長度→保留有效量而不保存全資料→避免本節read_all隨input增加的結果Vec。
Q3：note9、empty0、非法UTF-8單byte1仍成功，因未from_utf8；最短推論來自read契約與既教加總，closed答案只marker。
Q4：「這次只數 bytes，不要求資料是 UTF-8」解釋與7.3結果差別；新路徑／arguments只是下一節預告。
節末：有效前段／短讀／EOF／中斷、全bytes再驗UTF-8、短寫與flush皆已教，無必要問題；資料累計與固定buffer界線清楚，未實跑／看整頁。

原始SHA-256：b3dc6645cae5e4428f04ce06ed3c4dcb99eb3ab7879abd457eb27a678e1ae36a；visible SHA-256：d14061d967e43bc624b3d25cf8b6baa1be8b556d09ffc806a328086696059991。

### 005-001 — caller給路徑／資料與診斷的原需

Q1：caller指定檔案，檔名的OS路徑表示與工具內容UTF-8要求是兩件事；ch07-path-cli cwd固定。
Q2a：先預告原生路徑／字串，再接6.5 Iterator及7.3–4 IO，尚未必要新API。Q2b：「成功就交出原文字，失敗…原因與結束狀態」要求使用者可指定來源並由結果區分資料／診斷。
Q3：repo根cd examples/ch07-cli/path-cli，後續command以此起點，沿7.3不猜src目錄。
Q4：「檔名是…路徑，檔案內容才是…UTF-8文字」清楚定這節拆分的需要。
未知／影響：原生字串／CLI取得／結束狀態機制合理待本節教，未提前補。

原始SHA-256：32d2c6e5cad3abba1ae6dd1c8fa7ce828ad8944c1e40e1142829d6599c9ed24a；visible SHA-256：32d2c6e5cad3abba1ae6dd1c8fa7ce828ad8944c1e40e1142829d6599c9ed24a。

### 005-002 — Path／PathBuf與顯示／UTF-8借用首次必要

Q1：Path是路徑內容、本節&Path借，PathBuf owned可修改，分工類str／String；都不是File或已建目錄。std::path::Display是借路徑的格式化物件，異於Display trait／owned String。
Q2a：Path::new借既有文字；join shared借並回新PathBuf，push mutable改回unit，as_path借view。平台分隔／絕對片段可替換，不一律字串追加。display可能失真僅人讀，to_str→Option<&str>表示可否UTF-8，None非路徑不存在。Q2b：原需檔名OS資料→保原生owned／借用路徑，顯示／文字轉換另選→不靠失真訊息重建檔名。
Q3：notes借→join2026新owner→push note.txt→as_path，原notes不改；用display顯示，match to_str處理不可轉。Debug／PartialEq比路徑組成，不查同一實檔。
Q4：「操作結果沒有查詢實際檔案」與三段結果限定例子只核表示；表格給每個新method receiver／result，需用之前已教。
未知／影響：不能Unicode的實際平台例子尚未展示，但None契約足用，內部encoding不必要；無缺口、未實跑跨平台。

原始SHA-256：facf0c86551a1e93f4f25fd7e2036423ce3ac03254b81808bce1662779167a80；visible SHA-256：facf0c86551a1e93f4f25fd7e2036423ce3ac03254b81808bce1662779167a80。

### 005-003 — OS字串原生資料／UTF-8轉換與ownership

Q1：OS作業系統；std::ffi交換外部系統型別，OsString owned原生字串、&OsStr借，非必UTF-8 String；OsString無Copy但Clone，Debug／PartialEq已有。
Q2a：From合法&str一定建OsString；as_os_str借，to_str Option借UTF-8；into_string consumes→Result<String,OsString>，失敗交回原owner不是錯誤訊息。PathBuf::from接原生owner不先轉UTF-8，Path as_os_str另借原生view。Q2b：原需路徑保原生→借／owned轉文字皆顯式可失敗且保原資料→非UTF-8仍可交PathBuf做IO。
Q3：先借中文native顯示／match to_str再move進PathBuf；另Rust owned into_string match成功String／Err原生owner。
Q4：「不能只依這個成功例保證所有原生檔名都能轉換」限已合法材料，表格首次明示六API及ownership。
未知／影響：具體非UTF-8 OS案例未驗但非必要操作前提；機制與界線足夠、無必要缺口，未實跑。

原始SHA-256：0701d50d054229d840e0a2232b646ee5e2e575c9f72a6c3b9818ad43a8ffb426；visible SHA-256：0701d50d054229d840e0a2232b646ee5e2e575c9f72a6c3b9818ad43a8ffb426。

### 005-004 — current_dir與AsRef<Path>真正IO

Q1：std::env::current_dir取行程cwd，回io::Result<PathBuf>，非src／執行檔目錄；read_to_string也接受&Path，其AsRef<Path>角色沿5.5低成本借用。
Q2a：借owned cwd join相對檔名，join不自行消..，IO才由filesystem解讀位置；原生Path不需轉String。Q2b：固定cwd規則變可查的API→caller得到真正起點再組位置／讀→分工作目錄取得錯誤與檔案讀錯誤。
Q3：match current_dir，directory.as_path().join ../io-text fixture，read_to_string(filename.as_path())取得原筆記；cwd顯示因repo存放不同。
Q4：「join 不自行清除 ..」配合本例避免誤把表示組合當查檔／正規化；全文assert仍沿已讀IO。
未知／影響：AsRef接受此&Path由本段明說，不自行補內部泛型簽名；必要背景足夠，未實跑／實查cwd。

原始SHA-256：9c85cb6642c2cc51821155b63801910adab494b574da83dde123b4142d17a57f；visible SHA-256：9c85cb6642c2cc51821155b63801910adab494b574da83dde123b4142d17a57f。

### 005-005 — args_os／純參數parser／底線binding首次必要

Q1：CLI是參數／輸出／結束狀態介面，args_os建ArgsOs Iterator交owned OsString；parse_filename接完整IntoIterator<Item=OsString>而非自行取OS。ArgumentError明分缺／多，既教derive／Display。
Q2a：mut iterator next先略程式名，再取filename、再判多一筆；_program_name名稱表刻意不用結果，仍執行推進。args_os保原生，args遇非法Unicode可能panic；第一名不保證實執行檔位置。Q2b：caller需恰一檔名→保原生與明確次序檢查→少／多不先IO，不收Vec冒索引越界。
Q3：成功OsString move成PathBuf；固定owned array供三結果tests，Result比較沿5.1兩側PartialEq／Debug已供。由6.5 for建IntoIterator及本例array可交來源作短推論，未查額外實作。
Q4：「沒有啟動外部 process」限定兩tests僅參數規則，不算stdout／stderr／狀態已驗；parser公開依賴與owner移交清楚。
未知／影響：stdout等僅接下段預告，當下API與次序足夠、無必要缺口，未實跑。

原始SHA-256：8f0a7384199b79104fc462360c6864c10ef333306beb0e849f6fbeadc6ba4af5；visible SHA-256：8f0a7384199b79104fc462360c6864c10ef333306beb0e849f6fbeadc6ba4af5。

### 005-006 — 標準出口、ExitCode與診斷helper首次必要

Q1：stdout交工具資料、stderr交診斷，兩終端可同顯但caller可分存；Stdout／Stderr物件提供Write。ExitCode是main正常回交啟動者狀態，非錯誤payload。
Q2a：SUCCESS／FAILURE associated固定值不用call，from(u8)建指定2；main各分支同ExitCode、正常scope清理，process::exit立即停略清理。writeln!出口格式沿write!且加LF，io::Result<()>，report_error stderr失敗返FAILURE不遞迴報自己。Q2b：原需成功交原文字、失敗原因與狀態→資料／診斷／ExitCode分管→caller能單存資料並判失敗，仍正常結束資源。
Q3：parse錯先to_string報2；讀&Path錯format display僅診斷FAILURE；成功text as_bytes write_all至stdout，無標題／多LF，write錯報FAILURE。
Q4：「操作真正使用的是原生 Path」與三路徑code支撐不以失真顯示重建來源；每個helper輸入／結果由前文或此段提供。
未知／原判：無當下流程必要缺口。可選O1（出口完成位置）：7.4說flush推中間緩衝且「依輸出型別的契約理解完成位置」，此段僅說Stdout／Stderr提供Write，未說是否緩衝；可由code判write_all／writeln接受成功後的狀態，不能從已讀正文判省flush對出口失敗回報的界線。可最小補一句兩出口完成位置／是否需flush，並非讀分支需猜答案，故初判optional，未實跑。

原始SHA-256：58bc881f617a32dc23d2aec5e49b283f55d9bb7fd33ef4ccfca44623d24bf9d6；visible SHA-256：58bc881f617a32dc23d2aec5e49b283f55d9bb7fd33ef4ccfca44623d24bf9d6。

### 005-007 — Cargo參數分界／shell分流與狀態觀察

Q1：Cargo --後交工具arguments，--quiet減Cargo例行訊息不關工具出口；Bash >stdout／2>stderr可分保存，$?上一指令狀態。
Q2a：重導向會清空既有輸出檔，僅專用captured名；echo $?需立即接run，PowerShell用$LASTEXITCODE。Q2b：原需caller分辨資料／原因／成功→分流兩出口＋立即查狀態→後續工具不把診斷當筆記。
Q3：正常note終端兩行，Linux成功0且data原文／error空；missing1且data空／error原因，缺參數2；非法內容失敗非原生檔名失敗。
Q4：具體command與Linux結果對上前unit分支，未把跨平台數字當全保證。
未知／追加：本段正常capture觀察不解釋Stdout緩衝／flush錯誤完成位置，O1保持optional初判；無必要缺口，未實際執行、capture或查status。

原始SHA-256：795368100ebcde32b4f30050d851cbb068fdcfb25c68b644a4f5e7de272958ce；visible SHA-256：795368100ebcde32b4f30050d851cbb068fdcfb25c68b644a4f5e7de272958ce。

### 005-008 — 7.5節末原生兩檔名練習與prefix結束

Q1：parse_pair只處理完整參數序列，兩owned PathBuf成tuple或PairError，非讀檔／文字轉換；錯誤variants沿既教enum。
Q2a：略名後兩次next不足早return Missing，再一次檢查多參數，OsString按值交PathBuf；match核類別不加Display。Q2b：原需caller指定來源→原生資料一路保留、參數規則與IO／資料／診斷／status分開→可改數量仍不混成UTF-8要求。
Q3：由單檔parser增加第二next及最後多一檢查即可最短推得，不看closed答案；中文合法、缺第二、多第三三材料可驗owned結果／原因。
Q4：「不要轉成 String，也不要開檔案」限定純parser工作；完整參考答案只marker，外連未開。
節末：Path／OS表示、cwd／args iterator、標準出口及ExitCode的原需與操作關係足夠，無必要缺口；O1仍只是可選出口完成界線說明。本prefix至此實讀34單位，7.6第三方API僅預告、不使用／不猜後文。

原始SHA-256：838f8c7b8388e9809cecfcdb5ba14950c466d7bd2d45cb5c3d1433a5cc45a856；visible SHA-256：9f5111f817526940051faa5380c1f4b904114084ded0f54805029d289c39923e。

## Prefix初判封存

實讀7.1–7.5全部34單位；每個單位的原理解及原始／visible hash保存後才續讀。首次原位buffer-prefix.svg實看本體及native局部渲染，沒有看完整頁面；003-007／004-008／005-008三個closed答案只marker，未看其內容。沒有誤見7.6以後／live／作者來源摘要／完整source／其他第7章review或網路答案。

必要初判問題0。可選O1僅005-006的標準出口完成位置說明：已有7.4 flush契約，但新Stdout／Stderr只介紹Write，尚未說其緩衝與省flush的錯誤回報界線；補最小出口完成說明可接兩段，不妨礙目前依code理解main分支，因此保留optional，不升為必要。005-007正常capture未釐清此點，追加而未覆蓋初判。

本輪理解核對範圍包括Cargo來源／功能／版本與lock、doc／外部tests、完整文字IO與錯誤分類、有效bytes／短讀寫／模擬來源、原生路徑／arguments／資料診斷與ExitCode；各首次新概念四題及節末關係在上文。副本操作／fixtures實測／Cargo結果／跨平台／全頁版面皆未驗，不把正文預期或局部SVG當實測通過。

獨立prefix範圍指紋（reading-order＋34原始unit＋實讀SVG，共36項）存work/ch07-prefix-v9-raw-scope.sha256；該檔SHA-256：45c2c988fb4fba397b60bbaf89ec80a533ebcac2f5da665460486fd03587e32c。遮蔽metadata ledger SHA-256：c4d827e413f26ced16b094a169e8a09e317e21718eaaef803f4699cd6c64c7d5。原始單位hash包含被遮蔽bytes的機器計算，沒有讀取其答案作理解。背景entry原SHA保持5a09c8f5c308a2a77ad95c688099e0cf27265a09efe080e6709b209c78746c43。

至此停止，等待root另給7.6–7.9實際尾段；本prefix不推測whole7已完成，也不先搜尋後文。
