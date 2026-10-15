# 12.1 Cargo工程設定：共同設定與不同建置

第1.3節已能在workspace選package，第10.1節也已宣告本機與第三方依賴。現在想讓幾個一起維護的package共用版本欄位，並讓使用者選擇是否編入批次功能。這一節要分清：**共同設定由誰採用、這次有哪些介面，以及編譯使用哪組設定？**

我們繼續用商品小計：單價80、數量2得到160；再加另一筆單價120、數量1，合計280。這裡只算商品，不加入運費，讓設定改變與數字規則容易分開觀察。

## 先找到這份小workspace

本頁使用獨立的`examples/ch09-engineering/lesson-workspace/`。它有自己的完整根設定與lockfile，不使用外層舊`ch09-engineering` package。以下命令都從**教材repo根目錄**執行，明寫`--manifest-path`選到這一份：

```text
examples/ch09-engineering/lesson-workspace/
├── Cargo.toml
├── Cargo.lock
├── subtotal/
│   ├── Cargo.toml
│   └── src/lib.rs
└── app/
    ├── Cargo.toml
    ├── src/lib.rs
    ├── src/main.rs
    ├── tests/features.rs
    ├── examples/feature_answer.rs
    └── fixtures/compile-fail/cfg_bool_is_not_removal.rs
```

`subtotal`只提供乘法library；`app`使用它，再提供單筆與可選的兩筆介面。這樣同一頁能看到實際載入的全部程式。

## 共用欄位，仍由成員明確採用

`[workspace.package]`保存成員可採用的共同package欄位。`version`與`edition`已在前文介紹；本節再用兩項：

| 欄位 | 意義 |
| --- | --- |
| `rust-version = "1.99"` | package宣告至少需要Rust 1.99；它不是替電腦安裝或選擇工具鏈的指令。教材選工具鏈仍由`rust-toolchain.toml`負責。 |
| `publish = false` | 不允許透過Cargo把這份教材package發布到crates.io，仍能在本機建置與執行。 |

成員寫`version.workspace = true`，表示「我的version採用workspace.package中的共同值」。`edition.workspace = true`等欄位也是相同形式，**不是把edition設為布林值**。TOML的點號在這裡指定設定欄位的層級；`true`表示採用。每個成員自己的`name`仍分別填寫。

依賴也有共同表：`[workspace.dependencies]`保存可供成員採用的來源或版本設定。成員還須在自己的`[dependencies]`寫`lesson-subtotal.workspace = true`，才建立這項dependency；只在根設定列出，不會讓所有成員自動取得library。

根表中的`path = "subtotal"`從**workspace根manifest所在資料夾**出發。第10.1節直接在成員宣告的dependency path，則從那份成員manifest出發；同樣要找到subtotal，從app出發會是`../subtotal`。把共同來源移到根表後，起點也跟著改變。

## profile選編譯設定

**profile**是一組編譯設定。一般`cargo build`／`run`使用`dev`；加`--release`改用`release`。這與Rust edition、要執行哪個package都不同。

`opt-level`控制最佳化程度，常用數字為0至3。預設dev使用0，release使用3；較多最佳化工作通常增加編譯時間，成品的實際速度仍要對相同任務量測，不能由這個數字直接推定。

本例還在release設`overflow-checks = true`，要求執行時檢查整數運算是否超出型別範圍；超出就panic。第一章的有限整數範圍仍然成立，這項設定不會替`i32`增加能表示的數字。它讓release也保留這項檢查，避免只因建置模式不同而把超出範圍的運算悄悄放過。

profile須寫在**workspace根manifest**，成員manifest的profile設定會被Cargo忽略。以下是本頁小workspace的**完整**`Cargo.toml`；members與resolver回用第1.3節，後面三組是剛介紹的共同設定與profile：

```toml
--8<-- "examples/ch09-engineering/lesson-workspace/Cargo.toml"
```

### 被依賴的library也只有已知計算

`subtotal/Cargo.toml`完整採用共同欄位：

```toml
--8<-- "examples/ch09-engineering/lesson-workspace/subtotal/Cargo.toml"
```

它的完整`subtotal/src/lib.rs`如下。單價乘數量的規則不因設定移到workspace而改變：

```rust
--8<-- "examples/ch09-engineering/lesson-workspace/subtotal/src/lib.rs"
```

## 替自己的package定義feature

第10.1節的`features = ["derive"]`選擇的是dependency提供的功能。現在要替**自己的app package**定義名為`batch`的feature，控制是否提供兩筆小計函式。

