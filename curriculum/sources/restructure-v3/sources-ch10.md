# 第10章v3來源實讀 ledger

最新狀態：10.1–10.9皆完成所用EN與中文原文、必要實際前置/source實讀；以下保留prefix與逐節append的歷史，最後read manifests為全章實讀範圍。外部連結不代替本頁教學或答案。

最初prefix紀錄：

目前實作範圍10.1–10.2；實讀記錄不等於獨立教材驗收。EN與繁中Book/Cargo實際原文範圍、bytes hash及採用判斷如下。原始cache與編號文字在`/workspace/work/tutorial-implementation-20261015/ch10-preparation/`。

| 原文 | 實讀範圍 | 原始SHA256 | 採用／不採用 |
| --- | --- | --- | --- |
| [book-deps-tw](https://rust-lang.tw/book-tw/ch02-00-guessing-game-tutorial.html) | 394–552 | `0682bda1ec75458754d1e51a37d65c772f1cdd6198c17e9fdcee5b5fc80b6720` | 繁中Book實際dependency、下載、SemVer、lock解說；不用rand介面。 |
| [cargo-deps-zh-raw](https://raw.githubusercontent.com/sunface/rusty-book/main/public/docs/rust-course/cargo/guide/dependencies.md) | 1–66 | `2db81ba0b29743a0777b4f75a5c258d9a560b2d30c69de0c05493b6dce3cc825` | 中文Cargo依賴原文，對照依賴區段與來源。 |
| [cargo-features-zh-raw](https://raw.githubusercontent.com/sunface/rusty-book/main/public/docs/rust-course/cargo/reference/features/intro.md) | 1–160 | `0cc11f2d4472653dab8e397176d055d4ace819e9c19cdca7ae1fab044c8c1bc7` | 對照中文features說法；不採「定義便自動設定cfg」及舊nightly語法，以目前英文文件核對。 |
| [cargo-dependencies-en](https://doc.rust-lang.org/cargo/guide/dependencies.html) | 1–99 | `611d4460a9012fc31fc2c25d5280be8ec8042602ab35680194a6006093b030df` | 依賴宣告、registry與lock流程；不採用原例未教的regex API。 |
| [cargo-features-en](https://doc.rust-lang.org/cargo/reference/features.html) | 1–232 | `78c663fe82a9c62d4f7b7c9ebebfd8a736ee81b2e601f2b07fdf316aa68e0700` | 功能具名開關、預設功能與依賴features；不複製optional/no_std範例。 |
| [book-testing-en](https://doc.rust-lang.org/book/ch11-03-test-organization.html) | 1–220 | `d8009cafbf27c39e40dc1d66c5fbe605afd3d2d8c7d2827522e3a3cb937f481a` | unit與integration的crate邊界；不用super::*的額外名稱規則。 |
| [book-testing-tw](https://rust-lang.tw/book-tw/ch11-03-test-organization.html) | 1–170 | `4ff4726c53af68e4c98a3fc6e336a0304f413a0154937d61874c59b1e207684c` | 繁中測試組織原文，對照tests資料夾、公開邊界。 |
| [book-docs-en](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html) | 1–210 | `9d36b0ec0eeb0e672a832dd259b0d42321be611a0e56035822e7ab13e5ca3d67` | 文件註解、Markdown、Examples與文件測試；不擴充pub use/發布課程。 |
| [book-docs-tw](https://rust-lang.tw/book-tw/ch14-02-publishing-to-crates-io.html) | 1–195 | `8f9e07b77929023bcf09ad629d65276153cf58866355c6d20f843a2a42460894` | 繁中對應文件註解與可執行例子，採用相鄰說明方式。 |
| [rustdoc-tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) | 1–106 | `95572c7300af0f27eab4c7ed47125a509a9c5854fc80dc2d7fd162d894de3cb7` | 一般Rust fence的編譯執行與自動main外框；不用隱藏行或額外屬性。 |
| [cargo-add](https://doc.rust-lang.org/cargo/commands/cargo-add.html) | 1–160 | `00bf985a2312a78ec10854ae5384b6aa62515cdfc3834f0f5a62b5c651ded35d` | 已存在dependency更新、@版本、--dev、--features與--no-default-features。 |
| [cargo-update](https://doc.rust-lang.org/cargo/commands/cargo-update.html) | 1–82 | `125d952c66e8f97be3d0420c135e6e7e3f323035321fa9a2f5a249261d14b535` | 更新package spec而非-p選執行目標，可能連帶改相關依賴。 |
| [cargo-tree](https://doc.rust-lang.org/cargo/commands/cargo-tree.html) | 1–180 | `b1146e82138a7461ba6d11b622a3ad9eda6f498dbd185cbb063fece091ec535e` | 依賴樹、--depth 1、(*)和feature圖不保證等於完整編譯閉包。 |
| [cargo-version](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) | 1–180 | `52ca88c84270673046152d5e04631c6ae5feef7d6d4c9badb5d7d94de688e2f0` | 預設caret相容範圍及精確=要求，僅教major>0本例。 |
| [clap-features](https://docs.rs/crate/clap/4.6.7/features) | 1–181 | `6d9a0b6a1e4296e1dcc7eafe315bdc0222f942375bfdd89d1d7f906e5d6c3341` | 4.6.7預設功能與derive額外功能實際清單；10.1僅設定不呼叫Parser。 |
| [tempfile-docs](https://docs.rs/tempfile/3.27.0/tempfile/) | 1–92 | `970ae6902bfc7decbb70316cc62af8867373c1c00bfac60aca6dfb676bc9bb34` | 3.27.0暫存資料角色；10.1僅準備dev依賴，TempDir行為留10.8。 |
| [command-zh-raw](https://raw.githubusercontent.com/sunface/rusty-book/main/public/docs/rust-course/libraries/command/structopt.md) | 1–1 | `0cbea323295a9786f8a64e2f98f2a2321a4b39bd5d10ce7358a312d0d05e942f` | 只有structopt(todo)，非教材正文，不作教學依據。 |
| [json-zh-raw](https://raw.githubusercontent.com/sunface/rusty-book/main/public/docs/rust-course/libraries/json/serde.md) | 1–1 | `801a5260490c84da40e9b5e659f6029c4e8c8ca937c0ed5314b4bc7b07bf1feb` | 只有serde(todo)，非教材正文，不作教學依據。 |

read-manifest.json逐來源保存cache path、URL、取得時間、原始hash與range；local-read-manifest.json保存實際本機前置與舊來源全文hash。fetch-manifest只表示cache取得，不表示讀過。失敗403/404、中文TODO頁均未冒稱有效教材已讀；成功中文Cargo來源由GitHub raw取得，EN/TW文件/測試正文實際親讀。

舊7.1/7.2全文與tooling manifest/lib/main/public_orders實際讀過，保留其數字規則、公開使用角度與所有主要Cargo能力；root繼承設定改為intro最小manifest，原混合package拆為設定/文件/答案，所有原root source保留。crossref 10.1/10.2候選以實讀2.6/2.7驗證，沒有盲目全域換號。audit A0610-C01以行內表格解釋緊貼完整manifest處理；其餘10.6/10.9項仍待各節實作。

後續I/O/CLI cached頁面尚未在此prefix當已讀先備，寫各節前會補實際range/hash與必要新8/9正文source。只以root正式開放且作者親讀的實稿為前提。

## 10.3–10.4實讀 append

先親讀現行8.1–8.6全文與propagation實際完整六module/main，以及integer_parsing/as_ref/owned_text；9.1–9.4全文、9.4五個必要來源亦實際讀取。source/hash/range加入local-read-manifest.json。未以9.5–9.7未freeze內容當先備。

| 原文 | 實讀範圍 | 原始SHA256 | 採用／不採用 |
| --- | --- | --- | --- |
| [rbe-read-lines-cn](https://raw.githubusercontent.com/rust-lang-cn/rust-by-example-cn/master/src/std_misc/file/read_lines.md) | 1–42 | `eedc5fc8c0b2cc984c92cacc3b9a4944d24089eac8854c8a31e5e6d7ab161b0d` | 真正中文對應原文：檔案、buffer、逐筆工作角色；不採if let略過錯誤與BufReader未教API。 |
| [fs-read-to-string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html) | 1–67 | `47a0ddd4c3323145583064d8cb2fc60e1420d41e7ab05e3ea4aa2e21465c32b8` | 整份UTF-8、缺檔/格式錯誤、實際Result；不採例子的dyn Error回傳。 |
| [file](https://doc.rust-lang.org/std/fs/struct.File.html) | 1–108, 160–382 | `12a7ef0cbfff6ef095673182218c8957dce5061c6a84ca5ee3f4ad0d5508d455` | File資源/自動關閉、open唯讀與create截斷；未用nightly buffered、Seek或sync API。 |
| [read](https://doc.rust-lang.org/std/io/trait.Read.html) | 270–434, 675–800 | `6f70d786c833a9135c4a743f7f5b056a102f05906034d3cf7d5819d14e38032b` | read有效前段/短讀/0與空buffer/Interrupted，read_to_string追加與UTF8失敗不改String；不採unsafe或read_exact。 |
| [write](https://doc.rust-lang.org/std/io/trait.Write.html) | 132–460, 574–657 | `94483f38e85993952298cef6ca2d5bffc59cc4ad361889926d2ba03a53fda470` | write短寫、flush、write_all重試/完成、write_fmt與write! I/O結果；不採vectored/BufWriter新API。 |
| [io-error](https://doc.rust-lang.org/std/io/struct.Error.html) | 344–382, 648–717 | `3ae29be451cf4350810ceb113f360a5bb536741d4fb9e364827f552596879c44` | kind分類、Debug/Display標準能力；未採Error trait。 |
| [io-error-kind](https://doc.rust-lang.org/std/io/enum.ErrorKind.html) | 74–106, 174–234, 384–421, 596–631 | `5216374be61c24fd9928ef3413ff6f1360e305c63db067b41db623decfb8cf4c` | 非窮舉match需_、NotFound/InvalidData/WriteZero、Debug/PartialEq；不讀完整I/O Result相等。 |
| [rbe-read-lines-en](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html) | 1–95 | `26992b3194be8cebbe456518ea4200e8c54682928d1f033fa5ac90a70e76127b` | 英文實際逐行/緩衝角色；不把BufReader、map_while、collect或忽略錯誤搬進10.4。 |
| [rbe-read-lines-zh-raw](https://raw.githubusercontent.com/rust-lang-cn/rust-by-example/master/src/std_misc/file/read_lines.md) | 1–87 | `985b5b22ff1dce7d811f5d900df0dff75ca8eea64cb8d7c7b74cb4f600251ce1` | 取得的是英文內容，雖repo名稱像中文仍不當中文教材；後改讀實際cn鏡像。 |
| [string-api](https://doc.rust-lang.org/std/string/struct.String.html) | 607–718 | `a3cd9eaa72f1283dda4da4b23ed5dfd9ba53f6a43fc450a38b96a08d3c54dda6` | from_utf8接手Vec與檢查完整UTF8，不再逐chunk猜文字。 |
| [book-read-en](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html) | 1–94 | `18c2d182ee5fff359f8a57fa23004e6a3b0ad81f861f91a8d81a08829573956d` | 親讀Book檔案章全文：固定材料→read_to_string→可見內容；不採collect/索引args/expect外部檔案或後章Box dyn Error。 |
| [book-read-tw](https://rust-lang.tw/book-tw/ch12-02-reading-a-file.html) | 1–74 | `7fecae25ca7147cb6a8d93b6bfd6f83dc50422c721febd971b6c90eb2b919368` | 親讀繁中同章全文，保留先材料與工作目錄，再看完整文字；用match保留失敗。 |
| [rbe-read-lines-zh](https://raw.githubusercontent.com/rust-lang-cn/rust-by-example/zh/src/std_misc/file/read_lines.md) | 1–87 | `985b5b22ff1dce7d811f5d900df0dff75ca8eea64cb8d7c7b74cb4f600251ce1` | zh branch這個檔案也仍是英文；不冒稱已讀中文。 |

兩個看似中文repo/branch的read_lines原文仍為英文，已照實記錄；真正中文本文由rust-by-example-cn取得並全文讀42行。英文/繁中Book Reading a File全文先讀，再以std實際方法契約核對I/O新增內容。未抄Book範例的Box<dyn Error>/外部檔案expect。10.3/10.4 crossref逐筆比對實際前置後遷移，沒有改未凍結章節。

## 10.5實讀 append

| 原文 | 實讀範圍 | 原始SHA256 | 採用／不採用 |
| --- | --- | --- | --- |
| [rbe-path-cn](https://raw.githubusercontent.com/rust-lang-cn/rust-by-example-cn/master/src/std_misc/path.md) | 1–40 | `2faec0bf58f51219a5f809d5f103b0373d88df2e8d9570e45b6297fd8c8fd346` | 中文實際舊原文；採native path角色，不採過時posix::Path/windows::Path或Vec<u8>內部表示斷言，與目前std核對。 |
| [book-args-en](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html) | 1–204 | `bf0134e0942f543859488910326828b50090d2095fb77d8bb41ee15f740c9e9e` | 親讀EN CLI章全文；保留--、程式名稱與args Unicode風險，不採未教collect/索引/dbg。 |
| [book-args-tw](https://rust-lang.tw/book-tw/ch12-01-accepting-command-line-arguments.html) | 1–150 | `978c4e0cb4f5b80d322c83311b1ae2bf6fcd8f7ade23c5e24e38de9ecd930383` | 親讀繁中對應章全文；先教原生參數與Iterator，再接CLI。 |
| [rbe-path-en](https://doc.rust-lang.org/rust-by-example/std_misc/path.html) | 1–94 | `cf2d16ba40169a5c6ed53e6edd14c3be51603f38a477fd4c750e9d2996a21827` | Path/PathBuf/OsString關係與join/display/to_str；不採Deref或panic格式政策。 |
| [path-api](https://doc.rust-lang.org/std/path/struct.Path.html) | 1–240, 1224–1312, 1648–1717, 2916–2953 | `2f7ea21033e5a46a5ab4a490cfdef45232ad6ca0ae2eff5dd04d5e85ceb8b3aa` | new/as_os_str/to_str/join/display與PartialEq；不採nightly Path::as_path、Cow/Box。 |
| [pathbuf-api](https://doc.rust-lang.org/std/path/struct.PathBuf.html) | 1–462, 3130–3197, 3314–3495, 3860–3898 | `119a36d0dfa562a67e09c3657515824d9b313b12c94b65942b396386589024ac` | owned路徑/as_path/push/From<OsString>/Debug/PartialEq；Windows join/push例外校正，未教Deref/Box/Cow。 |
| [osstring-api](https://doc.rust-lang.org/std/ffi/struct.OsString.html) | 1–190, 220–320, 1480–1548, 1728–1826, 2412–2452 | `ca8d17a7937e2b930974288e57876ad08e40503d387fb873071089eb76db5434` | owned OS字串/as_os_str/into_string失敗交回原owner/Clone/From/Debug/PartialEq。 |
| [osstr-api](https://doc.rust-lang.org/std/ffi/struct.OsStr.html) | 112–165 | `d33aaa6498c28efd4404e7276b2593595ce840868f22f80c056c2088ac49be81` | to_str可能失敗、只借UTF8視圖；不採unsafe/map/collect。 |
| [args-os](https://doc.rust-lang.org/std/env/fn.args_os.html) | 1–64 | `f7bb2094c260f1c556acf430125fde0b5aeac213f7c29c9149e53839e59d258d` | 原生arguments、不檢查Unicode、第一筆可任意命名，不把它當真實執行檔位置。 |
| [current-dir](https://doc.rust-lang.org/std/env/fn.current_dir.html) | 1–54 | `635b8c9fa7dd0403de98000a7229a4aef8bd9bd4153167027bf11bb42d01affb` | 工作目錄PathBuf與可能I/O失敗，不以source位置作起點。 |
| [exit-code](https://doc.rust-lang.org/std/process/struct.ExitCode.html) | 1–170, 300–332 | `bd95c8be015fbf7ddd9338cfb0290339970ae6ac5e7540314e42c8934ef485fd` | main正常回傳、SUCCESS/FAILURE、From<u8>與platform界線；不採Termination trait或nightlyexit。 |
| [stdout-api](https://doc.rust-lang.org/std/io/fn.stdout.html) | 1–106 | `637a1acdbefb28b7787cdf6bf3de1601038330f1b62cefbe19f2fad973f41ac1` | Stdout行緩衝與flush、保留檔案尾端無換行的輸出政策；不採lock/mutex內容。 |
| [stderr-api](https://doc.rust-lang.org/std/io/fn.stderr.html) | 1–66 | `e6edc8536b3ac1aed51f50450e00dbfccf69a72de1b696e2e77e459f24e78eaf` | 診斷出口沒有預設緩衝，Write失敗仍需處理。 |

中文RBE Path版本較舊，正文已讀但技術現況以目前std核對；不抄其posix::Path/windows::Path分型、Path內部必為Vec<u8>等舊說法。Book EN/TW的args panic風險實際讀過，選args_os與已教next，沒有抄collect/索引。crossref10.5逐筆確認後遷移為9.4、10.3/10.4、8.1/8.3/8.5、6.2，沒有全域換號。

## 10.6實讀 append

| 原文 | 實讀範圍 | 原始SHA256 | 採用／不採用 |
| --- | --- | --- | --- |
| [clap-parser](https://docs.rs/clap/4.6.7/clap/trait.Parser.html) | 1–232, 312–371, 486–572 | `4fffe81df36db44b6f59a04e7a62421642654cfa14cd64fa1d693a7d94b90c3e` | Parser與try_parse_from的IntoIterator/Into<OsString>+Clone契約、derive feature；不抄alias/repl helper或dyn。 |
| [clap-derive-tutorial](https://docs.rs/clap/4.6.7/clap/_derive/_tutorial/index.html) | 1–390, 502–640 | `c5275f9a24ac0241c87d4b1fb2f8ac162090353889ebca89046bcd41e15a9bfc` | derive/command/name/version/about、requiredpositionals、bool long flags；不採subcommands/Option/default/Count extras。 |
| [clap-error](https://docs.rs/clap/4.6.7/clap/error/struct.Error.html) | 540–623, 1367–1429 | `fc24f0558c4dcf42ce83b85e979cdae305766594ecbdf1f1008999a6f7b661ad` | use_stderr/exit_code/print結果與不直接exit的控制邊界。 |
| [cookbook-cli-cn-body](https://raw.githubusercontent.com/rust-lang-cn/rust-cookbook-cn/master/src/cli/arguments/clap-basic.md) | 1–83 | `a99e356877e35e56fbf30222a620710244573e23cce87d609723b89a26857bd9` | 中文實際clap正文，版本舊builder API；採規則與help用途，不抄App/with_name/value_of。 |
| [cookbook-cli-en](https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html) | 1–284 | `3049cff89afb8541f7237c01cdd7f2ef6d76aaa8d02d770c72df58e92dae94a3` | EN實際參數章全文；採CLI角色/native path，無copy builder/collect/silent skip。 |

中文Cookbook主頁只是include目錄，實際追回arguments/clap-basic.md全文83行；沒有用頁名或TODO代替實讀。中文builder較舊，正文按鎖定clap4.6.7實際API；文末明記版本差異。crossref10.6逐筆實讀後移至10.5/10.1/6.2/10.9/10.8，A0610-O01已採最小詞彙修正。

## 10.7實讀 append

| 原文 | 實讀範圍 | 原始SHA256 | 採用／不採用 |
| --- | --- | --- | --- |
| [serde-derive-raw](https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/_src/derive.md) | 1–99 | `e7e5298024ed0125994533759c16bc78068832185e38664d460fffb7e83ef975` | 官方derive原文raw；serde.rs403後改官方repo，導入derive feature/trait與欄位契約。 |
| [serde-container-raw](https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/_src/container-attrs.md) | 1–90 | `b036415181c66b67a611db53da6865ba137eb1a0a9b3c297f6ff4806896ee71d` | deny_unknown_fields在JSON預設忽略的對照；不採flatten/tag/bound extras。 |
| [serde-lifetimes-raw](https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/_src/lifetimes.md) | 1–170 | `7271dcbdea33d057ddbe37acadc91379cb82e0613773143776ffcdd77d2e243a` | Deserialize輸入lifetime與owned String的區分；不教HRTB/DeserializeOwned/Visitor或zero-copy宣傳。 |
| [cookbook-json-cn-body](https://raw.githubusercontent.com/rust-lang-cn/rust-cookbook-cn/master/src/encoding/complex/json.md) | 1–51 | `2f36d401b9de9273bf8dced0c1e4946f191f588f5c6059af0c76e9a8037a57f2` | 中文實際JSON正文；採from_str與文字格式，不把Value/json!當此節先備。 |
| [json-from-str](https://docs.rs/serde_json/1.0.154/serde_json/fn.from_str.html) | 1–64 | `e04b637e296ed626b7fdc9c3efefd6334dc9aa1e9a01f97d2c423b4c9ab3d620` | from_str與Deserialize輸入lifetime、missing/shape/number overflow失敗條件。 |
| [json-to-string](https://docs.rs/serde_json/1.0.154/serde_json/fn.to_string_pretty.html) | 1–36 | `c528851b2af47bb500280137cb5c1d4087d5344a4915fa5218daabf5c60632a5` | 借讀Serialize、pretty String、Result保留可能的encode失敗。 |
| [cookbook-json-en](https://rust-lang-nursery.github.io/rust-cookbook/encoding/complex.html) | 1–130 | `126da23732c87302dee68bbaa778c9dd997438577f8a9cbf5884ceeea2a83f5f` | EN實際structured data全文；採JSON/Serde角色，不抄Value/json!/raw literal/TOML/endian新課程。 |

Cookbook中文主頁只是include目錄，實際追回complex/json.md全文51行。Serde403改官方repo source成功全文/range實讀；沒有把fetch成功當read。7.3/7.4實稿模型來源與receiver閱讀補足Deserialize lifetime語句的實際前置；未使用HRTB/DeserializeOwned。10.7 crossref逐筆實讀後採10.3/10.1/6.2/7.3/8.5/2.6/10.9/8.3/8.6。

## 10.8實讀 append

| 原文 | 實讀範圍 | 原始SHA256 | 採用／不採用 |
| --- | --- | --- | --- |
| [cookbook-command-cn-body](https://raw.githubusercontent.com/rust-lang-cn/rust-cookbook-cn/master/src/os/external/process-output.md) | 1–62 | `c09c85257e9bc1b5e9daf0faf685e985aea82178b3d10d7caf6646e1333688b4` | 中文真正process-output全文；採啟動/輸出/狀態，不抄regex/error_chain/隱藏行或外部Git。 |
| [cookbook-command-en](https://rust-lang-nursery.github.io/rust-cookbook/os/external.html) | 1–195 | `8f0c2226112e44a7f579aca4e251895ca7fedebd93d3dd48cafc268e042d8b59` | EN外部命令全文；採Command/Output/status分工，不抄anyhow/Git/shellpipes/服務與未選定helpers。 |
| [tempdir-api](https://docs.rs/tempfile/3.27.0/tempfile/fn.tempdir.html) | 1–75 | `1cc14b1ba68123daecece09b72fa6e9dc18519c93048120fada1ce7775b86b41` | tempdir建立與io::Result；只操作自有目錄。 |
| [tempdir-type](https://docs.rs/tempfile/3.27.0/tempfile/struct.TempDir.html) | 1–107, 408–455, 511–567 | `6116d4b3485566cb4410bb9c67c30b3ea9d4db192f8491689b6912fba0249a0c` | TempDir資源owner、path借用、close接手並回報清理失敗；不採keep/Drop實作教學。 |
| [command-api](https://doc.rust-lang.org/std/process/struct.Command.html) | 1–167, 241–305, 619–706, 804–869 | `63378e729b385f54665eb85a3272d5ee9dcf62bccee1d34f1084019c68825b12` | new/arg/current_dir/output，arg單筆不經shell，output等待與capture，Ok不等child成功。 |
| [output-api](https://doc.rust-lang.org/std/process/struct.Output.html) | 1–83 | `ab80af6bbc1800560eabe2d34a834c69a8ece631ec333b96dae72c9d78800838` | Output公開status/stdout/stderr三欄；不用nightlyexit_ok。 |
| [exitstatus-api](https://doc.rust-lang.org/std/process/struct.ExitStatus.html) | 100–190 | `7efb9ddf71a271d89a42ef86f4a8d165eca96c0af5de0f1721b9f32be1c08ef6` | success零、code Option與Unixsignal None；不用ExitStatusExt。 |
| [fs-write-api](https://doc.rust-lang.org/std/fs/fn.write.html) | 1–56 | `4f94669ff2e61c159d609d6008588c4d19bd1b9601621e8bddf9e14c2d91c301` | 完整取代檔案內容、&str/bytes與父目錄限制。 |
| [str-from-utf8](https://doc.rust-lang.org/std/str/fn.from_utf8.html) | 1–95 | `5f7463d10073df9388d96cbb23971bac3c5f399ac8ff3fc9ee11650131c15106` | 只借byte slice驗證UTF8→&str，與接手Vec的String::from_utf8區別。 |
| [macro-env](https://doc.rust-lang.org/std/macro.env.html) | 1–58 | `88db263ab7e672e9c97193b3ebf7748ea780e6d6b5050973d437c2a32b35b291` | 編譯時env!與缺值錯誤，不教執行時env::var/option_env。 |
| [cargo-targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html) | 188–239 | `9e4b8d6a445b31ff776ca945fc22c8a7cf15c33f8d4367a024ef070ca80841c4` | integration獨立crate、Cargo build binary與CARGO_BIN_EXE名稱。 |

Path既有原文另實讀250–289、1984–2052，補to_path_buf/try_exists；記錄已append到read-manifest.json。中文Cookbook主頁是include，實際追回external/process-output.md全文62行，不抄error_chain/regex/Git/隱藏API。10.8 crossref逐筆核對10.5/10.1/10.2/10.4與11.3後續預告，沒有以11作前提。

## 10.9實讀 append

| 原文 | 實讀範圍 | 原始SHA256 | 採用／不採用 |
| --- | --- | --- | --- |
| [book-modularity-en](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html) | 1–1195 | `93072555595e22f3dabae36a7409f5b4f85047ebd03db7e5be1f4616de1477c4` | EN完整正文；採設定/純工作/I-O/main分工與可測性；不採提前collect/index、unwrap_or_else、unimplemented!、Box<dyn Error>、process::exit。 |
| [book-modularity-tw](https://rust-lang.tw/book-tw/ch12-03-improving-error-handling-and-modularity.html) | 1–994 | `3882bb67fb1eb0a16d939550ba7261721e405fbf61226266c2c623475fdcfd66` | 繁中完整對應正文；採工作分層，完整CLI另有真child測試，main短小本身不當正確證據。 |
| [book-case-en](https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html) | 1–552 | `e00298901d6b918303bed4bb340a0c287f7d20e09109902857a62304e6f00d7e` | EN完整正文；採query和候選行都轉比較表示、保存原輸出；不新增環境變數課程，明定ASCII而非Unicode case folding。 |
| [book-case-tw](https://rust-lang.tw/book-tw/ch12-05-working-with-environment-variables.html) | 1–615 | `3db9950592a9d6919f8b6122be472583837b0a14b6fc4fe7d39978e1d935d870` | 繁中完整對應正文；採相同材料下大小寫選擇比較；不採Box/dyn或由to_lowercase泛稱完整Unicode忽略大小寫。 |
| [str-api](https://doc.rust-lang.org/std/primitive.str.html) | 2098–2197, 6396–6469, 6828–6883 | `912596d9c052c16d834aa595bd2b1a35b853de9b733fc95e5e3d5a686447ad06` | 實讀contains與目前inherent to_ascii_lowercase，另比對舊AsciiExt已deprecated段而不採。只教具體&str連續片段及ASCII A-Z轉換，不展開Pattern。 |
| [iterator-api](https://doc.rust-lang.org/std/iter/trait.Iterator.html) | 2362–2439, 3292–3382 | `2261960b32a5851593dfdfd85ba5b8ce546e39e0112988865e2d35cd764723a4` | 先讀zip段的enumerate例，再追實際enumerate條目：接手iterator、產生(usize, Item)、計數從0。只用lines，不增加zip課程。 |
| [String::push](https://doc.rust-lang.org/std/string/struct.String.html) | 2022–2079（新增實讀；原607–718保留） | `a3cd9eaa72f1283dda4da4b23ed5dfd9ba53f6a43fc450a38b96a08d3c54dda6` | mutable String追加單char的UTF8 bytes；不用未教Extend。 |

實際9.5–9.7全文與必要adapter/text source、舊07-09全文與search package全檔的read ranges/hash記入local-read-manifest.json。本節只用9.5 iter/fold/Vec比較與9.6 lines/已教順序，沒有將尚未讀的sorting source當先備；不引用第11章作第10章前提。

所有原文cache與fetch/read manifests位於work/tutorial-implementation-20261015/ch10-preparation。HTML行號指抽出main後的非空text行；raw Markdown行號保留空行，hash指實際cache原始bytes。外部403與中文舊API淘汰紀錄保留，沒有把只抓到標題/TODO/include當已讀正文。

## 全章來源最終交接

10.1–10.9所用原文range/hash/採用界線均在本ledger逐節保存，完整結構化資料為ch10-preparation/read-manifest.json及local-read-manifest.json；chapter-final-freeze.json記其最終hash。保留實際未成功抓取的fetch結果、只有TODO/include的來源與拒絕過時API的判斷，不將它們算成有效正文已讀。實際所用本機前置除1–7外，8.5/8.6與9.5–9.7均在root正式gate後親讀。沒有將第11章視作第10章先備，也沒有新增外書的環境變數/動態錯誤物件/發布流程等課程範圍。
