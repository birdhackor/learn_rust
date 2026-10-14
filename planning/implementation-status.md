# Rust 教材實作與驗收紀錄

使用者於 2026-10-14 同意依 [課程規劃 v2](curriculum-plan-v2.md) 開始實作，並明確要求每位作者先讀對應的中英文著名教材，再撰寫正文；需要時加入 SVG。規劃文件保留原始討論版本，本檔記錄實際實作狀態。

## 作者共同契約

- 讀者只具備基本 Python。以好懂好學為優先，依 clear-tutorial 與專案術語指引寫作。
- 先閱讀實際網路教材章節，在 `source-notes/chNN.md` 保存 URL、閱讀範圍、借鑑的教學關係，以及須避免的超前 API。外部連結不代替正文解釋。
- 作者只有取得實際完成且已核對的必要前文後才開始正文與程式。核對正文／API 時序與真實程式，使先備可用；完整獨立審閱與公開發布另設完成紀錄，不將先備可用說成全部驗收通過。可先平行閱讀來源與準備研究筆記。
- 新語法、型別、API、必要回傳型別與錯誤，必須在使用前教清楚。測試、練習、答案與 Cargo 設定也遵守；不用「後面再說」藏掉先備。
- 在 `author-ledgers/chNN.md` 記首次使用、實際先備位置、當下解釋與讀完可判斷的事。規劃中的前置節點不算已教證據。
- 每小節是一頁，按 v2 編號放在 `docs/chapters/NN-*.md`。真實程式放 `examples/chNN-*/`，正文引用來源；SVG 放 `docs/assets/chNN/`。
- Root 統一修改 workspace、lockfile、導覽與發布設定。作者不競爭共享檔案。
- 作者自查、程式檢查、首次順序閱讀、技術審閱、銜接審閱與實際畫面檢查分開記錄。保留凍結稿指紋與原始問題，不把修後理解回填為初讀證據。
- SVG 需在首次需要處搭配說明；檢查桌面與手機實際頁面，未看過的畫面不宣稱通過。

## 進度

狀態「研究」只代表讀來源；「撰寫」不等於驗收或發布。每完成一波，追加實際前文版本、檢查與審閱紀錄。

| 大章 | 狀態 | 作者／安排 | 可用前文 |
| --- | --- | --- | --- |
| 1 工具與程式基礎 | 已審閱並發布 | chapter01_foundations | 基本 Python；ch01-prerequisite-v3 凍結稿 |
| 2 struct／ownership／reference | 已審閱並發布 | chapter02_ownership | 第1章修後實際正文；完整修後v5 |
| 3 enum／泛型／文字／Vec | 已審閱並發布 | chapter03_data | 第1–2章實際正文；完整修後v6 |
| 4 trait／HashMap／lifetime | 已審閱並發布 | chapter04_traits／chapter02_ownership | 第1–3章完整實際稿；發布v8 |
| 5 錯誤／格式化／轉換 | 已審閱並發布 | chapter02_ownership | 第4章實際修後正文；發布v7 |
| 6 closure／associated type／Iterator | 已審閱並發布 | chapter03_data | 第4–5章實際正文；原v6、發布v10 |
| 7 Cargo 生態系／I/O／CLI | 九頁原首讀、獨立初判及修後交叉完成；發布候選檢查通過，待額外首讀局部 | chapter07_cli | 第5–6章實際正文；原v10、修後v13 |
| 8 smart pointer／進階型別 | 十頁初稿與先備核對完成，逐段首讀進行中 | chapter01_foundations | 第4–6章實際正文；完整來源v8 |
| 9 Cargo 工程／檔案處理 | 四頁初稿與先備核對完成 | chapter07_cli／chapter02_ownership | 7.5／7.4／7.8與6.7實際完成稿 |
| 10 thread／同步／atomic | 10.1–6初稿及先備核對完成；10.7撰寫中 | chapter10_concurrency | 8.8／8.10與第5–6章實際稿 |
| 11 async／HTTP | 11.6同步HTTP完成，其餘依actual小節前置 | chapter11_async | async等10.7；HTTP採完整第7章actual稿 |
| 12 GAT／variance／unsafe／FFI | 12.1–12.3完成；12.4等10.8 | chapter12_unsafe | 4.5／6.2–6.3／8.1–8.8實際完成稿 |
| 13 Pin／Future內部 | 來源研究完成，等待actual前置 | chapter12_unsafe | 等11.5與12.4 |
| 14 macro／API／測試／效能 | 14.1、14.4、14.5初稿及先備核對完成；14.2撰寫中 | chapter04_traits／chapter01_foundations | 7.9／8.9／9.2等實際前文已交接；其他依既定gate |
| 15 引導型最終專案與完整答案 | 15.1–3完成，15.4報告撰寫 | chapter07_cli／chapter11_async | A的實際14.4前置已齊；B等async前文 |

## 驗收與發布批次

第1章已完成本輪首讀、技術、銜接與修後複查，並通過桌面／手機受查位置與程式檢查；已於提交27dcd37發布。其餘章節供作者使用的先備核對與公開發布分開記錄。

