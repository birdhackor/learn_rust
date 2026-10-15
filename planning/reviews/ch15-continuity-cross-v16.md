# 第15章 CONT CROSS：原判互看與 v16 局部修後複查

結論：在本次受影響來源、first-use 與兩尺寸實頁範圍，四個已報必要項均已解決，四個不同的可選改善有具體收益，沒有發現這些修訂帶來的新必要負擔。這是非作者 CONT 的 cross／修後證據，**不是新 ORIGINAL、首讀重跑、全站驗收或發布簽核**。原四份 CONT 分級、八份同行分級與全部 unknown 都不回改。

## 時序、角色與固定範圍

我未寫第15章，曾寫第4章部分與第14章 macros／工具。這些作者背景不代學生先備；第15章四次 ORIGINAL 的82 BODY／180 archives 已實讀歷史及各輪畫面，仍以各自原報告為界。相同完整source只在自己核exact後沿用真正已讀；generated locks按原machine parse版本／registry／path契約，不冒稱逐行FULL。

1. Stage A 先 FULL 讀另外八份 TECH／FIRST originals，核其 SHA，保存互看後、回來源前判斷，再只核四個原 scope 的差異與主要 PASS。
2. Stage A 主檔與 initial 封存後，才取得 `ch15-review-fixed-v16`。
3. 自己先核 fixed diff／source／first-use，保存 `stage-b-before-root-source.md`，再親看自己的26張局部實頁、保存並封存自己的 source／visual 判斷。
4. 最後才 FULL 讀37行 `planning/reviews/ch15-root-disposition-v16.md`，SHA `1068785a18f78d1d2eaa5dd07b57c5d467b7dae803936c4cad9d5db4dc781edc`。未讀 Root 第15章 QA、作者結果或其他 live 新稿。沒有向 TECH／FIRST 傳判斷。

原 scope：`ch15-a-for-review-v11`、`ch15-b-prefix-for-review-v12`、`ch15-b4-prerequisite-v13`、`ch15-b5-prerequisite-v14`；原 union v15 只定位。union 的180 source hashes exact 對映原四 scope；BODY74份 raw相同、另8份只 frozen資產目錄，正規化後82 exact。不是宣稱所有raw BODY bytes相同。

唯一修後：`/workspace/work/tutorial-implementation/frozen/ch15-review-fixed-v16`，11頁82 BODY180 sources；manifest SHA `7f83b082a600c16180815a7970dc6f62dc51faefa816273798aff95fd321da06`。本輪機器核全部 BODY／source 對 manifest SHA，191份學生相對path→hash map（11頁＋180 sources）canonical SHA `32d9d7e5290081b2114086596190748666ddcae3d33fce26cee1d7685469b7a1`，算法與完整map存 `stage-b-student-fingerprint.json`。

精確差異與 Root scope一致：5頁中7個實質 BODY（001-001、002-003、007-002、007-005、009-006、011-004、011-009）；另8個 BODY 只改 frozen資產目錄。7份 archive source有改：四份A3／A4 tests、兩份B3 checks／complete、finalREADME；其餘173份 exact。八SVG原bytes不變，本輪沒有重驗它們。學生整檔12份有改，不把BODY的7個單位混計成7個完整頁面。

## 十二份 ORIGINAL 的分級與 FULL 互看

自己的四份原判均在 `planning/reviews/`，本階段開始與封存時核 SHA：

| ORIGINAL | 原分級，維持不改 | SHA256 |
| --- | --- | --- |
| ch15-continuity-a-v11.md | N0／I1：validate_root 主詞，可選 | `4a99ca927223e9bdc5e46b92ccf13cc03cb489f1f1615fb83bbd880e690b52ff` |
| ch15-continuity-b-prefix-v12.md | N1：match guard；I1：表格型別折字 | `7fa44c40873c102f14b044a0cc75c3fef7bb1cc9721a4bf0281079a842c82cf2` |
| ch15-continuity-b4-v13.md | N0／I0 | `e8801ab621c65f3119ac8fc14308dffce98e64e5b1b620fe265dcb5eb8721810` |
| ch15-continuity-b5-v14.md | N1：-D warnings；N2：fixture HTML／copy 首空行；I1：短參數折字 | `edea4315324b0336602d40ea5f728010989a99c3af1ad30efd0d7816e9cd18c6` |

