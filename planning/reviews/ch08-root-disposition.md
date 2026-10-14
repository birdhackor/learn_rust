# 第8章原稿初判後的處置與修後範圍

Root已全文閱讀三份不同角色的原始初判：first-reading-v1（全66四題）、technical-v8（66 BODY／150 source）與continuity-v8（66 BODY／150 source）。三份原SHA與原分級保持，沒有把同意peer改成第二份原始發現；root此前的全source先備核對與八圖實看也不代替它們。

原必要項均為0。以下可選項逐項採用最小改動，以改善名稱來源、證據歸因與圖的辨識；不新增Rust機制、功能或擴大讀者背景。

| 初判來源／位置 | 處置及理解收益 |
| --- | --- |
| TECH I1，8.1 Sized／Box首次 | 各補Std module與prelude提供名稱，讓既有use規則能對上為何直接寫名稱；不擴成匯入清單。 |
| FIRST O1，8.5混合Vec fixture | 說清fixture實際驗不同T不能直接混放，前段共同Charge能力的推論另由前段支持；不把沒有Charge實作的fixture當共同trait的執行證據。 |
| FIRST O2＋CONT O3，8.5 dyn圖 | 兩個method表標籤x200→x160，保留x290箭線與兩條資料／method資訊路線；減少文字被線穿過。Root此前親看漏掉這項，發現時點保留為原讀者與銜接者。 |
| CONT O1，8.4 bound組合 | 將「本節新增+」改為沿用7.6同時要求，與已教trait bounds銜接，保留新的大小／lifetime組合。 |
| CONT O2，8.9第三行輸出 | 將binary的「本crate」改成「library定義的trait」，正文與真source同改，讓定義所在crate與import責任一致。 |

CONT曾提出where首次、短片段imports及Box Debug等候選，已由實際前文／完整source否證，不作必要項；root保留其原驗證路徑，不另外修改這些契約。原必要0不代表全套編譯、平台或真人試讀已完成。

修後全章凍結為ch08-review-fixed-v9，10頁66units150sources；原v8不變。唯一改動Rust檔是newtype_interface的println文字，另一SVG只改兩個label座標；其餘source bytes全部與原v8相同。Root實跑該normal target，退出0，三行stdout逐bytes符合修後正文。文句與圖座標不要求重跑所有Rust；發布candidate仍按AGENTS跑完整checks。凍結稿另有七個BODY只是同byte圖檔的snapshot目錄路徑更新；已將路徑差異與六個真正受影響單位分開。指紋／影響六個BODY單位與原SHA在work/tutorial-implementation/ch08-fix-scope-proof.json。

目前只是root處置／局部source與stdout證據。TECH／CONT交叉、FIRST受影響六單位局部重讀、新dyn實頁桌面／手機與candidate全套檢查、remote發布仍待另存QA，不先宣稱全章發布通過。
