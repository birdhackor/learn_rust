# 8.9 用型別邊界保護單位，再選擇可用的trait介面

程式收到路程80公尺與等待80秒，底層都能用i32保存。現在希望「顯示路程」的函式只接受路程；把等待時間誤傳進去時，編譯器就指出問題，而不是安靜顯示80公尺。

5.2的type alias可以替既有型別取好懂的名字，卻不建立這條邊界。本節先觀察這項差別，再為路程提供有單位的加法與報告介面；最後說明哪些trait實作可以放在自己的crate，哪些會造成衝突。

## type alias縮短名稱，兩個名字仍可能是同一型別

`type MeterAlias = i32;` 與 `type SecondAlias = i32;` 都替i32取另一個名字。因此接收MeterAlias的函式仍接受SecondAlias：編譯器沒有取得兩種單位的型別身分，只看到同一個i32。

alias仍有另一個實際用途：讓重複的長型別有簡短名稱。8.5已教的 `Box<dyn Fn(i32) -> i32>` 可以取名PriceFn，讓回傳簽名聚焦「可調整金額的函式」。Box與Fn的呼叫、ownership、預設object lifetime契約都維持原樣，alias不增加能力，也不取消限制。

下列程式把兩項用途放在同一個aliases片段；price_fn以不同closure做減10或加10，沒有加入未教的並行能力：

```rust
--8<-- "examples/ch08-smart-pointers/src/newtypes.rs:aliases"
```

```rust
--8<-- "examples/ch08-smart-pointers/examples/type_aliases.rs"
```

```bash
cargo run -p ch08-smart-pointers --example type_aliases --locked
```

```text
把秒數誤傳成路程，編譯器仍接受=80
完整型別收到同一closure，80調整為=70
```

第一行是本節要攔住的單位錯用，alias沒有攔住它。第二行則展示alias適合的工作：PriceFn直接交給原完整型別，不需轉換、重新Box或clone closure。名字有助人讀懂意圖，能否互傳仍由真正的型別決定。

## newtype用另一個struct建立型別身分

**newtype pattern** 是以新型別包住既有資料的設計方式，例如用Meters包一個i32。它是一般struct的用法，不是新增的Rust關鍵字；一個具名欄位的struct也能達到這項目的。

這次使用 **tuple struct**：欄位有順序而沒有名稱。`pub struct Meters(pub i32);` 宣告公有Meters型別，括號內第一個欄位是i32；內層pub另讓這個欄位與建構方式可在library外使用。`Meters(80)` 建立值，`.0` 讀第一個欄位，沿1.7的tuple欄位編號方式。欄位仍是2.1的struct資料，只換成這種宣告形式。

Meters與Seconds是兩個不同struct，因而也與i32不同。derive只選4.2已教的Debug與PartialEq，便於觀察與檢查；沒有選Copy，所以整體交接仍會move。numeric欄位本身是Copy，借用struct時仍能複製出這個i32。

```rust
--8<-- "examples/ch08-smart-pointers/src/newtypes.rs:units"
```

```rust
--8<-- "examples/ch08-smart-pointers/examples/unit_newtypes.rs"
```

```bash
cargo run -p ch08-smart-pointers --example unit_newtypes --locked
```

```text
路程的數字=80，單位為公尺
經過的數字=80，單位為秒
借完路程，原型別仍是=Meters(80)
```

distance_value接收 `&Meters`，並借讀`.0`，沒有接手struct。即使兩份數值都是80，也不會讓Seconds自動成為Meters；把Seconds的reference傳給它，下面fixture得到 `E0308` 型別不符：

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/newtype_wrong_unit.rs"
```

這就是型別邊界的用途：caller必須先說清楚交來的是路程，不能只因為它是整數就通過。這份簡單newtype只區分單位，沒有驗證數值是否非負；公有欄位也允許caller直接建立任何i32值。5.4的私有欄位與驗證建構是另一份可選的介面設計，不能光靠「newtype」名稱假定它已完成。

## Add讓路程相加後仍帶著單位

知道兩段路程的單位後，希望相接的結果仍是Meters，而不是取出兩個i32相加、把單位丟掉。標準庫的 **`std::ops::Add`** 是 `+` 運算子的trait介面；為型別實作它，就能定義這個型別可用的加法。

先把本次需要的契約拆開：

| Add的一項角色 | 本次意義 |
| --- | --- |
| `Add<Rhs = Self>` | Rhs是right-hand side，即右運算元型別。等號指定**預設型別參數**；實作Add而未另寫Rhs時，右邊和左邊Self同型別。這是型別argument的預設選擇，不會替add提供body。 |
| `type Output` | 沿6.2，實作選定相加結果的型別。它可以選Self，也可以選其他型別，不由caller臨時改選。 |
| `add(self, rhs: Rhs) -> Self::Output` | 接手左、右兩個值，再交回結果；本例的Meters不是Copy，所以會move兩個輸入。 |

`impl Add for Meters` 使用預設右型別Meters，Output選Self。在tuple struct的impl中，`Self(...)` 也是用當前型別的建構方式，這裡就是 `Meters(...)`。body複製出兩個i32欄位相加，再包成新Meters。

同一片段的add_same沿6.2使用 `T: Add<Output = T>`：它要求兩個同型別輸入，結果也必須是T，再做同一個 `+`。這項bound沒有假定所有Add實作都交回原型別，而是明確選出符合這項工作的實作。

```rust
--8<-- "examples/ch08-smart-pointers/src/newtypes.rs:add-meters"
```

Meters的加法接手整體，`left + right` 不會因欄位是i32就自動保留left。下面在相加後再讀left，得到 `E0382`：

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/newtype_add_moved.rs"
```