八份同行均實際 FULL 讀完，逐段範圍與 bytes 另見 `peer-full-read-proof.json`。下表 path 同樣相對 `planning/reviews/`：

| 同行 ORIGINAL | 本次 FULL 行範圍 | 原分級 | SHA256 |
| --- | --- | --- | --- |
| ch15-technical-a-v11.md | 1–116 | N1／O1 | `042f134ed1b349a78ea5e2717b0b15c26d9197368b3c2b60089a0bbf614c3189` |
| ch15-technical-b-prefix-v12.md | 1–89 | N0／O0 | `467fe5f9207a38c6a4373e36aa82cd3cafc33a3c3089e340228f2ddb223cbd69` |
| ch15-technical-b4-v13.md | 1–131 | N0／O0 | `88f48a3b819a3d5be292a08f648a90447d18f2c76e044877af7b0d2c3950459d` |
| ch15-technical-b5-v14.md | 1–148 | N0／O1（延伸 A 的同一入口口徑） | `46b4d410cd6333ade7ee248a087651c19560e3e85c9332fb2ecba25b44793c41` |
| ch15-first-reading-a-v11.md | 1–90、91–180、181–255、256–322 | 沒有已確認 N／O；保留 unknown | `0592f6dd031d42c85aa1e5545fbc17a32756941a4ba7ba712fe4897877df18e6` |
| ch15-first-reading-b-prefix-v12.md | 1–70、71–135、136–192 | 沒有已確認 N／O；保留 unknown | `08e45aea114f96bc2b3a10c74d2c1b40f01153f89c5d05e4e1fb9420f79c198b` |
| ch15-first-reading-b4-v13.md | 1–58、59–101 | 沒有已確認 N／O；保留 unknown | `e88c2469abab6887a0c4915bf7c92fefaccfc5cb1e79900488f8f119631f9271` |
| ch15-first-reading-b5-v14.md | 1–57、58–105 | 沒有已確認 N／O；保留 unknown | `498e881ca0e19cdaaabc93f8fffdafc427d5ba106573ad5e6a63c218792fe6e7` |

FIRST 是有前文與第14章額外閱讀背景的 AI 順序讀者，不等於真正只會 Python 的真人。其正文／圖的首次記錄有價值，但沒有 FULL 開完整 starter、checks、README、manifest；所有 hints 閉合，沒有真正網頁 CSS／Chromium 或 Rust 執行。不能把它的 N0 擴成所有 source 已驗，也不能以其未見 header 直接認定 artifact 沒提供。TECH 的有限實驗、實頁證據歸 TECH，不算我的執行或新畫面。我的原始四輪來源／畫面界線也不回寫。


## Stage A 自己的來源判斷

以下保留原發現者／分級與互看時點，不以同行N就直接採納，也不以另一角色N0抵銷具體來源。

| 項目 | 原來源、已教／欠缺與我的 Stage A 判斷 |
| --- | --- |
| TECH A N1 | A3 checks `if path == &name` 兩邊是`&String`。actual4.2教String／str文字PartialEq與比較借用；actual4.3只另教數值reference比較。沒有泛化到&String的契約。採必要先備缺口，建議既教as_str；技術合法不等已教。**我原A漏此項**，互看後確認不算原獨立重現。 |
| CONT B-prefix N1 | 原003-006 take helper first-use `Ok(Some(entry)) if ... => ...`，普通match／if先備不等guard的成功後判條件、false選下一arm。維持必要；清理與觀察主方法仍有自己的PASS。TECH／FIRST未報不代表已教。 |
| CONT B5 N1 | 原命令已知Clippy／all-targets／轉交`--`，欠`-D warnings`使warning升error的最低意義。維持必要；不擴成整命令／lint系統未教。 |
| CONT B5 N2 | 真fixture、raw fence的首空行與程式索引正確，但自己原same-CSS實頁真copy變成非空0／2／3，和鄰接1／3／4不一致。維持必要呈現／操作問題，不改程式政策。FIRST raw首空行與TECH真檔binary成功，不反駁未覆蓋的HTML copy。 |
| TECH A O1／B5 O1，同項 | 入口800–1200含tests與完成頁透明列1657／1921口徑不同，核心約995／825。寬鬆約千行且只填少數body，不是硬上限違反；採可選期待管理，不要求刪tests。FIRST只觀察數量未列O，不回填命中。 |
| CONT A I1 | `validate_root`分工主詞「scan成功後」與真正先驗root的source不一致；下方code時序正確。維持可選文字收益。 |
| CONT B-prefix I1／CONT B5 I1 | 手機Timeout型別與首次短CLI參數被折字，維持各自可選呈現收益；不升必要，不要求所有code同屏。 |

