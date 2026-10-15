# 14.4 讓library的公開契約能驗證，也能持續維護

7.9的搜尋工具已把設定、純搜尋、報告與I/O分開。現在要把搜尋工作交給其他程式使用：caller選擇大小寫與最多幾筆結果，再取得原行文字。library改內部檔案或增加設定時，希望原本合法的caller能繼續使用，也保持原來的搜尋意義。

**API（application programming interface，應用程式介面）** 對這份library而言，包含caller可使用的公有名稱與型別，以及輸入、結果、失敗和ownership的約定。加上pub只能讓名稱可見，不會自動說清「成功」代表什麼。

本節使用 `ch14-api` package，library名稱是 `ch14_api`，命令從repo根目錄執行。工作仍是7.9的非空連續片段搜尋；新需要是有效的結果上限與可維護的公有介面。

## 先決定caller可以依賴什麼

| 這份API的約定 | caller取得什麼 |
| --- | --- |
| BatchLimit只接受1到1000 | 不能把零當成有效上限；過大值也明確拒絕。 |
| Options預設區分大小寫、上限100 | 預設行為有固定意義，不能只說「用預設就好」。 |
| search借用query、全文與Options | 不接手caller的設定或輸入，回傳的 `Vec<Hit>` 擁有文字。 |
| 成功保存最前面的符合行，最多上限筆 | 原行號從1開始，空行仍占行號，原大小寫與順序保留。零筆是正常成功。 |
| 忽略大小寫只折疊ASCII | 沿7.9，Ä與ä仍不同，不承諾所有語言的大小寫比較。 |
| 空query是EmptyQuery | read_matches先拒絕它，再決定是否讀檔。 |
| 讀檔失敗是Read | 保存原io::Error；caller能分類與借讀原因，不必解析診斷文字。 |

這些約定來自目前的工作需求；後面的型別與測試要支持它們。private欄位、Default與feature都有各自的作用，不能把其中一項當成「整份API一定相容」的證明。

## 私有newtype先守住不變條件

8.9已用newtype區分路程單位。這次BatchLimit包住usize，另外要求 **invariant（不變條件）**：library提供的每個有效上限都在1到1000。建構、複製與預設入口都要保留這項條件，不能只驗證某一個new呼叫。

失敗先用LimitError保存Zero或TooLarge原值，Display與Error沿8.6。這裡在**enum整體**加新attribute **`#[non_exhaustive]`**，表示將來可能增加variant。定義它的crate內仍可窮舉目前variant；外部crate分類時必須另留一個接住其餘值的分支。已知variant仍可在外部建構，這項enum標記不會把它們變成私有。

```rust
--8<-- "examples/ch14-api/src/limits.rs:limit-error"
```

BatchLimit的tuple欄位沒有pub，caller不能直接寫 `BatchLimit(0)` 或改內部數字。new檢查輸入，成功才建立Self，get shared借用並交回usize。derive明選4.2的Debug／Clone／Copy／PartialEq；複製有效數字仍有效，這些公開能力也是caller可依賴的介面。

source中的 `///` 是7.2已教的文件註解；Examples可由Cargo作文件測試。這次範例用已教expect，因為固定2已在有效域：

```rust
--8<-- "examples/ch14-api/src/limits.rs:limit"
```

這不是保證crate內任何作者都永遠寫不出錯誤，而是把對外建構與修改限制在可檢查的入口。新增一條產生BatchLimit的路徑時，也要核對同一個invariant。

### Default也要產生有效值

**`std::default::Default`** 是提供預設值的標準trait。它要求Sized，並提供簽名 **`fn default() -> Self`**：沒有receiver或輸入，直接建立一個Self，沒有Result失敗分支。Default由標準prelude帶進名稱範圍，因此能寫 `impl Default` 與 `BatchLimit::default()`。

**`#[derive(Default)]`** 在每個欄位型別都有Default時，使用各欄位的預設值組成struct；它不會呼叫我們的驗證new。usize的預設是0，bool的預設是false。對BatchLimit直接derive，就得到不符合非零規則的0。

