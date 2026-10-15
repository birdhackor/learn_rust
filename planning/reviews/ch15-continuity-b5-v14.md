# 第15章 B5 ORIGINAL 銜接初判：15.11，v14

本輪結論：**必要問題2項（N1、N2）、可選改善1項（I1）**。主要報告映射、取消理由、CLI設定與輸出責任已有充分銜接；待修是新增 Clippy flag 未教，以及實頁將範例材料的前導空行省掉。這不是產品 Rust 執行失敗，也不是整章 cross 或發布驗收。

## 角色與固定範圍

- author04，非第15章作者；沿自己已封存的 A42／B-prefix22／B4九單位實際閱讀背景，另有第4／14章部分作者、第11章技術審閱背景。是 ORIGINAL CONTINUITY 尾段延伸，**不是無背景 FIRST**。不以自己的專業知識或先前 gate 結論代替學生先備。
- 唯一新稿：`/workspace/work/tutorial-implementation/frozen/ch15-b5-prerequisite-v14`。一頁 **9 BODY、33 archives**；正文含全部提示、完成參考與原位 SVG。兩個完整 package 的 README、fixtures、所有 Rust、test helper、checks、manifest 均納入。
- 初始 manifest SHA：`06e8691b10acfa1303a84a8ca6ff907ec8f275f083613ddc4729950e87826bc5`；完整 BODY／source 指紋清單 SHA：`eea93c802d12b3d345cd1c814dfe18fdffcf99bd25d7fc43d14fbb3587dd08f6`。開始閱讀前已保存，封存再核。
- 33 archives 共 **27組不同完整 bytes**：25組非 lock 全部實讀，組內相同整檔先核 SHA／完整 bytes 才對映。complete report／app 的新 prefix 實讀，完整 tests suffix 與同輪已 FULL 讀的兩份 checks 精確相同後才對映，並非抽樣。
- 兩個 generated lock 完整 TOML 機器解析，**不冒稱逐行 FULL 閱讀**。各183個 package，182個 registry source 且有 checksum；唯一無 registry source 的是各自本機 package，沒有外部 path package。八個直接依賴選定版本與 manifest 一致，詳見 `generated-lock-analysis.json`。
- 未讀任何15同行（含前階段）、Root outcome／disposition／proof、作者 notes／ledger／handoff；未做 cross。未執行 Rust、作者 tests、真 binary／SIGINT、公開 HTTP 或舊 B1–4 suite。自有 preview 與瀏覽器檢查另外列出。
- 舊原判未改：A SHA `4a99ca927223e9bdc5e46b92ccf13cc03cb489f1f1615fb83bbd880e690b52ff`；B-prefix `7fa44c40873c102f14b044a0cc75c3fef7bb1cc9721a4bf0281079a842c82cf2`；B4 `e8801ab621c65f3119ac8fc14308dffce98e64e5b1b620fe265dcb5eb8721810`。本輪不重新裁定舊 N／I／unknown，也不假定舊問題已修。

## 新增必要問題與可選項

### N1：第一次使用 `-D warnings`，尚未交代這項新檢查規則

- **位置／原句：**固定 `001-009.md:27`：`cargo +1.99.0 clippy --all-targets --locked -- -D warnings`；同 source 的 `url-checker/README.md:26` 也有此命令。starter README 沒有這項命令，不列入受影響範圍。
- **當時已教：**1.10實際正文45–57已說 Clippy檢查常見問題、`--all-targets`的目標，以及 fmt 的 `--`轉交後方選項；15.1已教選用固定 toolchain，7.1已教 locked。因此並非整份命令陌生。
- **缺少關係：**本次新增 `-D warnings`，卻沒說它把本次警告視為錯誤，會改變檢查的通過條件。實際學生正文1–14限定搜尋沒有此 flag 的既教位置，本頁也未在命令前補最低意義。不能把知道一般 Clippy，直接當成知道這個嚴格設定。
- **影響／分級：**屬先備／操作規則的**必要理解負擔**，不是核心演算法阻礙。讀者只能照抄新的檢查政策，無法由既教內容判斷為何警告變成失敗；使用者要求命令與範例先教後用，這裡需最小補齊。
- **最小修法：**刪掉額外的 `-- -D warnings`，保留已教的一般 Clippy；若要保留嚴格政策，則在命令前用一短句說明 `--`轉交 Clippy、`-D warnings`把本次所有警告視為錯誤，README保持相同約定。無需擴寫 lint 章。
- **時點：**全 BODY／source 後自行回查必要前文時發現，先保存於 early notes，後才做實頁；沒有同行提示。未用實際執行 Clippy 的結果證明這項介紹缺口。

