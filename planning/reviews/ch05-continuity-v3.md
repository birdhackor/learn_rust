# 第 5 章 continuity 獨立初判：visual-fixed-v3

## 身分、範圍與證據界線

- 審閱者：`chapter11_async`，未參與第 5 章寫作。本輪是前後銜接／依賴審閱，不是首次讀者盲讀。
- 本輪讀者路線：具備基本 Python，依序完成凍結的第 1–4 章，再讀第 5 章主文、練習與必要圖。未把未來大綱當先備。
- 在保存此初判前，沒有讀取 `planning/reviews/ch05*` 的其他紀錄、作者 ledger／disposition、其他角色初判或第 5 章 live 正文。
- 已讀 `AGENTS.md`、clear-tutorial 主 skill／review-protocol；術語指引先前已讀。skill SHA-256：`e3890d1adbf84fc530016d45a5019a4dcc614c65d75efe3c42e9ab25e0b1618f`；protocol：`aa482b1ed4fdc28c74d7a0ba520034d7cc3d9db22d1ad344daed42a934299395`。
- 第 1–4 章依 `reading-order.txt` 實讀全部 151 個正文 units。第 5 章依序實讀全部 35 個 units，完成主文後再讀對應完整封存來源，包括折疊／外部連結所對應的練習答案及 fixtures。
- 第 5 章全部 40 個 `sources/` 檔案已讀，並逐檔核對 manifest 中 SHA-256；35 個 units 亦全部核對一致。這是內容／版本證據，不是編譯、執行或版面通過證據。
- 第 5 章必要 SVG 的原始內容與正文原位引用已讀；未啟動頁面或瀏覽器，**桌面／手機實頁、圖中文字是否實際可讀、折疊與浮動介面影響均未驗證**。未自行修改教材或重新執行 Rust 範例。
- 本輪未讀第 6 章正文；只評估第 5 章末的轉場是否交代下一階段需要。

## 凍結來源指紋

來源共同根目錄：`/workspace/work/tutorial-implementation/frozen/`。下面指紋對應實際閱讀的 manifest；其內保存 pages、units、完整來源的逐項 SHA-256。

| 來源 | units | manifest SHA-256 |
| --- | ---: | --- |
| `ch01-complete-source-v8` | 50 | `2ffad984aceb0e9fd822e5f0bb01a6958b6f063319a576e229df2da68c09e2cd` |
| `ch02-review-fixed-v5` | 31 | `b94b767b0086ad6e2ed1d00edb2bcf5759ab08656966387b02f52d361249b877` |
| `ch03-review-fixed-v6` | 32 | `45146f5eafd8d8b3af1cf74b5cb1f3cb7e173c9f04b26108a06d060c1f02ccc7` |
| `ch04-review-fixed-v7` | 38 | `d672bf58e8f197b3c603129adc3f9a72619d383a4a987a08c0db04e0b0333f14` |
| `ch05-visual-fixed-v3` | 35 | `6f8ea0ff68bcffa9d5af744ac7670304a5eb59c0e5dfa6b486194afee2811d9e` |

第 5 章 reading order SHA-256：`de9c5f94af5b474485815eef2c4e09240003903115416a835661855fecbbc47c`。必要圖 `sources/docs/assets/ch05/question-paths.svg` SHA-256：`a94a79cc4eb5f56cb515e567613830b407e818c568079e71d550ad2a85773934`。

## 全部 units 的依賴與轉場核對

此表由實際正文建立，沒有讀作者的概念清單。每列單位都實讀；「足夠」只指本輪內容銜接範圍，不涵蓋未做的實頁或執行。