FIRST B-prefix 的 scheme／Url import／tracked 定義未知，經 Stage A 回 actual11.6 的URL角色表、scheme回傳契約，以及完整starter的 `use url::Url`／tracked header確認，未新增必要缺陷。B5 status與write_report的精確u8／Result、check_text完整Result、StopEvent root pubuse，完整starter也均已有。正文先給完整起點再要求填body，並非練習者須憑空猜header。FIRST當時沒有開完整source的未知保留，不能回填成它已讀或已驗；BODY未重印也不等artifact缺聲明。

完整 Stage A 判斷包含逐項最短引文、actual4／11先備、unknown細分與8項重要PASS四題關係，仍保存於 **`/workspace/work/tutorial-implementation/review-ch15-continuity-cross-v16-author04/stage-a-source-judgments.md`**，106行，SHA **`8aa921aa47d371d7e1e25c6a58d1c8f21f72cbfccf8b6a0b3e13aa08f98a165f`**。**Root仍應FULL讀這份精確 Stage A 檔**；本報告不以較短摘要代它的原來源細節。same-byte initial與sidecar不改。

## v16 first-use／source局部複查

| 必要／可選項 | 修後已核位置與理由 | 結果／新增負擔 |
| --- | --- | --- |
| TECH A N1 | A3starterchecks／A3complete／A4starter／A4complete四處都改 `path.as_str() == name.as_str()`；四份完整checks後綴 exact。&str文字比較在4.2、借用String呼叫as_str在前文已有，無須新reference blanket impl。 | 已解決。仍檢查quote／LF經JSON往返後保原名稱，沒有偷換測試目的。 |
| CONT B-prefix N1 | 009-006、starter完整checks、complete的take先match取Entry，再普通if核index。Err／None與錯index兩路都先close再panic，正確路交tuple。兩source整份checks後綴 exact。 | 已解決。已教語法取代guard；增加五行服務相同檢查，沒有新API、沒有漏cleanup或擴大工作圈。 |
| CONT B5 N1 | 011-009命令前及finalREADME先解轉交與warning升error，再列Clippy命令，保「無warning不保功能」限定。 | 已解決，最低契約先於需要；沒另加lint章。 |
| CONT B5 N2 | 011-009以原index0–4五行表，明示空行／一個空白，請使用完整目錄提供的真檔；run仍指fixtures/invalid-urls.txt，fixture／Rust exact不變。 | 已解決。表→保留1／3／4→三筆input與空HTTP／時間欄位可銜接；不讓讀者從格式化code複製不可見首LF。真正copy結果另見下節。 |
| TECH工作量O1 | 001-001改核心約千行、tests／本機材料另供、完成頁列實量、不要求一次寫全tests。 | 採可選收益，期望與兩最終答案一致；保guided負擔與驗收。 |
| CONT A I1 | 002-003改「驗證成功後才開始走訪」，緊接scan先validate_root。 | 採可選收益，無新知識／重複流程。 |
| CONT B-prefix I1 | 007-005表格用已教短名Timeout＋局部nowrap，前Outcome模型與下方match仍完整定位variant。 | 採可選收益，三期限角色未變。 |
| CONT B5 I1 | 011-004首次兩個短參數和值局部nowrap，Settings、數字解析與正數先驗說明未變。 | 採可選收益；不把全篇／整段鎖不換行。 |
| FIRST提示後的三短澄清 | 007-002在policy前補scheme例子、原文／解析／final三角色旁補Url import；009-006在checks前補tracked只回tasks.len且含完成待收，明觀察過早放名額用途。 | 可保留的回顧，有減少回找收益，無新操作或新政策。不是FIRST原必要發現，不改Stage A「既有材料已提供契約」的判斷。 |
| FIRST B5 headers unknown | 來源／聲明不變。正文給完整起點、角色與caller責任已充分；不為closed-source首讀未知再印公有API全表。 | 沿Stage A未確認必要缺陷；FIRST當時未見仍保留，非被後文補成原首讀通過。 |

