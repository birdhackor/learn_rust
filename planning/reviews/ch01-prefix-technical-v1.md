# 第 1 章前三頁：獨立技術審閱初判 v1

審閱日期：2026-10-14。審閱者：`chapter10_concurrency`，本次未參與第 1 章寫作。階段：clear-tutorial 第二輪技術與證據。範圍只限 1.1–1.3 的 15 個凍結閱讀單位，不包含第一章其餘七頁。

本初判在讀作者帳冊、source notes、首讀紀錄、其他審閱紀錄或已知問題之前保存。審閱前讀通用 `review-protocol.md`；正文依 `reading-order.txt` 閱讀完整 15 個單位，並核對相應真實 source。這是獨立技術審閱，不是盲讀或首次閱讀證據。沒有修改教材。

## 凍結版本與執行證據

- 凍結資料夾：`/workspace/work/tutorial-implementation/frozen/ch01-first-three-v1/`。
- manifest 的 SHA-256：`64aea3d0d66661a4bfb1b9173743790a362fb0997e059ff8e303912c90fb97a0`，亦保存在 `/workspace/work/tutorial-implementation/technical-ch01-prefix-v1/verification.json`。
- 15 個 chunk 和 manifest 列出的 5 份 source，其實際 SHA-256 全部吻合。正式頁面的來源指紋沿用凍結 manifest：1.1 `e8bd1824f5ba12df80e2e1481dd697f11c24c6cdf5cdfb107befa0d9eba1ea32`；1.2 `72d1cf53fe158c84f5b9ed454497b7f7c1ae9d58cf2d754afa2a3dc60f59d60a`；1.3 `b45854a766efcc434069576aeed84238ddb64db3e12be684de2fd058161f994b`。
- 執行版本：`rustc 1.99.0 (b940084d7 2026-09-28)`、`cargo 1.99.0 (5f94df478 2026-08-27)`。
- 相稱執行紀錄保存於上述 `verification.json`；scratch 原始碼與產物同目錄。未重裝任何工具或變更 default toolchain。

| 驗證 | 觀察 | 支持範圍 |
| --- | --- | --- |
| 根目錄 `cargo check -p hello-world --locked` | exit 0 | 真實 hello-world member 可經現有 workspace 設定檢查。 |
| 根目錄 `cargo run -p hello-world --locked` | exit 0；stdout `Hello, Rust!` 加換行 | 文件中的 package 名稱、執行指令及輸出一致。 |
| 真實 main 與 two-lines fixture 各以 Rust 2024 編譯執行 | 兩者 exit 0；第二份依序輸出兩行中文，各一個換行 | 程式片段與練習的操作契約一致。 |
| 在 repo 外以已安裝的 1.99.0 執行 `cargo new hello-rust` | manifest 為 edition 2024；建立後尚無 Cargo.lock | 新建 package 路徑與文件所述起始狀態一致。 |
| 將 standalone manifest 與 main fixture 放入 scratch package，依序 check／build／run | check 後有 lock、無最後 binary；build 後有 binary；run 輸出 Hello | 三個 Cargo 指令的主要差異與本例實況吻合。 |
| 在 `examples/hello-world` 執行相同 `cargo run -p hello-world --locked` | exit 0；執行根目錄 target 中的 binary | 成員可找到上層 workspace 與共用建置目錄。 |
| 只移除單行 Hello 程式的 `println!` 後分號 | Rust 2024 編譯 exit 0，輸出仍相同 | 直接驗證下面 N1，不把語法疑點僅憑記憶列為問題。 |

repo 外第一次 `cargo new` 曾因 Cloud 的 `RUSTUP_HOME` 沒有設定 default toolchain 而失敗。這是本次檢查環境的狀態，與讀者按 1.1 預設 stable 安裝後的環境不同；重做時僅對驗證子 process 指定 `RUSTUP_TOOLCHAIN=1.99.0`，未把此狀態列為教材缺陷。

## 必要修正 N1：把示範格式誤說成不可省略的語法

