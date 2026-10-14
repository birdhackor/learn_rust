# 第11章 TECH 整章交叉與 v15 修後複查

## 角色、順序與固定來源

這是非作者 TECH11 的交叉與修後局部複查，**不是新的 ORIGINAL／blind first reading**。本人未寫第11章，但有14章工具作者、前置教材作者／審閱及正式API背景；自己的prefix／tail原判中已記可見歷史，本輪延續而不抹除。

自己的原判與同 byte initial 副本均未改：

- prefix `ch11-technical-prefix-v11.md`：`9d3d071f09f59016cb058983b816763d2da71782cd2d5b22b704384047a9b72d`。
- tail `ch11-technical-tail-v13.md`：`fdefe62ca564961c7e9d2fe3b95c69b96776928c9af7468580f077acfabbfe9e`。

先全文互看五份同行原判，修補工具截斷的未讀部分後，封存 own cross-before-fixed；再讀v15正文及sources、保存 own fixed-source 判斷；最後才讀root裁定。開工訊息已提示修正範圍與custom-assert，並非無提示發現。

| 階段材料 | SHA256／邊界 |
| --- | --- |
| own `cross-origins-before-fixed.md` | `ed3aef8252875f4325ce9d2c76b81c51c8eab79362d15a2b293aa569d4c83fc3`；先於讀v15 BODY/source與root disposition，原byte保留 |
| own `fixed-source-judgment-before-root.md` | `ad977353bbd956c845cc3a42bee947911ffa58b1238520ac4847fc8d40608d0d`；先於讀root裁定，記當時仍未看完實頁的邊界，不回補 |
| 唯一修後 snapshot | `/workspace/work/tutorial-implementation/frozen/ch11-review-fixed-v15`：8頁81 BODY、76 sources，包含9 SVG |
| v15 manifest | `6ffcc35ddcc04c2268c2d319f060df91550a142876d90f76091bec43ddbd9eb4` |
| whole frozen student material | `2f5c72d36557e58430bdf2dedf179d5ad4d9172cd78aa5033ee4ac89d0f39839`；算法及逐項SHA見own `student-material-fingerprint.json`／`.tsv` |

whole fingerprint算法為：81 BODY chunks與76 source archives逐項SHA，依 `(kind,path)` 排序，UTF-8每列 `kind TAB path TAB sha256 LF` 後再取SHA256。76來源已含9圖，不重複加圖計數。全部157項均對manifest；raw assets與對應archives也同byte。原snapshot未封存的其他共享檔不自行補成舊指紋。

本輪實讀受影響19單位：`005-009/011`、`006-003/007/009/010`及`008-001–013`。pages1–4／7 BODY與原先全文實讀版本相同；其餘66 sources及9 SVG byte不變，以diff/hash延續原判，不宣稱本輪重讀無變全部來源。另完整讀10份changed/new來源：HTTP lib／CLI test／status_line；local-service basic／lib／main／http tests／新absent_entered／closed_gate_solution／README。所有答案、helper、失敗分支均納入。

## 同行原判交叉，保留發現者與原分級

五份peer全文已讀，未以root摘要代讀。原hash：CONT-prefix `4529c857015481f5d31db4adbf31e44b03424b5a07bf1fbb2d9aa5d6c2bf9ad6`、CONT-tail `015c3767166eed36c2f688cebcb70015bf7ffa9e2248090a8eedf1bbc166ea4c`；FIRST-prefix `7b4b240c3418ff2bb90968325484e131bb0cfe571f7d9aaf14e2a3286777489d`、middle `2840c746ebe239364dff71780a37e3c9af01943035c45c1dc089c461f92d9831`、tail `c8f1ef0667df646d8607f096867a456ee55ee9271378c3ec93305a8640f24373`。

