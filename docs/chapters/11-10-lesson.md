# 11.10 用 newtype 保護單位與選擇運算介面

路程80公尺與等待80秒，都能用 i32 保存。現在希望「顯示路程」的函式只接受路程，誤傳等待時間時就由編譯器指出。type alias 能改善名字，但不會建立兩種型別身分；這次需要用 struct 建立邊界。

## alias 縮短名字，newtype 建立身分

`type MeterAlias = i32` 與 `type SecondAlias = i32` 都替同一個 i32 取名字。因此需要 MeterAlias 的函式仍接受 SecondAlias。

alias 適合替重複的長型別取短名，例如第 11.6 節的 `Box<dyn Fn(i32) -> i32>`。取名 PriceFn 後，呼叫、ownership 與 object lifetime 的契約都不變。

這一步完整 `examples/unit_aliases/newtypes/mod.rs` 只有 aliases 及其函式：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/unit_aliases/newtypes/mod.rs"
```

本節所有 example 使用以下完整 manifest，沒有 library。每個 example 是各自的 crate，自己的 helper module 也屬於該 crate：

```toml
--8<-- "examples/intro-workspace/type-stages/Cargo.toml"
```

完整 caller 觀察 alias 的兩種用途：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/unit_aliases/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example unit_aliases --locked
```

```text
把秒數誤傳成路程，編譯器仍接受=80
完整型別收到同一closure，80調整為=70
```

第一行沒有攔住單位誤用；第二行將 PriceFn 直接交給完整型別，不需重新 Box 或 clone，因為兩個名稱指同一型別。

**newtype pattern** 是用新型別包住原資料的設計方式，不是 Rust 關鍵字。本例使用 **tuple struct**：欄位有順序、沒有名稱。`pub struct Meters(pub i32);` 宣告公有型別，內層 pub 則公開第一個 i32 欄位。`Meters(80)` 建立值，`.0` 取第一個欄位，沿用 tuple 的編號方式。

Meters、Seconds 是不同 struct，也都不同於 i32。derive 只選已教的 Debug／PartialEq；沒有選 Copy，所以交接整體仍會 move。借用 struct 時可以複製出 `.0` 的 i32，並不等於 struct 本身有 Copy。

完整 `examples/unit_identity/newtypes/mod.rs` 此時只提供單位與借讀函式，沒有 Add：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/unit_identity/newtypes/mod.rs"
```

完整 caller：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/unit_identity/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example unit_identity --locked
```

```text
路程的數字=80，單位為公尺
經過的數字=80，單位為秒
借完路程，原型別仍是=Meters(80)
```

distance_value 接受 `&Meters`。即使欄位同為80，把 `&Seconds` 傳入也會得到 E0308：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_wrong_unit.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_wrong_unit.expected"
```

caller 必須明確交來路程，才通過這道型別邊界。公有 i32 欄位仍允許負數；這個 newtype 只區分單位，沒有自動完成數值驗證。

## Add 定義左右輸入與輸出

兩段路程相接，希望結果仍帶著公尺單位。**`std::ops::Add`** 是 `+` 的 trait 介面；實作它就能定義這種加法。

| Add 的角色 | 本次意義 |
| --- | --- |
| `Add<Rhs = Self>` | Rhs 是 right-hand side，右運算元型別。等號指定預設型別參數：未另選 Rhs 時，右邊與 Self 同型別。不是替 method 提供預設 body。 |
| `type Output` | 實作選定相加結果型別，可以是 Self，也可以另選。 |
| `add(self, rhs: Rhs) -> Self::Output` | 接手左右兩值，交回結果。非 Copy 輸入在本例會 move。 |

`impl Add for Meters` 以預設 Rhs 接收另一個 Meters，Output 選 Self。`Self(...)` 在 tuple struct 的 impl 中就是該型別的建構方式。

同一 helper 的 `add_same<T: Add<Output = T>>` 另外要求「結果仍是 T」。這是 associated type 的等式 bound，不能只憑型別有 Add 就假定輸出一定相同。

接著有500毫米與2公尺，要交回毫米。`impl Add<Meters> for Millimeters` 明選右型別，先把2乘1000，再加500，得到2500毫米。它只提供這個方向，不會自動產生反向加法或其他運算子。

下面是這一步完整 `examples/unit_add/newtypes/mod.rs`，只合併單位與加法：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/unit_add/newtypes/mod.rs"
```

完整 caller：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/unit_add/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example unit_add --locked
```

```text
兩段路程相接=200公尺
要求同型別輸出的函式=100公尺
先換右邊的單位，結果=2500毫米
```

這些小整數沒有超過 i32 範圍；wrapper 的計算仍受整數規則限制。換算後的型別是 Millimeters，保留了2500的單位。

相加接手整體；以下再讀非 Copy 的 left，得到 E0382：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_add_moved.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_add_moved.expected"
```

若要保留輸入，可以另設計借用 method，或依型別語意選擇 Copy；本例沒有暗中 clone。反向 `Meters + Millimeters` 也不符合目前 Meters 的右型別，得到 E0308：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_reverse_add.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_reverse_add.expected"
```

## orphan rules 限制哪個 crate 負責實作

路程清單用 `Vec<Meters>` 保存，報告時想顯示「2段路程，共200公尺」。能否直接為標準 Vec 實作標準 Display？

**orphan rules（孤兒規則）** 限制 trait 實作能放在哪個 crate。**local（本地）** 指型別或 trait 定義在目前寫 impl 的 crate。import 標準 Vec 或替它取 alias，不會改變定義來源。

對本例沒有泛型參數的 `impl Display for ...`，Display 或實作型別至少一方必須本地。否則不同 library 都能替同一對標準 trait／型別提供不同實作，合用時便有衝突。以下只替 Vec 取 CountList 別名，仍得到 E0117：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_foreign_alias.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_foreign_alias.expected"
```

