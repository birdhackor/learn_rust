# 第9章作者交接紀錄

## 實讀先備與階段範圍

Root已放行9.1／9.2與不依7章新能力的9.4。作者親讀新5.1–5.5、新6.1–6.4全文與必要實際source，並重讀1.6、3.5、3.8全文及2.7工具段；實際範圍與SHA256在/workspace/work/tutorial-implementation-20261015/ch09-author/prefix-read-ledger.json。1–4先前作者階段的實際閱讀仍保留，不以研究metadata冒充已教證據。

新6.4的associated type／Self::Output／equality bound已成立；9.2直接回用。9.1借用來源與最後使用點只需3.5／3.8，沒有等待中的7.3借用struct。新7.1–7.3於本輪中獲root放行，9.3仍須親讀實際正文/source才寫；9.5 Result collect及9.6 to_owned依實際8.5／8.6另行放行。

研究21筆原文與39-target快照計畫在/workspace/work/tutorial-implementation-20261015/ch09-preparation/，sources-ch09.md列核心採用與不採理由。該計畫不是完成稿或已教證據。9.4改以具名Countdown作回傳型別，不依尚未完成9.3的RPIT，root已同意。

## 9.1–9.2實際freeze

容器為examples/intro-workspace/iterators/，package intro-iterators；三欄manifest明寫name/version/edition，無lib/dependencies/build.rs。main只提示選example，9.1同頁列完整main與manifest。每個selected example載入自己的完整單檔，helper也在同檔。Root負責加入intro workspace member與lock；作者沒有改共享Cargo、nav或其他章。

| 節 | 當下問題與必要順序 | 完整可執行材料 |
| --- | --- | --- |
| 9.1 | 保存運費工作與門檻；closure/capture→推導→shared→mutable→move只讀→Copy capture→呼叫移出。建立與呼叫分開追蹤；字串與reference只用實際3.8/5.3/6.3。 | 8 examples含closure_answer；3 compile-fail與同名expected；closure-transfer.svg。 |
| 9.2 | 共用函式先定呼叫能力再用FnOnce/FnMut/Fn；Output回用6.4，fn pointer與Fn trait分開，無capture closure可轉pointer。 | 5 examples含callback_answer，所有helpers同檔；2 compile-fail與expected。 |

13個selected examples已實際cargo run成功，原始stdout/stderr與5個rustc失敗診斷在validation-9.1-9.2.json。5份.expected都從本次實際stderr核對E碼後建立；依維護harness要求同名、非空，沒有以自訂條件免除sidecar。fmt與Clippy --all-targets --locked -- -D warnings通過，原始紀錄為checks-9.1-9.2.json。每個新shell先source scripts/env.sh，沿root的INCREMENTAL=0、DEV/TEST DEBUG=0、JOBS=2。

snippet-audit-9.1-9.2.json確認13 examples與5 fixtures都同頁完整、manifest與marker存在、selected source無lib/外部module/舊ch06依賴、泛型角括號在code內。SVG沿用作者原創圖，研究時已render親看；本輪複製後XML核對，文字與實際take/text/saved一致。尚未做新版網站原位桌面／手機檢查，不把SVG預覽視為版面驗收。

28檔freeze保存在/workspace/work/tutorial-implementation-20261015/ch09-author/freeze-9.1-9.2.json，後續變更另記diff，不覆蓋初判證據：

- 09-01-lesson.md：`ff8a0d0ca07cbc731287f88935e7c8a6f7bb32b9e321dd6dbefff1124a1267e9`
- 09-02-lesson.md：`1f94efb9e804d4ff18de071441b7d0b19368326c497bd284ede0d47a5456ffd0`

這是作者局部實作與驗證；獨立技術／銜接／首讀由root安排。

## 9.3–9.4追加freeze

Root放行後，作者全文親讀實際7.1–7.3及14份必要manifest/source/fixtures：single_source、source_basic兩檔、source_scope、owned_output、placeholder、outlives、view_basic兩檔、view_answer、三份來源失敗例與manifest。範圍及指紋追加於prefix-read-ledger.json；未用未freeze的7.4 receiver規則。

| 節 | 實際安排與保留的契約 | 完整材料 |
| --- | --- | --- |
| 9.3 | 參數匿名泛型與回傳opaque的選擇方向分開；同函式各分支同具體型別；owned與borrowed工作；2024預設capture即使body未用來源仍保留依賴；use清單與真實借用分開。每項在首次snippet前解釋。 | 4獨立examples含impl_trait_answer；3 compile-fail與同名expected。 |
| 9.4 | 明寫具名Countdown；先定Item/next，再定IntoIterator的Item/IntoIter/into_iter與equality，之後接for；None後一般可恢復但for首None停止；NumberList、range與Countdown共用加總；三條ownership。 | 6獨立examples含iterator_answer；2 compile-fail與expected；iterator-next.svg。 |

新增10個selected runs全部成功，5份fixtures都被拒絕且包含本次核對的E碼；validation-9.3-9.4.json保存真實stdout/stderr。23個現有targets的fmt及Clippy --all-targets --locked -- -D warnings通過，checks-9.1-9.4.json保存原始結果。作者逐份讀取已保存stdout，核對正文數值／字串與先後；沒有因prose微調重跑原13程式。

