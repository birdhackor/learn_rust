# 15.10 B4 作者 ledger

唯一AUTHOR15.10，15.11關閉。source notes記fresh EN／ZH完整範圍、actual B1-v8／B2-v9／B3-v10／current11.5-v16全部實讀與固定API來源。A與舊B stages／frozen／d33 handoff不改；共享Cargo／lock／nav由ROOT處理。

ROOT已確認本節先完成typed停止Batch／受控事件／真Ctrl+C demo；正式CLI／JSON與完整退出優先序在15.11接續，最終功能不縮。來源失敗／關閉與Requested分開保存，後續致命錯誤優先於130取消。

| 必要關係 | 首次需要 | actual／本文安排 | 可預測什麼 |
| --- | --- | --- | --- |
| 取消等待與取消tasks | 入口held材料 | actual11.5＋B3 owners；local停止三責任 | select返回仍須abort＋join，Fixture也另shutdown。 |
| 完成／Cancelled／Unstarted | 新model前 | B1 Check、B3 TaskFailure、11.5分類 | 已完成保Check；未提交沒有Id，不能算網路問題。 |
| 完整原文與原index | 停止後報告 | B2／B3 Input與Id表 | 空洞及duplicates仍可對回原行；pending invalid不先解析。 |
| new不fill／record不補工 | constructor／classify前 | 11.5查stop再fill | 提前停止提交0；收取後先查停止才補工。 |
| try_recv＋biased select | runner前 | actual005-002／007完整 | 已排入停止優先處理；觀察前仍可能已有提交。 |
| abort_all＋全join | stop helper前 | actual11.5／B3 | 保實際Output，不把所有已提交者覆寫Cancelled。 |
| owned測試資源 | checks前 | local同task借用select／有限控制支路 | 觀察被取消後仍持所有children與Fixture，可先全收尾再斷言。 |
| 真Ctrl+C ready／controller | demo前 | actual005-010／011及fixedctrl_c | 確认listener已進入再READY；正常或停止都收回controller。 |

普通library直接使用mpsc／signal，因此把已教sync／signal features放normal dependency；單筆reqwest／url契約不變。每個新snapshot獨立，不把多份快照計為讀者反覆作業。

ROOT member lib-only check曾指出starter私有stop_reason未使用；作者針對此helper沿已教dead_code attribute暫允許，正文會在起始snippet前交代struct／impl／helper的限定範圍，full不需allow。這是起始版未連接流程的提醒，不能當作已完成停止政策。

首次作者scope：fmt通過、Clippy因pending while-let iterator提醒失敗，七lib tests已通過、starter零tests通過、signal example build通過。原始checks／Clippy／tests／fmt／build另存initial，不改成假通過。為for borrowed pending先教by_ref並親查fixed Std1.99.0來源，見source notes；後續只核此新B4受影響scope，不重跑舊B3。

## 完成證據與界線

採by_ref之後，兩package fmt check、all-targets Clippy（deny warnings）、完成版七項library tests及signal example build均通過。starter零tests沿初次實跑，不重複冒充新結果。repo外副本實際走過：all-targets check成功、零tests、追加checks後1通過／6失敗、只完成record與stop後2／5、完整run後7／0，最後all-targets Clippy通過。副本具有自己的manifest／source／lock；僅CARGO_TARGET_DIR沿用作者cache節省重編空間，完整記於guided-route.json。

真Unix SIGINT在已flush的READY後送出，原binary指紋保存。實際退出130，原index1／3為Cancelled、4／5為Unstarted，GET=2，全部client／listener／server完成；stdout與空stderr皆保存。初次host script把全形READY冒號誤比成ASCII，未送訊號而以觀察guard強制終止；保留舊script、失敗觀察與aborted標記，未補造未保存的stdout prefix，也不把那次host強制終止當Rust收尾證據。僅修host比對，再做受影響probe，沒有重編未變動Rust。

scratch strict build通過。作者在環境恢復後親看桌面1280×900、手機390×844各18張原位圖文視圖，共36張；包含原行表、typed model、全圖／圖說、by_ref、觀察guard、合成材料界線、兩步引導、提示與答案收合／展開、signal與demo角色。SVG實際桌面360×760、手機343×724，文字不出viewBox，原index／Id與停止分類可辨；手機長code由區塊內水平捲動，單張視圖不冒稱顯示完整長函式。四個details預設皆收合；DOM無未知HTML tag，頁寬未超viewport。無需改動已通過的Rust或strict build來源。

本步完成快照1035行Rust（含空白與snippet標記）：library core277＋既有Input29／single104＝410，參考tests470、有限signal demo155。這不是最終B版行數，也不把B1–4各快照累加成讀者作業。最終CLI／文字JSON／完整退出優先序仍待15.11授權，沒有提前實作。

正式handoff存work-only ch15-b4-author/handoff.json，記14份教材／程式檔、這兩份notes、原始checks、官方selected API、repo外引導、真訊號正反觀察、36張親看視圖與逐檔SHA。來源核對33份B1–3先備archive指紋皆不變；兩份B4 Input／single各與B3相同，checks整module與完成版suffix相同，repo外最終source與完成版相同。ROOT作者期precheck與自己的圖像觀察另存，不把這些作者證據當獨立QA或已發布。
