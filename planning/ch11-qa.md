# 第11章 QA 閉環與發布待辦

第11章七份原判、兩份cross、FIRST局部複查與TECH單README複查已封存，審閱／交叉／局部修正閉環。**review／Root指定實頁與發布候選完整check已通過，公開發布仍待完成**；目前不能寫已發布或候選全套PASS。本文件是協調整理，引用各封存角色及root處置，不算新獨立QA，不修改教材、原判、cross或Git index。

## 封存報告

七份original保留其各自版本／角色／可見背景。FIRST是AI輔助順序讀者模擬；TECH、CONT均非第11章作者，但有已聲明的其他章作者／工具背景，不能叫完全無提示的新手blind讀。FIRST前兩份依自己的v7／v10路線，兩專家prefix依v11，三角色tail依v13；不把各版本BODY當同hash。

| 角色／範圍 | 封存path | SHA256 |
| --- | --- | --- |
| FIRST prefix（11.1–4，36單位） | `planning/reviews/ch11-first-reading-prefix-v1.md` | `7b4b240c3418ff2bb90968325484e131bb0cfe571f7d9aaf14e2a3286777489d` |
| FIRST middle（11.5–7，32單位） | `planning/reviews/ch11-first-reading-middle-v1.md` | `2840c746ebe239364dff71780a37e3c9af01943035c45c1dc089c461f92d9831` |
| FIRST tail（11.8，12單位） | `planning/reviews/ch11-first-reading-tail-v1.md` | `c8f1ef0667df646d8607f096867a456ee55ee9271378c3ec93305a8640f24373` |
| TECH prefix（11.1–7） | `planning/reviews/ch11-technical-prefix-v11.md` | `9d3d071f09f59016cb058983b816763d2da71782cd2d5b22b704384047a9b72d` |
| TECH tail（11.8） | `planning/reviews/ch11-technical-tail-v13.md` | `fdefe62ca564961c7e9d2fe3b95c69b96776928c9af7468580f077acfabbfe9e` |
| CONT prefix（11.1–7） | `planning/reviews/ch11-continuity-prefix-v11.md` | `4529c857015481f5d31db4adbf31e44b03424b5a07bf1fbb2d9aa5d6c2bf9ad6` |
| CONT tail（11.8） | `planning/reviews/ch11-continuity-tail-v13.md` | `015c3767166eed36c2f688cebcb70015bf7ffa9e2248090a8eedf1bbc166ea4c` |

四份followup保留後見時點，不能補算original獨立發現：

| 角色／範圍 | 封存path | SHA256 |
| --- | --- | --- |
| TECH cross v15 | `planning/reviews/ch11-technical-cross-v15.md` | `13fd5004878ae6fab80491b8ac4b42a7f4531bb99a16fe6e43be4e2ad54664cc` |
| CONT cross v15 | `planning/reviews/ch11-continuity-cross-v15.md` | `1e5425d212ba204076580f07c8821e15c12b46b642975ff5ede4a73354485305` |
| FIRST LOCAL v15（19單位） | `planning/reviews/ch11-first-reading-local-v15.md` | `326832fb540116510151ea0ebfac9ec2178c852899865cb34f12a03ba6ffaa24` |
| TECH README LOCAL v16（單來源） | `planning/reviews/ch11-technical-readme-local-v16.md` | `f804751a6d1f03fbde811d0507c571b7fe3d4b6484ff76326138f505cb292c0e` |

原判均先各自封存，root全文讀後才開互看／修後；TECH與CONT先保存自己的交叉判斷及fixed-source核對，最後另層讀root處置。FIRST LOCAL逐單位保存四問及原句、答案維持閉合；不是另一份whole original。

## 修正、分級差異與證據邊界

