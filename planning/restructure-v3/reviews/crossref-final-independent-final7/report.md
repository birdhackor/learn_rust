# 最後7頁：獨立只讀引用核對

本輪只審目前canonical 1.7與18.7–18.12，七頁全文實讀，共1693行、168個詞法命中，分布123行；另逐一核對35個link label（17個canonical導覽、10個SVG alt、8個外部延伸）。未回寫前一份91頁初判，未讀他人finding作提示，未修改repo/source/nav/shared，未Cargo／測試／spawn。

**必要錯指0；可選精確化1。** 本輪沒有發現1.7使用超出1.1–1.6的早期必要程式，也沒有發現B路線把18.11三步引導與18.12完整答案指反。以下是範圍與證據，不是整本課程或所有Rust closure的重新技術驗收。

| 頁面 | 行數 | 詞法命中 | 命中行 | links |
| --- | ---: | ---: | ---: | ---: |
| 1.7 | 115 | 13 | 12 | 6 |
| 18.7 | 259 | 32 | 20 | 6 |
| 18.8 | 244 | 25 | 19 | 4 |
| 18.9 | 281 | 24 | 18 | 4 |
| 18.10 | 306 | 28 | 20 | 8 |
| 18.11 | 324 | 36 | 24 | 3 |
| 18.12 | 164 | 10 | 10 | 4 |


## FINAL7-O1：可選，分開Duration來源回指與當頁Copy說明

位置：`docs/chapters/18-08-lesson.md:53`。原句：

> runner借用同一份Client，取得每筆期限Duration。沿14.5，Duration具備Copy，可以把相同跨度交給每次B1 `check`；**每筆開始時各建立自己的期限**，不是整批只共用一個十秒倒數。client建立失敗仍在開始批次前處理。

實際目標`docs/chapters/14-05-lesson.md:9`：

> 標準庫 **`std::time::Duration`** 表示時間長度；`Duration::from_secs(1)` 建立1秒，`Duration::from_millis(20)` 建立20毫秒，1秒等於1000毫秒。`duration.as_millis()` 回傳總毫秒數，供本例顯示，會捨去不足1毫秒的部分。

14.5教了時間跨度與操作，正文沒有Copy一詞。18.8當場已說明此標準型別具備Copy，先前6.3也已教Copy概念，所以這不是必要先備缺口。只是「沿14.5，Duration具備Copy」可能讓人回14.5找具體Copy說明。

最小可選改法：將該短句改為「Duration是14.5已教的時間跨度；這個標準型別具備Copy」。這能明分回指和當頁補充，不需新增API或改Rust。root已回覆採納，由root落盤與窄核；本審閱者不改repo。原句、真目標與目前稿的exact path／line／quote／SHA在`exact-findings.json`，保留初判而不覆寫成最初無問題。

已只讀重看修後18.8:53，確認root已改為「Duration 是 14.5 已教的時間跨度；這個標準型別具備 Copy」。`post-fix-review.json`保存修後SHA；可選項已處理。

## 1.7只使用1.1–1.6作概念先備

- 1.7:3的subtotal(120,3)→360，能在1.6:21及:35找到呼叫、參數與回傳數字；1.6:79–81已區分普通block與函式呼叫／scope。
- 1.7:19回1.4的i32表示空間：1.4:33確實說型別名稱數字是bit數，並預告1.7。bit／byte／4bytes的必要解釋由1.7當頁提供，不靠未來章節。
- 親讀memory.rs、memory_answer.rs與完整intro-steps manifest。實際程式只有main／fn／i32／let／數字乘法／println／尾端expression；註解語法在1.7當頁先解釋。沒有reference、String、Vec、判斷或實際heap配置。selected命令的manifest-path／-p／--example／--locked均在1.3已教。
- frame、stack、heap、pointer、allocation／free都在當頁由角色與圖解建立。heap明細圖是概念材料，不以未教型別程式當必要答案。1.7:90的第3章ownership是未來預告；3.2:7–11真的教持有／交接／清理責任，3.8:21承接文字heap資源，沒有倒過來當1.7的必要先備。
- 五個foundations SVG已讀title／desc與全部文字節點，對應A/B/C、P/Q、H1/H2及同一H1生命週期。下一節link的label tuple／array對應實際1.8正文。此輪未做實際頁面視覺驗收。