`[features]`中的`名稱 = [其他功能名稱]`定義功能及它連帶啟用的項目。`batch = []`表示這個開關不連帶啟用其他項目；`default = ["batch"]`表示預設啟用batch。default是一份功能列表，不是另一套計價規則。

本例完整`app/Cargo.toml`如下。它一面明選共同dependency，一面定義自己的feature：

```toml
--8<-- "examples/ch09-engineering/lesson-workspace/app/Cargo.toml"
```

建置本package時，`--no-default-features`不啟用default列表，`--features batch`則明確啟用batch；兩者可以一起使用。`--all-features`啟用所選package的全部功能，本例會啟用batch。

這與第10.1節dependency設定中的`default-features = false`分工不同：後者控制那項**依賴**的預設功能，這裡的命令列旗標則選本次建置package的功能。若同一dependency被多個使用者請求不同功能，Cargo通常會合併它們所需的功能，因此feature適合增加能同時存在的能力。本例關掉batch後，單筆介面與160的答案仍保留。

## cfg決定哪些項目存在

第2.7節的`#[cfg(test)]`已依測試建置決定是否納入module。這種依建置條件保留或排除程式的做法叫**條件編譯**。

`#[cfg(feature = "batch")]`使用相同的屬性形式：啟用batch才保留緊接著的函式；未啟用時，該函式根本不在這次library的公開介面中。字串中的batch須符合manifest所定義的名稱。

`app/src/lib.rs`完整程式如下。兩筆資料各用`[i32; 2]`保存，第一格單價、第二格數量；兩次計算仍各自使用自己的資料：

```rust
--8<-- "examples/ch09-engineering/lesson-workspace/app/src/lib.rs"
```

### cfg!只交回bool

`cfg!(feature = "batch")`是另一個macro，依建置條件產生true或false，可以保存或顯示。它**不會排除包住自己的程式**。普通`if`即使條件是false，裡面的名稱與型別仍須通過編譯檢查。

因此呼叫可選函式時，也用`#[cfg(...)]`排除整個呼叫block，連同那個block才需要的use。以下main另外觀察兩個平台條件：`unix`表示目標屬於Unix類系統，例如Linux與macOS；`target_os = "windows"`表示目標作業系統為Windows。

這裡的**編譯目標平台**是程式準備在哪種平台執行；**host**是執行編譯工具的本機。沒有另選平台時，兩者通常相同。crate內的cfg看自己的編譯目標，不是在每次執行時重新偵測；`--release`也不會把Linux程式變成Windows程式。前文的example target則指Cargo選出的某個建置成果，這兩種target用途須依上下文分清。

完整`app/src/main.rs`：

```rust
--8<-- "examples/ch09-engineering/lesson-workspace/app/src/main.rs"
```

從repo根目錄執行三種配置：

```bash
cargo run --manifest-path examples/ch09-engineering/lesson-workspace/Cargo.toml -p lesson-engineering --locked
cargo run --manifest-path examples/ch09-engineering/lesson-workspace/Cargo.toml -p lesson-engineering --no-default-features --locked
cargo run --manifest-path examples/ch09-engineering/lesson-workspace/Cargo.toml -p lesson-engineering --no-default-features --features batch --locked
```

預設與第三種都先印`單筆商品小計=160，batch=true`，再印`兩筆商品小計=280`。第二種只印單筆160與batch=false，沒有批次那一行。三種最後都顯示目標平台條件；教材Linux環境是`unix=true，windows=false`，你的結果依實際target而定。

### false為何仍然編譯失敗

下面是獨立的故意失敗程式，不依賴library。它仍用相同兩筆資料，卻把呼叫藏在普通`if cfg!(...)`中：

```rust
--8<-- "examples/ch09-engineering/lesson-workspace/app/fixtures/compile-fail/cfg_bool_is_not_removal.rs"
```

沒有batch時，attribute排除了two_orders，但普通if的body仍要檢查，因此得到E0425與`cannot find function two_orders`。正確main使用attribute排除呼叫block，才讓未啟用功能的版本也成立。

若要單獨觀察這份診斷，可直接呼叫編譯器`rustc`。`--edition=2024`選本書使用的語法版本，`-o`指定成功時的產物位置；本次刻意沒有傳入batch條件，所以預期失敗、不產生成品：

```bash
rustc --edition=2024 examples/ch09-engineering/lesson-workspace/app/fixtures/compile-fail/cfg_bool_is_not_removal.rs -o examples/ch09-engineering/lesson-workspace/target/cfg-bool-failure
```

## 驗證功能組合，再使用release

**integration test**位於package的`tests/`，每份檔案是獨立測試crate，從公開library介面呼叫；不是把測試函式直接放進主要binary的main。assertion與`#[test]`沿用第2.7節。