| unit | 此處主要需要／新增內容 | 真正已教支援與初判 |
| --- | --- | --- |
| 001-001 | 批次件數合法範圍；逐筆處理與停止政策分開 | 3.2 的 Result 已保留原因而不自動停止；這裡先給 0–100 與逐筆需求，沒有先拿 panic 反定義問題。足夠。 |
| 001-002 | 具體錯誤 enum 保存類別與原數字 | 3.1 variants／pattern、3.3 Result 型別、4.2 derive／Debug／PartialEq 已教；此處補 `pub enum` 與 variants 公開契約。五筆涵蓋兩邊界／兩類拒絕。足夠。 |
| 001-003 | 主動 panic 與內部正數契約 | 1.7／1.10 已遇 panic；此處先教 panic! 格式契約，再明示故意不同的正數規則，沒有把件數0改判無效。足夠。 |
| 001-004 | unwrap／expect 取得值與失敗 panic；Debug bound | 在 code 前交代消耗結果、四種操作與錯誤能力；slice.get 已在3.6教，reference 解參照在2.3教。預期成功／故意失敗區分。足夠。 |
| 001-005 | Result 整體比較與邊界測試 | 本段先補 Result 的 PartialEq／Debug 條件，從4.2 enum／4.3 Option延伸，才 assert_eq 整體值。足夠。 |
| 001-006 | 讀者用既有pattern提供逐筆訊息 | Negative／TooLarge 都已教；不要求 Display 先寫答案。下節 alias目的是讀格式化接口，理由具體。足夠。 |
| 002-001 | type alias 是同型別的另一個名字 | 在第一段程式前說明 type、pub、右側型別；接收兩種名字不需轉換。3.3 泛型 Result 形狀已教。足夠。 |
| 002-002 | 名字不加入驗證；預告 fmt::Result | 用原 i32 的−1說明限制；下一節容器角色不陌生。足夠。 |
| 002-003 | tuple alias練習 | 1.7 tuple位置／型別、1.5 函式與本節 alias足以支持。兩個欄位角色仍明說。足夠。 |
| 003-001 | 多個呼叫者重用同一顯示規則 | 從5.1實際重寫報告需要接到型別能力，而非概括說「方便」。足夠。 |
| 003-002 | Display與Debug、fmt method／module | 4.1 trait實作與scope、4.2 Debug 已教；此處先說Display身分、借用與出口再展示實作。足夠。 |
| 003-003 | Formatter兩種借用角色；fmt::Result | 4.4 placeholder、4.5模型自身借用vs receiver、5.2alias／unit足以支援簽名。fmt::Error不等於業務錯誤被明確區分。足夠。 |
| 003-004 | write!輸出契約、format! owned結果 | 新macro用途在code前教；match shared enum的 `&i32` 支援來自3.1；每arm直接回write結果，不偷用?。足夠。 |
| 003-005 | 同一error產生三種表示 | 格式化只借用而saved是另一個String，與2.3／3.4／4.2銜接。結果不誤稱改變error型別。足夠。 |
| 003-006 | 欄位形成Display訊息練習 | struct、fmt／write、format、str比較已教；沒有多餘Debug需求。到下一節的轉換問題保持介面脈絡。足夠。 |
| 004-001 | 普通i32不能表達已驗證件數 | 從同一0–100規則提出下一個可判需求：接收者可辨認檢查完成。足夠。 |
| 004-002 | From／Into的來源／目的與成功契約 | 當段教trait型別參數角色、i32→i64能力及目的推導，再寫呼叫；Copy 已教。沒有把可能失敗檢查藏在panic。足夠。 |
| 004-003 | pub型別與private欄位維持有效Count | 4.1公開欄位做對照，本段直接教沒有pub的value不能在外部任意建構；receiver讀值已教。足夠。 |
| 004-004 | TryFrom／associated type Error | 此處在第一個type Error程式前教「實作填入角色型別」與Self::Error，並對照alias；不是等第6章才補最小意義。足夠。 |
| 004-005 | Count→i32／view的ownership差別 | 2.5 self vs &self、4.1 trait實作、5.4轉換契約支援；正文明确同module仍須維持規則。足夠。 |
| 004-006 | 邊界、原因、往返與練習 | Result整體比較已5.1教；expects有固定有效材料；練習先提供check→取回任務再給折疊答案。足夠。 |
| 005-001 | 文字解析／件數檢查兩階段；借用／owned另兩種需求 | 改資料的理由清楚：前節i32改成輸入文字。三類需求有區別，未因同名「轉換」合併意義。足夠。 |
| 005-002 | FromStr／parse、ParseIntError與明確型別 | 本段教整數格式、bound／turbofish及失敗資料能力後用API；沒有先unwrap外部失敗。足夠。 |
| 005-003 | 包裝兩種error與Display／From | 3.1 enum payload、5.3Display、5.4From支援；兩variant保留原原因，並說必定成功的封裝用途。足夠。 |
| 005-004 | Count的FromStr實作，手動轉換早退 | type Err接到5.4associated type；return已1.6教；有明確兩層失敗與ownedCount理由。Display不一定可往返的限定回應本節「都處理文字」的合理疑問。足夠。 |
| 005-005 | AsRef泛型借用／與Borrow不同 | 4.1bound、2.3 reference、4.3Borrow鍵身分、4.5receiver支援。Label完整身分兩欄不同，借出text相同，例子實際讓差異可見。足夠。 |
| 005-006 | ToString保存顯示；ToOwned取得concrete owned產物 | Display能力與associated type已教；str→String／[i32]→Vec的具體結果與只clone reference不同，正文／程式能對照原值未改。足夠（本輪判具體str／numeric slice，不廣推所有內含reference的型別）。 |
| 005-007 | 解析分類、owned結果與泛型借用練習 | 4.3 `let count;`後初始化已教；1.7 tuple解構＋1.8逐項for可合理接起每項tuple綁定。测试 `other =>` 的全值binding規則尚缺，見必要N1；其餘練習由AsRef→parse直接組合。 |
| 006-001 | 保持任務／資料不變比較手動與? | 前節兩階段已完整工作；不要把新語法當新能力的引入理由，這裡做到。足夠。 |
| 006-002 | 兩次match、錯誤From、整個函式return | 引文與code均保留順序與出函式界線；來源的return章號有小錯，見可選I1，不影響實際已教。 |
| 006-003 | ?成功取值／錯誤轉換早退與必要SVG | 在code前教雙路、結果型別、From的來源；SVG左繼續右整函式回傳可從相鄰文字追蹤。不需?Sized／Box／dyn／closure。內容足夠；實際圖版未驗證。 |
| 006-004 | caller繼續與helper早退分開 | 完整Result比較與main match都已教；六筆使三條路徑可見，沒有將?說成停止整程式。足夠。 |
| 006-005 | 回傳unit不能接Err、缺From原因 | unit在1.5教；本章ParseIntError／CountError／From已教。故意失敗範例的差異與正常版目的清楚。足夠。 |
| 006-006 | Option?的None早退／Result不自選錯誤 | slice.get在3.6教；Option Debug／比較在4.3教；*讀i32已教。不會借用與owned結果混用。足夠。 |
| 006-007 | 比較已教行為、兩筆按順序傳遞、下一章入口 | tuple／?／match／expect均已教；三組材料顯示第一筆／第二筆失敗的差別。第6章預告closure／Iterator的後續流程用途，當下不要求讀者先實作。足夠。 |