本輪FULL回讀上述7個changed BODY。6份Rust整檔差異逐處核，除上述替換無他改；未變部分以exact hashes連到自己原FULL閱讀，不假裝又逐行新讀180份。README完整diff只加旗標說明。沒有Rust重跑或用Root測試代替先備判斷。

## 未報問題的主要方法仍有獨立材料界線

本表是互看後回原文對齊，不稱新的執行證據。原四輪 FULL source／answers 的閱讀證據沿自己的 originals；本輪局部回讀位置／SHA 另存 `stage-a-focused-source-proof.json`。

| 原需要→機制→最低操作／用途 | 本輪實回原位置、例子與限定 | Stage A 邊界 |
| --- | --- | --- |
| A2：大小／digest 相同不能證明 bytes相同；壞代表不能吞掉同桶 B／B | union003-002 先三步列六檔；003-003 明不能把 Err 當不同、移除確切原生失敗者再重建，失敗每次減少仍在桶內記錄；003-004 先給 callback 參數／回傳、左／右失敗契約，再列三個 stub。A／B／B＋左來源 Interrupted／R／PermissionDenied 是可控制反例。 | 成功與問題分開保留、有進展理由；不保證所有檔案完整讀取、FS snapshot 或特定倍速。TECH selected probe 結果歸 TECH。 |
| A3：顯示文字不能取代原生 key；同位置 issue 仍需完整次序 | union004-003 先 a/b 與 a.txt 對照，教 Option Ord、位置→action→reason→path_debug逐欄；key排好才移除，Debug只診斷。完整 checks 的 quote／LF 還原用途合理。 | 固定平台 Path Ord，不承諾跨 OS 同一排列；此主方法 PASS 與 checks 的 N1 能並存。 |
| A4：有 partial data 不等致命失敗；scan成功不等stdout成功 | union005-002 先六類出口表，再 write_all→flush→0／1，任何 Err 2；說明部分 bytes不能撤回。A3 Report由A4接線，不另造格式。 | 被 writer 接收不等終端使用者已持久保存；不把 buffer Drop 當 flush結果。原 helper與 controlled writer 的目的已明。 |
| B1：收到404與沒有回應不同；只需 final headers | union007-002 用本機原路徑、redirect／loop／held 各列答案，明五次redirect、2xx與headers完成，不主動讀body；原文保 original／parsed／final三角色。 | 不以200保內容正確、不承諾網路完全未收到body；三種時間用途沿原正文已分，觀察guard不是產品deadline。 |
| B3：完成待收也占名額，不能用「正在跑」猜jobs | union009-002 先 held 1／2／0完成與0／1／2報告，明 JoinSet.len；009-004 把 pending／tasks／Id→完整Input 分開，先教 Client clone共享pool；009-006 completed-uncollected有限通知實驗於收取前查數，abort_all後逐份join。 | jobs不限制整張Vec／報告；合成Check不是HTTP證據；特定cancel-safe next契約不泛推任意Future。guard首用必要項不抹掉這些方法 PASS。 |
| B4：收到停止後不再fill；退出前要收已提交與未提交兩來源 | union010-004 new只準備／record不補，010-005 每輪try_recv→fill、biased stop先於join；by_ref先解借用與耗盡；stop_and_finish收真正Check／TaskFailure／Cancelled後才pending→Unstarted。 | 「不再提交」從觀察停止算，不宣稱OS抵達瞬間原子禁止；來源失去／Requested不同，非所有工作都覆寫Cancelled。 |
| B4驗收：測試cleanup不能幫產品補工，失敗也要留owner | union010-007 finish在drain前先存tasks.len／inputs.len，drain／shutdown後才assert；controlled_stop在同一test task借用Scheduler與Fixture、有限外層guard；已完成通知後保持Check／panic成果另分類。 | 十五秒只守這份可讓出runtime的觀察，不保同步panic或無限不讓出loop；本輪只讀，沒有執行或重現SIGINT。 |
| B5：同Batch映成同Report，正常資料狀態必須等出口成功 | union011-006 先cancel130後failed1否則0，011-007 先render、write_all、flush，再Ok(status)，先給ControlledWriter與拒flush反例；011-008保Scheduler owner、listener await後into_batch、typed來源／controller錯誤fatal。完整starter header已提供。 | Report為被動公開資料，不自動驗任意caller虛構模型語意；直接drop check_text Future不替代children收尾；source主方法 PASS 不抵銷 B5 literal UI N2。 |


