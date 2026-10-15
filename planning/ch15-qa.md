# 第15章 QA 閉環與發布紀錄

**最新狀態：review／cross／local、正式候選本機驗證、遠端正常profile CI與公開部署確認均已完成。** Root已全文核對協調稿182行、十二份ORIGINAL、三份followup及兩份精確Stage A；正式候選通過完整check與固定nightly Miri，首頁與109小節均已公開，證據見文末。以下保留協調者交稿時的狀態與歸屬，不回寫原判或未知，也不以局部N0、作者17項CLI或先備precheck代替正式候選驗證。

本次角色是協調彙整者，並非第15章的第四份獨立原判，也沒有自行再審教材。曾寫第2章、4.4–4.5、第5章、9.2–9.4及做過12／14等審閱的背景保留；沒有把自己的專業知識補成原讀者已懂。只寫本草稿及自有 work 證據，沒有改學生稿、workspace、lock、nav、Git index或任何原報告；沒有將這份彙整傳給TECH01／CONT04／FIRST04／作者07。

## 使用者標準與判斷口徑

- 讀者入口是懂基本 Python；以好懂、好學為最優先，進階內容也是必修。
- 每份 example、測試、helper、提示、完整答案及配置，都只能用真實前文已教，或在當處使用前先教的能力；概念名稱、目錄或可編譯不算已教。分段平行寫作仍須以實際凍結前文為依據。
- 實戰採引導方式：需求與材料、可編譯但明示未完成的起點、小步實作、行為驗收、分層提示、同階段快照及可用的完整答案。必要契約不可只藏提示／答案。
- 專案約千行是控制負擔的目標，不是硬行數上限；可以用第三方 library，但新增API仍遵守先教後用。入口與最終計數應說清楚核心、測試、fixture與重複階段副本口徑。
- 依 clear-tutorial／review-protocol 分開原需、機制、用途、例子支持與限定；技術正確、可操作、首讀可懂與發布可用各需自己的證據。SVG檔／build／bbox不能代替實頁親看。
- 正體中文與英文核心術語依 rust-terminology；heap、stack、ownership、trait等不強改中文。

## 已收原判版本與 FULL 閱讀

本協調者已逐段 FULL 讀十二份報告，共 **1,552行、294,031 bytes**；沒有以摘要代全文，報告讀取均無截斷。A v11（15.1–15.6六頁42 BODY／101 archives）、B-prefix v12（15.7–15.9三頁22 BODY／33 archives）、B4 v13（15.10一頁9 BODY／13 archives）與B5 v14（15.11一頁9 BODY／33 archives）均收齊三角色原判。本輪新增TECH B5的148行與CONT B5的87行；前輪已FULL讀FIRST B5的105行。四scope合11頁82 BODY；附件覆蓋由各原判／Root索引聲明，本協調者沒有另做180條附件或學生正文的獨立審查。

| 凍結 scope | 原判所用 manifest SHA | 本輪原稿界線 |
| --- | --- | --- |
| `ch15-a-for-review-v11` | `b0943a259e99dc064a7789964b5b21152ea50e632d6a37c98f29bde0e93c8c62` | A1–A4工作版／完成版及A最終答案；不含B實作。 |
| `ch15-b-prefix-for-review-v12` | `18b90d116556eea43dd67b5a996b6decc17ac4b8235a764ee821f4b42116b25f` | B1單筆、B2依序、B3有限提交；不含產品取消或最終CLI。 |
| `ch15-b4-prerequisite-v13` | `5f0dcdb5346281a01f5ce2a5a252bef83d841ebd8bc305be8ba5499a3f99a850` | B4停止／保成果／收尾與signal demo；不含B5。 |
| `ch15-b5-prerequisite-v14` | `06e8691b10acfa1303a84a8ca6ff907ec8f275f083613ddc4729950e87826bc5` | B5已交稿／actual gate開，三角色 ORIGINAL 均已收齊。 |

十二份報告均自行核SHA，原bytes／分級不改。舊九份FULL閱讀與副本在 `work/ch15-qa-coordination-draft-v1/`；FIRST B5及作者歸屬在 `work/ch15-qa-coordination-draft-v2/`；兩份B5原判、Root處置／scope／產品release紀錄在 `work/ch15-qa-coordination-draft-v3/`；本輪CONT cross、FIRST local與Root修後UI紀錄在 `work/ch15-qa-coordination-draft-v4/`。前兩版草稿SHA `58535428e746668ecd82958596bfb76aa2c514fdba5911127b52bce18c30dcec`、`e9d979ecc24d77f6515375b933cf7e3db0d73fa9fd0b4710bc33bc36bbb42292` 均保存。

root另提供整章原始scope索引 `ch15-complete-original-v15`：11頁82 BODY／180 archives，manifest SHA `2f1c938eeed57e58ff68f266eb9e6b0a370448cbd12bd08aa998a55c66945d3d`。本協調者僅核manifest索引數量，不開學生正文／source；四scope EXACT union與34份B5 student材料、166 fingerprints match是root本輪協調訊息的核對聲明，不冒稱本人另做same-byte稽核。這是whole ORIGINAL索引，不是修後候選或整章通過證據。