若某個型別適合Copy，可以依4.2另作選擇；若要保留非Copy輸入，也可以設計借用的計算method。本節採接手兩段、交回新路程的Add契約，沒有暗中clone輸入。

### 右型別可另選，但換算與方向必須明說

現在有500毫米的接段與2公尺的主段，希望交回毫米：先把2公尺乘1000成2000毫米，再與500相加，結果是2500毫米。不同單位不能只把底層的500與2直接相加。

`impl Add<Meters> for Millimeters` 明選Rhs為Meters，左邊Self是Millimeters，Output仍選Millimeters。下面只提供這個方向；Rust不會自動再產生反向的Meters加Millimeters，也不會自動取得其他運算子實作。

```rust
--8<-- "examples/ch08-smart-pointers/src/newtypes.rs:add-mixed"
```

把同單位、泛型函式與換算的結果一起觀察：

```rust
--8<-- "examples/ch08-smart-pointers/examples/unit_add.rs"
```

```bash
cargo run -p ch08-smart-pointers --example unit_add --locked
```

```text
兩段路程相接=200公尺
要求同型別輸出的函式=100公尺
先換右邊的單位，結果=2500毫米
```

數字採小整數，沒有超過i32範圍；wrapper中的運算仍沿1.5的整數規則，不是無限精度計算。第三行除了結果數字，型別也保留Millimeters，caller不必猜2500代表哪種單位。

下面故意反向。Meters目前的Add要求右邊也是Meters，不能收到Millimeters，因而得到 `E0308`：

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/newtype_reverse_add.rs"
```

## orphan rules把實作安排在有責任的crate

路程集合本來可以用 `Vec<Meters>` 保存，現在希望報告時顯示「2段路程，共200公尺」。能否直接替標準Vec加一份Display，讓所有Vec都照自己的規則顯示？

**orphan rules（孤兒規則）** 限制trait實作可放在哪個crate。**local（本地）** 指型別或trait定義在正在寫這份impl的crate；在自己的module import標準Vec或另取alias，不會把它的定義搬來。對本節沒有泛型參數的Display實作，trait或實作型別至少一方必須在本crate定義。

Display與Vec都由標準庫定義。若任何library都能為這一對外部trait／型別加入自己的實作，兩個library合用時可能同時提供不同Display，caller無法同時採用它們。這項限制讓我們為自己的型別或自己的trait負責，不任意改寫全域外部型別的trait契約。

下面用CountList替Vec取alias也無法通過，得到 `E0117`；新名字仍是同一個外部Vec：

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/newtype_foreign_alias.rs"
```

### 本地newtype可以提供自己的Display與有限操作

把集合保存到新的MeterBatch，就有本crate定義的型別，可以實作標準Display。這次 `pub struct MeterBatch(Vec<Meters>);` 的型別公有，內層欄位沒有pub，因而library外不能直接建立或讀寫這個欄位。

公有from_values接手已建立的Vec；count借讀段數，total借讀每項i32、相加後交回Meters。Display沿5.3使用Formatter與write!，只讀這兩項操作形成報告。這裡沒有再配置一個Box，也沒有clone整份集合：wrapper只是接手原Vec。

```rust
--8<-- "examples/ch08-smart-pointers/src/newtypes.rs:batch"
```

另一個合法方向是**自己定義trait，為外部型別實作**。BatchSize在此library crate宣告，所以這個crate可以為 `Vec<i32>` 提供batch_size，借用原Vec並交回len：

```rust
--8<-- "examples/ch08-smart-pointers/src/newtypes.rs:local-trait"
```

```rust
--8<-- "examples/ch08-smart-pointers/examples/newtype_interface.rs"
```

```bash
cargo run -p ch08-smart-pointers --example newtype_interface --locked
```

```text
路程報告：2 段路程，共 200 公尺
再次借讀總路程=200公尺
library定義的trait能實作在Vec上，項目數=2
```

