# 第15章原判處置與最小修正：v16

十二份獨立原判均已封存並由Root FULL讀完，才修改學生稿。三角色各有A v11、B-prefix v12、B4 v13、B5 v14；完整原稿union為`ch15-complete-original-v15`，11頁、82 BODY、180附件，與四scope所有來源／頁面同bytes。原判、未知與分級不回寫。修正及cross不算另一次獨立重現，也不以不同角色的N0否定其他角色已指出的關係。

本處置採納四個必要項、四個不同的可選改善；TECH A與TECH B5的工作量O1是同一項，不重複計數。下列為Root核原來源、實際先備及必要證據後的裁定，不是三角色全都自行發現。修後複查與整章發布另記於`planning/ch15-qa.md`，本檔本身不簽核執行或發布。

## 必要項

| 原報告／ID | 已教與仍缺的最小關係、影響 | 裁定、改動與新增負擔 |
| --- | --- | --- |
| TECH A N1 | String／str內容比較、數值reference已有；`&String == &String`的標準PartialEq沿值比較未在首用前明教。程式合法，缺的是checks先備，不是地址比較錯誤。 | 採納。A3 checks／complete、A4 starter／complete共四處改為`path.as_str() == name.as_str()`。直接沿已教能力，無新增blanket impl理論或鏡像測試。 |
| CONT B-prefix N1 | 普通match／if已有；take helper新用match guard，pattern成功後判bool、false再選arm的規則未教。 | 採納。兩份checks／complete先用普通match取Entry，再用if核expected；兩種錯誤仍先close再panic。去除新語法，保相同有限等待與owner清理責任，沒有擴大工作圈。 |
| CONT B5 N1 | 1.10已有Clippy與轉交選項；1–14實際正文無`-D warnings`的規則。無警告不等功能正確，也不能由一般Clippy推知這項通過條件。 | 採納。在15.11命令前及final README補最低說明：`--`轉交Clippy、`-D warnings`將回報警告提升為錯誤。固定工具的Clippy／rustc help亦核過。保原命令，無需另寫lint章。 |
| CONT B5 N2 | 真42-byte fixture與原索引1／3／4正確；但相同Markdown設定的實頁省首LF。CONT真正copy得到非空索引0／2／3，和鄰接預期不一致。 | 採納。移除會被格式化改變的raw展示，改以原index0–4與明示空行／一個空白字元的表呈現，執行仍用隨附原檔。Root亦從自己既有同設定preview真正點copy重現0／2／3，並親看兩尺寸素材畫面；該preview的兩段後加說明不同於final原稿，但raw block與格式相同。保持正確fixture與Rust不變，不為呈現問題重跑HTTP。 |

## 可選改善逐項裁定

| 原報告／ID | 理解收益與界線 | 裁定、最小修正與負擔 |
| --- | --- | --- |
| TECH A O1、TECH B5 O1（同項） | 開場800–1200含tests與A的995＋662、B的825＋898＋198口徑不同；各完稿已誠實列量，且約千行是寬鬆負擔目標，不是硬上限。FIRST亦記規模觀察但沒列N/O，不能替其補算命中。 | 採納。15.1先說核心約千行、測試／本機材料另供且完稿分列，不要求一次手寫全參考測試；保分階段練習。不砍必要驗收或為湊行數改架構。 |
| CONT A I1 | validate_root摘要說scan成功後走訪，隨後接線已清楚且功能正確；只需讓責任主詞一致。 | 採納。一處改「驗證成功後」，不升必要、不重寫scan。 |
| CONT B-prefix I1 | 三期限責任已清楚，手機卻將`Outcome::Timeout`拆字，增加辨讀成本。 | 採納。該格用已知短名Timeout並局部nowrap；保工作／觀察期限與到期結果，不改全域CSS。 |
| CONT B5 I1 | long名稱已教且完整Settings／命令可核，手機將短參數兩個連字號拆到上列。Root自有手機畫面亦見同一token兩個rect。 | 採納。只對CLI介紹中的兩個短參數和值局部nowrap；不整段不換行、不要求所有code同屏。 |

## FIRST未知與Root補查：保留原紀錄

B-prefix的scheme直觀詞義、Url短名import、tracked()確切定義在FIRST仍是未知，不是已確認N/O。Root補查完整來源與實際正文後，只做三項短澄清：scheme是冒號前部分；`Url`由`use url::Url;`引入；tracked只回tasks.len，包含已完成未收取者，供checks看名額。它們不引入新API機制，也不取代原有URL政策／單筆／排程解釋。這是受FIRST未知提示的Root編輯改善，不能算FIRST獨立必要發現或改其原分級。

FIRST B5保留Report.status／write_report精確數值返回header、check_text完整header與StopEvent確切re-export未核。完整起點／完整成品確有宣告；正文已說角色、三步任務、錯誤與caller責任，所用u8／Result／pub use已教。首讀角色未開完整source的界線不等於當處必須重印所有聲明；本輪不另加重複公有API表，也不把未知回填為FIRST已驗。TECH的真external consumer正例與private E0603只是其技術證據，不能代首讀紀錄。

## 執行、實頁與閉合界線

本次Rust變動只在既有tests／checks。相稱驗證用實際manifest名稱選六包fmt／Clippy、三份字串往返test及B3真收取後補工test；命令／結果存work。Root最初fmt用了不存在的package名稱，該失敗保留，改為從manifest讀名稱後才算檢查；不是教材錯誤。完整候選仍必須跑規定check與Miri，不能由局部通過代替。

八張SVG保持原bytes。Root先前親看A／B1–4與B5未報問題的圖，B5另有自己的桌面／手機原位及完整元素四圖，涵蓋換表示、分流及出口完成關係；這些不是對所有頁面或scroll位置的宣稱。必要及可選呈現項另以修後實頁確認。作者、非作者技術、銜接、順序讀者與Root的執行／畫面範圍各自歸屬，不互借成自己驗過。

TECH／CONT cross須先FULL讀同行原判、保存自己的來源判斷，再看本處置與修後差異。FIRST只做受影響單位的LOCAL，不重置舊路線，不再稱盲讀原判。所有必要項修後複查、可選收益／新增負擔、實頁與候選checks／CI／public HTTP齊備後，才可關閉整章QA。