完整`app/tests/features.rs`如下：單筆測試一直存在；批次測試只有batch啟用才存在，除了280也驗證數量零的結果120，能抓出兩筆誤用同一資料的錯誤。

```rust
--8<-- "examples/ch09-engineering/lesson-workspace/app/tests/features.rs"
```

下面分別驗證預設、關閉預設與全部功能。前兩次features.rs中的測試數應為2與1，全部功能又回到2。`--all-targets`指所選package的library、binary、examples、tests等建置成果，**不是所有作業系統**；本頁的完整來源包含下方練習答案，整包檢查也會編譯它；其中只用本節已教的介面。

```bash
cargo test --manifest-path examples/ch09-engineering/lesson-workspace/Cargo.toml -p lesson-engineering --locked
cargo test --manifest-path examples/ch09-engineering/lesson-workspace/Cargo.toml -p lesson-engineering --no-default-features --locked
cargo test --manifest-path examples/ch09-engineering/lesson-workspace/Cargo.toml -p lesson-engineering --all-features --locked
cargo clippy --manifest-path examples/ch09-engineering/lesson-workspace/Cargo.toml -p lesson-engineering --all-targets --all-features --locked
cargo build --manifest-path examples/ch09-engineering/lesson-workspace/Cargo.toml -p lesson-engineering --all-features --release --locked
cargo run --manifest-path examples/ch09-engineering/lesson-workspace/Cargo.toml -p lesson-engineering --all-features --release --locked
```

release的數字仍是160與280，建置訊息改成release，產物預設放在這份小workspace的`target/release/`。這驗證配置可以建置且已知答案相符，沒有量測效能。

## 對照教材repo的根設定

本頁的小workspace與第1.3節的入門workspace各有自己的根。教材repo還有較大的維護workspace，後續章節會明選其中的package。它同樣以members列成員、以`-p`選package，共用自己的Cargo.lock及target資料夾；`exclude = ["examples/intro-workspace"]`則將入門workspace從這份成員管理中排除，沒有把兩份lockfile混成一份。

下面精確節錄**教材repo根Cargo.toml**的三組設定，不是另一份可直接執行的完整manifest，也不要拿它覆蓋repo根檔案：

```toml
--8<-- "examples/ch09-engineering/lesson-workspace/fixtures/repo-root-settings.toml"
```

共同package欄位的語義與小workspace相同；後續成員寫`version.workspace = true`等形式，就是從這裡採用。共同dependency這次叫ch01-basics，path從教材repo根開始，因此是`examples/ch01-basics`；只有明選它的成員才會依賴它。本頁選用的是另一份小workspace的lesson-subtotal，沒有因此載入舊第一章library。

兩處release都使用opt-level 3與整數溢位檢查。改用哪份manifest決定你進入哪個workspace，`-p`再決定該workspace中的package；共用欄位不會把所有成員程式串成一個執行檔。

## 練習：關掉批次，保留單筆

寫一個example，每次都確認單筆80×2仍為160；只有batch啟用時，才呼叫`two_orders([80, 0], [120, 1])`並確認120。分別用`--no-default-features`與預設設定執行，先預測哪行會消失。不能把可選呼叫只包在普通if中。

再回答：只把lesson-subtotal列在workspace.dependencies，app就能呼叫它嗎？為什麼release仍可得到相同的商品小計？

??? example "完整可執行答案"

    設定、兩份library與所有測試沿用本頁已完整列出的檔案；只新增`app/examples/feature_answer.rs`：

    ```rust
    --8<-- "examples/ch09-engineering/lesson-workspace/app/examples/feature_answer.rs"
    ```

    ```bash
    cargo run --manifest-path examples/ch09-engineering/lesson-workspace/Cargo.toml -p lesson-engineering --example feature_answer --no-default-features --locked
    cargo run --manifest-path examples/ch09-engineering/lesson-workspace/Cargo.toml -p lesson-engineering --example feature_answer --locked
    ```

    第一種只顯示`保留單筆=160，batch=false`；第二種batch為true，並多出`第一筆數量零，兩筆合計=120`。第一筆零件商品貢獻0，第二筆120×1仍是120，沒有沿用前一例的280。

    app仍須在自己的dependencies明選lesson-subtotal。release改變編譯設定，本例的原始碼、輸入與未超範圍的整數運算規則相同，因此已知答案不變；這與成品速度是否改變是不同問題。

下一節加入編譯之前的Rust工作：先執行build script，再把建置當下的資訊固定進成品。

[下一節：在編譯程式之前執行Rust工作](12-02-lesson.md)