### N2：實頁的同名材料少了前導空行，原索引與答案對不上

- **位置／原句：**固定 `001-009.md:7` 稱隨附 `fixtures/invalid-urls.txt` 是真材料，明說「原index0是空行，1是ftp，2只有空白，3是`/notes`，4再是相同ftp」；緊接9–16行的 text fence 展示材料，23行預期1／3／4。
- **當時已教／正確部分：**B2已教先 enumerate全部原行、再略過空白，最後LF不增加輸入。實際 fixture42 bytes 確實以前導LF開始，保留索引1／3／4；文字說明也正確。原 frozen fence保有同樣前導LF與單space行，只比 fixture 多一個尾LF，不影響保留索引。本項**不是 fixture 或程式解析錯誤**。
- **缺少關係：**同CSS／相同Markdown設定的實頁 text code block省掉了前導空行，空白行中的space也不再顯示。DOM code的 textContent以ftp開頭；我另在同固定 preview真正點 copy，隔離 browser clipboard得到 `ftp://host/item\n\n/notes\nftp://host/item`，此材料的非空原索引為 **0／2／3**。它和本頁承諾展示的原材料／答案不同。
- **影響／分級：**屬素材呈現與答案對應的**必要理解負擔**。直接使用隨附整份目錄仍會得到正確材料；但讀者面前的同名展示與可複製內容，卻支持另一組原索引，必須自行調和。文字已交代正確規則，保留其效力；不能因此把相鄰展示的不一致當作無影響。
- **最小修法：**可直接刪掉這個被格式化改變的材料 block，保留原有0–4文字對應及完整 fixture；或改為有原索引、明示空行／空白行的材料表。若仍提供原始可複製區塊，需核實 render與copy真的保留首空行。無需修改 Rust、重跑HTTP或新增讀者測試。
- **證據／時點：**自己的 `usable-answer`／`strict-command` 雙裝置原位畫面先發現，才以 `visual-detail-proof.json`、`fixture-source-match-proof.json`與 `fixture-copy-actual-proof.json` 核對。第一次 copy locator用舊class沒找到按鈕，保存為未完成 attempt；後用實際 `data-md-type=copy` 按鈕點擊取得 clipboard，才算成功操作證據。沒有把 raw fence43 bytes誤算成與42-byte fixture整檔 exact。

### I1（可選）：手機把 long 參數的連字號與名字分到不同列

- **位置：**固定 `001-004.md:5` 的 `--timeout-secs 5`，自有 `cli-intro-mobile.png`，390×844；實際 inline code分成兩個 client rect，前一段在列尾，名稱接到下一列。其他長 inline 命令也有自然折行。
- **已有／收益：**7.6已教 long 名稱的兩個連字號，Settings code、README和後面的完整命令均可核對，因此不升必要缺口。讓這個短參數保持同列，可減少照著閱讀時重新拼接 literal 名稱的負擔。
- **最小改善：**僅讓這類可容納在手機寬度內的短命令／參數 token 不拆字；不要求整段 nowrap，也不要求消除所有中文換行。這是自身實頁新增可選項，與 N1 的新 flag 意義、N2 的材料改變不同，不合併成同一缺口。

## 主要方法與未報問題通過項

以下從正文重建原需／機制／操作／用途／例子界線；「足夠」只指閱讀證據，不是我執行後的 PASS。