- 位置：`002-002.md`，對應 1.2「先認識程式會用到的符號」，第一份程式下方。
- 原句：「行末的分號 `;` 結束這項工作；這一次不能照 Python 的寫法把括號或分號省掉。」
- 已教部分：在程式前已說明 main、函式外框、字串文字、println! 的 macro 使用及分號；示範程式本身正確，不缺這些概念。
- 技術契約：Rust block 可以以沒有分號的最末 expression 結束；本例末尾 `println!` 回傳 unit，符合 main 的回傳要求。因此移除這一個分號，仍是合法的相同輸出程式。[Rust Reference：Block expressions](https://doc.rust-lang.org/reference/expressions/block-expr.html) 的 final operand 規則支持此判斷，已另做最小編譯執行確認。
- 影響：讀者正在學「哪些符號是必要語法」。目前把教學選擇與語法必需合在一起；讀者嘗試小變化或後文學 block expression 時會遇到與此斷言相反的結果。
- 分級：**必要，技術錯誤／增加理解負擔**。不阻止目前範例執行，但原句直接宣告錯誤的規則，應修後再放行。
- 最小建議：保留所有例子中的分號，只把這句改成「這份示範使用括號，並以分號結束顯示工作；先沿用這個寫法。」或同樣只描述本例格式的文字。無須在 1.2 提前教 unit、尾端 expression 或分號的完整例外。
- 發現時點：獨立讀凍結正文後，尚未讀任何他人問題；未受既有問題提示。
- 修後複查：尚未做。

## 必要修正 N2：少量字形不符合專案正體中文約定

- 位置與原句：`002-001.md`「請换一個新名稱」；`002-004.md`「還没有執行它」；`003-005.md`「第一次還没有 lockfile」、「命令的起点一致」。
- 技術與專案契約：這些不影響程式行為，但 `AGENTS.md` 與已確認術語指引要求正體中文教材。
- 影響：違反文字格式約定；不是概念缺漏或程式錯誤，不誇大為讀者無法學習。
- 分級：**必要，專案用字修正**；可直接改為「換」「沒有」「起點」，不新增篇幅或先備。
- 發現時點：獨立閱讀凍結正文；未受其他紀錄提示。
- 修後複查：尚未做。

## 可選改善 I1：cargo check 的概括可再精確一點

- 位置：`002-004.md` 表格中的「檢查原始碼是否能通過編譯所需的檢查；不產生供你執行的最後可執行檔。」
- 已教部分：check 不執行、不產生最後 binary，build 產物與 run 的差別都清楚。本例實測完全一致。
- 官方契約：[Cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html) 的 DESCRIPTION 說明略過最後 code generation，部分診斷只會在 code generation 階段產生。因此 check 通過不是所有程式後續 build 都必定成功的完整保證。
- 影響：原句可以合理理解為一般性描述，不直接保證 build 成功；在最小 Hello 程式中沒有操作缺口。不應為了例外在本節加教 code generation 的內部階段。
- 分級：**可選，技術措辭精確度**。若改「先檢查原始碼中的編譯問題；不產生……」，能縮小概括而不增加負擔。保留現有主線也不妨礙本例理解。
- 發現時點：獨立正文閱讀與 Cargo 官方契約查核；未受其他紀錄提示。
- 修後複查：尚未做。

## 必要修正 N3：幾份 manifest 與新旗標先顯示，再解釋最低契約

- 位置一：`002-003.md` 在「本節的 manifest 如下」後先展示 version／edition／dependencies 等具體欄位，完整欄位角色在 code block 後。前面已教 TOML 區段與等號的表示、package 名稱與語言規則，但這不等於已教 version 的 0.1.0 與 dependencies 的作用。
- 位置二：`003-004.md` 在「根 manifest 的結構如下」後先展示 members／resolver／workspace.package／rust-version／publish 及 false，之後才逐項說明。成員 manifest 又先展示 `.workspace = true`，之後才說它代表逐欄採用而不是布林 edition。
- 位置三：`003-005.md` 先放 `cargo check -p hello-world --locked` 與 run，才解釋 `-p`／`--locked` 的操作。package 名稱、根目錄與 lockfile 的一般角色已教，但尚未把這兩個旗標與選取／拒絕更新的行為連起。
- 契約與問題類型：這些範例的 TOML／Cargo 技術契約正確，後面的解釋也足夠；缺的是**使用前的位置**。本次委派明示程式片段、指令、manifest 都遵守「範例只用當時已教內容」；因此不能用 code 後立即補足來當先備已成立。
- 影響：讀者第一次看見或可複製範例時，部分欄位／旗標仍靠後文才能理解。不是完全沒教，也不是 Cargo 範例不能跑；是這份教材最重要的排序約定未滿足。
- 分級：**必要，先備時序／增加理解負擔**。對目前 Rust source 本身沒有同樣問題，main 與 println! 都在 source 前已有足夠介紹。
- 最小建議：不增加完整 Cargo 章節；把現有欄位／旗標的角色文字挪到相應 block 前，先讓讀者知道接下來的材料做什麼，再看具體設定。根設定可先教各欄位、再顯示根 block，之後介紹繼承操作再顯示成員 block。`-p`／`--locked` 先用短句說用途與 lockfile 存在條件，再給兩行可執行命令。
- 發現時點：本輪獨立核對全部 commands／manifest 的首次使用位置後提出，尚未開放他人紀錄；不是沿用首讀者問題。
- 修後複查：尚未做。

## 其餘核心契約與使用時序的核對

| 閱讀位置／當下必要關係 | 凍結正文提供的依據 | 核對結論 |
| --- | --- | --- |
| 1.1：rustup／rustc／Cargo 的角色 | `001-001` 先解釋編譯；`001-002` 表格分開安裝版本、編譯、專案工作。 | 足以開始操作，沒有把 Cargo 當另一套語言。由 Cargo 呼叫 rustc 的角色敘述符合官方工具流程。 |
| 1.1：shell 與 Rust 原始碼的區別 | `001-003` 在指令前解釋終端機、無 `$` 提示符、cd 與 pipe 的動作。 | 可以理解指令輸入位置與 pipeline 在此做什麼。curl 的這組下載／執行寫法與官方安裝文件一致；不要求先教所有 shell 或 curl 選項。 |
| 1.1：linker 與作業系統建置工具 | `001-003` 交代其將已編譯部分連接成 executable 的用途，說明 macOS／Linux／Windows 的取得方法。 | 與 [Rust Book 安裝章](https://doc.rust-lang.org/book/ch01-01-installation.html) 一致；C／C++ 建置工具是安裝條件，沒有偷渡 C 語言教學先備。未實測 Windows／macOS 安裝，不能宣稱各平台實際驗收。 |
| 1.2：建立獨立 package | `002-001` 在 cargo new 前解釋 package、名稱與 cd，new 後辨認 Cargo.toml／src/main.rs。 | 實際生成符合。獨立起步不依賴尚未教的 workspace 繼承。 |
| 1.2：第一份 Rust code 的符號 | `002-002` 在 code 前教 fn／main／()／{}、固定字串、println!／macro、分號與格式字串限制。 | 除 N1 的過度斷言外，片段只用當時已教的材料。沒有變數、reference、String、Box 或其他後章 API。macro 完整實作可待 14 章，此處使用契約夠用。 |
| 1.2：standalone manifest | `002-003` 在 code 前教 TOML 的區段與欄位；之後解釋 name／version／edition／空 dependencies。 | edition 與 rustc 發布版本分清，欄位契約與 [Cargo manifest](https://doc.rust-lang.org/cargo/reference/manifest.html) 及實測相符；具體新欄位的說明時序另列 N3，不判使用前全數通過。 |
| 1.2：可觀察變化 | `002-005` 只增加第二項已教 println!，先要求預測順序。 | 相同 main 內依序輸出兩行，結果與目的相符。不把單次輸出提升為所有程式邏輯正確的證據。 |
| 1.3：package／crate／workspace | `003-002` 各自定義，區分 binary／library，明說前頁 hello-rust 與 repo hello-world 是不同資料夾。 | 與 [Rust Book Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) 及 Cargo workspace 契約一致；library 是預告，不在本節 code 使用。文字目錄足以辨認這份小檔案結構，不必要強加 SVG。 |
| 1.3：toolchain selection | `003-003` 說 rust-toolchain.toml 請求工具鏈，首次可能下載，不影響一般練習目錄。 | 依 stock rustup 安裝背景合理；[rustup overrides](https://rust-lang.github.io/rustup/overrides.html) 確認工具鏈檔案作用。更高優先的明示 override 屬未使用的進階情況，此處不必要列出所有例外。 |
| 1.3：根 manifest 與成員繼承 | `003-004` 先明示節錄模型與不可覆蓋真實 repo；分別教 members／resolver／workspace.package／rust-version／publish／TOML bool／逐欄採用。 | 模型與實際 hello-world manifest 契約相合；`edition.workspace=true` 不誤寫成 edition 為 bool。[Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) 支持繼承機制；部分最低解釋在 block 後，時序另列 N3。 |
| 1.3：-p 與 --locked | `003-005` 先示例再立即逐項說明，且 package／name 已由前段教；說 lockfile 紀錄及 locked 不等於 offline。 | 兩條指令可執行；工作目錄與名稱來源清楚；lock 版本已存在，不在首次 new 時要求 locked。[Cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html) 的 package selection／manifest options 支持敘述；旗標首次操作前的位置仍需 N3 修正。 |
| 1.3：成員工作目錄仍共用 target | `003-005` 清楚說回 repo 根目錄作後續起點。 | 在成員資料夾的實測與 [Cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) 的 parent 搜尋／共用產物契約吻合。 |
| 1.3：下一步 | `003-006` 預告 ch01-basics 與變數。 | 只建立下一節目的，沒有在本輪已使用變數或要求讀者知道新 package 的 implementation。未讀其正文，不核對後續七頁。 |

## 未驗證與結論邊界

本輪發現一項直接技術錯誤 N1、少量正體字形修正 N2、manifest／新旗標的先備時序 N3，以及一項可選措辭改善 I1。受審 Rust source 本身沒有未介紹的後章語法；manifest／commands 能執行且契約正確，最低介紹位置仍需調整。這不等於整本教學或首讀理解已通過。

未做全站建置、桌面／手機實際頁面與 SVG 視覺檢查；本輪三頁凍結資料中無 SVG。未實際安裝 Linux／macOS／Windows 工具、未用目標讀者做理解測試、未讀第一章後七頁。技術審閱的通過項不能代替其他角色的首次閱讀及銜接紀錄。後續跨角色覆核若有新增發現，另加註時點／提示來源，不回寫本次獨立初判。
