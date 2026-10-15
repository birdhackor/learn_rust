# 15.9 B3 作者 ledger

AUTHOR只開15.9。actual B2-v9七BODY＋whole兩pkgs、11.3-v6九BODY/whole、needed11.2-v5實讀及fresh EN／ZH完整章、fixed source選取見本節來源筆記。原A/B1/B2/frozen與globals不改。

| 必要關係 | 首次需要 | actual／本文教法 | 可預測什麼 |
| --- | --- | --- | --- |
| 依序→有限同時工作 | 入口材料 | B2圈內await、11.3三狀態；本節沿用兩slots圖 | pending還沒建立future；完成未collect仍占slot。 |
| 預設8／正jobs／Empty | constructor介面 | 11.3正上限、B2空清單；local ConfigError角色 | 在任何spawn前拒絕0與Empty，不等不存在的完成通知。 |
| scheduler owned client | firststate/fill | 11.2Send/static；local fixedClientClone契約 | task擁有pool共享把手，沒有短期外借；每筆budget重新建立。 |
| Id→Input rawmetadata | firstmodels/fill | 11.2Id與remove；本節原Input留登記、String副本入task | 即使沒Check的panic/abort，也保留index/original；duplicates各有task。 |
| 正常Check／TaskFailure | firstresultenum/next | B1typedCheck，11.2JoinError layer | HTTP/input/request/timeout有Check；TaskFailure不偽造HTTP時間或原因。 |
| collect→remove→fill | next之前 | 11.3已教因果與方法；本節同一Scheduler跨steps | 一份回收才補空位，不等同批最慢者。 |
| 完成次序／結果次序 | runner／sort前 | 6.7sort/usizeOrd；Input來prepare唯一原index契約 | runtime結果可先1再0；最後報告仍原行順序，不由Id排序。 |
| guarded觀察與完整cleanup | helper之前 | local abort_all與join-cancel-safe；owner保留JoinSet | guard失敗不遺失children；逐項await→server完才panic。 |
| 受控task故障與HTTP測試 | checks之前 | 11.2mpsc/故障材料、11.8真Fixture；分開兩種材料 | 人工panic/abort驗task層身份，不偽稱網路輸入故障；真HTTP驗slots/refill。 |

引導提供已教Input／prepare／single core與真正編譯骨架，新fill／next／runner需要自行寫；完整checks手動加入，分層提示與完成快照沿同一路徑。共享Cargo/lock/nav ROOT負責，新targets存在即通知。

## 實作與驗證範圍

正文 `docs/chapters/15-09-bounded-url-checks.md`、360×800 SVG、`b3-starter`／`b3-complete` 與README完成。ROOT已整合兩members與根lock；其兩library check只證編譯，不當完整作者驗收。ROOT最初offline metadata因平台cache缺檔失敗，後續metadata network成功；不把失敗改列通過。

作者檢查原始commands／exit見 work-only `ch15-b3-author/checks.json`：兩pkg fmt、all-targets Clippy、complete七lib tests、starter零tests、真HTTP有限demo均通過。七tests在raw log為7／0、0.11秒；這是該次測試結果，不是效能主張。demo保留重複URL及invalid，結果index1／2／3／4、HTTP200／Input／HTTP200／HTTP404，Fixture收尾確認三份GET。最初Clippy指出 `unnecessary_sort_by`，改用actual6.7已教的sort_by_key；原始觀察與後續完整raw logs分開保存。drain採既有while let，不偷用未知Option方法。

repo外獨立工作目錄真正走過：初始check／零tests→手動加checks（1過6失敗）→fill＋next（3過4失敗）→接runner（7過0失敗）→Clippy。保留每份原始失敗log與 `guided-route.json`，不把受控失敗算命令異常；完成時該目錄src/lib.rs逐byte等於complete，checks整module逐byte等於compiled module。完成快照合計742行Rust：排程／模型157、沿用Input29、沿用single104、tests／helpers418、demo34；含空行與snippet markers。各stage copies不累加成讀者作業，未擴CLI／JSON／使用者取消功能。

## 實頁與收尾時點

scratch strict preview build通過；正文與SVG逐byte等於repo現稿。final screenshots共40張：desktop1280×900與mobile390×844各20處，作者逐張親看入口、材料、models、完整圖／圖說、state、起點、cleanup／guard契約、合成task／完成待收實驗、三步引導、held／mixed／runner、demo、提示收合／展開、答案開頭／末尾。圖桌面360寬、手機約343寬，四步的task／Entry／input_index對應、完成未收取仍占名額與最後排序可讀；手機合理縮放，不依賴點擊放大。文字換行正常，長code需在其block內橫向捲動；截圖不是所有長code的全內容快照，也不是全站視覺驗收。提示與答案預設四份收合、可展開；DOM補查無頁面橫向溢出／未知HTML tag。原始initial screenshots仍保存，正式指紋只列final40。

恢復後依ROOT範圍完整讀11.8-v15的13個units，補教bounded ack／owned handle收尾，見source-notes時點紀錄；沒有改B3 Rust，也沒有重跑已PASS測試。Formal handoff保存所有student files／notes／proof／selected API／40screens SHA。本節目前是作者交稿，尚等ROOT完整actual gate及獨立QA；AUTHOR在15.9停下，15.10／15.11未開工。
