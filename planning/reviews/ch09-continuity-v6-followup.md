# 第 9 章 CONT：交叉與 v6 修後局部覆核

本輪確認原 CONT O1／TECH I1 的兩處粗體呈現，以及原 CONT O2 的三個答案字級，已在凍結 v6 的獨立實頁修妥。沒有新增必要待修項。原先教學依賴、方法理由與必要圖的通過範圍保持，未把這輪交叉或修後畫面回填為第二份盲讀。

## 時點、來源與原判保留

審閱者 `chapter03_data`，2026-10-14。原 CONT 的第 3／6 章作者背景仍適用，未撰寫第 9 章。本輪在 root 開放 cross 後，才全文讀取 `ch09-first-reading-v1.md`、`ch09-technical-v5.md` 與 `ch09-root-disposition.md`；未讀 peer10、後章、作者 ledger/source notes 或 live 第 9 章正文。

三份 original 在本輪核 SHA 均保持：

| Original | SHA256 | 原始範圍／分級 |
| --- | --- | --- |
| FIRST | `7feff159c1a6f23bc9e61c53bf0893fd261c7a624b11a39cbd7d32dc4e91740d` | 27 BODY 順序閱讀、closed 答案未見、三張原位單圖；necessary 0、optional 0。沒有完整桌面／手機實頁審查。 |
| TECH | `38dee2d6274c72f72826743163d343db51e572e9873baef839eb3317207918fc` | BODY/archive/selected 官方契約、六個相稱 probes、原位實頁抽查；necessary 0、optional I1。 |
| CONT | `5f3112ed7fc6db3e4b9192d4a095bf03556330190d8608c92af6425b818fa5e6` | BODY/archive、實際依賴與 28 張原位桌面／手機圖；necessary 0、optional O1/O2。 |

粗體 delimiter 問題由 TECH I1 與 CONT O1 在未互看時，各自實見原樣星號、提出相同呈現收益，屬兩份 original 的獨立重現。FIRST 的正文／單圖範圍沒有報此實頁現象，不能將這輪得知或同意算成 FIRST 原發現。答案字級 O2 保持為 CONT 原發現，沒有事後補成 TECH/FIRST 的獨立命中。

自己的 original 報告、SHA、原 notes 與 `sealed-original` 均未改。此 followup 另封 SHA，僅記 cross／修後證據。

## 自行重建 v5→v6 差異與固定預覽

唯一修後教材來源：`/workspace/work/tutorial-implementation/frozen/ch09-review-fixed-v6`，manifest SHA256 為 `8663faec8eb6fbe5931b1388880182f96d465910d1118dd6d442beb5a6e995f8`。

我自行比對兩份 manifest、逐個 BODY 與 archive byte；將凍結圖 URL 起點規格化後，實質 BODY 差異僅：

- `002-001`：`**…crate 編譯。**這支程式…` → `**…crate 編譯。** 這支程式…`。
- `004-001`：`**…完全相同？**檔名…` → `**…完全相同？** 檔名…`。

其餘 BODY 的差異只在三張 SVG 的凍結資料夾 URL。原 39 份封存 source 逐 byte 相同，Rust/Cargo/fixture/SVG 均未改；唯一新增為封存的 `docs/stylesheets/tutorial.css`。CSS SHA256：`fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`。它只有 `.md-typeset details { font-size: inherit; }` 及作用註解。

上述自行結果與 root 的 `ch09-fix-scope-proof.json` 相符，但沒有只採信該 proof。自己的證據在 `/workspace/work/ch09-continuity-v6/independent-diff-proof.json`。

獨立預覽位於同一 work 目錄的 `preview/`：從 v6 BODY 重組四頁，保留折疊狀態，僅去掉閱讀包重複頁標題、改 SVG URL；將 URL 還原後逐頁 byte 相同。三張 SVG 同原 v5。使用原 CONT 固定 preview 的 theme/extensions，明加 `extra_css = ["stylesheets/tutorial.css"]` 並只複製 **v6 archived CSS**，未讀 live 正文或用 live CSS 偷換。新 config SHA256：`c40f2f781e68ac2e65c113b36efefd8f2182ddecab8f3ef5e0de33ec85e83917`。

局部 `zensical build --strict` 通過。瀏覽器載入的 CSS 內容 SHA 與 archived CSS 相同；不以 CSS 原始碼或 build 成功代替下面的實頁驗收。未重跑同 byte 的 Rust 或 all-workspace。

## 對照同行的原需、理由與支持範圍

交叉的對象是正文已提出的關係，並非以同樣「0 necessary」相互代答。對讀各方具體四題／P1–P7 與原 CONT 的實際段落依據，結果如下：