| 原報告 | FULL 行數 | 本輪核得 SHA-256 |
| --- | --- | --- |
| [ch15-technical-a-v11.md](reviews/ch15-technical-a-v11.md) | 116 | `042f134ed1b349a78ea5e2717b0b15c26d9197368b3c2b60089a0bbf614c3189` |
| [ch15-continuity-a-v11.md](reviews/ch15-continuity-a-v11.md) | 108 | `4a99ca927223e9bdc5e46b92ccf13cc03cb489f1f1615fb83bbd880e690b52ff` |
| [ch15-first-reading-a-v11.md](reviews/ch15-first-reading-a-v11.md) | 322 | `0592f6dd031d42c85aa1e5545fbc17a32756941a4ba7ba712fe4897877df18e6` |
| [ch15-technical-b-prefix-v12.md](reviews/ch15-technical-b-prefix-v12.md) | 89 | `467fe5f9207a38c6a4373e36aa82cd3cafc33a3c3089e340228f2ddb223cbd69` |
| [ch15-continuity-b-prefix-v12.md](reviews/ch15-continuity-b-prefix-v12.md) | 90 | `7fa44c40873c102f14b044a0cc75c3fef7bb1cc9721a4bf0281079a842c82cf2` |
| [ch15-first-reading-b-prefix-v12.md](reviews/ch15-first-reading-b-prefix-v12.md) | 192 | `08e45aea114f96bc2b3a10c74d2c1b40f01153f89c5d05e4e1fb9420f79c198b` |
| [ch15-technical-b4-v13.md](reviews/ch15-technical-b4-v13.md) | 131 | `88f48a3b819a3d5be292a08f648a90447d18f2c76e044877af7b0d2c3950459d` |
| [ch15-continuity-b4-v13.md](reviews/ch15-continuity-b4-v13.md) | 63 | `e8801ab621c65f3119ac8fc14308dffce98e64e5b1b620fe265dcb5eb8721810` |
| [ch15-first-reading-b4-v13.md](reviews/ch15-first-reading-b4-v13.md) | 101 | `e88c2469abab6887a0c4915bf7c92fefaccfc5cb1e79900488f8f119631f9271` |
| [ch15-first-reading-b5-v14.md](reviews/ch15-first-reading-b5-v14.md) | 105 | `498e881ca0e19cdaaabc93f8fffdafc427d5ba106573ad5e6a63c218792fe6e7` |
| [ch15-technical-b5-v14.md](reviews/ch15-technical-b5-v14.md) | 148 | `46b4d410cd6333ade7ee248a087651c19560e3e85c9332fb2ecba25b44793c41` |
| [ch15-continuity-b5-v14.md](reviews/ch15-continuity-b5-v14.md) | 87 | `edea4315324b0336602d40ea5f728010989a99c3af1ad30efd0d7816e9cd18c6` |

## 角色背景與實讀／執行限制

TECH01未寫第15章，曾寫1／8、11.4及14部分；CONT04未寫第15章，曾寫4部分及14部分。FIRST04是AI輔助順序讀者模擬，非真人Python新手研究；已有actual1–12、FIRST13及明示14背景，A32之後插入LOCAL13的時序亦原樣保存。三角色各scope是原獨立判斷；本稿的同意或彙整不增添一次獨立重現。

| 原角色／scope | 實讀與原分級 | 該角色真正執行／視覺及限制 |
| --- | --- | --- |
| TECH A | 42 BODY／101附件；63種完整bytes；N1／O1 | A1附checks可編譯、所選stub預期FAIL；五項selected PASS含壞代表後重建、writer flush、真CLI完整／partial與typed公有原因。36張實頁親看；不是全suite／所有CLI／全平臺。 |
| CONT A | 42 BODY／101附件；63種完整bytes；N0／I1 | 沒跑Rust；32張1280×900／390×844實頁親看，CSS實載。測試數與輸出是source判準，不是其實測。 |
| FIRST A | 42正文、17個fold皆closed；N0／O0 | 三SVG孤立390渲染，未實頁、完整source／manifest／README或Rust。保留800–1200目標與1657實際量差異觀察，未列N/O。 |
| TECH B-prefix | 22 BODY／33附件；29種完整bytes；N0／O0 | starter編譯、一項碰巧PASS、一項預期FAIL，四項complete selected PASS含期限、owned原行、完成待收名額及收後補工。32張兩尺寸實頁；不是全19 tests。 |
| CONT B-prefix | 22 BODY／33附件；29種完整bytes；N1／I1 | 沒跑Rust；26張兩尺寸實頁親看。原guard候選查正文先備後確認；不是技術執行錯誤。 |
| FIRST B-prefix | 22正文、12個fold皆closed；N0／O0 | 三SVG孤立390渲染，未實頁／完整source或Rust；scheme／Url import／tracked()未知保留。 |
| TECH B4 | 9 BODY／13附件；11種完整bytes；N0／O0 | 四項filtered PASS、一項starter預期FAIL；真binary在flushed READY後對PID送Unix SIGINT，exit130與逐行分類／GET=2吻合。26張兩尺寸實頁；未按鍵十秒路徑等未另跑。 |
| CONT B4 | 9 BODY／13附件；11種完整bytes；N0／I0 | 沒跑Rust／真Ctrl+C；20張兩尺寸實頁親看，另核固定Tokio source。未撤銷B-prefix的原N1／I1。 |
| FIRST B4 | 9正文、4個fold皆closed；N0／O0 | 一SVG孤立390渲染，未實頁／source／Rust／真signal；前綴未知不回填。 |
| FIRST B5 | 9正文、4個fold皆closed；N0／O0 | 一SVG原位source與孤立390渲染，未實頁／完整source／README／Rust／install／真CLI。精確API header與StopEvent re-export仍未核；作者驗證不是其首證。 |
| TECH B5 | 9 BODY／33附件，27種完整bytes；N0／O1 | 25種非lock來源FULL，兩generated lock只完整TOML解析、不冒逐行FULL。四項filtered PASS，另一項ready／mixed HTTP test日誌1 passed但shell未另存Cargo獨立exit；starter flush預期FAIL、external consumer exit0／private E0603、五項真CLI（含兩held GET後SIGINT130及/dev/full fatal2）。30張兩尺寸實頁；未重跑全部16＋2、全部17 CLI、release／install或全站。O1延續自己A的同項口徑，不是第二次獨立命中。 |
| CONT B5 | 9 BODY／33附件，27種完整bytes；N2／I1 | 25種非lock來源FULL；兩lock完整TOML解析、不冒逐行FULL。未執行Rust／真CLI／SIGINT；24張兩尺寸實頁，另有DOM與真正copy的clipboard證據。N2是呈現材料索引改變，不是fixture或產品解析錯。 |

