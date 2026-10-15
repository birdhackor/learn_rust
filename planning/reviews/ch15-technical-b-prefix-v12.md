# ORIGINAL TECH15：B-prefix，v12

本輪只審15.7～15.9，即B1單筆、B2依序批次、B3有限提交。三頁的正文、完整source與驗收材料可對上；本次**沒有已確認的必要修改項，也沒有已確認的可選修改項**。這不是whole B可用或第15章全章QA結論，B4／B5仍在本角色未讀範圍。

## 身分、輸入與隔離

- reviewer為 `/root/chapter01_foundations`，未寫第15章正文／source／答案。保留既有第1／8章、11.4及14.4／5／7／8作者背景；此TECH不能冒充基本Python讀者的無記憶首讀。
- 唯一新輸入：`/workspace/work/tutorial-implementation/frozen/ch15-b-prefix-for-review-v12`。manifest SHA-256：`18b90d116556eea43dd67b5a996b6decc17ac4b8235a764ee821f4b42116b25f`。
- 按001→002→003順序全文讀22 BODY（7／7／8），以及33 archives：B1／B2／B3的starter與complete共六packages，含完整lib／support／checks／README／Cargo／examples／答案與三圖XML。
- 33 paths有29種完整bytes；每種都全文讀完。自己驗全部副本完整bytes相同，才沿已讀代表對映；三份checks另各自證實是已全讀complete lib的**精確完整suffix**。不是抽樣，也沒有把近似版本當same bytes。
- 封存內60個實體檔始末SHA完全相同。三圖archive與assets亦逐bytes相同。B1 complete的core（test module前、只去尾端分隔空白）和B2 single.rs另有自己exact-byte證明；四份single副本完全相同，B3兩份input副本亦完全相同。
- A原判 `042f134ed1b349a78ea5e2717b0b15c26d9197368b3c2b60089a0bbf614c3189` 與原分級未改，只核SHA保護，不先cross。
- 未讀15作者notes／ledger／handoff、Root gate／outcome／disposition、CONT／FIRST15、B4／B5或15.10～15.11。未改student、global、staging或任何original。只在自己的work副本新增驗證材料及preview。
- clear-tutorial、review-protocol、術語與AGENTS沿前輪實讀，這輪核指紋完全未變；不是另讀作者說明來替正文補前置。另依managed cloud runtime指引核本輪環境；有限網路驗證只用自有loopback Fixture，沒有公開網站答案或外部服務依賴。

## 依真正需要核actual先備

本次重新讀 `ch11-review-fixed-v16` 的下列**實際BODY及其中source**，不是只看章名或Root宣稱。原先1～10與7的必要工具契約，沿本人的前輪實讀；14在本prefix沒有新增需要，沒有為湊範圍多讀作者記錄。

| first-use所需能力 | 本次實讀的actual位置 | 如何支持B |
| --- | --- | --- |
| spawn／Future Output的Send與static、mpsc、JoinHandle、abort再await、Id、JoinError、JoinSet／remove | 11.2 `002-001`～`002-006` | current-thread仍遵守spawn bound；task自己的Check／Err與JoinError分層；Id查回原輸入，不當穩定輸入編號。mpsc容量／clone／send／recv／關閉與unit材料均已教。 |
| pending IntoIter、JoinSet.len、完成待收仍占名額、new→fill／next→收取後補工 | 11.3 `003-002`～`003-006` | B3重用同一機制，改成HTTP task、完整Input登記與原行排序；沒有在此偷用全清單先spawn。 |
| Duration Copy、std Instant、u128毫秒、timeout／Elapsed、paused clock與std時鐘不同 | 11.4 `004-002`／`003`／`007` | B1以同一Duration跨度給每筆新預算，std elapsed不當虛擬10秒判準。我是11.4作者，重讀不冒獨立QA。 |
| URL absolute／scheme／標準化、reqwest builder／redirect／proxy、StatusCode Copy與2xx、Error查詢 | 11.6 `006-004`／`005` | B1先本機解析，再送GET；原文字與parsed／final Url有不同角色。 |
| async Client與runtime features、send為普通method形成Future、finalheaders與body界線、elapsed範圍 | 11.7 `007-002`～`007-005` | B1把get／send放async block內再交timeout，沿既有API；client prep與parse在外。沒有把blocking timeout契約搬到async。 |
| Fixture ready／動態port、gate、通知、hits／release／shutdown、Client Send/Sync與clone、pause/resume、is_redirect | 11.8 `008-002`～`008-012` | 後兩項明確在 `008-009`／`008-010` 先教，並非B1 tests偷用。shutdown的 `Result<usize,String>`、通知不等完成及client／server各自收尾也有真實實作。 |
| module、type alias、泛型／slice／排序、Clippy／manifest／path dependency／Feature | 1、4～7、9的既有actual；B1～B3正文新增處 | 支援型別仍是本package的型別；不因source搬module就假設跨crate同名型別可互換。 |