下面是可編譯、執行時故意失敗的fixture。它重新定義這個錯誤設計，不改正常library。沿1.10的條件檢查，`assert!(條件, "訊息")` 可自訂條件為false時的panic訊息；本例用它指出default違反非零規則：

```rust
--8<-- "examples/ch14-api/fixtures/runtime-fail/derived_default.rs"
```

正常版本改為手動Default，明選有效值100。這項入口不需要驗證未知輸入，因為100是library自己選定的固定值：

```rust
--8<-- "examples/ch14-api/src/limits.rs:limit-default"
```

預設不一定是零或空資料。它應符合型別契約，而且其具體行為要說明；將來把100改成10，即使所有caller仍可編譯，得到的結果數量也可能改變。

## 公開設定操作，保留內部欄位的安排

Options保存ignore_case與有效BatchLimit，兩欄都私有。這次可以derive Default：bool沿標準預設false，BatchLimit則使用前面的手動預設100，兩項都符合Options約定。caller從預設開始，再以 **`with_ignore_case(self, bool)`** 或 **`with_limit(self, BatchLimit)`** 選擇新設定。

這兩項method仍用2.5的owned self接手整份設定。新的寫法 **`mut self`** 中，mut讓函式內這個self binding可修改；它沒有把參數變成 `&mut self`，caller仍交出原值。body修改指定欄位，再交回新的Self，因而可以串接呼叫，每次把前一個結果交給下一項；不用讓caller持有mutable reference。with_limit只接受已驗證的BatchLimit，不新增一條從任意usize繞過規則的路徑。

ignore_case借讀bool，limit借讀並Copy出BatchLimit。這些getter提供的是值與能力，不承諾欄位的名字、順序或記憶體配置：

```rust
--8<-- "examples/ch14-api/src/options.rs:options"
```

私有欄位能讓library增加內部狀態而不要求caller補struct literal；公有method與預設行為仍是約定。不是藏住欄位就能隨意改回傳型別、成功條件或公開Clone／Copy能力。

## 公有錯誤分類與source各保存一項角色

SearchError也在enum整體標non_exhaustive。EmptyQuery表示caller提供空搜尋文字；Read擁有原io::Error。Display只說這一層context，source沿8.6借出下層原因，保留外層reference仍借self的契約：

```rust
--8<-- "examples/ch14-api/src/errors.rs:search-error"
```

caller可以match已知錯誤來決定動作，再保留其他分支處理將來增加的種類。它也能顯示Display並沿source報告；Display文字供人閱讀，typed variant與原原因才供程式分類。本節沒有承諾所有診斷文字是固定可解析格式。

Hit是被動的結果資料：原行號與owned文字都公有，讓caller直接讀取與組成這份資料。這是另一種介面選擇，和需要維持驗證條件的BatchLimit不同；將來增加Hit欄位會影響外部struct literal，要列入相容性判斷。

```rust
--8<-- "examples/ch14-api/src/search.rs:hit"
```

search沿7.9的lines／enumerate／ASCII比較與contains。符合時保存原文字，達到有效上限才break；每行原index都計入行號，而非只數已保存的行。結果擁有文字，輸入結束後仍有效。

```rust
--8<-- "examples/ch14-api/src/search.rs:search"
```

read_matches沿7.5的 `&Path` 與7.9的read_to_string。它先檢查query，讀取失敗才建立Read；成功後把文字借給同一search，取得owned結果。使用者不能因為選了另一個入口，就突然讓空query成為有效值。

```rust
--8<-- "examples/ch14-api/src/search.rs:read-matches"
```

## pub use把公有入口與內部module分開

**re-export（重導出）** 用 `pub use` 讓已有的公有item也能從另一條公有路徑使用。`mod options;` 保留私有module，`pub use options::Options;` 則讓外部能用 `ch14_api::Options`。Options仍是同一個型別，沒有複製定義或建立alias wrapper；外部也沒有因此取得 `ch14_api::options` 這條私有路徑。

同理，library root重導出前面已教的BatchLimit、錯誤、結果與函式。本package另定義名為text的feature並預設啟用；啟用時提供 **`render_text(&[Hit]) -> String`**，借結果形成 `行號:原文字`、每筆有LF的文字報告，沿7.9的表示規則。9.1的cfg控制text module與這項公有函式；未啟用時這項函式不在本次介面中，搜尋能力仍在。後面會展示實作與對應manifest：