TECH的期望stub失敗是針對確定行為的紅燈，不是接受任意非零。CONT source通過與FIRST正文可理解，也不能代替TECH的真child／signal。三角色的隔離preview與局部CSS確認均不等於正式GitHub Pages、全頁所有scroll、深色／無障礙或所有瀏覽器驗收。本協調者本輪沒有新跑Rust、開實頁、看captures、測signal或發公共HTTP。

## 原判問題與 Root v16 處置：保留原分級

十二份原判合計 **四個必要項、四個不同的可選項**。TECH A O1與TECH B5 O1是同一入口口徑，合計只算一項；FIRST的行數觀察／未知不補算命中。以下處置來自Root，非本協調者另審來源或三角色共同重現。各角色N0不撤銷別人的局部N，原報告及原發現時點均不回寫。

本協調者FULL讀 [ch15-root-disposition-v16.md](reviews/ch15-root-disposition-v16.md) 37行，核SHA `1068785a18f78d1d2eaa5dd07b57c5d467b7dae803936c4cad9d5db4dc781edc`。**TECH／CONT兩份cross均核4N已解決、4個不同的O有收益且未增不合入口的先備；FIRST LOCAL7未新增確認N／O。review-only處置已閉合，原分級不回寫；candidate／CI／public另待。** 下表列最小改動與兩角色已完成的局部核對，精確範圍詳後表。

| 原來源／ID及分級 | 原已教、缺少關係與影響 | Root最小修正與cross結果 |
| --- | --- | --- |
| TECH A **N1：必要局部先備** | A3 checks／complete的`path == &name`兩端是`&String`；String／str內容比較及數值reference已有，shared String reference的PartialEq契約未在首用前教。code合法，不是address比較或演算法錯。 | A3 checks／complete、A4 starter／complete四處改已教的`path.as_str() == name.as_str()`。不擴blanket impl理論；TECH／CONT均核四處一致、目的不變。 |
| CONT B-prefix **N1：必要首用語法** | 15.9 take helper及checks／complete使用`Ok(Some(entry)) if entry.input_index == expected => ...`；普通if／match已教，match guard成功後判bool、false再選arm的規則未教。 | checks／complete改普通match取得Entry後再if核expected；錯誤仍先close再panic，保有限等待與owner清理。TECH／CONT均核首用與相鄰過渡已解決。 |
| CONT B5 **N1：必要新命令規則** | 15.11最終命令與final README首次用`-- -D warnings`；一般Clippy／轉交選項已有，警告提升為錯誤的通過條件未教。不是產品功能錯。 | 命令前及final README補`--`轉交Clippy、`-D warnings`把回報警告提升為錯誤；保命令，不另寫lint章。TECH／CONT均核說明先於首用。 |
| CONT B5 **N2：必要材料與索引對應** | 真42-byte fixture與文字原索引1／3／4正確，raw fence保前導LF；實頁與真正copy卻省首空行，非空索引成0／2／3。不是fixture或Rust解析錯。 | 刪受格式化改變的raw展示，改原index0–4的明示空行／空白字元表，執行仍用原fixture。Root亦自有copy重現；TECH／CONT與Root各自修後雙尺寸範圍可讀，不為呈現問題重跑HTTP。 |
| TECH A **O1**、TECH B5 **O1：同項可選工作量口徑** | 15.1說800–1200含程式與tests；A列995＋662＝1657，B列825＋898＋198＝1921。兩完稿誠實列量，核心符合約千行方向；非硬上限違反，不能為湊行數砍必要驗收。 | 入口改核心約千行，tests／本機材料另供，完稿分列且不要求一次手寫全部參考tests；保分階段練習。TECH B5延續自己A原O1，不能算同行重現或第二個獨立命中。TECH／CONT及FIRST local核入口口徑一致，仍保可選分級。 |
| CONT A **I1：可選責任摘要主詞** | 15.2 validate_root摘要稱scan成功才走訪；後續接線已正確，功能與先後可懂。 | 一處改「驗證成功後」，不升必要或重寫scan。TECH／CONT與FIRST local核措辭與接線一致。 |
| CONT B-prefix **I1：可選手機名稱** | 三期限表`Outcome::Timeout`在390px拆字；工作／觀察期限與到期結果已有說明。 | 該格改短名Timeout並局部nowrap，不改全域CSS；TECH／CONT與Root各自雙尺寸token可辨。 |
| CONT B5 **I1：可選短參數token** | CLI介紹`--timeout-secs 5`在390px將兩連字號與名字拆行；long語法與完整命令已教，不升必要。 | 只對兩個可容納的短參數和值局部nowrap，不要求整段不換行。Root原手機亦見兩rect；修後TECH／CONT與Root各自實頁均為單rect。 |

