# 第 1 章前三頁：技術交叉覆核與修後複查

日期：2026-10-14。角色：`chapter10_concurrency`，未參與前三頁的作者工作。本次在獨立技術初判封存後，才讀首讀、銜接與協調者紀錄；這是受已知問題提示的交叉與修後檢查，**不是重新盲讀**。

## 版本與範圍

- 原始技術初判保留於 `planning/reviews/ch01-prefix-technical-v1.md`，封存 SHA-256：`79482dfaa3f4edb62bd6a791beb6d418d3d3ffb536daa033667f210055f19255`。本輪未修改它。
- 本輪互看的紀錄：`ch01-first-reading-v1.md`、`ch01-prefix-continuity-v1.md`、`ch01-prefix-disposition.md`。先讀角色各自來源判斷，再對照協調者裁定。
- 修後版本：`/workspace/work/tutorial-implementation/frozen/ch01-prefix-fixed-v4/`。
- v4 manifest SHA-256：`3f515e98ccd3d4536bfd791835347ebf85d5a2d01365df99590d0cd1c5f63c27`。
- v4 檔案包含全章的 manifest 資訊；本角色只開 reading-order 的前 15 個正文單位，即 1.1–1.3，沒有開尾七頁的正文、程式或 SVG。
- 這 15 個 chunk 的實際 SHA-256 與 v4 manifest 全吻合。v1 中 hello-world 的五份 source 指紋與 v4 相同；這輪變動為說明、順序及安裝步驟，不重跑無关的正常 Rust 程式。
- v4 三頁原稿指紋：1.1 `7f9ad9c8c7d3218bf5f303d37cfe786a8ae8d818b98f233bc9862eede00439de`；1.2 `5d3d0e44d6b3145f9799e0eb517aa1adcf88c789920a59811e0a8cfe040f63b7`；1.3 `713fd7e96d5eb3792e3490b09e7414b684e390b8d871fffa23b9e9656a2df217`。

## 互看後的差異與理解收益

| 來源初判／差異 | 交叉覆核 | 收益與處置判斷 |
| --- | --- | --- |
| 技術 N1 抓到分號斷言；首讀與銜接未報 | 初始 snippet 正確，AI 首讀者沿示範操作也能理解；不表示「不能省略」這個額外技術斷言正確。Reference 規則與最小實測已直接反證該句。 | root 採用只描述示範格式的修正合適。不需為糾正一句錯誤提前教完整 expression／unit；原來的基礎閱讀成果仍保留。 |
| 技術 N3 與銜接 N1／N2 都抓到 manifest 契約後置；首讀整個單位末評足夠 | 首讀紀錄對 manifest 的角色回答有後置段落可據，能支持讀完整單位後建立理解；不能反推讀者在第一次看設定之前就已學過新欄位。兩個專家角色的時序問題獨立重現，非同意他人答案才新增。 | 前移原有解釋能滿足使用者的範例前置約定；正文不變長，不增加 Cargo 演算法或完整設定規格。採納收益具體。 |
| 技術 N3 把 -p／locked 操作前的最低契約也列必要；銜接角色判相鄰解釋可接受 | 原文先有選 package 的問題與 lockfile 一般角色，確實讓操作意圖可預期。差異在本專案更嚴格的「例子只能用當時已教內容」，而非完全沒有解釋旗標。 | root 把現有旗標與 lockfile 段前移，改善成本很小，又消除首次可複製操作的未教部分；支持採用，不把銜接角色的整體理解通過判成無效。 |
| 技術 N2 把正體字列必要；銜接 I1 列可選 | 兩者報出同一組字與相同格式收益，都沒有主張它妨礙核心理解。差異是專案強制字形約定與教學缺口分級的取向。 | 直接校對最合適；保留兩種原分級，將它視為格式修正即可。無需統一升成主要概念阻礙。 |
| 首讀 F1 與銜接 N3 抓到 Ubuntu／Debian 安裝入口不足；原技術初判未列 | 只有基本 Python 背景，不能假定已會 apt／sudo。官方 Book 提到 build-essential 能提供材料，並不能代替本書讀者完成被點名平台分支的實際步驟。角色的缺口限缺工具的 Ubuntu／Debian 路線，不是主張 linker 概念完全未教。 | 這個漏項成立。增加 apt／sudo 的最低角色與兩行命令，可以把本節的環境準備做完，且不要求 C／Linux 管理完整課程。本角色是在互看後核實，**不得計成本角色原先獨立發現**。 |
| 技術 I1 的 check 措辭改善；其餘角色未報 | 初始 Hello 的 check／build／run 操作及輸出皆正確；可選改善只縮小 check 對所有編譯階段的可能概括。 | root 採用更短措辭有益，不需加入 code generation 的旁支。本例主線從一開始就有足夠操作依據，不必為文句修正重跑它。 |

檔案樹是三角色共同保留的核心通過項：前文分開自行新建的 hello-rust 與 repo 的 hello-world，再用樹標記根 manifest、成員 manifest、binary crate 根檔。這些來源對應足以回答本節的多 package 選擇問題，沒有必須以 SVG 才能理解的缺口。這項 source 判斷不等於本角色看過實際手機版面。

## v4 修後逐項複查

下列行號皆是 v4 凍結 chunk 的行號，不拿新文字改寫 v1 當時讀者答案。