```rust
--8<-- "examples/ch14-api/src/lib.rs:exports"
```

如果將來搬動內部options檔案，保持root公有路徑與操作契約，caller便能繼續import同一名字。若原本已把整個module公開，caller也可能依賴舊路徑；新增重導出不會讓移除舊路徑自動成為相容修改。

## feature是可選能力，也是一份公開承諾

9.1已教feature定義／啟用與cfg，本例text只增加 **`render_text(&[Hit]) -> String`**：借用結果，沿7.9的format與push_str形成 `行號:原文字`，每筆有LF。它沒有修改搜尋規則、Options預設或結果ownership。

```rust
--8<-- "examples/ch14-api/src/text.rs:text"
```

package欄位沿既有workspace繼承，本repo的publish共用值是false。feature表的 `default = ["text"]` 表示一般建置啟用text；`text = []` 則表示此開關不連帶啟用其他功能：

```toml
--8<-- "examples/ch14-api/Cargo.toml:package"
--8<-- "examples/ch14-api/Cargo.toml:features"
```

現在由真正的外部example crate使用公有root路徑。範例先顯示預設，再選忽略ASCII大小寫與上限2；只有text編入時，才保留報告呼叫block：

```rust
--8<-- "examples/ch14-api/examples/consumer.rs"
```

```bash
cargo run -p ch14-api --example consumer --locked
```

```text
預設：ASCII忽略大小寫=false，最多100筆
1:Rust第一行
3:rust第三行
借完設定，上限仍是=2
text feature報告：
1:Rust第一行
3:rust第三行

```

第4行雖然也符合，已到上限所以沒有加入結果；第3行仍保存原本小寫rust。report只借hits，設定也仍可讀。

使用9.1已教的 `--no-default-features` 再跑consumer，前四行相同，只有最後的可選報告段不出現。再加 `--features text` 則重新取得報告；本例只有一個可選feature，因此 `--all-features` 也就是啟用text。

```bash
cargo run -p ch14-api --example consumer --no-default-features --locked
cargo run -p ch14-api --example consumer --no-default-features --features text --locked
```

Cargo在需要同一化的依賴建置中會合併功能請求。別讓feature代表互相排斥的搜尋規則，也別讓啟用text反而取消原能力；兩個依賴者同時選功能時，原合法工作應仍能成立。移除既有feature名稱或從default移除caller需要的能力，都可能破壞既有使用者。

### 外部分類有未來分支，原因仍可讀

下面同時觀察兩項invalid limit、空query、以及用7.9已教的PermissionDenied建立可控制Read原因。最後這項不是靠真實檔案權限碰運氣，而是檢查公開錯誤如何保存、顯示與借讀原因：

```rust
--8<-- "examples/ch14-api/examples/typed_errors.rs"
```

執行 `cargo run -p ch14-api --example typed_errors --locked`，前三行分別分類零上限、1001過大與空query；接著顯示自己的讀取context、原原因與 `PermissionDenied` 種類。原io錯誤Display措辭依平台可能不同，測試比較typed種類，不把作業系統文字當固定契約。

## 跨crate負例要在真正的crate邊界檢查

以下三份source是故意不能編譯的fixture。前兩份分別嘗試繞過BatchLimit的私有建構、窮舉外部non_exhaustive enum；第三份用私有module路徑取得Options。正確用法已在前面教過：驗證new、保留其他error分支、從root公有路徑import。

```rust
--8<-- "examples/ch14-api/fixtures/cross-crate-fail/examples/private_limit.rs"
```

```rust
--8<-- "examples/ch14-api/fixtures/cross-crate-fail/examples/non_exhaustive_match.rs"
```

```rust
--8<-- "examples/ch14-api/fixtures/cross-crate-fail/examples/private_module.rs"
```

它們分別應得到 `E0423`、`E0004`、`E0603`。non_exhaustive在定義crate內沒有限制窮舉；把enum搬進同一main裡，就測不到本次的外部規則。