### 本地 wrapper 公開自己需要的介面

改用 `pub struct MeterBatch(Vec<Meters>);`，就有本地 struct 可實作 Display。內層欄位沒有 pub，helper module 外不能直接建立或讀寫這個欄位。

from_values 接手 Vec；count 借讀段數，total 借讀各 i32 後交回 Meters。Display 沿第 8.3 節使用 Formatter／write!，沒有消耗集合。這層 wrapper 接手原 Vec，沒有為包裝另建 Box 或 clone 集合。

另一個合法方向是**本地 trait，外部型別**。本例宣告 BatchSize，再為 `Vec<i32>` 實作 batch_size；它只是借讀 len。

這一步完整 `examples/unit_interface/newtypes/mod.rs` 只有單位、報告與本地 trait：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/unit_interface/newtypes/mod.rs"
```

完整 caller：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/unit_interface/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example unit_interface --locked
```

```text
路程報告：2 段路程，共 200 公尺
再次借讀總路程=200公尺
本例定義的trait能實作在Vec上，項目數=2
```

newtype 不會自動繼承內部 Vec 的 method。MeterBatch 沒有 push，下面完整負例即使使用 mutable binding，仍得到 E0599：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_no_push.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_no_push.expected"
```

第 11.4 節的 Deref 能提供借用式 method 查找，但本例沒有替 MeterBatch 實作它。這次只承諾建立、計算、報告；公開整套 Vec 操作會增加 caller 可依賴的介面，不是包住 Vec 就必須公開全部操作。

### 泛型還要看 uncovered type parameter 的位置

外部 trait 的泛型實作還要檢查次序：先看實作的 self 型別，再依序看 trait 的型別 arguments。在第一個本地型別以前，不能出現 **uncovered type parameter**，也就是尚未被一般型別包住的 impl 型別參數。例如單獨的 T 就是 uncovered；`Wrapped<T>` 則是本地 wrapper。

`impl<T> From<T> for Wrapped<T>` 的 self 型別首先就是本地 Wrapped，符合這項來源檢查。Wrapped 是泛型 tuple struct，欄位型別為 T；from 接手它，into_inner 再整體交回。標準 From 也提供對應 Into，沿第 8.4 節的轉換關係。

完整 `examples/unit_from/newtypes/mod.rs`：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/unit_from/newtypes/mod.rs"
```

完整 caller：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/unit_from/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example unit_from --locked
```

```text
本地wrapper提供From，取回原數字=80
owned文字也能整體交回=Rust
```

反過來 `impl<T> From<Wrapped<T>> for T`，self 的 T 出現在本地 Wrapped 以前，得到 E0210：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_uncovered_from.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_uncovered_from.expected"
```

into_inner 是自己型別的 inherent method，不是在為任意 T 新增外部 trait 實作，所以能完成取回工作。不能只因 trait argument 有本地型別，就忽略前面的 uncovered T；也不能反推所有外部 self 型別都禁止。

規則對 **fundamental types** 有特殊處理：在本地性判斷上，`Box<LocalType>` 視為本地；但 `Box<T>` 不會把 T 視為 covered。相對地，一般 Vec 外層可以 cover T，卻不因內部型別本地就讓 Vec 本身變本地。這是判讀泛型 orphan 診斷的區別，本例沒有另外依這項特殊規則增加 impl。

## coherence 也要求適用的 impl 不重疊

**coherence（一致性）** 要求實作既通過 orphan 檢查，也沒有互相重疊。**blanket implementation** 是涵蓋一整類型別的泛型實作；例如 `impl<T> Label for T` 已涵蓋每個 T，包括 Meters。

再寫 `impl Label for Meters` 不會覆寫前者，兩份都能匹配 Meters，因此得到 E0119。即使 Label 是本地 trait，也不能這樣重疊：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_overlapping_impls.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/newtype_overlapping_impls.expected"
```

若不同型別要不同標籤，就不要先承諾所有 T 共用同一份實作。orphan 限制跨 crate 的實作責任，重疊檢查確保適用 impl 的唯一性，兩項都需要成立。

## 練習與完整答案

建立私有 `Vec<Seconds>` 欄位的 TimeBatch。from_values 接手集合，total 借讀後交回 Seconds；Display 顯示 `2 段等待，共 90 秒`，輸入為30與60。報告後再確認 total，表示它沒有消耗集合。

完整 `examples/units_complete/newtypes/mod.rs` 此時才合併本節所有介面與三項測試：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/units_complete/newtypes/mod.rs"
```

完整答案根檔案：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/units_complete/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example units_complete --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example units_complete --locked
```

應顯示90秒，三項測試檢查加法單位／換算、空與非空集合的報告、alias 與 generic wrapper 的交接。路程與等待仍是不同型別，不能把 Seconds 交給 distance_value。

[下一節：Any 與執行期型別判斷](11-11-lesson.md)