## 核心方法的理由與練習支持

以下把機制與需要分開；這是本角色自己的內容覆核，不是其他首讀者答案。

| 焦點 | 機制／性質與來源 | 需要／用途與最短連結 | 示例／練習能支持什麼 |
| --- | --- | --- | --- |
| Result／panic政策 | 001-002「Err本身不停止」與001-003「正常流程不往下」 | 001-001要求逐筆報告所有件數；用Result把決策交到caller，才能報告一筆再下一筆。充分。 | 001-002五筆材料顯示合法邊界、兩類錯誤、最後一句；001-006用相同pattern自己寫消息。未聲稱外部資料必須panic。 |
| alias／Display | 002-001不產生新型別；003-003fmt::Result回報出口寫入，003-004write返回该結果 | 003-001多個caller要同一規則；Display把規則放型別，caller只决定顯示或保存。充分。 | 003-005同一error Debug／Display／format對照；003-006數字由fields取得，檢驗重用規則。沒有要求模擬出口失敗才理解当前主要用途。 |
| From／TryFrom／Count | 004-002不失敗轉換、004-004Result與Error、004-005private接口保證 | 004-001接收者要知道檢查已做；只讓成功路徑建立Count，公開接口不提供無檢查改field。充分。 | 無效0邻界−1、100邻界101；合法往返與只借用value()／消耗into()不同。未靠alias假稱已驗證。 |
| FromStr與原因保留 | 005-002文字格式，005-003錯誤分variant，005-004兩階段 | 005-001輸入是文字，因此要先取整數再驗證；兩種失敗影響報告，不可只看顯示文字猜原因。充分。 | abc vs101與i32 overflow vs件數上限，ownedCount在source結束後存在。005-007的catch-all binding尚待補。 |
| AsRef／Borrow／owned | 005-005低成本借用、鍵身分強約束；005-006實際產物型別 | 005-001三種需求分開；字節統計只借用，持久保存要owned；Label id可區分鍵、text字段投影給統計。充分。 | 兩Label不同id同text實際呈現AsRef不承諾鍵等同；獨立String追加不改原，numeric Vec追加不改array。無性能比較主張。 |
| ?與政策 | 006-002手動match+From+return，006-003兩分支同規則 | 006-001保持業務工作，減少重複寫“取成功／轉錯誤早退”；006-004caller逐筆繼續，符合章首政策。充分。 | 比較三路完整Result；count_pair成功／第一筆整數失敗／第二筆範圍失敗。原任務不因語法變化縮減。 |