B4三份N0/O0仍只指原15.10；B5是TECH N0/O1、CONT N2/I1、FIRST N0/O0。Root修改與之後cross不算新的獨立原判，也不把大部分方法可懂當作消除局部first-use或呈現缺口。

## 主要通過理由與保留未知

這些是原報告已明列的理解與材料支持，協調者尚未做另一輪source／通過圖覆核：

- **A資料與輸出鏈：** root不能開始和item可繼續分流；大小／摘要只是候選，完整bytes才同群；失敗保存確切原生offender並有限重建，保B／B；native排序後才String且診斷ties完整；同Report供text／JSON，checked metrics的致命錯誤不偽裝partial；write與flush雙成功才選0／1，任何輸出失敗2。final公有入口與Error借用仍守原責任。
- **B1–B3：** 每筆保原文與原行，輸入／HTTP／請求／期限分層；final headers不等body內容正確，Some(0)不等未嘗試；enumerate在判空前，trim只判空不改原行，重複URL不合併；jobs限submitted-uncollected，done未收仍占名額，收取映射後才補工；Id與原索引不同，task fault不捏造Check。測試觀察guard不是產品整批期限或所有Future可安全取消的保證。
- **B4：** 先觀察停止再fill、new與record不補工；保已完成Check／panic，實際cancel才Cancelled，無task的pending是Unstarted；by_ref在首用前教同一iterator與owned Input；abort_all後還要join，測試先拍快照再後備清理，不能替產品漏等補成PASS；listener進入與READY有條件，真GET的server仍須獨立收妥。已觀察停止不等OS瞬間原子禁止提交。
- **B5三角色各自範圍：** Batch→owned Row／Report保原序、空洞與duplicates，Requested才令整批cancelled；同Report形成text／JSON，compact引用使含LF的original／detail仍一筆一行，手工毫秒不冒HTTP量測。write_all與flush完成後才交0／1／130，Read／Output等fatal2優先。新Serde型別、default_value_t及正數驗證、test-only ready body與headers-only產品、local install先說角色再用；三步規則與checks用途在閉合答案前。TECH另真測mapping／codec／flush／設定優先、external root入口與private E0603，真child SIGINT保報告；CONT從完整source／實頁核責任但未run，FIRST只核主文與孤立SVG。各自材料支持不互借成相同執行證據。

原FIRST的未知及其他角色未驗範圍保留，**不是待填成已教的空格，也不自動升成新缺陷**：

| 原未知／未驗範圍 | 本稿如何保留；後續承接 |
| --- | --- |
| A早期通用tests數量對映、group序與text layout | FIRST後文實際釐清時只追加，不能重寫較早段成當時已知；本稿不重新評分首讀。 |
| A完整Source helper內部、checks／完整答案／實頁未開 | FIRST有主文最低契約，但未讀的部分不冒FULL source／runtime PASS；TECH／CONT的完整閱讀另列。 |
| 入口行數目標與參考量 | A的995＋662＝1657與B的825＋898＋198＝1921：TECH A／B5是自己延續的同一O1，FIRST有觀察但兩處皆N0/O0，CONT未報此O。Root已採核心／驗收分列的入口措辭，TECH／CONT與FIRST local已核收益，不能補算FIRST原命中。 |
| B的scheme直觀詞義、Url短名exact import、tracked() method確切定義 | FIRST原推測／未核保留。Root受未知提示後補三句：scheme為冒號前部分、`use url::Url;`引入短名、tracked回tasks.len且含已完成未收者。這是Root編輯澄清，非FIRST已確認N/O或新獨立命中；不從B4 local tracked回填B3當時已知。 |
| 只做B／略過A的入口 | TECH B4明記未另驗B-only路線，eprintln先備沿A1；不能把A→B順讀驗證當成完全跳過A也已驗。後續依實際入口核必要背景。 |
| filesystem／平台／HTTP／取消／效果 | metadata非一致快照、hardlink指標非保證物理回收；未驗各OS、任意race／proxy／DNS／TLS／同步不yield錯碼。有限guards、selected tests、真Unix signal不證明所有故障路徑或真人學習成效。 |
| B5精確介面與完整驗證 | FIRST未見Report.status／write_report數值返回header、check_text完整Rust返回型別及StopEvent確切pub use，原未知保留。Root核完整起點／成品有宣告、正文已有角色／錯誤／caller責任，決定不重複加公有API表。TECH真external consumer是其技術證據，不代FIRST開過完整source；三份B5原判與三final已收齊，review-only閉合；產品候選與發布仍待。 |

