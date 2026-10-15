# ORIGINAL CONT15 B-prefix：15.7–15.9 / v12

本輪完整讀三頁22 BODY、33 archives，確認**必要1項（N1：helper首次使用match guard未先教）、可選1項（I1：手機表格拆開variant名稱）**。主要專案銜接成立：一原文一結果→原行索引與依序批次→有限task、逐份收取與故障對應。此結論只限B1–B3原始v12，不是B全篇、整章或Rust執行驗收；N1需在首次使用前補足。

## 角色、隔離與固定版本

我不是第15章作者，曾寫4.1–4.3、14.1–14.3及14.6，並有前章審閱背景。這是有背景的非作者銜接初判，不稱新盲讀或真人基本Python讀者研究；自己的專家知識不算學生已教。

唯一新稿是 `/workspace/work/tutorial-implementation/frozen/ch15-b-prefix-for-review-v12`：15.7 B1七units、15.8 B2七units、15.9 B3八units，共22。manifest SHA `18b90d116556eea43dd67b5a996b6decc17ac4b8235a764ee821f4b42116b25f`。全部22 BODY／33 source實際bytes指紋清單SHA為 `7e11e3278257c07b72fade17c02f1d9366d0f9bfaa22b1bc8a06585fe50ccf02`：按manifest順序列BODY檔名＋SHA，再按source path排序列原路徑＋SHA，各列LF。own `student-files.sha256-list` 可重核。

沿clear-tutorial／review-protocol／術語指引，先順讀正文並保留早期未知，再核來源／必要前文／selected官方契約與自己的實頁。完整提示及答案也補讀，不能冒稱全程closed的FIRST路線。未讀第15章任何peer、作者notes／ledger／handoff／Root gate／outcomes，未開15.10／15.11或B4／B5，亦未自行cross。A原判仍為 `4a99ca927223e9bdc5e46b92ccf13cc03cb489f1f1615fb83bbd880e690b52ff`，沒有回填或改字。

沒有執行Rust、作者套件、公共HTTP、無界例或故意UB；沒有修改student、shared Cargo／nav／lock、原判或Git index。下列tests指**已全文讀的來源判準與預期**，不是本輪親跑PASS。

## 全文覆蓋與真前文

33個archives包含六份literal manifest、全部starter／complete lib與modules、checks、三個examples、六份README及三SVG。按完整SHA有29組unique whole files，合計106760 bytes；每組先真正FULL讀完，才以相同完整SHA對映其餘位置。只有四份single.rs合成一組、兩份input.rs合成一組；不同完整lib中重複的checks仍各次全文讀，沒有用片段相同代整檔相同。`source-full-read-coverage.json`逐path記代表檔、SHA、組別及模式，`unique-sources.json` 29組均full_read；BODY22單位亦逐一保存。

自己的 `early-body-notes.md` 按B1／B2／B3補記當時可說明的原需、機制、限制與未查範圍；`early-source-notes.md` 保留來源、回讀、N1從候選到確認及I1實頁發現時點。B1時先待查時間／錯誤能力，B3時先待查guard語法，不把後續確認回寫成一開始已知。

實際必要前文以現有正文的具體段落回讀，不以章名放行：

