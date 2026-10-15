# 第18章A作者紀錄

本輪負責18.1–18.6、A1–A4工作版／完成快照與duplicate-finder；不修改B、根Cargo／lock、nav或其他章。依clear-tutorial親讀原稿、實際來源、EN／ZH教材，再建立引導。研究與驗證原始證據保存在`/workspace/work/tutorial-implementation-20261015/ch18a-author/`。

## 入口與必要概念

A使用已完成第1–12章：9.2 callback/FnMut、9.4–9.7 iterator與排序；10.5原生CLI與flush、10.6 clap、10.7 JSON、10.8 child fixture、10.9完整工具；11.5 static、11.7 Error；12.3走訪／觀察、12.4摘要／短讀／比較／checked指標。作者在此前任務親讀必要前綴並親寫9、12，本輪另全文親讀10.6／10.7／10.9／11.7及所需實際sources；見prefix-and-old-read-ledger.json。未把14／15／16章當成先備。Root稍後交接17.4實際freeze，18.6使用前仍須本人讀實稿。

| 當下工作 | 首次需要 | 已教依據／本頁位置 | 能核對的結果 |
| --- | --- | --- | --- |
| 原生檔案記錄與問題分流 | 18.2 starter | 12.3；18.2需求表、三函式角色、完整source在check前 | Ok／Err／Ok不丟最後成功；root失敗不同於項目問題 |
| 可重複callback與確切失敗路徑 | 18.3初段重建／函式介面 | 9.2；18.3先給A／B／B與嚴格減少，再說FnMut參數結果 | 移除左代表A，B／B仍能成群；Err不是false |
| owned結果與暫時借用群 | 18.3 models | 7.3與12.4；models前說明bucket有效範圍 | 局部bucket可結束，owned路徑仍交給下階段 |
| 完整起點與答案 | 每階段最早Cargo前 | 所有實際載入manifest/modules/helpers/tests同頁完整snippets | check成功與新增行為通過分開判 |

## 實際來源與隔離

原A共98檔／60種不同內容，全部實際內容已讀；同SHA副本逐bytes核對，未用摘要代替原文。初始hash群在source-groups.json。所有stage manifests皆literal、無path dependency；a1不載content，a2不載report/app，a3不載app，a4不載最終root-API版本。checks並非Cargo自動tests，讀者用編輯器加入自己副本。作者失敗驗證也只在work下隔離副本追加，不改原starter。

按原語意改交叉引用：舊9.3／9.4→實際12.3／12.4；舊6 callbacks／iterators→9.2、9.4–9.7；舊CLI7→實際10；舊error5→8，Error8.6→11.7。by-section提案只作索引，沒有當作已教證據。保留A2真實callback契約，沒有套用12.4「無callback」的資料盤點差異。舊15.4的「修後」對話用語、15.6的offender／合同另在對應稿處理。

## 第一波18.1–18.3作者freeze

完整A1／A2起始與答案均已同頁內嵌，包括manifest、所有src、checks、完成版同檔tests與demo。18.1六份真fixture逐份給完整bytes，兩空檔明確為零bytes；沒有以外連補答案。原來源Rust未改，只同步A READMEs的章號與具體中文稱呼。

`validation-wave1/commands.json`：9條成功、2條預期行為失敗。成功包括兩starter check、A2未加入新checks前8舊tests、A1完整8tests、A2完整20tests、兩個六檔demo、四package fmt與Clippy alltargets。新增checks後A1為1成功／7失敗，A2為13成功／7失敗；真stdout/stderr原樣保存。另有一次作者隔離副本未明選toolchain的設定錯誤，原始`initial-toolchain-*`保留；改成正文指定的`cargo +1.99.0`後成功，不混計行為失敗。

三張舊ch15圖親讀XML，Cairo初次300px中文字為方塊；該預覽不能當中文字已驗。Root同意另存同內容至projects-a-v3，明選已安裝Noto Sans CJK TC，最低18px改19px。三張新圖已親看300px PNG：中文字、箭頭、問題路徑與狀態可辨，無截斷。這僅為獨立SVG預覽；尚未看新版網站原位desktop／mobile、折疊與累積閱讀路徑，交root／continuity另驗，未宣稱整頁驗收。

第一波exact hashes見`freeze-18a-wave1.json`；保留此初始freeze，後續變動追加，不回寫初判。作者自查不冒稱獨立審閱。

## 第二波18.4–18.6與最終freeze

18.4先說Option的Ord、診斷完整排序欄位、owned schema／null與Option::as_ref，再給所有starter與checks。18.5先固定資料／診斷／0／1／2與write_all→flush順序，再接clap與泛型writer；全部library與main都在第一條check前可取得。18.6同頁逐檔列完整995行core、662行Rust驗收helpers、六份完整bytes fixture與README；沒有刪scope或補行數。README以四反引號包住，避免內部shell code fences打斷外層。

Root交接17.4後，作者親讀實際全文與ch14-api manifest、六份library sources、typed_errors與public_contract；exact ranges/hash在actual-17-4-read-ledger.json。18.6的pub use、non_exhaustive與來源／行為相容引用此actual前綴，不再視17.4為待寫。其他14／15／16內容未當前提。Root窄移12.4既有source到首run前的prose-order followup已通知；本文依原本已全文讀過的概念與source，未假定有新增API。

`validation-wave2/commands.json`：23條成功、3條預期行為失敗。成功包括A3／A4 starter check與原20／29tests、A3完整29tests、A4完整41tests、四份階段text／JSON demo、最終17tests／release build／text／JSON／doc、五package fmt／Clippy，以及repo外獨立完整答案check／17tests／release／JSON／本機install／安裝後binary JSON。新增A3 checks後21成功／8失敗；A4 library為29成功／6失敗，單選CLI則2成功／4失敗。每一條真stdout／stderr獨立保存，不把負面測試中的成功分支當成未完成stage已完成。Rust沒變，第一波通過targets未重跑。

全六頁`all-snippets-and-closure.json`逐筆核實snippet來源與region存在，且每個實作頁starter／complete所需全部.rs與manifest有同頁全檔snippet；18.6亦包含README。六檔材料以全部byte序列表達，空檔明列。`loc-final.json`核對一份最終package995+662=1657物理行，排除stage副本／lock／產物。`source-isolation-plan.md`列最早命令實際載入的來源與測試，不靠all-targets名稱誤以為隔離。

第一波原freeze與兩份原ledgers另保存，最終freeze另列exact file list與hash；作者章節到18.6為止STOP，B與後續工程未啟動。全部檢查只在本次Linux環境執行；平台條件下未跑的非Unix行為不冒稱已驗。獨立SVG中文300px預覽已做，網站原位desktop／mobile仍待root／continuity；未執行全書check.sh或聲稱通過獨立審閱。
