# 執行計畫 v1 獨立審查

**結論：APPROVE。required_changes = []。**

我完全同意以這份計畫作為本輪實作規範，沒有需要先改計畫才能開工的必要項。此結論只放行實作；新版正文、程式、圖與網站尚未完成，不能據此宣稱教材已驗收。

審查對象為 `implementation-plan-v1.md`，SHA-256：`86fd1787dcfe8f66efef395af6d9393cdfd45ceca5d4ca6ec88cde09ac8381ba`。依專案 AGENTS、clear-tutorial、Rust 術語指引及 review-protocol 審查。沒有取得前位審查結論，沒有修改儲存庫、執行 Git、Rust／Python 工具鏈檢查或瀏覽器驗收；本輪只讀來源、靜態解析對映與指紋，並寫本報告。

| 審查面向 | 結論與依據 |
| --- | --- |
| 教學章序 | 可以執行。數字函式先於 1.7 計算機基礎；String 在 pattern ownership 前；泛型在完整 Option／Result 前；associated type 在 TryFrom／FromStr 前；lifetime、HRTB、GAT／variance 與地址穩定有連續必修路線。未發現必須先推翻 18 章主序的依賴循環。 |
| 全部 109 節保全 | `proposed-outline.json.old_to_new_mapping` 的 109 個舊路徑與現行 zensical nav 的 109 個章節路徑集合完全相等，無漏項或額外項。常數泛型、短讀短寫、原生路徑、IntoFuture、atomic Ordering、FFI、Loom／Miri、效能及兩個工具里程碑均有明確保全規則。這證明計畫覆蓋，尚不證明改稿後內容保全。 |
| 27 筆問題 | 原 audit 有 27 個唯一 ID，補充的 `issue-dispositions-proposed.json` 也有同樣 27 個 ID，無遺漏或額外項；每筆有採用理由與最小改法，原 confirmed／optional 分級保留，修後證據仍為待完成。全部引文仍能在原檔找到；audit coverage 的 110 個來源指紋均與目前檔案相同。 |
| 可執行材料 | 計畫第 9、40、44、50、64、86、88–90 行合起來，要求所有新節的完整來源、設定、tests、helpers、失敗例與答案均按真實已教內容檢查，並有實際 source／manifest 交接及逐節驗收。這已禁止直接沿用會載入後續能力的完整 package，適用範圍不只第 1–2 章。下文列出本輪實際確認的風險，供執行既定工作時定位。 |
| 作者並行 | B 先建立 1–5 的真實前綴；C 的 lifetime 線只先做不依賴 trait 的部分；D 等錯誤、impl Trait、Display 的實際交接；E／F 對 FFI、unsafe Send／Sync、Loom／Miri、executor 另設等待邊。A／B 專案也各等自己的檔案與 async／HTTP 先備。不是以章名存在冒充可並行。 |
| 圖與選讀 | 1.7 的四組圖有固定數字材料與時點，明分概念 heap 圖和實際程式，未要求先借 Box／String／reference 語法。async 保留 lazy 與受控等待各自的證據範圍，並區分 JoinSet、task、借用等待 future。Python／JS 比較放在 3.8 後，Rust 正文不依賴選讀。 |
| 同頁完整例子與答案 | 有唯一實際 source、snippets、分檔呈現、完整起點與分層提示的明確交付；不以 GitHub 連結代替答案。保留現有兩個可用工具並核對核心程式口徑，不用測試行數或無關進階功能湊規模。 |
| 舊入口與引用 | 一對多相容入口、舊 URL、原 heading anchors、正文交叉引用及命令目錄均在遷移範圍。補充 anchor 清單含 109 頁、890 個 heading ID，各頁內無重複 ID；本輪只核對清單結構，未自行重建網站確認其生成。 |
| 審閱與發布 | 有凍結稿、非作者技術與銜接初判、指定重大新寫章的無作者歷史逐段首讀、全新節的基本覆核，以及未報問題圖文抽查。瀏覽器桌面／手機驗證有尺寸與證據要求；無法驗圖則不發布。嚴格 build、fixtures、兩個 workspace 與必要 Miri 檢查有明確門檻；現有 workflow 確實在 check 通過後才部署 main。 |

我特別核對了「階段快照」是否只修第一章。現行程式確有下列問題：