前兩行借用同一份集合，報告沒有消耗它；第三行使用的是library已提供的本地trait實作，example只import BatchSize並呼叫它。

newtype也不會自動繼承內部Vec的method。MeterBatch沒有提供push，下面即使有mutable變數，也得到 `E0599`：

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/newtype_no_push.rs"
```

8.3的Deref能提供額外借用與method查找，但本節沒有為MeterBatch實作Deref。若我們只承諾集合建立後能計算與報告，就只公開所需method；把所有Vec操作自動公開，會增加caller可依賴的介面。限制介面是這次選擇的目的，不是newtype一定要模仿內部型別。

### 泛型impl還要看未包住的型別參數出現在哪裡

前面的本地trait／型別判斷，對泛型外部trait實作還有一項限制：**實作的self型別先於trait的型別arguments檢查**。在第一個本地型別以前，不能出現尚未被一般型別包住的impl型別參數，稱為 **uncovered type parameter**。例如直接的T就是未包住的參數；本地 `Wrapped<T>` 則提供了我們自己定義的型別。

用已教的From看這項規則。下列 `impl<T> From<T> for Wrapped<T>` 的self型別先是本地Wrapped，所以符合這項本地來源檢查。Wrapped是tuple struct的泛型形式，括號內的欄位型別是T；from接手T並建立Self，into_inner再整體交回它。這份From沿5.4提供對應Into，不另新增轉換規則。

```rust
--8<-- "examples/ch08-smart-pointers/src/newtypes.rs:generic-wrapper"
```

```rust
--8<-- "examples/ch08-smart-pointers/examples/newtype_from.rs"
```

```bash
cargo run -p ch08-smart-pointers --example newtype_from --locked
```

```text
本地wrapper提供From，取回原數字=80
owned文字也能整體交回=Rust
```

反過來的 `impl<T> From<Wrapped<T>> for T` 不能成立：依序先看self型別T，再看trait argument中的本地Wrapped；第一個本地型別之前已出現未包住的T。不是看到Wrapped就能替任意T加入這份外部From，下面fixture得到 `E0210`：

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/newtype_uncovered_from.rs"
```

需要取回內容，可以使用上面的inherent method into_inner；它是在自己的Wrapped實作區塊裡，並沒有替所有T新增外部trait實作。這項區別回應兩份程式的差別，不是說所有「外部self型別、本地trait argument」都禁止，具體沒有未包住型別參數的情況可以合法。

完整規則另把Box等部分包裝型別視為 **fundamental types**：在本地判斷上，`Box<LocalType>` 也可視為本地；但 `Box<T>` 不把其中T視為已包住。本節沒有以這項特殊規則設計新的impl；讀官方泛型orphan診斷時，不能把「所有外層型別都包住T」當成通則。

## coherence也禁止兩份impl同時匹配

**coherence（一致性）** 要求trait實作通過orphan檢查，也不與其他實作重疊。即使trait是自己宣告的，也不能讓同一個具體型別同時符合兩份impl。

**blanket implementation** 是涵蓋一整類型別的泛型實作。例如 `impl<T> Label for T` 為每個T提供同一個Label；它已涵蓋Meters。再寫 `impl Label for Meters`，不是以更精確版本覆寫前者，而是兩份都符合Meters，所以收到 `E0119`：

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/newtype_overlapping_impls.rs"
```

若不同型別需要不同標籤，就別先承諾「每個T都走相同Label」；按工作需要為各個型別提供互不重疊的實作。orphan處理crate間可提供實作的責任，重疊檢查處理哪份impl適用的唯一性，兩項都要成立。

## 練習：等待時間也有自己的報告介面

仿照MeterBatch建立私有Vec欄位的TimeBatch，內容必須是本節Seconds。提供from_values接手集合，total借讀數字並交回Seconds；Display顯示 `2 段等待，共 90 秒`，使用30秒與60秒的資料。不要把結果交給distance_value：這項函式需要Meters，單位邊界仍要保留。

先自行實作與執行，確認Display後仍能借讀total；再對照 [完整參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch08-smart-pointers/examples/newtype_solution.rs)，或執行 `cargo run -p ch08-smart-pointers --example newtype_solution --locked`。

library測試檢查同單位與跨單位結果、空集合與兩段路程的報告，以及owned文字能在generic wrapper交接後取回。故意失敗範例則確認單位混用、move後使用、反向未實作加法、外部alias實作、缺少method、泛型orphan與impl重疊各自被編譯器攔住。

延伸閱讀：[英文Book進階traits](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html)、[英文Book進階型別](https://doc.rust-lang.org/book/ch20-03-advanced-types.html)、[簡體對應](https://kaisery.github.io/trpl-zh-cn/ch20-03-advanced-types.html)、[Reference coherence／orphan](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence)、[Add官方契約](https://doc.rust-lang.org/std/ops/trait.Add.html)。