因此fixture有自己的package，透過7.1已教的path dependency依賴真正ch14-api。以下literal package欄位和1.2／1.3相同：name、version、edition與最低rust-version都明寫，publish=false禁止上傳。空的 **`[workspace]`** 讓這個目錄成為另一份workspace根，不加入外面的教材workspace；它的Cargo.lock也獨立保存，避免一般tests把故意失敗目標當正常範例建置。

```toml
--8<-- "examples/ch14-api/fixtures/cross-crate-fail/Cargo.toml"
```

新增旗標 **`--manifest-path 路徑`** 明選本次Cargo從哪一份manifest開始；路徑從目前repo根目錄給出。下面只check選定example，預期非零結束與對應錯誤，不是應修成綠燈的一般測試：

```bash
cargo check --manifest-path examples/ch14-api/fixtures/cross-crate-fail/Cargo.toml --example private_limit --locked
cargo check --manifest-path examples/ch14-api/fixtures/cross-crate-fail/Cargo.toml --example non_exhaustive_match --locked
cargo check --manifest-path examples/ch14-api/fixtures/cross-crate-fail/Cargo.toml --example private_module --locked
```

## 用同一份caller檢查修改前後

**source compatibility（來源相容）** 問原合法caller不改程式碼，能否在新版繼續編譯；**behavior compatibility（行為相容）** 另問結果是否仍符合原約定。能編譯只是前一項，不能替預設值、輸出規則或失敗分類作保證。

先隔離一個縮小的Options例子，只保留本次要觀察的default與ignore_case getter，不帶整套搜尋。1.0.0內部只有私有bool；1.1.0增加私有limit與新getter，原default的false保持：

```rust
--8<-- "examples/ch14-api/compatibility/v1/src/lib.rs"
```

```rust
--8<-- "examples/ch14-api/compatibility/v1_1/src/lib.rs"
```

兩個library用**同一份實體consumer.rs**，不是把caller同步改寫成適應新版：

```rust
--8<-- "examples/ch14-api/compatibility/consumer.rs"
```

兩份manifest都是前面已教的隔離workspace。新設定 **`[[bin]]`** 描述一個binary target；name指定Cargo目標名稱，path以**該manifest目錄**為起點，選擇共用的 `../consumer.rs`，不要求入口一定是src/main.rs。package名稱相同，因此consumer都借助 `api_contract_demo` library名稱；version則各自指定真正的1.0.0／1.1.0。

```toml
--8<-- "examples/ch14-api/compatibility/v1/Cargo.toml"
```

```toml
--8<-- "examples/ch14-api/compatibility/v1_1/Cargo.toml"
```

```bash
cargo run --manifest-path examples/ch14-api/compatibility/v1/Cargo.toml --locked
cargo run --manifest-path examples/ch14-api/compatibility/v1_1/Cargo.toml --locked
```

兩次都應顯示 `未修改的caller：ignore_case=false`，也通過同一個assert。私有欄位增加沒有要求caller補建構欄位，舊操作與此項結果保留。這只驗證這份caller用到的契約，不代表任意API改動都已被證明相容。

若只把新版default改成true，caller仍可編譯，但同一assert會失敗：它依賴的區分大小寫預設已改。若原本Options的所有欄位都公有，增加一欄也可能讓外部struct literal缺欄位而不能編譯；不能在發版時才任意把原公有欄位藏起來。

### SemVer把相容性判斷寫進版本

**SemVer（semantic versioning，語意化版本）** 以major.minor.patch表示版本。對1.0.0以後的公有API，patch修正錯誤而保留相容契約；minor增加相容能力；不相容修改使用新的major。版本號是在表達維護者的承諾，Cargo不會自動證明程式符合它。