| 原項、當時理解與來源 | 本人原判／交叉判斷 |
| --- | --- |
| CONT-prefix N1必要：Scheme及摘要三處String／str異型相等未教 | **本人漏查**。4.2已教String的能力與as_str後借用文字比較，但同型PartialEq不能代答異型impl；FIRST閉合答案路線／未查source不構成此契約獨立通過。改既教view可消除額外能力而不岔出相等trait課。 |
| CONT-prefix N2必要：11.6獨立7.9入口尚未教Vec::is_empty | **本人漏查入口限定**。完整順序在11.5已有該能力，卻不能拿它替獨立同步入口補先備。FIRST-middle也已讀11.5，原未報不代表7.9路線已驗。slice.is_empty於7.4明教，改as_slice維持範圍。 |
| FIRST-middle單一bar未知／可選降低猜測待查前文 | 本人未抓首次使用。原11.5 main使用or pattern，11.6才明教；必要前文未找到相應基本機制。本輪確認需first-use橋接，**不把FIRST當時猜到兩failure共用出口，回寫成其原必要發現**。 |
| 本人prefix N1必要flush；CONT-prefix I2原可選一致性 | 同一責任關係各自獨立報出，但**分級不同**。本人人工Write probe只證render不呼叫flush，沒有證舊Stdout帶LF的實際CLI漏錯；CONT以library交writer仍可成立及main責任區分判可選。保留兩個原grade與證據限；採明確完成責任有收益，不用新測試冒稱舊真Stdout失敗。 |
| CONT-prefix I1可選：早期HTTP表格等待對象過寬 | 本人未列。Response/text實際使用前已充分介紹，故原body／Request機制通過仍成立；初始表格寫窄可降低把body錯誤讀成未取得Response的負擔。 |
| 本人prefix I1可選：manifest-path名稱對manifest用途 | 原fixture／空workspace角色已教；補選項定位是小收益。本人先前14章CLI作者背景已公開，未稱blind，不重講已教path dependency。 |
| CONT-tail N1必要：Client Send／Sync晚於首次spawn | **本人漏查取得知識時點**。原expert查正式API正確不代表讀者當下知道；owned move不創造Send，inspect跨await的&Client又用到Sync。FIRST原未報不能代答trait契約。 |
| CONT-tail I1可選：selectedServeFuture與Output移交 | 本人原核正式API，未提出這項呈現改善。IntoFuture本身不保證Send；原稿限定選定service而非所有轉換皆可spawn，故保留原可選分級。 |
| 本人tail N1必要：Request提早失敗未解除entered等待 | 原own server-abort／hits0有限probe獨立成立，CONT/FIRST未報。健康gate與cleanup通過仍有效；它們未驗Rust故障支線，不能當故障重現。新/loop測試不是本人的舊server-abort probe。 |
| 本人tail I1可選：手機表格拆method識別字 | 原完整API可由主文／code讀出，故並非必要遮斷；表格辨認改善有收益。CONT-tail其他圖/fold畫面未涵蓋本指定表格，不據此取消實際觀察。 |
| FIRST-tail IP全名未知 | 原loopback位址／port角色已能理解，無必要缺口。補全名有小收益；不把未知變成FIRST原必要或可選新發現。 |

FIRST-prefix／middle／tail的逐單位理解各自保留。三者沒有讀全部archives、沒有Rust執行，也未看實際桌面手機頁；SVG為獨立390寬圖。middle保留fold數的早期筆誤與後更正、額外AGENTS／術語可見聲明；tail保留最後單位傳輸恢復的時點，不叫新盲讀。CONT有實際頁面／圖文證據，沒有Rust實跑；本人的original selected probe與實頁不能轉算為它們的執行或首讀證據。本輪未重算獨立發現得分，不補算後見同意。

## 修後首次需求、機制與用途