| 原需要與首次位置 | 當下機制、操作及真正已教前提 | 用途與例子支持範圍 |
| --- | --- | --- |
| `001-001–002`：讓別人用網址清單，並由退出碼區分全過、項目未過、取消、fatal | 先完整列0／1／130／2政策；完成 write＋flush 才能交資料狀態，來源失敗不冒Requested。B4已教停止、分類與owner收妥，7.9已有資料／診斷分流與出口確認。 | 原需先於設計。fatal2蓋過輸出前候選狀態、Requested蓋過一般項目失敗，各自有工作含義；不是只列數字。部分bytes可能已送出也有當下用途：不能把失敗stdout當完整JSON。**足夠**。 |
| `001-003`：同Batch變可交換的owned model | 先定Row／Report的原index、original、kind與各Option意義；Requested才令整批cancelled為true。新教u16／u128 JSON整數、Option Some／None→null、str Serialize與compact to_string，均在相應code前；Serde derive／deny_unknown_fields來自實際7.7。 | 映射隔開網路錯誤型別與可交換資料，沒有假設Error本身可序列化。六份手工材料保duplicates及空洞，區分HTTP200／404、Input、Timeout、Cancelled、Unstarted與缺量測；手工毫秒明說不是實測。資料與用途已連起，**足夠**。 |
| `001-003`：文字與JSON借同Report | JSON用pretty表示，文字用to_string引用original／final_url／detail，避免雙引號或LF破壞一筆一行；數值Option保數字／null。7.7已教格式跳脫與往返，7.9已教String::push LF。 | 人讀與程式交換共享一個model，免維護另一套資料或手寫跳脫。codec手工detail有LF，JSON讀回全模型，文字fold只計行並查跳脫；不以詳細診斷字串當分類規格。**足夠**，不把合成detail當真HTTP失敗。 |
| `001-003` RunError、`001-008` explain／public library | 新錯誤保設定、Read／Client、Signal／Closed／Controller、Encode／Output；Display與source分工沿實際8.6／14.4。From io只放Output，讀取處明選Read。non_exhaustive與private module／root pub use，14.4實際正文有完整角色和操作。 | typed錯誤供分類，Error source借原原因供人讀；String型訊號沒有捏造source。explain沿既教loop／match逐層顯示，非未知Any／downcast黑箱。**足夠**；不宣稱所有底層診斷固定。 |
| `001-004`：數值CLI設定與完整起點 | 實際7.6已教Parser、long、PathBuf、try_parse_from、help／version的Err與use_stderr／exit_code。當下新增整數參數跟值、usize／u64範圍、default_value_t的typed值／ToString角色，並明確0另由程式先驗。 | default不冒Settings Default，解析整數不冒有效正數。先保存完整manifest／lock／src／checks／fixtures，再依步驟修改三stub；不是要求從snippet拼全部工具。**足夠**。N1另列，不能用此項放行未知工具flag。 |
| `001-004`：內置Fixture使整目錄可獨立保存 | 全部axum／Tokio操作回到實際11.8：State、route、IntoResponse、IntoFuture、bind動態port、短持Mutex、atomic及兩方owner；新ready_response在code前教Response::text接手Response、完整body／Result與外層十秒觀察。manifest供dev axum，沒有外部path。 | 只驗固定小ready字串；產品single仍headers-only，沒有暗中擴成通用Web教學或產品讀body。ready失敗先shutdown，已有本機owner也等完。**足夠**；我未實跑ready／server。 |
| `001-005–007`：循序自己完成三個動作 | build_report保來源fatal，再map每份Entry；empty_row／row說明是自定helper，指定要加的imports；status先cancel再查passed；write_report先render、write_all與flush最後才status。6.6已教map／collect／fold，9.3已教partial move，故移check.original後仍能處理別欄位。 | 九checks進度3／6→5／4→6／3→9／0對應三動作。純Row與writer材料先說用途、cfg module整份追加；規則在閉合提示／答案前已教。這是引導練習，完整參考另可取得；**銜接足夠，數字只按來源與邏輯核對，未重跑**。 |
| `001-007`：已收bytes仍須查flush | ControlledWriter兩bool分拒寫／拒flush，bytes與flushed角色在code前說明；Write／PermissionDenied既有7.4／7.9前文。錯誤以同型From交Output。 | 拒寫應無bytes、未flush；拒flush應已有bytes、flushed為true，仍Err；成功才130且JSON能讀回同Report。材料使出口失敗覆蓋取消的必要關係可見，**足夠**；沒有拿一次成功寫入冒完整出口。 |
| `001-008`：真入口／可用library保同責任 | run先validate再read，再建立client／未spawn的Scheduler；listener entered後借owned Scheduler run，正常／Requested都finish listener再建報告。check_text先驗正數，借caller client、取得owned Report；明說caller維持stop sender或Requested後await。 | 兩入口共用工作與驗證；來源／controller故障仍fatal，drop整份future不冒已等待children。main的help／version／診斷也走同write_output確認flush，正常返回ExitCode。**足夠**；不承諾任意取消下自動async清理。 |
| `001-009`：取得可獨立使用的真版本 | 工作目錄先選package，--manifest-path不替程式換CWD；release沿實際9.1。新install在使用前說--path .選本機package、安裝binary目錄、不上傳registry及locked角色。 | 隨附invalid資料不送HTTP，真網路結果依環境，jobs只限已提交tasks，不限制整張清單／結果記憶體。核心825／tests898／Fixture198的計數可由固定來源分類核對，三個新練習不要求全手寫1921行。**操作說明足夠，N1／N2例外另列**；安裝及CLI行為未由本輪執行。 |

主線的最短因果：外部caller需要相同資料與可靠的完成狀態；先把typed Batch交為owned Report以固定交換欄位，再由同Report形成兩種表示，最後將編碼、write與flush的失敗保留為fatal，才可以交原資料狀態。這些需要由`001-001–003`明說，不是從新方法產物反推。若只算出130卻沒完成輸出，原問題仍未完成；ControlledWriter正對應這個缺口。

