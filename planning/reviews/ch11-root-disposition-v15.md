# 第11章原判裁定與最小修正

原始來源是 prefix `ch11-prefix-seven-for-review-v11` 與尾段 `ch11-local-service-prerequisite-v13`。FIRST按自己的prefix／middle／tail凍結路線保存原判；TECH與CONT各有prefix及tail原判。全部原判封存並由ROOT全文讀完後，才修正。最終修後為 `ch11-review-fixed-v15`：8頁81 BODY、76來源。這是協調者處置，不算另一份獨立發現。

| 原發現與分級 | 核對、理解收益及採用的最小修正 |
| --- | --- |
| TECH prefix必要：render未flush；CONT prefix原為可選一致性建議 | 採用。7.9的輸出完成責任需要write與flush均保留錯誤，render末尾改flush，正文在code前說明由render完成兩項。新增controlled Write會接收bytes、flush回PermissionDenied，驗錯誤保留；不冒稱重現真實stdout失敗。TECH與CONT各自分級、理由及原實跑範圍保留。 |
| CONT prefix必要：三處String與str異型PartialEq未教 | 採用已教的as_str再比較，不增加一套異型相等契約。Scheme及status_line的三處均調整，學習主線仍是HTTP分類。 |
| CONT prefix必要：宣告7.9跳讀入口尚無Vec::is_empty | 改用已教的as_slice().is_empty，補足該入口。正常順序已在11.5學Vec此method，原問題對入口的限定保留，不宣稱全順序讀者都缺Vec契約。 |
| FIRST middle可選／當時未知：單一bar的or pattern先用 | 11.5完整main前補p1 \| p2表示任一pattern，與boolean雙bar分開；不提前另教整套pattern。不回填為FIRST原必要發現。 |
| TECH prefix可選：manifest-path用途 | 第一次相應命令前補選獨立fixture manifest的用途，不搬Cargo重複章節。 |
| CONT prefix可選：timeout表格容易把範圍讀大 | 把表格限定等待最終response；body期限仍按後面的實際介面教，不用本表代替所有HTTP步驟。 |
| TECH tail必要：client提早失敗可能讓entered永遠等不到 | 保留raw next_started語意，新增獨立5秒觀察wrapper，說明Sender仍存不會因Request錯誤自動回None。caller收所有owned client handles、Fixture.shutdown後才判觀察或報錯。小泛型finish_client只封裝已教abort與owned await、保留原Output；雙client及雙Fixture均明確收尾。 |
| CONT tail必要：Client Send／Sync晚於首次spawn | 首次spawn前說所選Client的能力、owned move不創造Send、持&Client跨await需要Sync支援reference的Send；Clone／pool契約仍留首次clone前。 |
| CONT tail可選：selected server future的spawn能力 | 首次basic code前限定所選Axum轉出的future有Send與static，io結果可移交；不把一般IntoFuture等同任意spawn，也不引入內部Box／Pin／tower。 |
| TECH tail可選：helper表格識別字換行 | 保留完整短method名，不從識別字中間斷行；型別仍在code span，避免HTML吞尖括號。 |
| FIRST tail未知：IP全名 | 加Internet Protocol及本節IP位址／port定位用途。它原本不阻礙loopback角色理解，保留未知的原分級，不換算成必要缺口。 |

尾段新正常／失敗流程都在正文交代。新absent-entered測試以/loop取得真redirect錯誤，先確認HTTP Result已算出，才pause檢查觀察期限；最後保留Request原因並收尾server。這是作者修後新增材料，不是原TECH的server-abort／hits0 probe，也不算CONT獨立重新發現。正常SVG只解正常路線；stdout明標收尾後的步驟紀錄，沒有假裝該行印出時仍未放行。

ROOT在修後原始碼另外發現兩個custom-message assert形式缺首次教學。僅改回已教if與panic，沒有為修收尾再引入新的assert介面。這是ROOT後見補查，不回填各獨立原判。

驗證按版本保留：prefix的scoped fmt／Clippy、四library與兩CLI tests及status-line已實跑；tail初版修改的原八tests加新probe已實跑。最後兩處if／panic只重跑CLI與dual各一項、其餘六HTTP及probe未重跑，fmt／Clippy與最終引用source strict更新通過。作者的證據不冒稱獨立QA；原來正常八test與所有故障probe都有各自來源、版本及界線。

目前獨立整章cross、19 BODY局部閱讀及ROOT修後實頁核對進行中。所有原判字節／分級保持；整章尚未宣稱發布通過。