## 待閉合清單與 signoff 狀態

| 工作 | 目前狀態／完成後需要保存的證據 |
| --- | --- |
| B5作者驗收與actual gate | **已交稿、gate已開。** 作者及Root precheck各自歸屬；不代獨立三角色QA。 |
| 十二份 ORIGINAL | **收齊並FULL讀完，SHA相符。** 保各原N/O、未知、執行／視覺與角色背景，不回改報告。 |
| 已報N／O裁定 | **Root已逐項處置：4N／4unique O。** TECH A／B5工作量O1合一；FIRST未知不補grade或命中。處置完成不等修後複查完成。 |
| 修後source／BODY scope proof | **Root已建v16最小修正scope。** 12學生檔變動、15 raw BODY差異中7材料／8資產絕對tag路徑；8SVG原bytes。本協調者FULL讀scope JSON及核manifest索引，沒有另做學生source審查；詳下段。 |
| TECH／CONT cross、FIRST local | **三份final收齊，review-only已閉合。** 本協調者FULL讀TECH97／CONT122／FIRST81，共300行並核SHA；Root亦FULL兩份精確Stage A。這只關閉修後處置scope，不是新增ORIGINAL或整章發布簽核。 |
| Root修後實頁／未報問題圖抽查 | **已收Root四處×兩尺寸的8PNG proof，CONT另自有26張局部畫面。** Root報五受影響頁currentSHA精確；本協調者只讀JSON／報告，未親看PNG或重驗五頁來源。原未報問題圖證據沿各自範圍，非全頁／全scroll驗收。 |
| 兩專案可用答案與負擔 | **Root工作版optimized release build與JSON smoke已通過，但整章判定未閉合。** A exit0、B invalid-only exit1；不是candidate或CI，也不代作者debug17 CLI、跨平臺／安裝或完全跳A的B-only路線。 |
| 完整候選check／selected Miri | **Root完整precheck已0，正式候選仍待重跑。** Root訊息：479 tests／498 normal-target results／158 negative／C3＋1，Miri四tests及指定負例exit0。precheck Cargo member order與最終穩定排序不同，不能當final candidate proof；本協調者未讀raw log或自行run。 |
| Root發布／Actions／live HTTP | **第15章未簽。** 尚無本輪可閉合的candidate／CI／公開頁版本證據；第14章既有成功不得借簽15。 |

本次三份final與Root disposition齊備，閉合review-only；正式candidate／CI／public仍須各自再驗，**不得把review-only完成當成整章發布QA通過**。可續補本稿記錄之後實測，immutable signoffs只鎖原判／三final及Root處置，不鎖這份會更新的QA文件。

## 修後 followups與review-only signoff

本協調者本輪FULL讀三份sealed final共300行，SHA相符，沒有另做教材獨立審查或讀過未sealed草稿。CONT Stage A106行／SHA `8aa921aa47d371d7e1e25c6a58d1c8f21f72cbfccf8b6a0b3e13aa08f98a165f`、TECH Stage A84行／SHA `70f06905e8f14b19ee3d2aa8f8feb85ec29f797f424cdb4c361d7ef8a85fa980` 均已由Root另FULL讀。本協調者未將這兩份精確Stage A另外全文打開，只按final report與Root訊息歸屬，不冒其來源判斷是自己的獨立重現。