| v15位置及最強來源 | 技術、先備與負擔核對 |
| --- | --- |
| 005-009命令前；005-011第一個OR code前 | --manifest-path直接連到選定fixture的Cargo.toml，並指定package目錄為相對命令起點；原空workspace隔離不變。單一p1 \| p2說任一形狀共用branch，與1.6的boolean `\|\|`分開，只教本次需要。 |
| 006-003三結果表 | 改「等待最後回應」準確限定Request出口；404仍Ok報告，後文Body variant仍保留已取得status，沒有用表格抹除read-body工作。 |
| 006-007 render／run；006-009完整RefuseFlush | render最後交flush Result，run包AppError::Output；flush拒絕傳至main作業失敗。write完整保存bytes、flush PermissionDenied的控制writer先於test介紹，確認接收bytes與本節輸出完成不同。7.4的Error::from(kind)、slice.is_empty及7.9的Write責任足夠；for byte借讀＋*byte／Vec.push均已教。沒有引入新測試helper API。 |
| 006-009 Scheme／stderr；006-010摘要題與完整答案 | 三處String先as_str，沿4.2 `002-004`借用文字PartialEq。stderr使用as_slice().is_empty，沿7.4 `004-005`，不依賴11.5入口。閉合答案外也說保存String、借讀再assert，學生不用偷看答案才知道做法。 |
| 008-003 basic與選定server Future | spawn前說所選axum服務轉出的future有Send+'static，io::Result Output可移交。正式axum0.8.9 `WithGracefulShutdown::IntoFuture`選`ServeFuture`、Output io::Result；io::Error有Send。沒有把所有IntoFuture宣稱可spawn，也不印內部Box／Pin／tower當新先備。client及固定URL先準備，再spawn；保存HTTP結果、通知與server.await後才expect／assert。 |
| 008-006 raw next_started／bounded helper／finish_client | 原無期限等待與Fixture保留Sender明說，Request錯不會自動變None。wrapper把11.4 timeout套到借用recv，教Result<Option<usize>,Elapsed>三形狀；取消這份等待不取消client或丟Fixture，沿11.5 recv取消安全。5秒entered觀察與10秒HTTP預算不同起點／目的；不保證所有機器5秒成功。finish_client<T>只封裝已教owned JoinHandle abort＋await，已完成仍保留T Output，先教再用。 |
| 008-007 start；008-008首次client spawn | ready URL解析移到spawn前；ready失敗保存原因並shutdown後才回診斷。Client Send／Sync在第一個spawn之前，接10.1 `T:Sync`等價`&T:Send`及11.2的Future／Output要求；async move只交ownership。Url與報告／錯誤owned能力同處交代。Clone共用pool仍留008-010首次clone前。 |
| 008-008 main；008-010雙gate／期限；008-012取消；008-013答案 | entered每次最多5秒，失敗分支先finish_client所有owned handles，再Fixture.shutdown，最後報結果。雙gate兩份把手均收；release失敗也先回收。正常路線亦client await→shutdown→assert。兩份Fixture的第二份啟動失敗先收第一份；雙數量/範圍檢查不偷掉剩餘owner。shutdown與client等待各有把手／完成對象，503是Response而不是Request失敗。 |
| 008-011新absent_entered全文及新archive | /loop的有限redirect Request錯不進held，仍保留entered Sender；容量1 mpsc結果通知有15秒timeout。current-thread task先算HTTP Result，send之後沒有下一await即回傳，收通知後才pause，避免把自動跳時鐘套在尚未完成真HTTP。5秒entered取得Elapsed後resume，finish_client保留原Request/is_redirect，再shutdown，最後驗server6hits。這是指定有限失敗材料，不宣稱所有server故障或所有排程都經形式證明。 |
| 008-008 stdout解釋及圖說 | stdout明說「收尾後列出的步驟紀錄」，不暗示印第一行時gate仍關著。正常SVG仍是ready→handler entered/hits→通知→release取得Response/body→shutdown await；圖不按高度量時間，也沒有把ready或send成功等同工作／雙方完成。 |
| Root後見customassert：main與雙gate | 兩處確實回到已教if＋panic；原所有結果Debug可顯示，沒有新的assert第二參數介面。此項只算root修後查漏，不算任何角色原始重現。 |

本輪重新讀必要前文：4.2的String／&str比較段、7.4的ErrorKind/From/is_empty段及10.1的Sync/shared reference關係；例子及tests不用outline當已教。完整changed sources與BODY片段對應，CLI期望stdout仍是三行舊文本，但其現在出現的時點已就地說明。

未報問題的通過項也保留並核其範圍：原future工廠準備與lazy工作區別、兩層Result及ID、submitted-unreaped上限與補工、timeout不回滾／不搶佔、停止分類及全部把手收尾、HTTP最後Response／可選body及JSON不同工作均沿無變原來源。尾段以handler entered而非固定sleep提供測試條件；hits是進入計數，排除ready／extractor事先拒絕，不當成活躍或成功數。正常圖的entered、release、client完成、server完成四個不同觀察也仍可逐一對回source。這些是有對象的通過，不是所有可能故障／完整HTTP協定已驗。