本地首次需要前也有足夠契約：B1具體 `Option<u128>` 比較與 **`&mut JoinHandle<Check>` 等待仍保有owner**；B2 `trim` 的Unicode判空、原文不改、Vec.is_empty與0-based索引；B3 Client clone共用pool、暫時allow(dead_code)、caller索引唯一遞增、abort_all不移除／不等待，以及next的具體cancel-safe等待範圍，均先說明再展示code。沒有用future的poll介面或後續使用者取消功能當先備。

## 主要通過項：原需求、機制、用途與材料

1. **B1先有可信的一筆，不混淆HTTP與傳輸結果。** 原需求是保存原網址，知道最後2xx是否通過，或在解析／scheme／request／deadline哪一步失敗。Url解析與scheme檢查先於網路；404／500仍為Http，redirect上限則沒有final Response。typed錯誤不先壓文字，Input沒有開始請求所以None耗時；Some(0)仍代表已嘗試。模型、helper、core與六checks各支持這些分類，而不是只靠demo印字。
2. **一份deadline包整份headers工作，範圍說得精確。** build_client未設定reqwest的total timeout，固定source預設亦為None；wrapper包GET建立／send／redirect到finalheaders。取status的Copy值及owned final URL後drop Response，沒有主動text／bytes讀完整body，也不承諾網路完全沒收body。std時間量到這份工作結束，不包含client建立／parse／報告；失敗也有自己的時間範圍。兩層Result用match總能形成一份Check，沒有用?抹掉一般項目結果。
3. **測試工作期限、觀察期限與task ownership分開。** 先真handler entered再pause，才有受阻headers的已知條件；10秒工作、5秒entered guard、20秒結果guard角色不同。借用handle等待到期不取消task，Err分支resume／abort／await／shutdown；取得Output後不再等待同一把handle。正常路徑也先收服務再assert。它驗具體可取消網路等待，不承諾搶占任意同步無限圈或撤銷server副作用。
4. **B2把資料身分建立在原行位置。** enumerate先於判空，trim只建立bool判斷視圖；留的是原line的owned String。空白／LF／CRLF／單CR、來源String結束後仍能用、重複文字各留一項、索引有空洞都由明確材料核對。Empty只是沒有非空行；全invalid仍有每筆Input結果。每圈await一份後保存、才取下一份，所以依序安排由程式結構建立，沒有用某次排程剛好如此作證。
5. **B3的jobs限制對象明確，而且真的先收取才補工。** pending只有資料，JoinSet追蹤submitted-uncollected tasks，正常完成但待收者仍占名額。fill每次檢len、只spawn一份、立即登記完整Input；next從正常Id或JoinError.id移除一次對照，組好Entry才fill。集合空與pending耗盡關係承接11.3。已收結果／全部輸入／原文複本成本仍隨清單長度增加，正文沒有說jobs限制全記憶體或保證全網路情況更快。
6. **task故障仍有原行與原文，卻不假造HTTP答案。** task正常Output是Check，Input／Request／Timeout不必panic。panic／abort沒有Check，所以Id表保完整Input，task另有String複本；Completion::TaskFailure保JoinError與original。合成材料透過mpsc先確認兩份task進入，再分別放行panic及abort；它們不是假HTTP請求。source最後確認兩個原index／original與is_panic／is_cancelled，正常caller不會因task故障丟掉身分。
7. **完成待收的驗收真正建立了完成條件。** current-thread中，empty容量1channel的單次send後沒有下一個await便return；owner取得通知時，該task已在同poll完成，但尚未經JoinSet收取。再fill不應提交pending，才驗到completed-uncollected，而不是僅觀察還阻塞的task。這項可能在空fill的stub碰巧通過，正文也明說不能當整份排程成功；本人有限實跑同樣如此。
8. **收尾流程不補新工，明示自己能處理哪些失敗。** abort_all只要求取消，drain逐份join到None，不呼叫會fill的Scheduler.next；另shutdown Fixture，再panic／assert。timeout取消next時，底層join_next_with_id cancel-safe，取回結果後沒有下一個await，因此不會取走成果又在中間丟失登記。此推論是具體實作及固定API契約，不推廣到任意Future；正文也沒有宣稱guards可攔任意錯碼同步panic。
9. **引導起點能讓讀者只接當步能力。** B1一個check分輸入／HTTP／wrapper三步；B2兩個body分讀行／依序；B3三個body分fill／next／runner。支援core已有、stub固定標記不當觀察結果、checks整module手動加入、repo外path先改、參考答案折疊及完整artifact入口均清楚。每個checks helper的用途／owned或借用／失敗收尾在需要前已有最低解釋；主文有要求與判準，提示關閉不會藏住首次需要的核心知識。此角色已見答案，不能將這項結構核對冒充首次盲寫試驗。