## 必要 N1：測試第一次使用全值 binding pattern，缺少最小操作說明

- 發現者／時點：本 continuity 角色在獨立讀 `005-007.md` 時；未受其他角色或舊問題提示。
- 分類／嚴重程度：先備依賴／操作時序；**增加理解負擔，必要小修**。主要解析觀念仍然可理解；範例包含測試，同樣須遵守先教再用。
- 原位置：`docs/chapters/05-05-parsing-and-borrowed-conversions.md` 的「檢查解析政策，並串起借用輸入」；frozen `005-007.md` 內測試 `malformed_and_unrepresentable_integers_keep_integer_errors`。
- 原句／code：`other => panic!("預期整數解析失敗，得到 {other:?}"),`。
- 已教部分：ch03 `001-003.md` 說pattern描述形狀、variant內部的amount／code綁定，以及「`_` 則符合任何值但不綁定名稱」。当前测试之前解釋了四種材料的失敗歸屬。
- 缺少的最小關係：**單獨名稱在pattern位置能接受所有剩餘形態，並把整份matched值綁到該名稱**。前文教過variant里拆payload，但沒有解釋這個沒有variant外框的pattern。也沒有說明這里的other是完整 `Result<Count, ParseCountError>`，不是其中某一個error。
- 影響與可合理預見的困難：讀者要理解為什麼Ok、其他Err都會進這個arm，以及 `{other:?}` 在顯示什麼，需自補上述規則；只把它當另一個variant名或把other當error payload，便無法由已教內容對上這個測試的覆蓋。這是來源合理預見的負擔，非實際目標讀者回報。
- 最小建議：在測試前用一兩句介紹 `other` 的全值binding與剩餘情況含義，可顺带對照 `_` 不留名字。無須新增完整pattern章節、测试或改變解析邏輯。
- 後文釐清：本章後面仍使用 `_`，沒有補出這項全值binding說明。
- 狀態：留給root裁定／作者修正，本角色沒有修改；未做修後複查。

## 可選 I1：修正 return 的前文章號

- 發現者／時點：本角色獨立讀 `006-002.md`。
- 類型／分級：引用位置／選讀改善，**不阻礙主線**。
- 原句：`return 的作用與第 1.5 節相同`。
- 實際前文：1.5 `005-002.md` 教尾端expression與unit；1.6 `006-003.md` 的「提早交出結果」才明確教 `return 值;`。
- 理解收益：讓想正常複習提早回傳的讀者找到真正的首次解釋；當前5.6自己已說清整個函式回傳，所以不是必要理解缺口。
- 最小建議：第1.5改為第1.6。不新增內容或要求重做測試。
- 狀態：留root裁定；沒有修改來源。

## 必要圖的來源查核與未做範圍

`006-003.md` 的 `question-paths.svg` 保留在新規則説明後、短程式前。原SVG的上方Result操作分成左右路：左框Ok→「取出value」→「繼續下一行」，右框Err→「From轉成回傳的錯誤」→「整個函式提早回傳」。相邻文字說一次只走一條、右路不會回左路，又將From接到已教的ParseCountError封裝。因此此圖的對象、箭頭與執行界線有足夠來源對應，沒有靠箭頭顏色猜同一物件。

其他主要關係可由文字與具體對照看清：Count建構／private field、ParseCountError兩種原因、兩個Label身分與text投影都在正文顯示。沒有發現必須新增圖才能補足的概念缺口。仍可選擇圖解降低負擔，但不能從原始碼坐標、圖片存在或build成功推定實際頁面清楚。本輪**未看實際SVG畫面，也未看桌面／手機HTML**，故視覺結果保持未驗證。

## 本輪結論與後續界線

第5章主線銜接具體而穩定：沿用件數規則，從失敗政策、顯示、已驗證型別、解析原因，走到錯誤傳遞；章內主要 API 的最小意義都在程式前教授。未發現提前依賴Box／dyn／closure／Iterator等未來機制。關聯型別不是僅出現名字，004-004已經教Error角色及Self::Error，足以支持後面的Err／Owned。

本輪列 **1項必要小修N1、1項可選定位修正I1**。N1未修不把嚴格“所有example先教再用”的範圍說成完整通過；其餘內容依賴通過與未做的實頁／執行分開保存。本文件為獨立初判，先生成SHA-256 sidecar封存，等待root之後提供獨立交叉覆核材料；不提前查看其他角色或live修正稿。