| 當下需要 | 真正已教的位置／最短契約 | 本輪使用與界線 |
| --- | --- | --- |
| spawn與兩層錯誤、Id／AbortHandle／remove、mpsc | 11.2 1–135行：spawn接手Future，Future與Output需Send＋static；「spawn本身不在呼叫中同步執行future本體」；JoinError與工作Output不同；Id有Copy／Eq／Hash；AbortHandle不能await；remove取走entry。 | B1 held task、B2 BatchTask、B3完整Input登記與panic／abort回收都有真來源，不以async move製造Send。 |
| 有界集合與待辦資料 | 11.3的三態表、IntoIter、new／fill／next：「已完成但尚未收取的結果仍屬集合」「得到Report後才fill」。 | B3只換Input與HTTP core，保同一個數量不變條件；不是限制全部輸入／結果記憶體。 |
| Duration／Instant／u128、nested Result與合作式timer | 11.4時間表、timeout／取消／阻塞限制／paused clock：Duration可Copy；std Instant量真時間；timeout取得內層Output或Elapsed，不能硬切同步本體。 | B1每筆工作10秒、entered5秒、paused結果20秒各自起點與範圍明確；不拿elapsed_ms驗虛擬10秒。 |
| URL、回應與錯誤的選定能力 | 11.6 API表、124／138行：Url::parse／scheme／as_str、StatusCode可Copy、as_u16／is_success、ParseError與reqwest::Error有Debug／Display；HTTP404仍有Response。11.7 1–112行給async send的Future、headers／body分界。 | B1先parse／檢scheme、保original，再等final headers；不靠derive生出原本不存在的錯誤相等能力，tests用match分類。 |
| 真Fixture、借用控制端與雙方收尾 | 11.8 112–255行的完整helper表、entered有限觀察、release／shutdown、Client Send／Sync、Clone共用pool、pause／resume。 | URL字串不是server owner；Check／Entry是owned結果；Clone不是深複製pool。B1／B2保request handle，B3保Scheduler，最後都另收server。 |
| 行與owned文字、排序、Cargo與tests | 6.7 lines／sort_by_key、6.5 owned Vec iteration、1.9／1.10 module與cfg tests、7.1 path／dev-dependencies／版本要求；這些已在自己A原判必要回讀中定位。 | B2當地另先教enumerate、Unicode trim只判空、Vec::is_empty；B3當地先教dead_code allow、abort_all、join取消安全。沒有拿14作者背景或未來B4當先備。 |

前文回讀範圍及實際文件SHA在 `actual-prerequisite-read-proof.json`，不宣稱重新全文審閱1–14所有頁。最初合併輸出的11.2／11.3曾截斷，不把那次當全檔FULL；之後必要selected段落完整回讀／定位。對N1另直接讀3.1的pattern／arm規則及對reference的match，搜索1–11與14正文後，再對12／13學生正文做同一項必要語法查找；這只是搜索，不稱全章新審閱，沒有讀同行報告或15未授權內容。

官方只查需要的固定cached crate source：Tokio1.53.3 `join_set.rs` 289–318、457–465，確認join_next_with_id取消等待不移除task、abort_all不等待／不移除；reqwest0.13.5 `client.rs` 72–96，確認Clone與內部共用pool。檔案SHA及selected範圍保存於 `selected-official-proof.json`。這是所選版本正式來源的局部核對，不是整份API全文、重新上網研究或Rust實驗；不把官方內部select等例子算學生先備。

## 重要通過項：需要、機制、操作、用途