| 來源焦點 | 同行原判與本人的交叉覆核 | 保持的通過範圍 |
| --- | --- | --- |
| 9.1 共用依賴、profile、feature/cfg | FIRST 001-001–006、TECH P1 與 CONT 將三種管理工作分判。共同來源需 member opt-in；feature 只加可選介面，profile 不改 edition/target；cfg 排除與普通 if 的名稱檢查有正常／失敗對照 | 160/280 及零數量 120 支持規則與功能組合，不支持效能或 cross-platform 執行。這兩處修字不改該鏈。 |
| 9.2 建置固定資訊、stdout 協定、host/target | FIRST 002-001–005、TECH P2、CONT 皆從「同成品回報建置選擇」接四時點、交付名稱、env!。TECH 的六個 probes 增加其技術證據，沒有當成 FIRST 當時實跑或已見答案 | build/runtime 差別、配置拒絕與 target 資料來源足夠；OUT_DIR/生成檔案仍相稱導覽。修後粗體改善辨認新步驟，不新增概念。 |
| 9.3 walking 分工、原生身分、失敗與 partial move | FIRST 003-001–007、TECH P3、CONT 的政策—新 API—記錄—結果路徑一致；必要操作在正文先教，Option map 不冒充 iterator 丟錯。核心保留原路徑、最小 Unicode 報表政策保原因，成功查到 5 不代完整性 | 根失敗與空成功、item Err 後繼續、PathBuf/Borrow<Path> 與 lossless 身分、TempDir owner 皆由實際前文／當頁教過；完整 CLI exit policy 合理留後續。 |
| 9.4 長度／摘要／內容分工 | FIRST 004-001–003、TECH P4、CONT 的原問題都是完整 bytes 相同，理由都是利用既有 metadata 再集中候選工作；不是從 digest 產物反定義需要 | 分配工作理由足夠，沒有速度量測。修後強調的是原問題，三判準表與例子不變。 |
| 9.4 短 read、fill_chunk 與有效 slice | FIRST 004-004、TECH P4、CONT 均分清 read 切分與 bytes 序列；ABC/D/E 的剩餘格限制有正文圖說，不因最初 ABC/DE 概念例誤判圖矛盾 | 異短讀仍相等、尾 byte/長度不同、有限來源 EOF、Interrupted 重試／其餘 Err 分左右。技術讀 source、FIRST 正文推論與實跑範圍分開。 |
| 9.4 File 觀察／錯誤身分 | FIRST 004-005、TECH P5、CONT 均保留「盡力而為」的條件；path 與 open handle 是不同查詢，沒有把借用有效性變成檔案鎖 | 可見變動／具體 offender 可理解；時間精度、改後恢復與替換 gap 仍不提供一致安全快照。沒有以必要限定可刪來判冗餘。 |
| 9.4 真群組、指標與有限元件 | FIRST 004-006–009、TECH P6/P7、CONT 均直接由手工 A/B/B 看見第二代表比較的必要；singleton 獨立確認成本、native path 去重、借用群組與 checked metric 各有理由 | 不需真碰撞、不把 Err 當 false、非空組支持 index 0；兩個 hard-link 路徑的量不等於磁碟可回收量。完整重試／報告政策不在此刻程式中偷用。 |

未從同行原判找到需要改寫學生正文的新增缺口，必要圖也沒有新漏項。原 CONT 的每項必要先備仍以其已讀 actual 1–8 及當頁首次使用位置為依據；不是因看過 TECH 的官方解答才放行。

審閱定位精度另分開：FIRST 的 `001-005` 用「1.10 integration test」作簡稱；integration test 的明確教學其實在已可用的 7.2，1.10 是 assertion／同檔測試。原 CONT 已核 actual 7.2 的獨立測試 crate 契約。這是 reviewer 引用位置的細節，沒有變成第 9 章的學生先備缺陷，亦未改 FIRST 原文。

## 修後親看：兩處 entry 與三個展開答案

用自己的 Chromium/CDP 在固定 v6 preview 生成並親看 **16/16 張**：兩個 entry × desktop/mobile（4 張）、三個答案展開 ×兩尺寸（6 張）、三張 SVG 原位 ×兩尺寸（6 張）。實際尺寸為 1280×900／390×900，device scale factor=1。截圖與 DOM/互動紀錄保存於 `/workspace/work/ch09-continuity-v6/screens/`、`visual-metrics.json`、`capture.mjs`，未沿用 v5 小字畫面宣稱修後已驗。

### O1／I1：兩處粗體已修

9.2 entry 的新步驟與 9.4 entry 的完整 bytes 原問題，在 desktop/mobile 都呈現預期 strong，沒有 literal `**`；DOM 分別保留同一句 strong text。句子與後句內容未丟，package 身份／指令起點、判準表與接續段落仍可讀。手機 document width=390，沒有新整頁橫向溢出。

結果：原可選項的實際收益已取得——讀者能直接辨認新增工作與原問題，少了星號雜訊。仍保留原「可選呈現改善」分級，不把修後變化倒推成原稿有必要觀念缺口。

### O2：code 字級、內容與互動已修