## 全來源與有限證據的界線

FULL新來源包含兩完整起點／答案：starter各module、complete725行batch（含全七項排程tests與所有helpers）、兩份checks共362行、兩main、兩份config integration test來源、兩README、manifest、fixture與SVG。重複六組整檔核完整SHA才映射；complete report／app 的209／153行tests suffix也核exact，沒有用後續作者檢查摘要代替。

完整答案來源有16項library tests（batch7＋report5＋app4）、config integration2，與正文數量相符。合成panic只驗TaskFailure資料，不冒實際HTTP；panic task以五秒borrowed handle guard觀察，失敗abort後await才panic。原B4 finite controller／snapshot-before-backupdrain／收client後server的材料仍在complete source，普通HTTP混合test另將同Batch交map驗http／input／request。這些是所讀的source覆蓋，不是我已實跑18tests或真CLI／SIGINT。

實際回讀7.1、7.6、7.7、8.6正文、14.4必要部分及對應Error／pub use source；局部回讀1.10、6.6、7.8、9.1、9.3與11.8所需角色及操作。更早已實讀B1–B4與7.9等沿自身原紀錄，沒有將整份未重讀前文冒稱本輪全章FULL。逐檔／實讀範圍與當時SHA見 `prerequisite-proof.json`；N1搜尋只對學生1–14正文，不讀任何review／作者資料。

僅相稱查固定正式registry source：reqwest0.13.5 Response::text的消耗／UTF-8與錯誤；serde_json1.0.154 compact序列化、str escaping、Option null及u128；clap_derive4.6.7 typed default生成。這些是本機正式cache實讀，不冒fresh網路研究，也沒有執行其中範例。未為一般教學文句跑Rust或全套測試。

## 自有實頁、圖與折疊

隔離preview僅由本輪9 BODY按原序組頁，去掉凍結拆單位的重複頁名；code、段落與fold不改。原SVG同bytes轉相對URL；前文連結僅preview重定位、未點外連。theme／Markdown與實際站點設定相同、CSS經HTTP200取得並核同SHA、noTop。strict建置無問題；此事與視覺驗證分開。

Chromium **親看24張原PNG**：1280×900及390×844各12張，含SVG上／下及圖說、兩張表、CLI新參數、writer機制、閉合的三提示／答案、開答案、library入口、真版本使用、嚴格命令及install。所有擷取點無整頁水平溢出，Top數0；長code只在自身容器水平捲動，未假稱每行都在初始viewport可見。N2另做同固定頁的DOM／真正copy小操作，不是新增Rust執行。

- SVG藍箭頭追同一Batch→owned Report→借Report選表示→write_all／flush→完成後0／1／130；右橘線將來源、編碼、出口問題交fatal2。沒有把render String當真正出口。上／下兩視圖將整圖、圖說與接續render理由對齊，必要字線不用放大可讀。
- 材料表對index1／3／4／6／8／9與保留欄位，policy表對工作／結果。手機換行後仍能逐格追蹤，沒有發現另需擴寫的表格概念；I1是短參數token，而非所有自然換行。
- hints及完整答案預設關閉；主要規則、checks如何追加、三步操作與結果限度都在開答案之前。實際開答案能定位完成碼；沒有用fold中的實作補當下先備洞。
- N2是在實際載入相同Markdown設定後才顯現的素材差異；原始source正確不能替代此實頁核對。保留raw fence／fixture／DOM／copy各自bytes與索引，沒有將它們誤稱整檔相同。

上述是代表性必要元素與過渡的原位檢查，不是每個捲動點、深色模式、所有連結、整站或真CLI顯示驗收。未修改學生／共享設定／index，也沒有借用Root或作者畫面當作自己親看。

## 封存與停止

本輪必要 N1／N2待協調者處置，可選I1保留其較低分級。沒有以通過的大部分主線掩蓋兩個局部缺口，也不把工具文句問題升成整個專案不可用。作者聲稱另做的真binary／受控服務／SIGINT檢查僅保留為**正文主張**，我未讀作者結果，故不替它背書；執行與發布均未由此角色驗證。

自有工作目錄：`/workspace/work/tutorial-implementation/review-ch15-continuity-b5-v14-author04/`。保存initial scope／學生SHA清單、early notes、33來源coverage／27組整檔映射、兩lock解析、byte alias、前文與官方selected source proof、N1學生限定搜尋、preview source／DOM／build、24 PNG及其親看紀錄、N2 raw材料／實際copy證據、行數分類。報告另存same-byte `.initial.md`／SHA sidecar與seal proof。封存後停止，等待Root另授cross；本輪沒有先看任何15 peer或處置。