- `examples/ch01-basics/src/lib.rs` 載入 pricing 與測試；pricing 使用 if、return、for。第一章移走流程控制後，原 package 不能直接充當新版第一章完整快照。
- `examples/ch03-data/src/lib.rs:1` 載入 list 與 text；前者使用 Vec，後者使用 Option、match 與 UTF-8 存取。新版 3.8 的 String 範例若仍以這個完整 package 為材料，就包含尚未教的內容。
- `examples/ch04-traits/src/lib.rs:2` 載入 counts；`counts.rs` 使用 HashMap 與 clone。新版 6.1 必須按自身先備成立，不能只因這些內容也在第 6 章便提前納入完整材料。
- `examples/ch06-iterators/src/lib.rs` 同時載入 associated、adapters、callbacks、opaque、streams、text_ops。associated type 前移到 6.4 後，不能把整個 Iterator package 當作當節完整版本。現有早中段 manifests 的 `.workspace = true` 也不能冒充已教完的學生設定。

這些是**已存在、而且計畫已要求處理的實作工作**，不是新版已修好。我沒有把「原 package 能編譯」或「範例 main 本身很短」當作先備成立。第 50 行列 B 階段的快照安排，沒有取消第 9 行適用全書的規則；第 86／90 行也明確逐節驗收。因此無須以再加一條同義原則作為開工前置。實作時可以依材料使用少量獨立 package 或階段快照；本審查沒有要求機械地建立 131 個 package。

27 筆處置的方向也合理：模型／契約不一律刪除；缺圖與 1.7 合併；timeout 命名問題不虛構為缺少 Guard API；錯誤回指、名稱不一致、編修歷史殘字及展示引句各回到其真實理解負擔。逐筆採用並不等於機械擴寫，草案大多是短而直接的替換。新位置與修後證據目前空白，與尚未實作的狀態一致，不能提前填通過。

可選的紀錄改善只有一項：在既定的 sections／ledger 中加入「當頁命令、manifest、實際載入的教材自有檔案、首次可用小節」欄，直接以以上四個 package 作最初樣本。這只是讓計畫已有的逐節檢查更容易稽核；不增加新的放行條件，也不能取代正文閱讀。

實讀範圍包含：計畫全文；重構 report 的章序、Cargo、先備、並行與保全論證；outline 全部章節安排及 109 對映；audit 全部 27 筆發現及 coverage 指紋；27 筆處置草案及 anchor 清單；AGENTS／SKILL／術語／writing-prompt／review-protocol；Cargo、check／fixtures／Miri 腳本、CONTRIBUTING、zensical 與 CI workflow；上述早期與中段 source、String／associated type／lifetime／Sized 與常數泛型的正文或相關段落；async foundations／取消流程相關正文與 source；GAT／variance／macro 的結構與必要引入位置；兩份最終工具的答案頁結構與 library 入口，以及三份相關 SVG 的文字與對應。沒有逐字重新審閱全書 109 頁或全部 source，也沒有重做外部教材研究。

補充盤點也支持以上判斷：`proposed-migration-manifest.json` 列有 131 個 canonical 節點及 109 個 legacy 去向；我另抽讀 `inventory-early/sections.json` 的 1.4、3.8、5.3、5.5、6.1、6.2、6.4，已明列 no-lib、text-only、charge-only、associated-only 等具體隔離。這些仍是規劃資料，不是已完成的 API 或編譯驗收。

我另讀了原 `ch03-data/Cargo.toml` 的 `manual_map = "allow"` 及 `ch06-iterators/Cargo.toml` 的 `unnecessary_fold = "allow"`。不能把這些欄位默默帶回初學 manifest，也不能為了讓 Clippy 通過，在 5.3 提早使用 map／closure。計畫既有的設定／完整 source 先教後用規則已涵蓋它們；採先教精確 allow 契約再就地使用，或僅由維護檢查命令指定相應 lint，均可在現有章序內完成。associated-only 快照沒有必要繼承 fold 的 lint 設定。這是實作時必須滿足的既定規則，不是尚未裁定的章序變更，因而不新增開工前待修。

已讀主代理提供的 `baseline-verification.json`：它記錄原 baseline 因建置產物塞滿磁碟而失敗，清理產物並縮減除錯資訊後，check.sh 全套通過，且現有 Chromium 可用。本輪沒有獨立執行這些命令，也沒有逐行複核 baseline log；這份執行紀錄只能按其來源報告，不能當成本審查已執行測試或視覺驗收。