- 第1章：10頁、16個可執行範例、數值library與四項測試、一張SVG；`scripts/check.sh`通過，故意失敗兩項compile-fail及一項runtime-fail符合契約。根程式及每個範例輸出另有作者實跑紀錄。
- 前三頁v1逐段首讀已封存；技術與銜接角色在看他人問題前各自保存初判。尾七頁v2進行逐段首讀，不宣稱已通過。
- v3修後前文：補安裝操作入口、Cargo旗標／manifest契約先教後用、校正分號規則。這些修正不回寫為v1讀者當時已理解。
- root已看前三頁與1.9／1.10的桌面1280×800、手機390×844實際畫面，SVG手機渲染寬343px，文字與兩種箭頭可辨；其他頁面版面尚待檢查。

## 研究時發現的實際前置調整

1. **5.4 先教 associated type 的最低使用。**`TryFrom` 的實作需要 `type Error`，5.5 的 `FromStr` 需要 `type Err`；不能等待原規劃 6.2 才介紹。在第一次實作前教清楚每個 impl 指定的 associated type 與 `Self::Error`，6.2 再回收並擴展到介面設計。這是依選定 API 補必要先備，不增加一個進階旁支。發現來源：第5章作者的官方 API 閱讀。
2. **11.2 先教 Tokio task 身分。**取消或 panic 時，不能只靠 future 的正常回傳值取得輸入 index。介紹 `task::Id`、`AbortHandle::id`、`join_next_with_id` 與 `JoinError::id` 的使用契約，以已有 HashMap 維持 task 到輸入的對照。11.3 與最後 B 專案回收這個機制。
3. **11.5 接上實際 Ctrl+C。**先教可控制的 mpsc 中止來源，再教 `tokio::signal::ctrl_c` 的 feature、`io::Result<()>` 與監聽失敗；等待訊號的 controller task 在一般完成時也須取消並等待收尾。最終 CLI 有真實中止入口，測試仍採可控制来源。

### 本輪追加

第1章prefix與tail的原始首讀及獨立初判已封存，修後v8複查進行中；全外連答案現一併封存。第2章31個單位進行順序首讀；第3章已有17個可執行target、六項測試、七個故意失敗案例。根檢查的建置、fmt、Clippy與test通過；新增第9章fixture的.expected契約後，全部18個故意失敗案例通過。作者自查與root先備核對不當作獨立驗收。

本機預覽現含第1–3章，仍未對外發布；導覽變動僅供實際畫面檢查。第2章四張SVG已看手機／桌面並修正缺字、浮動TOC遮圖與尺寸；第3章作者先看過兩圖實際畫面，root另行覆核。

4. **7.2 的實際測試只使用已教數值。** 原規劃以4.2為可能先備；作者保留最小數值library與assertion，不使用Debug/derive，因此在1.10後已具備實際需要的能力，可提前完成工具頁。後續包含自訂資料顯示的測試仍等4.2，不把工具頁提前當作derive已教。

第1章公開部署成功：[Actions 38554462772](https://github.com/birdhackor/learn_rust/actions/runs/38554462772)。Root實際請求首頁及十節網址全為200、H1相符；未發布2.1網址仍404。第4章分工：4.1–4.3由trait作者，4.4–4.5由ownership作者，後者先重新實讀對應中英文來源；其最小先備是實際4.1、2.4、3.4，因此可與4.2–4.3平行，但不使用未教的後節API。

4.1 actual正文已核對並凍結為ch04-trait-prerequisite-v1（8個閱讀單位與來源），供4.4–4.5、6.2及8.1–8.2三支寫作使用。Root已親看trait圖桌面／手機原位：兩次呼叫各自的reference及true/false結果可追蹤，共用的是判斷規則；圖宽400／343px，無缺字或遮擋。這是先備與畫面檢查，獨立首讀/技術/銜接仍待讀者順序到第4章。

首讀亦採可核對的分工：first_reader 持續逐章讀第1→2→3；另由無作者歷史的first_reader_04先實際補讀第1–3章正文（保存入口背景與未知，不宣稱那是前章逐單位首讀驗收），再逐單位首次讀第4章。兩者都只取得指定凍結正文，不能先看作者摘要／答案／審閱問題；技術與銜接角色待各範圍首讀封存後再對照。不同讀者的已知前文與路線各自明記，不把它們混成單一讀者連續全書首讀。

### 最近一波先備交接

第4章完整v5首讀保存38個單位；修後v6的初始化語法、圖線、鍵欄位對照與練習文字已另存局部複查，不覆寫初判。第5章5.1–5.5、第6章6.1–6.5與第8章8.1–8.4／8.7的實際正文、來源、答案及故意失敗案例已由root核對先備。6.4凍結為ch06-impl-trait-prerequisite-v3；第8章五頁凍結為ch08-ownership-prerequisite-v3。這兩份實際材料打開8.5與8.8寫作，不將先備核對當成完整發布驗收。Root在6.3答案另發現array到slice的尚未解說簡寫，要求使用已教的range slice寫法；修後才凍結下一份完整第6章前文。

### 8.1 核心範圍補足：常數泛型

依root核對課程覆蓋後的明確追加指示，在8.1 Sized／?Sized之後、Box之前補常數泛型最小契約：區分T型別與const N:usize值參數、N在array型別及body中的角色、推導與turbofish明選。作者先實讀英文Reference與中文RustCourse對應，新增一份normal example及一份長度不符E0308 fixture，fmt／Clippy與相應執行通過；不為純helper增加鏡像tests。新段不改既有Box／Deref／borrow契約或章節順序，舊freeze及首讀原判保留；作者紀錄見第8章ledger，修後先備釋出與獨立驗收由root分開確認。