| 本例要做的修改 | 對既有caller的影響與版本判斷 |
| --- | --- |
| 修正錯誤行號，回到原已承諾規則 | 行為回到既有契約，可作patch修正，例如1.0.1。 |
| 增加私有內部欄位，保持原公有操作與行為 | 如實跑快照，可安排minor，例如1.1.0。仍須保留其他公開能力與承諾。 |
| 刪除或改名search、增加必填參數、縮窄可接受型別 | 原caller可能不能編譯，屬不相容修改，不能偽裝成patch。 |
| 把預設上限100改10 | caller可能仍可編譯，但結果被截得更少，要依原行為約定判斷不相容影響。 |
| 在原本exhaustive公有enum增加variant | 外部match可能缺分支，屬breaking；事後才加non_exhaustive也會使原match不合法。 |
| 在一開始non_exhaustive的enum增加variant | 外部早已必須保留其他分支，這項來源相容風險被減少；新失敗的行為意義仍須說明。 |
| 移除text feature或它原承諾的公有函式 | 原選這份能力的caller可能失敗，要當作公有API變更評估。 |

本repo教學package版本仍沿共同0.1.0，這與相容性快照明選1.x的示範不同。Cargo對0.x的相容界線採最左邊非零部分：0.1到0.2屬不相容界線，0.1.0到0.1.1通常在同一相容範圍；0.0.x的每次patch變動都可能跨界。讀依賴版本時，不能把所有0.x都當成相互相容。

## 發布前檢查要涵蓋caller與打包內容

tests/public_contract.rs是真正外部crate，只走公有入口，核對邊界值／有效default、ASCII與原行資料、零結果、空query優先、Error原因，以及text開關。檔案成功測試用repo的fixtures/note.txt；Cargo執行這份package的integration tests時，以package根目錄作工作目錄，因此 `Path::new("fixtures/note.txt")` 指向已提供材料。

先跑正常與沒有預設能力的組合，再看全部features。本節的負例留在前面隔離fixture，不會讓這組一般測試失敗：

```bash
cargo test -p ch14-api --locked
cargo test -p ch14-api --no-default-features --locked
cargo test -p ch14-api --all-features --locked
cargo clippy -p ch14-api --all-targets --all-features --locked
cargo doc -p ch14-api --no-deps --locked
```

文件建立沿7.2，不代替doctests或caller測試。root的rust-version沿1.3宣告最低支援版本；選新的語法或dependency時，要核對這項版本與支援平台，不只是自己的機器剛好能編譯。本例固定工具鏈就是所宣告的1.99。

**`cargo package`** 在本機把package組成`.crate`壓縮檔，再解開並編譯檢查，不上傳到registry。**`--list`** 只列將被打包的檔案，不建立壓縮檔；**`--allow-dirty`** 允許目前尚未提交的變更參與這次本機打包，不表示這份內容已提交或發布。

```bash
cargo package -p ch14-api --list --allow-dirty --locked
cargo package -p ch14-api --allow-dirty --locked
```

檢查清單應包含src、正常examples、測試與所需資料；組裝成功會產生 `target/package/ch14-api-0.1.0.crate` 並完成解包編譯。兩個相容性library快照與跨crate負例有各自nested package，Cargo不把它們當成本package的內容；共用consumer.rs仍是外層檔案，清單也會列出它。列清單能直接核對這條界線。

正式registry發版還需要選定公有名稱、description、授權metadata與支援契約，並以release notes交代變更。教材成員的publish=false與本次本機組裝保留；發布教材網站和上傳library到registry是兩項工作。

## 練習：新增有效上限，不改原caller的資料規則

以 `BatchLimit::new(1)` 建立有效上限，從Options預設開始。查 `Rust`，材料為 `Python\nRust第一行\nRust第二行\n`，預期只有一筆，原行號2、文字Rust第一行。用assert核對，說明為何上限1不會讓它變成原第1行。

完成後對照 [完整參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch14-api/examples/limit_solution.rs)，或執行 `cargo run -p ch14-api --example limit_solution --locked`。再判斷：把default從100改成1，與增加一個讓caller明選1的操作，對既有caller是否有相同影響？前者改未選設定的行為；後者讓原預設與原caller保留。

延伸閱讀：[英文Book公有API與發布](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)、[簡體對應](https://kaisery.github.io/trpl-zh-cn/ch14-02-publishing-to-crates-io.html)、[Cargo相容性契約](https://doc.rust-lang.org/cargo/reference/semver.html)、[Default](https://doc.rust-lang.org/std/default/trait.Default.html)、[non_exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute)。