這些是 Stage A互看後對原承諾的覆核，沿原完整source／答案閱讀；修後未變部分沒有重跑。必要／可選項的裁定不代替這些方法的覆蓋，方法PASS也不抵銷checks／旗標／呈現缺口。

## 實頁：自己的26張局部畫面

固定preview只含5個受影響完整頁，由fixed chunks按原序組合；只去重H1、重定位same-byte資產、將未複製前文改為主站**預定公開URL格式**。未開外連，不宣稱那些頁已發布。theme／Markdown與主config exact，CSS實際HTTP200，SHA `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`。初次standalone strict build因4個未複製前文連結拒絕，保留log；只重定位preview連結後build0，不算學生站缺檔。

親看1280×900、390×844各13張：入口規模、validate_root責任、scheme→policy、Url→owned模型、tracked→checks、take普通if、B3閉合提示、B5短參數、fixture表、旗標→命令、B5閉合答案→library，以及期限表。初次名為three-budgets的兩圖實際定位tools table，保留並按真對象記，**不當期限表證據**；後加兩張唯一文字定位的正確期限表，均已親看，因此總26，不是24張同時冒充正確表。

- 新說明皆在正文first-use，B3／B5 hints與答案保持閉合，練習→對照→demo／library過渡未變。沒有要求開答案才理解新旗標或helper。
- 兩尺寸Timeout、首次`--jobs 2`與`--timeout-secs 5`都是nowrap單一ClientRect；手機完整可辨，不宣稱所有後續inline／長code不換行。
- 新fixture表兩尺寸所有index與空行政策可對照。手機URL cell自然折兩行，DOM仍完整字串，且要求用隨附檔案，不承諾從cell拼檔，未新增必要缺陷。
- 真點新表下方第一code copy取得兩條build／run命令；舊省首LF的假fixture複製入口已去除。真檔仍42bytes、indexes1／3／4，這輪只核既有檔bytes／表格，不執行HTTP／Rust。
- 所看位置沒有Top按鈕、沒有頁面級水平溢出。長source仍是既有codeblock水平捲動，不宣稱每行單屏全見或全網站可讀性驗收。八SVG unchanged，沒有以這26張圖冒稱重看所有圖。

自己的實際proof在 `/workspace/work/tutorial-implementation/review-ch15-continuity-cross-v16-author04/`：`stage-b-preview-source-proof.json`、`stage-b-visual-proof.json`（26份各PNG SHA／bounds／actually_viewed）、`stage-b-dom-proof.json`（兩尺寸table cells／ClientRects／folds／真正copy）。Root處置前 source SHA `a9195568369a69e50053513d471067323819c29cb5c07231d11d5f3961f28040`、visual SHA `b334fc91bdf8e2f1fdc53c83ebff974f588b043ff8883556a0571f508c3fe203`，均有同byte initial；`stage-b-preview-link-clarification.md`保留已封visual誤寫「已發布網址」的更正，不靜改原紀錄。自己的local server已停止。

## 最後對 Root 處置與實際限度

在自己的 source／visual 判斷封存後讀Root處置，四必要／四不同可選的理由與Stage A一致；採其最小修法，沒有必要性分歧須抹平。scheme／Url／tracked受FIRST未知提示的澄清是Root編輯改善，不是FIRST原N；B5 headers不再新增重複表符合實際完整starter入口。Root補查／真copy／實頁仍歸Root，不能當我的原重現。

Root scope記六包fmt／Clippy及4個filtered targets0；Root另訊告完整check479tests／158negative／C3＋1已0。**本輪沒有執行或獨立讀那些完整rawlogs**，只按Root來源歸屬，不以它們判first-use／視覺或聲稱自己驗過。TECH有限probes與真CLI／signal仍只限TECH原報告；FIRST raw／閉合路線、我的原32／26／20／24張圖及未跑Rust界線照舊。

本輪已核的必要缺口無待修；Windows／全HTTP／FS競態、安裝／release、未看的畫面／任意future取消等不在此局部證據內，仍未驗。保留原例的限制：jobs不限制整張輸入與報告記憶體、headers不代表body／內容正確、guard不保不讓出runtime的loop、library成功不代真stdout／signal驗收。AI審閱不等真人Python讀者測試。

可供Root整章協調使用的只是此cross關閉範圍；整章QA、完整候選、CI／Miri、GitHub Pages發布皆不由本報告簽核。報告／initial／sidecar與proof封存後停止，未開其他章scope。