| 項目與原發現來源 | 最小修正／保留界線 |
| --- | --- |
| flush：TECH-prefix原必要，CONT-prefix原可選一致性 | render明訂寫入與flush一同完成、run保留Output錯誤；controlled Write接bytes而拒flush。原人工writer只證不呼叫flush，沒有真Stdout失敗重現。兩個原grade及理由不改。 |
| 三處String／str異型比較：CONT-prefix原必要 | 用已教as_str再比較，未另增異型PartialEq課。TECH原漏查保留，FIRST閉合答案路線也不算該能力已驗。 |
| Vec::is_empty：CONT-prefix原必要，限11.6獨立7.9入口 | 改as_slice().is_empty沿7.4。完整11.5→11.6原已有Vec method，不把入口限定擴大為所有順序讀者缺先備。 |
| 單一bar：FIRST-middle當時未知／可選降低猜測 | 首次11.5 main前教or pattern任一符合共用branch，與boolean雙bar分開；不回填為FIRST原必要發現。 |
| manifest-path／HTTP早期表格 | TECH-prefix旗標原可選、CONT-prefix窄表原可選：就地定位fixture manifest，表格限定等待最後Response；body工作仍另教。 |
| Client Send／Sync：CONT-tail原必要 | 首次client spawn前說選定Client能力及shared reference所需Sync；owned move不創造Send，Clone共用pool仍在首次clone前教。TECH原漏查時點保留。 |
| selectedServeFuture／helper表／IP | CONT-tail選定Future能力原可選、TECH-tail手機method拆字原可選、FIRST-tail IP全名原未知。選定Future與Output能力橋接、完整短method名及IP定位補充；不把一般IntoFuture等同任意spawn，亦不把IP未知升級。 |
| entered缺席：TECH-tail原必要，CONT／FIRST原未抓 | 保留raw next_started無期限語意，增加5秒bounded觀察及先收全部owned client handles、再shutdown、最後檢查。原TECH私有server-abort／hits0實驗、健康deadline測試與其他角色的健康流程通過各有範圍。 |
| 作者修後新/loop regression | 新absent_entered用真redirect Request錯、不進held通知；確認HTTP Result已算出才pause測觀察期限，resume後保留client錯誤並收尾server。這不是原server-abort重現，也不算CONT原獨立抓到失敗出口。 |
| Root修後customassert查漏 | Root發現兩處第二訊息參數未教，改回已教if＋panic；不是七份original的新獨立發現。修正code與stdout收尾後紀錄時點均在followup核對。 |
| v15 CROSS新可選README字誤 | TECH C-I1及CONT I-new在cross補讀source指出型別缺最外層結尾括號／code span，BODY與真fn本來完整。Root等三份cross/local封存後修為inline code `Result<Option<usize>, Elapsed>`；TECH v16單README通過，不後見升級成原必要項。 |

跨角色未驗範圍保留：FIRST original／19單位LOCAL沒有Rust、完整archives／答案或實際桌面手機驗收，只在到該段時看孤立SVG；原未知與逐段恢復時點不改。TECH原prefix有std-only控制writer，tail有有限server-abort與一項正常headers-held期限實跑，不能泛化所有故障。TECH v15有19受影響單位、10 changed/new來源與26張1280×900／390×844實頁；CONT v15全文81 BODY並核10變動來源，54張1280×800／390×844選定實頁。兩份cross均未新跑Rust，圖／折疊與其餘無變資料依各自原證據延續，不宣稱全站所有畫面或真人學習效果已驗。

## 最終學生版本與執行範圍

最終學生來源：`/workspace/work/tutorial-implementation/frozen/ch11-review-fixed-v16`，8頁81 BODY、76 sources、9 SVG。manifest SHA `564f274334c43f866892eccd33272c3fb5f7036bc5028f409526355b538f107c`。v15→v16只有local-service README changed，SHA由 `dc7cd5dd0a11312ad68212f457c85126ac1401ba9bdbdba50ce676d3f8959cbe` 變成 `4ab13b311a60c03dc494942e20fc0a2671cc01ccba1d4e75c587fa983917a3fc`。BODY中的凍結asset絕對目錄正規化後完全同byte；Rust／9 SVG完全exact，不能把raw資產目錄不同算學生內容改寫。範圍proof為 `ch11-readme-v16-scope.json`，TECH單檔local重核指紋及原fn，不重跑Rust或新實頁。

