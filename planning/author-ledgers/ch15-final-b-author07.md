# 15.11 B 完整版作者 ledger

本輪只寫15.11及新starter／url-checker；舊A／B1–4與其封存notes／proof不變。ROOT持shared Cargo／lock／nav。開寫前fresh英中章節、actual前文及fixed selected API的真實範圍見source-notes/ch15-final-b-author07.md。

必要學習問題：停止後已保存的原行結果，如何交成穩定文字或JSON，並讓呼叫者分辨全過、項目未通過、取消與fatal？沿B4同一份Batch，保留原文／index／duplicates／空行gap、HTTP與transport區分、十秒單筆預設、jobs8有限排程與全owner收尾。最終core負擔約千行，不累加各stage副本；最後按實際core／tests／helpers列數，不為硬上限砍必要驗收。

本節新增CLI數值long＋default_value_t、報告Option／整數Serde能力與緊密JSON引用，均在第一次code前給角色及結果契約。最後公開介面沿14.4私有實作與root re-export；Error.source沿8.6，不把內部task登記expect當正常檔案錯誤。

預定引導保留可編譯未完起點，提供完整原演算法與必要表示helpers；讀者依序補原Batch映射、退出優先序、write／flush。checks先真失敗再局部完成，提示與答案預設收合。來源失敗／來源關閉回typed fatal，不冒稱使用者取消；正常完成也結束listener。任何作者受控網路／child觀察保有限界線與owner清理，真SIGINT只在明確入口／工作已進入證據後送。

## 完成安排與首次使用定位

| 必要能力／關係 | actual先備與本節解釋 | 可驗的結果 |
| --- | --- | --- |
| 停止後仍按原index保存每份輸入 | B4 v13整頁及完整source；本節先固定工具政策與六份手工材料 | duplicates與空行gap保留，Unstarted不先當Input問題 |
| owned Report與Serde整數／Option | 7.7／7.9；本節第一個model source前補u16／u128／Option能力 | 無headers／無量測用null，JSON往返不借原錯誤 |
| 同模型兩種表示與JSON引用 | 7.7 pretty；本節render source前先教to_string | 文字原文／detail有LF仍維持一筆一行 |
| typed fatal與候選資料狀態 | 8.6 v9／14.4 v1；第一個error source前教原因分層與source | fatal2優先130，來源失敗不變使用者取消 |
| CLI正數、預設值與真正入口 | 7.6–9；config source前教default_value_t與數值long | 0先被拒絕再讀檔，省略jobs8／單筆10秒 |
| 內置已知Fixture的ready helper | 11.8既有契約；test helper source前先教status後交出Response與text await | 讀固定ready body只屬測試，產品headers-only |
| 編碼／write_all／flush與最後狀態 | 7.4／7.9；ControlledWriter source前說bool／bytes／flushed角色 | 已收bytes但flush失敗仍fatal，不交130 |
| 私有實作與公有library邊界 | actual14.4十一BODY；run source前交代Scheduler／listener所有權 | 正常／取消皆全await，caller不能丟future冒稱完成 |

## 真正執行結果

工作證據入口：`/workspace/work/tutorial-implementation/ch15-final-b-author`。工具版本沿本輪已用的rustc1.99.0、cargo1.99.0及uv0.13.0；selected API／locks與raw log各有獨立fingerprints。

- 兩包fmt check及all-targets Clippy -D warnings最終EXIT0；兩包config integration各2項通過，成品library16項通過。沒有重跑舊B1–4。
- repo外起點全targets check及零tests EXIT0；追加九checks後3／6 → 補map5／4 → 補status6／3 → 補write9／0。完成練習另fmt／Clippy EXIT0。
- 另一份repo外完整成品以自己的manifest及183packages lock，完整tests為16library＋2config，全通過。兩份repo外只有CARGO_TARGET_DIR共用編譯快取，不共用manifest、source或lock；種子根lock唯讀，生成兩份package專用lock後核直接版本。
- 真binary的17項CLI案例全通過：help／version0、用法及設定／讀取／Empty／無效UTF-8 fatal2、invalid text與JSON1、重複文字byte相同、真HTTP全過0／混合1／held timeout1、正常輸出失敗2、真正SIGINT130與SIGINT後輸出失敗仍2。
- 兩個SIGINT probe都只在兩份held GET handler明確已進入、且未送headers後發送；run在提交前已等待listener entered。host服務finally release／shutdown／server_close／join，child有有限guard；所有17案例均未觸發host強制終止，沒有拿terminate／kill當產品cleanup證據。probe SHA及各stdout／stderr／原始條件都保存。
- preview strict build EXIT0。作者逐張親看40張兩裝置實頁PNG；四份details預設closed、實際點開後再看提示及答案頭尾；圖360×780／手機343×743，藍色交接及橘色fatal旁路可追。DOM無未知tag，文件寬1265／375（瀏覽器15px垂直scrollbar）沒有超出viewport。長code需區塊內水平捲動，並未宣稱每行同屏可見。

## 保留的失敗與修正

初次all-targets check雖成功，starter有dead_code warnings；只在未讀欄位的私有batch／single限定allow，正文先說明，映射完成移除。getter移到tests時最初漏Entry import及一個呼叫，E0425／E0599原log保留；修後一項boolean match遭Clippy建議matches!，本節改用明確tuple材料保持已教語法。相關test局部再驗，之後repo外完整16＋2與兩包Clippy都成功。checks.json保留中間Clippy101，末筆才是最終EXIT0，不抹除原判。

實頁腳本第一次用uv Python沒有Playwright，ModuleNotFoundError另存；改用環境既有system Python後正常生成40圖，不改專案依賴。最終唯讀audit最初把test module前尾端空行也當核心byte相同，及把含15px垂直scrollbar的文件寬誤要求等於viewport，兩個assert失敗log保留；改成只忽略核心邊界空行、文件寬不超viewport，沒有因此改教材或Rust。最後14項identity audit通過。

## 成品大小、保護與限制

依最終真檔案逐行（含空白、snippet標記）：核心825、tests898、Fixture helper198，共1921行Rust；不含lock／生成輸出，也不累計各stage副本。本節新練習只需補三個動作，原七項排程驗收與test helper是完整參考資料，不要求同時手寫。沒有為壓數字移除有限cleanup或有用契約檢查。

B1–4全部舊正式handoff中的files／evidence／screenshots共用唯讀SHA核對，均不變；A／舊frozen／shared Cargo／lock／nav本作者不修改。Input／single只去snippet標記，其餘同B4；十一個排程／停止函式相同。新兩份Fixture完全同byte，checks整module同成品；練習完成的report／app同成品，獨立portable成品整目錄也相同。

親看範圍是各20個選定位置，不是所有scroll／所有裝置；release-profile build與cargo install本輪未執行，install契約只核官方help。正常網路依環境、proxy及當時服務而變；本機probe不擴成公共網站可靠度證明。這是作者交稿與先備gate材料，不宣稱獨立第15章QA、merge或GitHub Pages發布。正式handoff封存後STOP，交ROOT核final delta及後續三角色安排。