| 項目 | v4 的新原句與位置 | 修後判斷 |
| --- | --- | --- |
| N1：不可省略分號的錯誤斷言 | `002-002.md:22`：「本例用行末的分號 `;` 結束顯示工作。」原本「不能……省掉」已移除。 | **已修妥。** 新句描述示範格式，不再把本例末尾分號說成語法不可省略。前面的 macro 使用契約仍只教本次採用的帶分號形式，不需要提前教未使用的尾端 expression。Rust source 未變。 |
| N2：正體字形 | `002-001.md:12`：「請換一個新名稱」；`002-004.md:8`：「還沒有執行它」；`003-005.md:9`：「第一次還沒有 lockfile」；`003-005.md:20`：「命令的起點一致」。 | **已修妥。** 原報的四處字形全部改正，概念、API 與範例結果不變。此項是文字約定通過，不當作額外教學實測。 |
| N3：standalone manifest | `002-003.md:5` 先教 TOML；`:7–9` 先說 name／version／edition／dependencies 的具體角色；完整 TOML 從 `:13` 才開始。 | **已修妥。** 每個展示欄位在 block 前已有當下用途，版本不是工具版本與空依賴區段的理由也已先說清；無新增完整 manifest 規格。 |
| N3：root manifest | `003-004.md:7` 先說 workspace／members／路徑清單；`:9` 先說 resolver 3；`:11` 先說共同欄位／rust-version／publish／bool。完整根 TOML 從 `:13` 才開始。 | **已修妥。** 說明仍回到原檔案樹位置；節錄模型不可覆蓋實際 repo 的限制保留。不再由 TOML 樣本先引入未教設定。 |
| N3：member 繼承 | `003-004.md:26`：「共同欄位需要由成員明確選擇採用」；`:28` 先解釋 edition.workspace=true 與其他欄位同樣繼承。成員 TOML 從 `:32` 才開始。 | **已修妥。** 讀者先知道 true 在此選擇取根共同值，才看 publish.workspace=true；能由已教根的 publish=false 推得不允許發布，沒有引入相反布林意義。 |
| N3：-p／--locked | `003-005.md:5` 先說「-p hello-world 指定 package」及 name／path 區別；`:7` 先說 lockfile／locked 契約；`:9` 先說初建獨立專案無 lock。兩行命令從 `:13` 才開始。 | **已修妥。** 已有 package／manifest 名稱、check／run 背景再加這些必要選項，操作前置完整。版本保留與工作目錄仍是不同約定。 |
| I1：check 措辭 | `002-004.md:7`：「先檢查原始碼中的編譯問題；不產生供你執行的最後可執行檔。」整張表在可執行命令之前。 | **已採納且適當。** 沒有暗示 check 完成所有 code generation；不必加入內部流程。build／run 的主要差別與舊版執行證據仍相容。 |
| 首讀 F1／銜接 N3：Ubuntu／Debian 入口 | `001-003.md:25` 先教 apt、update、install build-essential、sudo 權限與密碼／確認；`:27–30` 才列兩行命令。已學終端位置在 `:5`。 | **書面操作缺口已修妥，技術契約有來源。** 讀者可以辨認用哪個工具、各步做什麼與輸入位置。不宣稱這輪在未配置的 Ubuntu／Debian 完成了實際安裝。 |
| 同段 macOS 操作 | `001-003.md:19` 先說 xcode-select 的工具角色及 --install 請求，再列 `:21–23` 的命令。 | **順序合適。** 修補 Ubuntu／Debian 沒有使 macOS 路線退化，兩者的 linker 需要仍在 `:17` 先教。未做 macOS 實際安裝。 |

## 新增平台操作的原始來源核對

- [Debian apt(8)](https://manpages.debian.org/trixie/apt/apt.8.en.html)：DESCRIPTION／update／install 支持 apt 為互動式套件管理入口、update 取得套件資訊、install 安裝指定 package。文件沒有把 update 誤說成升級所有已安裝 package。
- [Debian build-essential package](https://packages.debian.org/trixie/build-essential)：其 dependencies 列出 GCC／G++、libc 開發資料、make 等，支持它作為取得編譯建置工具的入口；與先前已查的 Rust Book linker 建議相容。
- [Debian sudo(8)](https://manpages.debian.org/trixie/sudo/sudo.8.en.html)：DESCRIPTION 支持允許使用者以管理者身分執行指定命令，並依設定可能要求認證。教材用「可能要求你的登入密碼」沒有宣稱每次必定輸入。

上述是新操作的契約查核，未實際修改系統套件、權限或工具安裝。沒有必要為了判句與順序重裝環境。

## 本輪結果與限制

在 v4 受審前三頁，原技術 N1／N2／N3 與可選 I1 已修妥；互看後確認的 Ubuntu／Debian 入口缺口亦已在原位補好，新增命令前有最低角色與動作解釋。修正以重排既有解釋與少量必要安裝步驟完成，沒有增加後章 Rust API 或換成新範例。

原技術初判保持封存，本輪沒有當成重新獨立重現。首讀者的原始通過／未知與其他角色不同分級也保留；不以互看後答案回填原先理解。

本角色未親看正式桌面／手機頁面；協調者記錄的 1280×800、390×844 視覺檢查屬 root 的證據，本角色只引用，不能算自身視覺驗收。未讀尾七頁，未做全站建置、全書 API 順序檢查或真人讀者測試；本結果限這 15 個修後閱讀單位與上述已報項。