| 角色與 sealed final | 收件內容、已解關係與保留限制 |
| --- | --- |
| [CONT cross v16](reviews/ch15-continuity-cross-v16.md)，122行，SHA `e317f8e06c34a4471fae0108ce71f6dd3f8ebf19055580afa93495dd947d8543` | 先FULL八份TECH／FIRST originals、自封Stage A，再核v16來源與自身視覺，最後才讀Root處置。承認原A漏TECH的reference比較項，互看後確認不能算原獨立命中；維持自己guard／flag／fixture呈現N及可選分級。核四處as_str、take普通match＋if與錯路先close、flag首用前說明、真fixture不變與0–4表；4N均解決，4unique O具收益，無新增必要負擔。FIRST未知提示的三短澄清可保留，B5精確headers仍只屬FIRST未開完整source的未知，不增公有API重複表。 |
| CONT局部source／實頁界線 | 自核191 student fingerprints、180來源中7變173 exact、7材料BODY／8tag-only BODY，八SVG不變未重看。自己26張1280×900／390×844畫面，CSS HTTP200同SHA；兩短CLI與Timeout各單一rect、fixture表完整，真copy已改成build／run命令而非假raw fixture，無Top／整頁overflow。初次strict前文連結失敗及定位錯表兩圖均保留，後補正確期限表；不冒26張皆首次正確或正式live。未跑Rust、HTTP、SIGINT或Root raw logs，沿原PASS的材料界線，不簽candidate／CI／公開發布。 |
| [FIRST local v16](reviews/ch15-first-reading-local-v16.md)，81行，SHA `329855af732712e2fb9096d9cbf99e534f134a0c915640fda46baf00c09aa212` | 只按controller順序讀7個材料BODY，closed-filter下先存四題／機制與原需用途再NEXT。本輪不是新blind或全82重讀；新增scheme／Url import／tracked契約理解，入口核心與驗收口徑、root驗證時點、普通if helper、Clippy規則與原index表均可銜接，沒有新確認N／O。未知解除只追加於LOCAL，不回填原grade／原時已懂。 |
| FIRST保留未知／未驗證 | B5 status／write_report精確返回header、check_text完整Return及StopEvent root pubuse仍未核，不推完整artifact缺宣告。七段無SVG或fold，沒有新圖、答案展開、完整source／manifest／README／實頁或任何Rust／CLI／install／HTTP執行；nowrap只核source含義，不能借CONT／Root畫面當FIRST實頁通過。 |
| [TECH cross v16](reviews/ch15-technical-cross-v16.md)，97行，SHA `dc1bad122f5d2180b1440f36cf87bffb493a58294032bd0547f5edb38f16078b` | 先FULL八同行與自存Stage A，再核v16並先存source判斷才讀Root處置；4N已解除，4unique O有收益且未帶新先備。承認原B-prefix漏guard、原B5漏flag與HTML首LF，不回寫N0或補算新獨立命中。六份Rust全檔唯一替換、其餘bytes exact與README／七BODY及相鄰過渡核對；ordinary helper仍先close後panic，錯index診斷由外包Result變Entry不承諾逐byte相同。FIRST未知回顧合理，不新增重複公有API表。 |
| TECH實頁／執行限度 | 自有22張1280×800／390×844PNG，CSS200同SHA、無整頁overflow、folds仍closed，兩短參數單rect、Timeout及0–4表可辨；手機長診斷／Clippy命令右端未另核，保既有code內橫捲界線。初次selector strict失敗與後續續拍保留，不冒每次擷取成功；未重拍八SVG。沒有新Rust／probe；只讀Root六包fmt／Clippy與四selected raw logs且歸Root執行。absence搜尋僅其18份known snapshots與A BODY，不冒新逐字排除全1–14；未驗final candidate、CI、live、全OS或任意故障。 |

Root修後UI proof `/workspace/work/tutorial-implementation/root15-fixed-preview/visual-proof.json`（175行）本協調者FULL讀完，SHA `f2c113e591df969f9a823ad005b9404f264aec89cc967e685467be766ef32cce`。Root本人實看期限表、CLI短參數、fixture表、Clippy說明四處×1280／390共8PNG；JSON記CSS200與SHA `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`、所有所看處無Top／overflow、短參數各1 rect、fixture原index0–4完整。五受影響頁與current SHA精確是Root協調訊息聲明；本協調者未自行逐頁核source或親看這8PNG，不將JSON閱讀當自己的visual。

Root另訊完整 `scripts/check.sh` precheck exit0：479 tests、498 normal-target results、158 negative、C3＋1；selected Miri exit0：四tests及指定Miri負例。這是Root執行歸屬，本協調者未開raw logs或重跑；`release15-check.log`／`release15-miri-check.log`將改名precheck保留。**因precheck Cargo member order與最終穩定排序不同，正式candidate仍會重跑，不提前算candidate／remote CI已通過。**


review-only signoffs：`/workspace/work/tutorial-implementation/release15-signoffs.json`，SHA `dbc58a15eca0b194faa22195de350550b05587076fb7fc85125337efebb84261`。`closed=true` **只表示 review／cross／local／處置閉合**；JSON鎖十二份ORIGINAL及三份final＋Root disposition的path／SHA，未將本QA納入immutable followups。candidate、候選check／Miri、CI與public各欄仍為pending，後續由Root真實測補證。

## Root最小修正與工作版檢查：非候選簽核

本協調者FULL讀 `/workspace/work/tutorial-implementation/ch15-minimal-fix-scope-v16.json`（174行），SHA `4bc1d57cf9f12cc187506a36335e4661b35635f5fd896d1ad1d7cd0dbe1d0c32`。修後 `ch15-review-fixed-v16` 的manifest SHA `7f83b082a600c16180815a7970dc6f62dc51faefa816273798aff95fd321da06`，索引11頁82 BODY／180 archives；191學生檔的same-byte／diff範圍是Root proof所述，本協調者未開來源逐條重驗。

- Root scope列12 changed student files；真正材料BODY為 `001-001`、`002-003`、`007-002`、`007-005`、`009-006`、`011-004`、`011-009`。另8 raw BODY只有absolute frozen asset tag路徑改變，不當成8份教學修正；8張SVG維持原bytes。
- Rust只改既有checks／tests的String比較及take helper。Root從manifest解析正確package名後，六包fmt／Clippy皆exit0，A3、A4 starter、A4 complete的三項字串往返test與B3真收取後補工test皆exit0。最初fmt猜錯不存在package的失敗保留，不算學生稿錯，也不寫成每次命令皆成功。
- 這些是Root相稱執行的JSON紀錄，不是本協調者run或整套test。BODY／helper受影響首次位置、unknown澄清及可選收益現由三final與Root局部UI證據承接；正式候選仍待。