## 本人有限執行結果

在 `work/ch15-technical-b-prefix-v12/probe-workspace/` 複製B1 complete、B2 complete、B3 starter／complete，另用actual11 v16的local-service／http-single支援。B的Fixture path依正文改成自己副本絕對位置；prior兩package的workspace繼承由自己的小root明寫既有0.1.0／2024／1.99／publish=false。只有B3 starter按正文追加checks；complete Rust bytes與封存相同。沒有把negative加入repo正常targets。

使用 `cargo +1.99.0`、自己的offline／locked解析，selected reqwest0.13.5、Tokio1.53.3、url2.5.8、axum0.8.9。每個命令及exit code/log SHA在 `probe-workspace/probes.json`。

| 有限目的與所選test | 結果與不能擴張的範圍 |
| --- | --- |
| B3 starter追加checks後 `--lib --no-run` | 編譯成功；不是功能通過。 |
| starter `completed_but_uncollected_task_still_occupies_the_slot` | 1 PASS、6 filtered。這是正文明說的stub碰巧通過，不拿來放行starter。 |
| starter `zero_jobs_is_rejected_before_any_request` | **預期行為失敗**：exit101，`零 jobs 必須拒絕：Ok([])`；不是compile error。1 fail、6 filtered。 |
| B1 `whole_headers_work_expires_after_real_handler_acknowledgement` | 1 PASS、5 filtered；真handler成立後才測Tokio deadline，未以wall-clock數字判虛擬期限。 |
| B2 `owned_lines_keep_original_indexes_spaces_and_duplicates` | 1 PASS、5 filtered；owned來源／索引空洞／空格／重複／單CR。 |
| B3 complete `completed_but_uncollected_task_still_occupies_the_slot` | 1 PASS、6 filtered；有正確fill時仍不超交pending。 |
| B3 `collect_one_before_refilling_and_sort_only_after_collection` | 1 PASS、6 filtered；真held0/1→收1→補2→收2/0，再排回0/1/2與原文。 |

合計starter編譯、1個starter碰巧PASS、1個expected stub FAIL、4個complete selected PASS。未重跑unchanged全套／workspace／Clippy／benchmark／Miri；沒有UB負例。本輪未執行的redirect／全invalid／fault等case是完整source與契約閱讀通過，不寫成本人額外執行PASS。沒有把過去Root gate或作者結果加進本表。

## 固定官方API實讀

讀的是registry快取**固定版本**的selected docs／實作；不是下載全文就當全讀。逐檔SHA與行範圍存 `selected-api-proof.json`。