## B路線與18.11／18.12的實際分工

- 18.7的單筆與期限，18.8的原行身分／依序批次，18.9的JoinSet名額與完成待收，18.10的停止／Cancelled／Unstarted，逐節前後指引均能在下一個實際正文與normal source找到。沒有把測試cleanup當正式停止功能。
- 18.9／18.10的18.11前指確有CLI、文字／JSON、退出優先序與明確輸出完成；18.11標明「先做三小步，18.12另給整份工具」，相鄰link label也一致。
- 親讀b-final-starter的report/app，確認三個未完body是build_report、Report.status、write_report；read／client／signal等core起點已提供，沒有藉答案偷載另一個future library。
- 親讀正式report/app全檔，確認同頁兩份答案真的包含empty_row／row／映射、狀態與write_all→flush；final的兩個test module與starter兩份checks逐字相同（五加四），可以按18.11指引替換，不需從18.12補缺。
- starter root沒有mod test_fixture，batch也沒有七項tests；正式root才用cfg(test)載fixture，batch七項＋report五項＋app四項＝16，config另2。18.12的16+2前指符合實際檔案，不把未選target或零tests當功能證據。
- 18.12同頁包含目前工具目錄所有Rust、manifest、lock、README與fixture檔，靜態比對沒有漏檔；final batch normal body沿B4，main相較starter只改crate路徑。完整程序仍是headers工作，fixture ready才讀小body，兩種責任沒有被引用混淆。

## 其他目標的語意對照

- 14.5有Duration／stdInstant／timeout／合作式邊界／test-util；14.8有async Client共用pool、普通send future工廠及完整HTTP core；is_timeout雖首次介紹在14.7，14.8真正內嵌的error.rs也使用它，18.7回14.8不構成空引用。
- 14.3有Send/static spawn／Id／JoinError；14.4明說完成但未取回仍算tracked；14.6有S1/T0/W1、先查停止再fill、abort後drain、ctrl_c entered及listener回收；14.9有動態listener／ready／gate／entered／shutdown。B的相應回指對應這些真正段落。
- 9.6的lines、CRLF、單獨CR、is_empty、owned文字；10.9與14.6的enumerate；9.7的sort_by_key；10.7／10.9的Serde String/bool/usize/Vec；11.7的Error/source及17.4的non_exhaustive/pubuse都有實稿支持。u16/u128／Option JSON與trim/by_ref/default_value_t是B當頁補充，沒有假稱全在前文教完。
- 35個link label均按內容核對。8個外部target已實際取得，讀相關原文段落，不只HTTP成功；Tokio shutdown／中文優雅關閉、固定ctrl_c、by_ref與Book TDD／stdout-stderr標示均相符。這些是延伸閱讀，不拿外書未教API補本書程式。

## 證據與限制

`coverage.json`保存7頁全文範圍、目前SHA與counts；`reviewed-references.json`保存每個命中原句與數字目標；`link-label-review.json`逐個保存35個label判斷；`target-evidence.json`保留高風險target原句；`read-ledger.json`分開記此輪source／前文實讀範圍；`three-step-answer-verification.json`保存同份checks及全答案檔案比對。外部頁cache及精確實讀range／hash另在external-read-ledger.json，下載整頁不表示全文已讀。SVG alt的文字證據在image-label-evidence.json。

本輪只做靜態語意引用／必要source核對，沒有重新跑Cargo、測試、HTTP、SIGINT、bench或建站，沒有把作者freeze當本輪獨立執行證據。未把全部七頁所引用每份完整source closure逐行重審，也未驗桌面／手機圖文布局。舊91頁報告保持不動；全131頁的整合結論由root另合併。

完成，STOP。