另FULL讀 `/workspace/work/tutorial-implementation/release15-optimized-projects-proof.json`（75行），SHA `f13b804f8a28697c9ecb1e7313c08b1564401bd41435b7471cb693f739f310f0`：

| Root工作版實測 | 可支持範圍及限制 |
| --- | --- |
| `cargo build -p duplicate-finder -p url-checker --release --locked --offline` exit0 | 根工作版lock／optimized3、overflow-checks=true的兩產品release build，紀錄附source及lock指紋；不是最終candidate checks。 |
| duplicate-finder對隨附duplicate-demo、`--json` exit0 | JSON契約true，stderr0；binary SHA `05f1bb90379895796b58df210675f7688d4cca10ab4c05a04469c4ab1f3815fe`。不是所有filesystem／平台故障重現。 |
| url-checker對隨附invalid-urls、`--json` exit1 | JSON契約true，stderr0；binary SHA `3f763b0fa97af9779ef4960ae8c61332eb8a4d2bce25b147d9ca2565a383280e`。純invalid輸入不送HTTP，不能當release真網路／SIGINT驗收。 |

Root proof標示built source來自原稿union v15；本輪final產品Rust未改，但candidate學生source／lock一致性仍須正式候選證明。工作lock SHA `dec09bef1aa193c4a0656ba9f2bc7a56edf5e7c8552bdec27e9e34d0ca5c060e`。本協調者只FULL讀JSON與核其SHA，未讀raw build log、重跑binary或發HTTP。

## B5作者交稿資料：只作 author-attributed 證據

作者正式handoff：`/workspace/work/tutorial-implementation/ch15-final-b-author/handoff.json`，SHA `940f353a3c44edc7862a9e5545f73ccce3d948f5da5490db0ab18ee51262692f`。本協調者核原檔SHA、讀selected checks／guided／CLI／line-count／visual／limits欄位，**未逐一重讀90份底層raw evidence或40張captures，更沒有自己run**。另FULL讀作者 `planning/source-notes/ch15-final-b-author07.md`（41行，SHA `ff626bf53752b9f1d9ed728c86b88110db4a51d922c4bb9081a274362a81cd9a`）及 `planning/author-ledgers/ch15-final-b-author07.md`（47行，SHA `d566a41d3c69b47f19093a5f43f13f68d49b6959ceacfa254869574c253b8056`）；其中外部英中教材／官方API閱讀是作者聲明，不算本協調者重新讀過URL。

| 作者交稿所述／handoff欄位 | 可引用範圍及保留限制 |
| --- | --- |
| 兩包fmt／Clippy、16 library與各包2 config integration | 作者記最終exit0；handoff保留最初Clippy101及後續修正後0，不把歷史失敗刪成每次皆綠。本協調者只讀紀錄，不冒獨立重跑。 |
| repo外guided起點與portable成品 | 作者以自己的manifest／各183-package lock，只有target cache共用；起點0tests，追加checks後3／6→5／4→6／3→9／0，portable 16＋2通過。這是作者驗收引導路線的執行證據，不是FIRST曾自行編寫／運行。 |
| 真binary的17項CLI案例 | 作者raw metadata均exit符合expected，涵蓋help／version0、配置／讀檔／Empty／UTF-8 fatal2、invalid text／JSON1、重複text相同bytes、真本機HTTP全過0／混合1／held timeout1、一般輸出失敗2、SIGINT130及SIGINT後輸出失敗仍2。未用公共網站固定答案。 |
| 兩項SIGINT與輸出錯誤 | 作者只在兩份held GET進入且未送headers後發signal，產品提交前已等listener entered；服務finally release／shutdown／close／join，17項均無host forced termination。`/dev/full`實測是該host範圍，不推所有OS相同。 |
| strict preview與40張實頁 | 作者報1280×900／390×844各20選定位置親看；CSS／圖／折疊有紀錄，手機code內橫捲。不是本協調者的親看，也不是全scroll／全裝置／正式部署。 |
| 最終行數／負擔 | 作者按physical Rust lines含空白及snippet標記：core825＋tests898＋Fixture198＝1921，不含lock／生成輸出／stage重複。新練習只補三動作，驗收／Fixture另供；不是一次要求手寫1921。FIRST有同一數量觀察，但未將它列N/O；Root已採入口口徑澄清，TECH／CONT與FIRST local已核該關係；仍非候選或發布證明。 |
| 作者限制 | 本次debug portable binary用於17 CLI；作者未做release-profile build或cargo install，install只核help契約。未重跑舊B1–4、未作公開網路可靠度／跨OS／全記憶體或任意Future取消安全保證，作者自檢不等獨立QA。Root另做的optimized release build與兩JSON smoke在前節獨立歸屬，不沿作者debug紀錄補算通過，也不回寫作者曾做release／install。 |

## 第14章已發布證據：與第15章分開

