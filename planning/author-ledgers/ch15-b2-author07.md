# 15.8 B2 作者 ledger

只寫15.8；實讀actual B1-v8七BODY＋whole兩pkgs、6.7 lines、11.4 Duration與11.8controls。fresh EN／ZH Book TDD及fixed1.99 selected契約見本節sources。共享Cargo／lock／nav由ROOT；原A／B1與frozen不改。

| 必要關係 | 第一次使用 | 前文／局部教學安排 | 可預測什麼 |
| --- | --- | --- | --- |
| 原行索引與保留後位置 | 第一份材料 | lines已教；本文新增enumerate，0-based對照第幾行 | 略空白後仍有索引間隔，重複原文仍不同項目。 |
| 空白與原文身分 | 第一行處理 | 本文先教trim借用／Unicode White_Space；只判空 | 原line有非空內容就完整保留，不用trim結果替換original。 |
| owned輸入與typed結果 | firstmodels | B1Check與Vec已有；Input/Entry各自先說角色 | text借用結束後Input仍有效；Entry以index包住B1 Check。 |
| Emptyfatal與item失敗 | firstinterfaces | 本文InputError::Empty與Vec::is_empty先教 | 零有效輸入拒絕；全invalid仍兩份結果，不以?抹掉某筆後續。 |
| 依序await與每筆期限 | runner任務 | B1check與11.4DurationCopy；本文圈內await後push | 完成一筆才取下一筆，錯誤仍append；每筆重新建立工作期限。 |
| Fixture有限驗收 | beforechecks | B1entered/borrowedhandle guards、11.8hits/release/shutdown；本文helper契約先教 | 原文字與結果位置可驗，gate收尾不猜sleep。 |
| 引導artifact | firststarter | B1literal manifests／repo外helperpath已教 | 零測試只證編譯；追加checks後真失敗，再分行處理與runner兩步完成。 |

source snapshot沿B1單筆core另保存single.rs，不引入另一crate的同名實體Check；學生只修改本步兩個函式。所有tests／提示／完整答案遵守相同先備。

## 完稿、實際操作與原始證據

正文 `docs/chapters/15-08-sequential-url-batch.md`、兩完整packages（b2-starter／b2-complete）、六項checks、README、唯一example `b2_batch_demo` 與SVG已完成。artifact不藏本步答案在starter：新兩body固定空Ok，單筆core是已完成B1的內容。

證據目錄：`/workspace/work/tutorial-implementation/ch15-b2-author`。

| 實際檢查 | 結果與範圍 |
| --- | --- |
| ROOT新兩members／lock | `b2-members-lib-check.log`，ROOT僅兩lib check EXIT0；未把此稱為alltargets／tests／QA。 |
| canonical fmt／Clippy | scoped fmt check無diff；all-targets Clippy warnings denied成功，`fmt-check.log`、`clippy.log`。初次demo多餘&格式化觸發Clippy，保留`clippy-before-format-borrow-fix.log`；最小去&，正文說明格式化借讀，affected Clippy通過，不重跑已通過的libtests。 |
| canonical complete／starter | `complete-tests.log`六tests通過，example target為零tests而非另一份功能驗收；`starter-zero.log`零tests；`demo.log`真本機四份結果，兩份200／Input／404，三份GET後shutdown。 |
| repo外完整引導路線 | `guided-route.py/json/run.log`明記cwd／command／actual exit／expected counts；literal manifest只改既有helper絕對path。starter alltargetcheck成功、0項；追加checks 0pass/6fail；只補prepare 2pass/4fail；full 6pass/0fail，outside Clippy通過。兩個故意失敗的原logs保存。 |
| source一致性 | `predecessor-bytes.json`：B1 gate九sources仍逐bytes不變；兩single.rs與actual B1非test core相同，只因fmt刪與舊tests之間的一個尾端分隔空行。checks整mod與complete相同，complete source與outside已通過版相同。第一次assert未扣除分隔空行而失敗，查diff只此空行，沒有誤報算法差異；修正proof比較範圍。 |

沒有rerun B1／11舊tests，也沒有把public網站當固定答案或新增工具功能。混合結果驗HTTP／Input／Request仍繼續與duplicate兩次GET；B1的Timeout工作核心原樣沿用。held test只記當下hits／pending、放行兩筆與對應，依序關係另核for圈內await後才取下一份，不把單一checkpoint當所有排程證明。

## 作者首用與圖文自查

enumerate／trim／Vec::is_empty在首次source前教具體輸入／輸出與必要政策；索引在略行前產生，trim只判空不換original。Input與Entry先教型別角色才展示SVG，避免圖先用未知名稱。圖說使用`Outcome::Input`區分輸入模型；提示1明回最初四行材料，pure讀行test先說first／second不須URL合法。

scratch preview嚴格snippets，沒有修改ROOT nav。`docs-build-final.log` strict build成功；BODY／SVG與preview逐bytes相同。親看桌面1280×900、手機390×844的入口、models、starter、guards、實作兩步、三提示／答案頭尾、圖／相鄰圖說共20原位圖；最後局部prose／移圖後另親看修後圖／圖說／提示與新增checks解釋共八圖，最終保存22張。各截圖可能只包含段落的一部分，沒有冒稱整頁每字都已在單張圖中驗。

SVG原生340×664、字18px，不依桌面欄寬stretch；0／1／2／3到保留1／2／3及兩份U的對應可追，Input問題仍保存結果。手機正常段落／表格換行，長code在block內水平捲動；render.json記頁寬不超viewport，SVG所有text bounds位於0–340。PNG實看與DOM記錄分開，scratch server／browser finally收尾。第一次preview tool因working directory尚未建立，在任何執行前拒絕；先建立目錄再正常strict build，沒有假稱首次已建置。

固定資料不需重做舊教學／新mirroring tests。這些是作者證據與ROOT precheck，非獨立首讀、全章QA或發布；formal fingerprints保存`handoff.json`。現在STOP15.9正文／程式，等待ROOT B2 actual gate。