作者／root selected執行依 `planning/reviews/ch11-root-disposition-v15.md` 保留版本：prefix scoped fmt／Clippy、四項library與兩項CLI tests、status_line已實跑；tail第一輪修正原八tests加新absent regression已實跑。最後兩處if／panic僅再跑CLI與dual各一項；其餘六HTTP及absent regression未在該末次改動後重跑，fmt／Clippy與最終引用source strict更新通過。這些不是新增獨立QA，也不是候選完整check的證據。未以公共HTTP固定答案、全WIP workspace或無限pending／死鎖重跑補數量。

root處置v15文尾記的是當時cross／實頁尚進行，原文不改；本彙整追加如今review/cross/local閉環，root發布驗證仍pending的狀態。README版次追加不改原v15 CROSS SHA或26／54畫面版本。

## 下一步

- Root全文核本彙整與signoffs JSON，完成剩餘修後實頁。
- Root建立與固定v16相符的發布候選，再做完整check及學生byte／依賴／原判hash核對；本任務未staging或執行候選工具。
- 候選通過後由Root按使用者既有授權發布，再另記Actions與公開HTTP／遠端SVG核對。現在尚無第11章候選PASS、發布commit、Actions成功或公開頁面的證據。

`/workspace/work/tutorial-implementation/release11-signoffs.json` 的 `closed: true` **只指review／cross／local已關閉**；候選與發布欄位均pending。列七份original與四份followup的相對repo path／SHA，格式符合既有候選工具的只讀hash契約。本整理不執行該工具、不改Git index／Cargo／nav／首頁／workflow／scripts或其他章scope。

## Root後續實頁與候選前置（2026-10-14）

Root已全文核所有七original、四followup及此彙整；readme局部通過的原SHA保留。另以final current11正文/source與現有CSS建立自己的strict preview，build退出0。親看1280×800／390×844各七處、共14文字位置及正常gate兩處：manifest定位、or pattern、render write＋flush、測試能力說明、5秒／10秒與borrow取消區分、selected serverFuture／Output能力、absent通知與時鐘條件。client-bounds檔名實際選到server能力段，本紀錄如實限定為server段，不把它冒稱Client能力新畫面；Client先用位置及helper表已由TECH26與CONT54原範圍核。14處fold均closed、DOM無整頁水平溢出；長code僅block內可橫捲。gate五步全可見且與圖前正常路線限定相鄰。Root這次不重看其餘八SVG，沿原actual gate與無變SHA保留。這些是指定局部，非整站每viewport驗收。work-only root11-fixed-text-views.json與root11-gate-figure-views.json保留原圖、metrics。

候選完整check／公開部署仍待實際執行。

## 發布候選完整驗證

候選release11-candidate只含1–11章79小節及首頁，84個本章學生檔在current／INDEX／candidate皆精確相符v16；七original四followup SHA仍相符。Cargo只有已審members及候選獨立lock；C checker與Miri CI尚未納入。首次全check因磁碟ENOSPC停止，失敗raw log保留；只清可重建的舊候選target後重新check。第二次退出0：strict／fmt／all-target Clippy／all-target與doc tests、120個指定故意失敗fixture均通過。359個target結果、198個通過tests、零失敗（不以數量證真人學習）。

公開部署與遠端HTTP仍待完成。

## 公開發布確認

已提交並push 36df1675673e5c2c743524699f6071e15555c803（本機branch work，明確HEAD→remote main）。Actions [38655225227](https://github.com/birdhackor/learn_rust/actions/runs/38655225227) check／deploy及全run completed success。Root實際HTTP首頁＋79小節共80筆皆200/H1精確，9SVG SHA逐項符合v16，12.1仍404。release11-live-http.json保留逐頁資料；複用helper最初commit/run兩metadata欄仍是第10章，後以本次actual successful run校正，HTTP量測不改或冒稱重跑。
