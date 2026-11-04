# 第 9.2–9.4 節作者追加來源與先備

作者：`chapter02_ownership`。日期：2026-10-14。這是重新分工後自己的實際閱讀，保留原 `source-notes/ch09.md`，不把其中另一位作者讀過的 URL 算成本人的閱讀。使用 AGENTS、clear-tutorial／writing-prompt／Rust 術語指引。

## 9.2 動筆前實讀

| 實際來源／範圍 | 可借鑑的說明 | 本課序取捨 |
| --- | --- | --- |
| [Cargo Book：Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)，lifecycle／inputs／outputs、rustc-env／error、build-dependencies／change detection | 先分開編譯 script、執行 script、編譯 package 三步；stdout 是協定，且重新編譯與重新執行是不同動作 | 不搬開頭 cc／C／linker 程式。先做模式與 target 資訊傳入 Rust crate；每個 cargo:: 指令先解釋。只用固定 Rust 1.99 支援的雙冒號形式與 cargo::error |
| [Rust 語言聖經：build script 原始章](https://raw.githubusercontent.com/sunface/rust-course/main/src/cargo/reference/build-script/intro.md)，完整原始章 | 中文依生命週期、輸入、輸出、依賴分段，適合前接讀者既有的 Cargo 經驗 | 其單冒號 cargo: 是舊相容形式，本課用現行 cargo::。其「任何檔案改變觸發重新編譯」混合 rebuild/rerun，本文依官方契約拆開。不搬 C、links metadata 或完整交叉連結設定 |
| [Cargo Environment Variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html)，build script／crate compilation 區段 | HOST／TARGET／CARGO_CFG_* 與 OUT_DIR 的來源、提供階段分開；build script 執行時才有的變數不能用其編譯階段 env! 讀 | script 的 cfg! 看 host；CARGO_CFG_TARGET_OS 看要編譯的 package target。OUT_DIR 只先教輸出目錄及可保留舊檔的契約，本節沒有生成檔案，不插 include!／concat! |
| [std::env::var](https://doc.rust-lang.org/std/env/fn.var.html)／[VarError](https://doc.rust-lang.org/std/env/enum.VarError.html)／[env!](https://doc.rust-lang.org/std/macro.env.html)，完整必要契約與 enum variants | var 是執行時 Result<String,VarError>；NotPresent 與 NotUnicode 不能都默默當預設；env! 是編譯時 &'static str／缺值或非 Unicode 是編譯失敗 | 模式只接受 demo／practice，避免任意外部文字塞進逐行 Cargo 協定。Cargo 提供的 target OS 用已教 expect 說明保證來源；使用者配置失敗則 cargo::error，停止建置 |

原始網頁下載／可重讀材料存 `work/ch09-author02-research/`；實際已讀資料與關係以本筆記為準，不以檔名或 URL 出現代替閱讀。

## 實際先備

- 已實讀 `ch09-tooling-v1` 全六 units：workspace 繼承、profile、features、cfg attribute 與 cfg!、host／target、integration test。未修改 9.1 package 或正文。
- 已實讀 `ch07-path-cli-prerequisite-v4` 路徑／OS 字串、工作目錄、args_os／CLI／stdout／stderr／ExitCode 內容；9.2 選用 env 仍當節教 var，不假設名稱同 module 就已教。
- 已實讀 `ch08-complete-prerequisite-v8/004-003` 的 &'static str／literal 有效性及相鄰 type bound 說明，再在 9.2 使用 env! 的實際回傳型別。
- 第 5 章是本人原作者；本輪另讀 `ch05-review-fixed-v7` 相應轉換／borrow／錯誤傳遞實稿。沒有把第 5 章標題或後文能力當可用證據。
- 已讀 7.4 起始 buffer／short-read 契約；9.3／9.4 寫前仍須讀其完整必要 source 與第 6.7 實稿。

9.3／9.4 的中英文遍歷、hash 與正式第三方 API 尚待本作者重新線上閱讀，未據原調查宣稱完成。9.4 等自己 9.3 交 root 先備核對後開閘。

9.2 追加實稿先備：root 開放 `ch07-fixtures-prerequisite-v8` 後，已讀全七 units；7.8 的 `env!`／compile-time environment 已實際教過，9.2 回收此概念，再明確使用 8.4 的 `&'static str`。目前沒有用未教的 `Command::env`。1.6 實際條件式 unit 已教 `&&`／`||`／`!`，模式檢查使用已教 `&&`。

## 9.3 動筆前實讀

- 英文 [Rust Cookbook：Directory Traversal](https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html)，path／nested directory／duplicate names／traversal／size 各例：借鏡從檔案工作引入 walker；其多份範例 `filter_map(Result::ok)` 會略過失敗，不符合本課完整／不完整分開的需要。未教的 path inspection、same_file／glob、lossy Cow 不搬入本文。
- 中文 [Rusty Book：目錄存取](https://rusty.course.rs/files/dir.html)，Metadata／同名檔／follow_links／隱藏項目／depth 大小各例：文字先說工作再選 crate 值得採用；其 `error_chain!`、遞迴 same_file、吞 error 以及 `!is_dir()` 當作一般檔案都不採用。`!is_dir()` 尚含 link／特殊項目，應明確要求 `is_file()`。
- 正式 [walkdir 2.5 WalkDir](https://docs.rs/walkdir/2.5.0/walkdir/struct.WalkDir.html)、[DirEntry](https://docs.rs/walkdir/2.5.0/walkdir/struct.DirEntry.html)、[Error](https://docs.rs/walkdir/2.5.0/walkdir/struct.Error.html)：實讀 traversal／new／follow_links／follow_root_links／path／into_path／file_type／metadata／Error.path。root 預設會追 link；本課先以 symlink_metadata 拒絕，再明寫兩個 false。Item 是 Result，Error.path 為 Option，不能假造永遠有 path。
- 標準 [symlink_metadata](https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html)、[Metadata](https://doc.rust-lang.org/std/fs/struct.Metadata.html)、[FileType](https://doc.rust-lang.org/std/fs/struct.FileType.html)：實讀 link 不追蹤、len 的 u64 bytes、modified 的 io::Result<SystemTime>、is_file/is_dir/is_symlink。查詢失敗與後續內容讀取失敗分開。
- 實稿先備：已讀 actual6.5 全文、6.6 起始 map/filter/collect、6.7 全文（map entry、Ord/Ordering 與 slice sorting、名稱／身分規則）；actual7.4 全文含 reader/writer、short/error fixtures/tests/答案。沒有以單純名詞列表替代 body。
- 9.3 追加選用正式 [SystemTime](https://doc.rust-lang.org/std/time/struct.SystemTime.html) 的 Debug/Clone/Copy/PartialEq、[PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html) 的 Eq/Hash/Ord/Borrow<Path>，並核 [create_dir](https://doc.rust-lang.org/std/fs/fn.create_dir.html)、Unix [symlink](https://doc.rust-lang.org/std/os/unix/fs/fn.symlink.html)／[OsStringExt::from_vec](https://doc.rust-lang.org/std/os/unix/ffi/trait.OsStringExt.html) fixture。這些新契約均放在實際 source／tests 之前；沒有把 fixture API 當免教的維護知識。

## 9.4 動筆前來源研究（正文仍等 9.3 gate）

- 英文 [Rust Cookbook：Hashing](https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html) 的檔案 SHA-256 範例，以及中文 [Rusty Book：Hash](https://rusty.course.rs/algos/cryptography/hashing.html) 的相應範例實讀：逐次 read 只送有效 buffer 前段值得回收；不搬 ring、data_encoding、error_chain!、HMAC 或密碼儲存工作。它們示範 digest 計算，沒有替本課完成真 bytes 等價群組的證明。
- 已下載正式 BLAKE3 1.8.7 Hasher／Hash 文件；已讀 new/update/finalize 及 Hash 的 Display/Eq/Hash/Clone/Copy 契約。update 按順序處理連接 bytes；finalize 不消耗 hasher。尚需對當節選用 File metadata／SystemTime 與 checked arithmetic API 進一步查核。

追加查核已完成：[File::metadata](https://doc.rust-lang.org/std/fs/struct.File.html#method.metadata) 描述打開 handle 所指檔案，不同於重新按路徑查詢。後續元件將觀察路徑與 handle，讀取前後都比較 len/modified，不以共有 reference 或打開 File 當作內容固定證明。[u64::checked_add／checked_mul](https://doc.rust-lang.org/std/primitive.u64.html) 按值接收數字，成功 Some(result)、溢位 None；不是 wrapping arithmetic。`u64::MAX` 為最大可表示值，測試可控制溢位，不虛構巨大檔案。

[Option::map／is_none](https://doc.rust-lang.org/std/option/enum.Option.html) 另已讀必要官方段；不是 Iterator adaptor，map 接手 Option，Some 才執行 FnOnce；None 保留。[SystemTime](https://doc.rust-lang.org/std/time/struct.SystemTime.html) 的 PartialEq/Copy 實作已查。筆記中的查核不提前替代9.4正文首次需要的解釋。

root9.3實際gate釋出後，已讀凍結v3七units，才開始9.4；其typed ScanCause／RootError、Snapshot與原生identity契約維持。9.4獨立package不改9.3 source。首次normal/tests遇到每個selected API前依正文表格/段落核對，實際18項及4example/main驗證見author-ledger。

修後另核正式 [TryFromIntError](https://doc.rust-lang.org/std/num/struct.TryFromIntError.html) 的checked integral conversion失敗角色與Debug；此型別不是actual5.4自訂Count轉換的已教API，已放回9.4 metrics source前首次介紹。[hard_link](https://doc.rust-lang.org/std/fs/fn.hard_link.html) 的original/newpath與同檔案系統fixture契約亦核。本次追加查核在初稿source precheck後，沒有回寫成動筆前已核。原先錯把arrayCopy當可用能力的test已改雙份同值初始化，不新增trait教學。

工具讀取 Rusty 站點的直接 HTTP 曾得 403，後由 web connector 實際 open／click 對應本文 body，不把失敗 URL 算完成。中文 hash 正確路徑是 algos/cryptography/hashing.html；先前 encryption/hashing.html 不可用。