三個 fold 起始均為 closed，展開後各有 Rust 與 Bash 兩個 code block；一般 code 和 fold code 的 computed font-size 在兩尺寸均為 **12.8px**。原 v5 fold 的 10.9227px 不再出現於這些 code。親看六張展開畫面確認字型／字距可讀，fold 標題與正文位置保持，沒有把答案外的必要步驟藏進 fold。

| 答案 | 一般／fold code 字級（兩尺寸） | 手機 code 寬／內容寬（Rust；Bash） | 實際互動 |
| --- | --- | --- | --- |
| 9.2 build identity | 12.8px／12.8px | 326/395px；326/742px | 橫向 scrollLeft 可到 69／416；兩個 copy 都取得完整內容 |
| 9.3 path identity | 12.8px／12.8px | 326/753px；326/523px | scrollLeft 可到 427／197；兩個 copy 都取得完整內容 |
| 9.4 A/B/B solution | 12.8px／12.8px | 326/682px；326/508px | scrollLeft 可到 356／182；兩個 copy 都取得完整內容 |

桌面三個 fold 的 code 寬 906px，內容未超出，無須橫捲；手機長行仍能在 code 內捲動，不擠寬頁面。各 fold 內 Rust `textContent` 的 SHA **逐 byte 等同 v6 archived solution.rs**，結果見 `fold-source-byte-proof.json`：字級改善沒有丟型別參數、行、空白或程式內容。實際點擊 12 個 copy（3 folds ×2 blocks ×2尺寸），clipboard 與 code 內容忽略末尾換行後均相同；不是只看 button 存在。

對照量測曾修正兩項審查工具選擇，未修改學生頁：highlight 的 line spans 會使 `innerText` 額外插入行分隔，copy 對照改用實際 `textContent`；橫向可捲動對象是 code 元素而非外層 pre。早期量測保存在 `capture-attempt-innertext.json`，最終紀錄同時保存 copied／expected SHA 與實際 scrollLeft。這些是測量對象的校正，不列為學生 copy／scroll 缺陷。

結果：O2 的原收益已取得，展開答案與一般 source 同級；沒有新增先備、重複一份 source 或更動答案規則。

## 必要圖及銜接：沿用原判準再看

| 原位圖 | v6 本人再看時追蹤的必要關係 | 判定 |
| --- | --- | --- |
| 002-002 build-phases | 編譯 build.rs→host script 讀 LESSON_MODE／交 TEACHING_MODE→主要 crate 的 env! 固定→成品不重讀；箭頭為先後，換名由相鄰圖說點明 | 340×570，desktop/mobile 字線／四框及圖說完整，原位置未變。通過。 |
| 004-004 short-read-comparison | 同 ABCDE，bars 為 read 切分；左 ABC/D/E與右逐byte，各填4格或EOF後，ABCD與ABCD、E與E；4/4096只是容量差異 | 340×455，兩方、兩段及剩一格只能 D的圖說可連續追蹤，基本理解不需放大。通過。 |
| 004-007 candidate-content-groups | 上框手工同 digest 桶，b1 先與 a 不同、再與 b2 相同，下框才是 a 與 b1/b2 的真正組；第二代表比較是核心動作 | 340×480，名稱、箭頭、兩個真正組及圖前後文字完整。通過。 |

三圖不在 details，CSS 未改它們的尺寸、相鄰文字或資料關係；自己仍親看新頁，沒有只以 source 同 hash 放行圖文。

原先缺圖檢查也保持：9.1 根／成員設定與 path 起點對照已能建立共同來源；9.3 link 位置／政策表、資料與錯誤模型、Ok/Err/Ok 材料已足夠；9.4 三判準表、borrowed/owned 型別與兩個必要圖支持目前資料流。v6 只新增空白與字級，沒有產生新的空間／素材／答案對應需要，因此未發現額外必要圖缺漏。

章節過渡仍是設定控制→建置前固定資料→執行期檔案觀察→bytes 內容確認。兩個 entry 的原需意義保持；fold 變大只影響答案展開後的頁長，closed 主文的必要閱讀線沒有變成依賴答案。後章內容未讀，不借其實作補任何前提。

## 範圍與 STOP

- 本輪實際查證 v5/v6 差異與第 9 章指定三頁的兩處 entry、三 folds、三 SVG；四頁已重組並 strict build。沒有重新全文盲讀 27 BODY、重跑未變 Rust 或再次 all-workspace。
- 沒有另渲染舊 1–8 頁，不把這次三個 fold 的同級字體說成共享 CSS 全站回歸全部通過；root 另核舊頁。原 actual 先備的教學依據仍保持，並非在本輪換成 live 版本。
- 未測真手機、dark theme、所有 code-copy 或外部連結；只將上述 12 次 copy 和實際 scroll 證據算已驗。未實測跨平台、效能、一致檔案快照或完整後續 CLI。
- 原必要項仍 0；原兩個可選呈現項均已修後覆核通過；未新增待修項。不修改原判、不改學生或共享檔，封存本 followup 與 SHA 後 STOP。