## 正式API、執行與實際頁面

本輪另核官方固定版：std1.99.0 io::Error的Send/Sync、reqwest0.13.5 Client Send/Sync與clone共用pool、Tokio1.53.3 JoinHandle::abort對已完成task保留結果。HTML/URL/SHA保存在own `official-web/manifest.json`。沿original固定cache核Tokio1.53.3 timeout／mpsc recv cancellation，以及axum0.8.9 selectedServeFuture與Output；沒有混用latest或另一版API。

**本輪未執行Rust tests、沒有新probe**。original prefix人工Write實驗、tail server-abort及一項正常headers-held期限測試的版本／own lock／結果仍在原判中；不冒稱它們跑過v15。root裁定記prefix selected checks、尾段8＋1及最後CLI／dual filtered PASS，是協調者提供的版本紀錄，本人沒有拿其敘述變成自己的執行。無變全workspace／公共HTTP／無限pending／deadlock均未跑。

建立own exact-v15 preview：8頁從81固定chunks按序組合、9圖同byte、固定CSS SHA `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`；Zensical locked0.0.70 build成功。初次capture一個文字selector不符，修正後22張capture成功；環境restart令own server消失，重啟後再補4張。這些是審閱工具恢復，不算教材失敗或Rust證據。

**親看26張原位PNG**，desktop1280×900、mobile390×844，各13張：

- 11.5 manifest定位與OR機制在相應命令／code前。
- 11.6 URL部位表與真正三結果表、render flush、比較／Write測試契約、閉合summary答案。第一次表格capture為原URL表，後來補真正變動的三結果表，沒有錯把第一張當受影響表格。
- 11.8 helper表上／下半、bounded觀察解釋、Client Send/Sync於spawn前、原位正常gate SVG、absent-entered機制、閉合closed-gate答案之前的練習與兩把手說明。

手機method名現在完整，next_started_within不從中間拆字；表格高於一屏，以兩個實際位置核完，沒有假稱單一畫面看完全表。說明文字正常換行，頁面無橫向溢出；程式長行保留可橫向捲動的code區，不靠放大讀核心說明。新Result型別參數在HTML正文完整可見。SVG340×572於兩尺寸原位，五步／箭頭與handler、client、server作用可逐一追蹤；圖後的通知／結果各有範圍清楚。必要契約與練習條件位於閉合折疊外。

這是受影響區塊的局部實頁，**沒有重看整章所有viewport／所有展開折疊**。其他8 SVG在original prefix既有兩尺寸實看及byte未變，沿原範圍保留；不把26張新畫面叫全站視覺驗收。視覺位置/數值/closed狀態見own `visual-capture.json`、`visual-capture-extra.json`及26個PNG。

## 最後另層比對與剩餘項

保存own fixed-source判斷後才全文讀 `ch11-root-disposition-v15.md`。採納方法與本人的來源核對一致：原grades／出處／漏查保留，flush沒有真Stdout失敗重現，新/loop不回填舊server-abort，root後見customassert獨立記時點。root裁定不是本人的另一份original；它當時「cross／實頁仍進行」的狀態也不回改。

**後見可選編輯 C-I1**：v15 `sources/examples/ch11-async/local-service/README.md:5` 的短引文是「取得Result<Option<usize>,Elapsed，」，缺最外層最後 `>`，且未包inline code。已教BODY008-006及真fn完整簽名均為 `Result<Option<usize>, Elapsed>`，所以沒有新的必要先備／API錯誤，也不撤銷主線通過；但repo README單獨定位此方法時可避免型別文字不完整。最小建議只改成完整inline code，不增加新解說。這是本輪cross補讀source才列、已通知root，**不是原TECH／CONT／FIRST發現**；本人未改教材，v15指紋仍指該固定字節。Root隨後核對並採納此可選來源格式修正，等本輪各角色封存後另出v16來源byte proof；本報告仍限v15，未提前核不存在的修後來源。

v15已核必要項及受影響過渡通過，沒有新增必要缺口；可選README字誤已採納、來源修正待另版核對。此結論限本章指定修後來源及本輪局部驗證，不宣稱新盲讀、全套Rust重跑、全站視覺或已發布通過。封存本報告及SHA後停止，不讀12 peers或任何新章scope。