snippet-audit-9.1-9.4.json確認23 examples、10 fixtures各有同頁full-file snippet、10 expected非空、marker完整、selected source無lib／外部module／舊ch06依賴。全部51檔指紋在freeze-9.1-9.4.json，其中新增23檔另存freeze-9.3-9.4.json。原28檔逐檔核對未變，freeze-9.1-9.2.json仍保留供獨立判讀。

- 09-03-lesson.md：`af37b908bf94c477a21a412fdeb780ec55b75fdb3d4dd91916e57ee22370e458`
- 09-04-lesson.md：`af1cd2280166c7e5b70cbe1b67dea4f09efb7001d5a81b948b7070833751e1b1`

兩張圖都是400px直式、24px文字，研究時曾render親看；複製後XML與實際source逐物件核對。仍未做新版網站原位桌面／手機檢查。9.5–9.7未寫；Result collect／to_owned與後續完整整理仍待root交接實際8.5／8.6。

## 9.5–9.7完成與整章freeze

Root交接實際8.5／8.6後，作者親讀兩頁全文及必要實際source，並讀6.5／6.6的HashMap、Entry、grouping完整教材。實際範圍與SHA256追加於prefix-read-ledger.json。新增材料包含整數解析、FromStr與兩份From、owned_text／AsRef，以及Result／Option傳遞；9.5完整答案中的`?`在這次親讀後才寫，沒有用原計畫充當已教證據。外部原文沿用先前完成的21筆實讀，未為本輪擴充研究。

| 節 | 實際學習問題與順序 | 完整材料 |
| --- | --- | --- |
| 9.5 | 先分開接手iterator值與惰性走訪；map的Item、filter的&Item與原Item保留、collect／FromIterator目標契約，再到fold累積與Result首Err停止。完整答案以同一ParseIntError透過`?`回傳。 | 6個獨立examples含adapter_answer；2份失敗案例及expected；adaptor-pipeline.svg。 |
| 9.6 | 先核對Unicode scalar與行界，再教Entry三個操作與callback執行條件，最後整理owned key及三條map走訪路徑。候選key立即建立、值callback按需要呼叫；來源結束後結果仍可使用。 | 5個獨立examples含text_counts_answer；1份失敗案例及expected。所有9.6 selected sources均不載排序函式。 |
| 9.7 | 先定Ordering、PartialOrd／Ord及NaN的None，再分別教穩定排序與不保證同值相對順序的排序。完整報表同時指定數量與名稱的tie-break，不靠HashMap原順序。 | 5個獨立examples含獨立完整text_report_answer；1份失敗案例及expected。 |

新增16個selected examples及4個失敗案例已實際執行；作者逐份閱讀保存的stdout，核對正文數值、順序與assertions。validation-9.6-9.7.json保存最後39-target fmt及Clippy --all-targets --locked -- -D warnings通過的真實輸出。最初兩次Clippy指出unnecessary_fold的輸出也保留在validation-9.5-core-initial.json與validation-9.5-answer-initial.json；最後只在scaled_sum與parse_total兩個教學函式使用具名allow，首次snippet前先說明原因及範圍。兩份函式修改後各重跑一次，紀錄在validation-fold-final-attributes.json，沒有重跑原51檔已通過的範例。

validation-all.json合併39個實際成功runs、14個被拒絕的fixtures及最終fmt／Clippy證據；每筆保留實際stdout／stderr、source hash，14份同名非空expected都與真stderr核對。snippet-audit-ch09.json確認全部39個examples、14個fixtures及7份manifest各自同頁完整，無lib／外部module／後文source載入、泛型角括號未裸露，9.6不載排序，原51檔全部未變。未執行全repo維護harness，不把selected檢查稱為全書驗證。

第三張adaptor-pipeline圖與本頁keep_and_scale的values參數對齊：圖從已是&[i32]的values開始，標values.iter()，正文也先界定這個起點；不把main中的Vec及as_slice呼叫混成同一時點。400px直式24px文字，已實際render並親看400與336px寬預覽。三張SVG的XML均有效；仍未做新版網站原位桌面／手機檢查，該工作交root安排。

整章79檔freeze為/workspace/work/tutorial-implementation-20261015/ch09-author/freeze-ch09.json（7頁、69個package檔及3張SVG），SHA256：`a39f8aac1ed09d6b36029811174562f7ecf6e634dc0aa377b58747d0e6415fee`。新增28檔另存freeze-9.5-9.7.json（SHA256：`8bfde8c8fd75beb1fd07e7376f8a7defa747d42ff9b640ee9988af7ac78df51a`）；原freeze-9.1-9.2.json、freeze-9.3-9.4.json及freeze-9.1-9.4.json均保留，原51檔指紋未變。

- 09-05-lesson.md：`256e3a7a8380bee5e863db21b94d89e11578304f1ba8194147b6b12515eea71e`
- 09-06-lesson.md：`d8be1e5ed7a5ac9c930e1b2b412fbe3b8efd80ea513be061913cff919f59d68d`
- 09-07-lesson.md：`fce1c2a012b97dcf775ba970e78f9cc500c2ab39fcc8cf59fef7189536ad91ba`

此處追加的是作者完成與局部驗證，保留先前各階段原紀錄；獨立技術／銜接／首讀及網站版面由root排定。第9章已全部交接，作者依root指示停止，不自行啟動第12章。