| 原位置／最短原句 | 原需與機制各自成立的理由 | 操作、完整材料與最強結論界線 |
| --- | --- | --- |
| B1 001-001／002：「若失敗，究竟是輸入不合法、請求失敗，還是等待逾時？」 | 先固定一筆要回答什麼，再選Check保存original／elapsed／Outcome。解析前拒絕和網路已嘗試分開；404／500有Response，redirect原網址與final URL不同。 | /notes、file、200／404／500、redirect、loop、held各有要求。只GET／最多五redirect／final2xx、不主動讀body；不把200說成內容正確，也不說網路沒收到任何body byte。 |
| B1 001-003／005：「輸入在解析處拒絕時是None」「task還在跑」 | 原需是一份失敗也有原文與分類；None表示未開始，Some(0)仍已嘗試。工作deadline與觀察通知／結果的guard分開，borrowed JoinHandle等待到期不能代取消task。 | held helper先真handler ack再pause；漏送請求由5秒guard、漏期限由20秒guard指明，resume後abort／await client、再shutdown，收尾後assert。它們不是一般產品的整批deadline或任意錯碼的硬中斷。 |
| B1 001-006：「函式承諾一筆總有一份Check」 | parse早退、HTTP／Request先成立，再包async work＋timeout；nested Result各回答工作是否到期、請求是否完成。match保每種結果，不用?跳過該筆。 | 完整check、六checks與demo均核；先保存Copy status及owned URL再drop response，沒有讀body的await。error.is_timeout保依賴自己的timeout分類，不假設只有wrapper能逾時。 |
| B2 002-002／003：「不能用網址作為唯一key」「只用trim視圖判空」 | 先有原行身分，再略空白；同URL兩行是兩個工作。保原行不是trim後改文，invalid非空行仍交B1，無非空行才Empty。 | LF／CRLF／孤CR／全形空白、scope內String結束後仍用owned Input、原index1／3／4與duplicates都有實際checks。圖的U是同一URL，沒有把index壓成0／1／2。 |
| B2 002-004／006：「取一份→await完成→保存→取下一份」 | 新需要是把一筆核心接成可追蹤清單；借同一Client，每筆複用Duration跨度但建立自己的timer。Check已包含業務失敗，runner不因中間失敗丟後續。 | 完整兩body、六checks／helpers、single core及demo均核；混合URL三種正常／問題仍五結果、loop六GET合計九，兩held按程式結構逐筆await。觀察的hits與某次排程不單獨證明序列性。 |
| B3 003-001／002：「不一次提交整張清單」「收取1……才讓len少一份」 | B2等待前項讓後項無機會，改少數工作同時等待；pending仍資料，JoinSet len包含完成待收。fill前逐次檢數量、next收一份後才補，才能維持上限。 | 三held／jobs2控制0、1先進入，先收1再提交2，再收2、0；最後按原index排序。另合成完成未收取實驗核真正邊界，不只核等待中兩tasks；非全部輸入記憶體或速度保證。 |
| B3 003-003／004／007：「task故障也不能捏造一份HTTP狀態」 | 網路Input／Request／Timeout有Check；panic／abort無Check，需Id表保存完整Input而非只index，task另有String複本。Client clone保pool，owned task不借外Client。 | Checked／TaskFailure兩路都回原文，panic input4／abort input9受mpsc控制；opaque Id只查表，原index才報告排序。索引唯一遞增是caller明訂契約，沒有偷偷把任意Input Vec當已驗證。 |
| B3 003-006：「收尾不呼叫會補工的Scheduler.next」「中間不再await」 | 測試故障需停止已提交tasks並收齊，不能在cleanup又補pending；next只有join等待，取得後同步分類／remove／fill／return，配取消安全方法，owner可在guard到期後drain。 | abort_all後逐join至None再shutdown，才panic／assert；synthetic lab Check明說非真HTTP。容量1空channel、send後無其他await與current-thread條件共同支持「owner見通知時該task已完成」，不是任何send都代表完成。N1另列該helper語法洞，這份收尾機制理由本來已足。 |

六份starter／complete的操作起點也清楚：repo外先把Fixture path換真實絕對路徑，再選+1.99.0；零tests只驗編譯，不算功能。checks整份cfg module追加到lib，不再包一層，不靠checks目錄自動發現。B1每次只改check的三個小步，B2兩body，B3三body；先前core與資料規則保留，完整副本沒有倒灌後節CLI／取消能力。

六／六／七tests與中間預期有可判材料：B1 parse先1、HTTP後5、加deadline後6；B2 prepare先2、runner後6；B3 stub1通過6失敗、fill＋next後3通過4失敗、runner後7。尤其B3完成待收一項會被空fill恰好滿足，正文／README明說單項綠燈不是排程已完成。這些是來源推論，沒有執行確認。

三層hint均先資料流／責任，再收已教能力，最後局部拆步；資料模型、期限、工作與觀察、索引、Id、上限與失敗層次都在fold外。答案不是完成作業唯一能取得必要機制的位置；但helper同樣受先教後用約束，不能以「可直接貼checks」略過N1。

## N1 必要：take helper的match guard首次使用未先教

- 原位置：15.9 `003-006.md:46`，guards code的take；相同code見 `b3-starter/checks/bounded-tests.rs:75`、`b3-complete/src/lib.rs:232`。
- 短引文：`Ok(Some(entry)) if entry.input_index == expected => (fixture, entry),`。
- 已教：1.6的if／bool、3.1的`pattern => 工作`、pattern拆欄位與順序分支、if let；B2 observe_started已用普通match得到correct，再以if分流。B3主文也清楚說take確認原index，不缺這個用途。
- 缺失關係：這裡把if放在pattern與箭頭之間，屬match guard；尚未說明pattern先符合後才判bool，guard為false要繼續後面的arm。它不是此前arm的body內if，也不是if let。必要前文未找到這項規則；不能只由「確認是哪個index」或syntax高亮算已教。
- 實際影響：讀者在作業前就需理解並貼入完整helper，判斷成功條件、錯index如何走收尾。主要排程可懂、code可編譯，不消除此處嚴格先教後用缺口；範圍限這個新語法，不升成整個取消安全或排程機制錯誤。
- 最小補法：偏好沿B2已教的普通match＋if，保同樣條件與收尾，並同步正文引用及兩份完整checks；或在該code之前用一個短句先教guard的真假分支規則。無需增加完整patterns大節、第三方API或改測試政策。
- 發現時點：讀B3正文時保存候選，全文sources、actual3.1與前文搜尋後確認；封存前另查12／13學生正文也未見該規則。未看任何同行／作者／Root問題清單，不算後見提示。