- reqwest0.13.5：Client derive Clone及內部Arc／共用pool；ClientBuilder預設total/read/connect timeout為None、no_proxy清除與停自動代理；send普通method回Future；Policy::limited的實際previous-chain上限（初始URL不算redirect）；is_redirect／is_timeout原因查詢。
- Tokio1.53.3：JoinSet.len／new，join_next_with_id的回傳／empty／cancel-safe，abort_all不移除、仍須join；`&mut JoinHandle<T>`等待的cancel-safe與drop-detach契約；pause/resume的current-thread／目前狀態要求、Tokio與std Instant差異、自動推timer。
- 這些官方source內部有比本prefix更低層的語法。它們是review核契約的材料，不是要求學生抄入helper或提前學poll／unsafe實作。

## 實頁範圍與限制

自己的preview用22 expanded BODY按原序重組三頁，去掉重複合成H1、改frozen SVG至本地同bytes。複製通用tutorial.css並以HTTP確認200且同bytes；三圖也親核served bytes與frozen相同。使用既有theme／extension、三頁nav、未啟用navigation.top；沒有更改shared配置。

第一次strict partial build因缺少六個**前章支援頁面**而停下。這不是student link缺頁判斷：本preview本來只放B1～B3。我只在自己的render副本將這六個前章link targets改成full-site外部URL，保存逐項變換；不改正文／code，也不測遠端是否已發布。第二次strict build顯示No issues found（0.21s）。因此這是縮小preview的render檢查，**不是整站strict或所有入口link驗收**。

1280×800與390×844共32張，**全部本人親看**：三圖各裝置原位＋元素12張；B1分類表、三預算表、borrowed handle契約、提示關閉／答案開啟，B2索引表／保原文，B3caller契約／completed證明／提示關閉20張。三圖的原文保留與解析早退、1/2/3索引不壓縮、完成待收len2及收1後補2／最後排0/1/2，均可辨認。B1/B2/B3圖文字未因窄版吃掉關鍵名稱，表格與摺疊能換行。

**實見限度：**desktop B3元素圖因圖高800而頂部被sticky header覆一小段；其desktop原位清楚顯示標題、手機全元素則完整，合看才能支持全部標籤，不說那一張單圖本身毫無覆蓋。code本來有內部橫捲，窄版右側不在截圖內不等於程式缺字。未檢每個scroll位置、每個折疊、dark mode、所有瀏覽器、鍵盤／screen reader、shared release或GitHub Pages實站。

## unknown、漏查及停止界線

- 目前沒有已確認N／O；不為湊問題把已教API重教當缺陷，也不由單次觀察推速度、全網路可用或產品取消政策。
- 本輪沒有判B4／B5的CLI／JSON／使用者取消／最終答案、總行數或整份B負擔，也沒有把A的N/O套到B。不得據本份宣稱第15章閉環。
- 我已讀全部答案且有async前章作者背景，不能判基本Python讀者第一次自行寫完的時間或卡點；FIRST／CONT另有自己的角色。source看得懂不等於已完成真讀者教學實驗。
- 只執行上表所列finite probes；不是全部19個complete tests、所有starter預期紅燈數或每個中間完成階段的逐步實跑。其他分支的支持來自本人的完整source／BODY閱讀，保留此證據等級差異。
- 本機loopback沒有驗公開HTTPS、DNS／憑證／proxy的每種系統組合、任意server副作用、所有同步阻塞／不yield錯碼或非Linux平台；正文也未作這些承諾。Guard允許有限環境觀察失敗，不是所有機器必在指定真秒數內成功的證明。
- 沒有用作者notes／Root summary或peer原判補缺失。開始的API／helper／圖文unknown，在actual11與官方契約、source、所列有限執行／實見後才關閉；過程保存在原始checkpoint，不回填最初已知。

證據根：`/workspace/work/ch15-technical-b-prefix-v12/`，含input／instruction／prerequisite指紋、29類source-byte-map、checks suffix證據、selected-api-proof、coverage-final、probes logs/lock、32圖manifest與6項prior-link變換。same-byte initial與報告SHA另存 `original-seal.json`。本ORIGINAL封存後STOP，等B4／B5新gate另授權，不先cross。