本協調者讀的是root既有量測JSON，沒有新發HTTP或Actions請求：

- `/workspace/work/tutorial-implementation/release14-live-http.json`，SHA `efaf68a52af581d60cbfad46bd2af75faaecbab24df5dcca34a756b0a90a25b0`：commit `680133581c1c9f1399b327389a3b1f7938c202ff`，首頁至14的99頁HTTP200／H1符合、九張14 SVG HTTP200／SHA符合；15.1路徑404是該次「尚未發布15」預期，不是15驗收成功。
- `/workspace/work/tutorial-implementation/release14-actions-jobs-success.json`，SHA `c8b5f8ca682d85415e5416a252e114e625ff49ece216d76746f100e45f4b4eb5`：[run 38670598992](https://github.com/birdhackor/learn_rust/actions/runs/38670598992) 的check／deploy皆completed／success，文件與Rust檢查及selected pure Rust Miri step真成功。這只支持該commit與所選路徑，沒有替15的originals、修正、完整CLI或發布簽核。

原九份FULL閱讀與root14參照留在 `work/ch15-qa-coordination-draft-v1/`；FIRST B5與作者歸屬在v2；TECH／CONT B5、Root處置／scope／release JSON在v3；本輪CONT final、FIRST LOCAL7、Root UI proof與原判保護在 `work/ch15-qa-coordination-draft-v4/`。本稿可續補候選／remote實測，本次只閉合review-only並交SHA後停止，等Root正式candidate／CI／public證據；不把文件處置閉合當作發布已完成，不增獨立QA。

## Root正式候選驗證

Root使用封存修後v16及明列的35項維護文件建立`release15-candidate`，沒有帶入既有ch01修改或ch11／12未提交sidecars。191份學生檔在working、index及candidate逐SHA一致；十二份原判、三份followup及Root disposition共16份維持封存SHA。workspace保原41項順序再加本章19項，共60 members；首頁加109小節，導覽含完整15章。

正式候選的`bash scripts/check.sh`實際退出0：strict文件建置、fmt、workspace all-target Clippy、all-target及doc tests均通過，498個正常Rust test target結果／479項tests零失敗，158個指定negative與C11正常3／指定compile-fail1通過。`release15-check.log`及`release15-check-counts.json`保存本輪結果；先前不同member順序的precheck另存`release15-precheck.log`，沒有拿它替代正式候選。

本機僅將dev／test除錯符號設0並關閉incremental以控制磁碟使用；opt-level、debug assertions、範例自訂profile及正式CI設定不變。這次Cargo test的benchmark target只屬驗收模式，沒有重跑原效能量測。`git diff --check`的兩處空白是兩份`invalid-urls.txt`刻意保留的index2單一空白行，須保留以驗原行索引，並非待刪排版錯誤。

同一正式候選的`bash scripts/check-miri.sh`實際退出0：固定`nightly-2026-10-01`使用既有sysroot，3個miri_contracts及1個owner答案通過，隔離負例命中指定freed allocation診斷。raw為`release15-miri-check.log`；故意UB只交Miri，正常host未執行。此證據限指定路徑，不推所有unsafe或平台皆已驗。遠端正常profile CI、nightly安裝／Miri步驟與公開頁面將在實際完成後追加。

## 公開部署與遠端Miri確認

教材提交`ce8275febe2a1a4eba943cd97b9f7ec7f37d28bc`已push。[Actions 38677655876](https://github.com/birdhackor/learn_rust/actions/runs/38677655876) completed success，check及deploy兩job與固定nightly安裝／Miri兩步驟均實際success。遠端使用正常profile，沒有沿用本機省空間的除錯符號設定。Root核run的head SHA精確，再請求首頁及109小節共110頁：全部HTTP200且H1相符；本章八SVG的HTTP200及SHA逐份符合v16。逐項結果保存於`release15-actions-success.json`、`release15-actions-jobs-success.json`及`release15-live-http.json`。

另核公開首頁實際包含15章／109小節及兩個引導型專案；15.11的原index0–4表格、空行／單一空白對應、Clippy旗標說明均存在，誤導的raw fixture複製區已移除，證據為`release15-selected-public-text.json`。兩個實戰均有可編譯起點、分步驗收、收合提示及可用完整答案；約千行指核心，不將全套tests／fixture算成讀者須一次手寫的量。

提交範圍檢查曾把未變更的既有`ch15-b.md`研究筆記也算入changed paths，Python保護檢查報錯但當次shell沒有失敗即停止，仍繼續commit／push。Root隨即逐檔核**全部1569個committed files**與通過測試的正式candidate完全一致，228個changed paths只落在明列範圍；191學生檔與16封存報告均維持SHA，證據為`release15-commit-proof.json`。這是範圍計數／命令保護錯誤，不冒稱保護檢查首次就成功；後續mutation命令使用失敗即停止。正式CI及公開驗證均綁定上述實際教材commit，不借其他版本。

本段只記這個教材commit的實測；之後僅補QA與進度文件不改學生稿，仍由正常push工作流程檢查。各原判、後文釐清、cross與LOCAL保留原角色與時點；AI審閱不等真人Python新手測試，所選平台／輸入的成功也不外推全部故障或所有OS。