## I1 可選：手機三期限表的Outcome::Timeout拆字

- 原位置：15.7 `001-005.md:38`，三期限表的工作10秒／到期結果欄；親看 `views/01-three-budgets-mobile.png`，390×844。
- 原句：「正常單筆 `Outcome::Timeout`」。手機顯示為 `Outcome::Tim`／`eout`兩行；桌面完整。
- 已教及缺失關係：Timeout variant、三期限範圍、Some真時間與虛擬clock在相鄰正文與圖都明確，沒有缺必要API或分類；只是要把名稱拼回才直接對應enum。
- 影響／分級：可選名稱對照改善，沒有阻止表格的期限角色理解；不把普通窄欄換行升必要問題。
- 最小補法：該欄可只標完整短名`Timeout`並保持名內不斷行，或調短欄位標籤；保留三期限的時點／範圍與到期角色，不增加Rust概念。
- 發現時點：本輪自己的原始v12實頁親看；不是繼承別章或同行的版面結論。

## 實際實頁與證據限制

own work為 `/workspace/work/tutorial-implementation/review-ch15-continuity-b-prefix-v12-author04/`。自己的預覽只重組指定22 BODY與三SVG；去重複H1、改封存asset URI及隔離預覽必要href，label／code／順序／fold不變，不含B4／B5或其nav。theme與Markdown extensions逐項等於當前專案設定，no navigation.top；實際三頁及26個capture的`.md-top`皆0。

tutorial.css真正HTTP200載入，SHA `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`，與專案copy相符；每張metadata亦保存實際stylesheet href。初次mkdir遇ENOSPC，只清本人舊TECH11-tail可重建target821MB，source／lock／raw logs／原判不動；記`housekeeping-proof.json`。首次strict build因隔離三頁缺前文link target失敗，raw `preview/build.log`保留；只重base預覽href後，`preview/build-final.log`實際退出0。隔離建置問題不算教材缺陷，也未請求那些public href。

Chromium桌面1280×900／手機390×844，各13位置，共**26 PNG均以view_image親看**；`visual-capture-proof.json`逐張保存SHA／尺寸／selector／fold／bbox與實際親看標記。選定範圍：三圖（B3上下互補）、B1三期限表／starter轉入、三階段closed hints、B3 cancel-safe契約／失敗層次、B3答案closed與真正click展開。

三圖source與BODY角色一致：B1原文藍線與parse早退橘線不混timer；B2原index與owned Input／Entry對應可追，U兩份不合併；B3完成待收仍len2、收取才補2、最後原序整理，數字是input_index而非Id。B3手機top畫面最下框邊緣略裁，bottom補完末框及caption，沒有說一張全圖無裁。兩種尺寸的主要字／箭頭可普通閱讀，未靠放大或藏在fold內的圖補機制。

提示default closed，答案展開後code留自身橫向捲動區，不冒稱長答案在一張畫面全部可見；26張整頁horizontal overflow均false。I1以外未見選定位置的必要圖文辨識缺口。此範圍不是全站、全頁每一行、深色模式、所有scroll／menu／links／無障礙或正式GitHub Pages部署驗收。

## 未驗與封存

保留未驗：沒有Rust／CLI／19 tests／examples實跑、依賴解析／lock重建、任意故障與同步無yield錯碼、真外部HTTP／body／所有平臺／吞吐與記憶體量測。圖與有限受控材料不保證所有網路排程或一般Future都取消安全；B3尚無產品CLI／使用者取消，不以本輪測試drain當作它們已實作。

本報告只封B-prefix原判，samebyte initial＋SHA另存；N1／I1及早期待查時點保持。A original與其他角色原判一字不改。封存後STOP，不開peer／處置或B4／B5；後續另等Root授權的新scope。
