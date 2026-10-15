# 11.6 用 dyn 保存不同型別的共同能力

第 6.1 節的 Order 與 Bundle 都能提供 amount，但資料形狀不同。現在要把80元訂單與120元組合放進同一份費用清單，取得200；呼叫端再加入20元運費，既有加總函式也應得到220，不必為運費多寫一個分支。

## 先保留資料與共同操作

沿用非負小整數與不溢位的乘法，重新定義同款模型，讓本章例子獨立可跑：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_borrowed/dyn_charges/mod.rs:models"
```

實作相同 trait，不會把兩個型別變成一個。以下完整負例讓兩者都有 Charge，仍不能直接混進同一份 Vec，會得到 **E0308**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/mixed_charge_vec.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/mixed_charge_vec.expected"
```

本節負例都可放入獨立專案，執行 `cargo check`。enum 能包住兩種資料，讓每項都成為 KnownCharge；種類固定時這已能完成相同加總。這次額外需要呼叫端自行新增費用型別，不必回來修改 enum 的 variant 與 match，才引出新的介面。

## `&dyn Charge` 借讀共同能力

**trait object** 讓使用處保留 trait 的操作介面，而不在該位置寫出內部具體型別。型別寫成 **`dyn Charge`**；dyn 並沒有取消 Rust 的型別檢查。

dyn Charge 是第 11.1 節的 DST：只看介面，不知道原資料是多大的 Order 或 Bundle。因此透過 reference 或 owned pointer 使用。**`&dyn Charge`** 借讀原資料，不接手，也不需要先配置 Box。

需要 `&dyn Charge` 的地方，可以把符合 Charge 的 `&Order` 或 `&Bundle` 轉成這份視圖。這稱為 **unsizing coercion**：由具體型別的 pointer 轉成 DST 介面。它與第 11.4 節藉 Deref 逐層借出欄位不同，也沒有 clone 原資料；轉換時仍會檢查具體型別的 trait 實作。

下面比較泛型與 dyn 兩個 borrowed 介面。generic_amount 保留 T；dyn_amount 使用共同視圖，兩者都只交回 i32：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_borrowed/dyn_charges/mod.rs:amount-interfaces"
```

本節所有階段仍使用 no-lib package，完整設定如下：

```toml
--8<-- "examples/intro-workspace/type-stages/Cargo.toml"
```

這一步的完整 helper 只有模型與兩個讀取函式：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_borrowed/dyn_charges/mod.rs"
```

完整根檔案只以 `mod dyn_charges;` 載入這份快照：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_borrowed/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example dyn_borrowed --locked
```

輸出依序是泛型讀訂單80、dyn讀訂單80、dyn讀組合120、原訂單數量1。來源仍由呼叫者持有；借用視圖的使用期間，來源仍須有效。

## owned 清單：每項都是 `Box<dyn Charge>`

清單要接手資料，就使用 `Vec<Box<dyn Charge>>`。Box::new 仍先接手具體值；在需要共同型別的位置，`Box<Order>`、`Box<Bundle>` 可 unsize 成 `Box<dyn Charge>`，每份配置仍保存原具體資料。

本頁 Charge 沒有自身 lifetime bound。在函式簽名或 struct 欄位寫 **`Box<dyn Charge>`**，省略的 object lifetime 預設為 `'static`，相當於 `Box<dyn Charge + 'static>`。依第 11.5 節，這要求內容不依賴短命借用，不是要求 Box 永遠存在；本例只有 owned 數字，符合條件。

**dispatch** 是選擇對應 method 實作的過程。泛型在具體 T 已確定的版本中可以用 **static dispatch**，編譯時便知道實作；把泛型帶入具體型別、形成相應程式的過程稱 **monomorphization**。

trait object 提供 **dynamic dispatch**：handle 保存找到原資料與相應方法的資訊，方法入口由 **virtual method table（vtable）** 提供。Order 與 Bundle 的 handle 型別相同，但連到不同的資料及 amount 實作：

![兩個 Box dyn Charge handle 分別找到 Order 與 Bundle 資料，以及各自 amount 的方法入口](../assets/types-v3/dyn-dispatch.svg)

箭頭表示找到資料或方法，不是 clone、move，也不承諾精確位元組排列。

這一步的完整 helper 加入 KnownCharge enum 與 total。total 只借 slice，以 while 逐項呼叫 amount；method 查找經 Box 的 Deref 連到 dyn 介面：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_list/dyn_charges/mod.rs"
```

完整 caller 先用 enum 算同一批80+120，再用 dyn 算一次。Shipping 由 caller 自行定義、實作 Charge，只提供 fee；total 沒有新增分支：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_list/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example dyn_list --locked
```

```text
已知enum種類的總額=200
同一批資料用dyn的總額=200
使用者新增運費後總額=220
```

Vec 的元素型別始終只有一種 Box trait object。新 Shipping 能參與同一份加總，支持了本節的擴充需要；不是證明 enum 無法計算費用。dyn 有方法間接查找，Box 另有配置；這些輸出也沒有證明效能排名，編譯器在能確認目標時仍可能消除間接呼叫。

沒有 Charge 的型別仍不能加入。下面把 i32 放進 Box 會得到 **E0277**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_missing_trait.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_missing_trait.expected"
```

## 工廠回傳：具體型別、opaque 或 dyn

第 9.3 節的 RPIT 仍選定一種具體結果。以下完整負例雖然兩分支都有 Charge，Order 與 Bundle 仍不是同一個 opaque 型別，因此得到 **E0308**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/opaque_charge_branches.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/opaque_charge_branches.expected"
```

若確實需要兩種資料形狀，可讓兩分支各建立 Box，轉成相同的 `Box<dyn Charge>` 結果。make_bundle 直接交回 Bundle；make_opaque_bundle 固定選 Bundle、只公開 Charge；make_charge 則能在兩種資料間選擇。

同樣地，兩個 closure 表達式有不同型別。**`Box<dyn Fn(i32) -> i32>`** 保存共同的 Fn 呼叫介面，Box 中的工作可用 `work(value)` 呼叫。本例 closure 只按值保存 Copy 的 base，符合回傳 Box 預設的 static 要求。

以下完整 helper 只加入這些工廠，沒有稍後的 borrowed object 或 compatibility 型別：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_factories/dyn_charges/mod.rs"
```

完整 caller：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_factories/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example dyn_factories --locked
```

```text
具體回傳可讀price欄位=120
opaque介面提供amount=120
dyn工廠選訂單=80
dyn工廠選組合=120
兩種closure經dyn呼叫=20、40
```

泛型或參數 impl Trait 由 caller 提供具體型別；RPIT 由實作固定一個結果型別；enum 列出已知種類；dyn 保存不同具體型別的共同能力。選擇依資料種類、ownership 與公開操作，不是由語法長短決定。

## object 裡也可以保存借用

現在 BorrowedCharge 只借 prices slice，逐項加總，不複製清單。**`dyn Charge + 'a`** 要求被保存的具體型別內借用至少符合 `'a`；**`Box<dyn Charge + '_>`** 則用第 7.2 節的 placeholder，依唯一輸入來源推導回傳 bound。

這與完全省略 object bound 的 Box 預設不同。`impl Charge for BorrowedCharge<'_>` 也保留模型的相應來源，沒有固定成 static。

完整 helper 與 caller 如下，只有本步需要的 Charge、dyn_amount 與 borrowed 模型：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_lifetime/dyn_charges/mod.rs"
```

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_lifetime/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example dyn_lifetime --locked
```

輸出為模型借原資料200、dyn reference仍讀200、原資料項目數2。owned Box 沒有取消內容對 prices 的依賴。

若把回傳型別改回預設的 Box trait object，就要求 static。以下完整負例得到 `lifetime may not live long enough`：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_borrow_default_static.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_borrow_default_static.expected"
```

| 本頁 Charge 的型別位置 | 有效性關係 |
| --- | --- |
| `&'a dyn Charge` | 外層借用為 `'a`，object bound 由 reference 推導為 `'a`。 |
| `&'a (dyn Charge + 'static)` | 借用可以短，內部具體型別仍須符合 static。 |
| 簽名中的 `Box<dyn Charge>` | 內容預設 static；Box 值仍可很快結束。 |
| 回傳 `Box<dyn Charge + '_>` | 本例連到輸入 slice 的來源。 |

括號把 dyn 與它的 bound 收成被借型別。局部 expression 中的 object lifetime 也可能由使用推導；不能把簽名的 Box 規則推成所有省略位置都 static。

## dyn compatibility：隱藏具體 Self 後，介面仍須可用

**dyn compatibility** 表示 trait 符合建立 trait object 的規則；舊名是 **object safety**。Charge 的 shared receiver 提供原資料，回傳 i32 已確定，不需 caller 再知道具體 Self。

在 trait 名稱後寫 `: Sized` 是 **supertrait 要求**：實作者也必須符合 Sized。dyn 本身是 DST，因此整份 trait 若要求 Self:Sized，就不能成為 dyn。以下完整負例得到 **E0038**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_requires_sized.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_requires_sized.expected"
```

可動態呼叫的 method 也不能另有泛型型別參數：一份方法表不提供 caller 任選 T 的所有版本。下面同樣得到 **E0038**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_generic_method.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_generic_method.expected"
```

若 method 按值回傳未知 Self，dyn caller 也不知道具體結果表示，不能使用這份介面。以下完整負例得到 **E0038**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_returns_self.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_returns_self.expected"
```

需要保留這種 method 時，可以個別限定 **`where Self: Sized`**。**where clause** 把 bound 寫在簽名尾端、body 或分號前；這個 method 只供已知 Sized 的 Self 呼叫，不放進可動態呼叫介面。

RebuildCharge 用 `: Charge` 要求共同 amount；rebuild 個別排除 dyn 呼叫，trait object 仍能使用繼承的 amount。這與把整份 trait 限制成 Sized 不同。若硬要透過 dyn 呼叫 rebuild，下面負例會指出 rebuild 不能透過 trait object 呼叫：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_rebuild_call.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_rebuild_call.expected"
```

普通 associated type 則可以使用，但 object 型別要選定結果，例如 **`dyn Measure<Output = i32>`**。這沿用第 6.4 節的 equality bound，讓 measure 的結果型別明確。只寫 dyn Measure 的完整負例得到 **E0191**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_missing_output.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/dyn_missing_output.expected"
```

現在完整列出合法的 RebuildCharge、Measure、Bundle 及 caller；這個階段不載入先前的工廠：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_compatibility/dyn_charges/mod.rs"
```

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_compatibility/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example dyn_compatibility --locked
```

輸出具體型別可rebuild120、dyn仍可amount120、已選Output結果80。直接以未知 Self 當其他參數，或回傳 RPIT 的 method，也不能直接當可動態呼叫的方法；這些限制不表示它們不能用於一般具體型別。本節列的是當前需要的判斷，不是所有進階 trait 規則。

## 練習與完整答案

在 caller 定義 ExtraFee，amount 為40。把80的 Order、120的 Bundle 與它放進相同 Box trait object 清單，確認240；不改 KnownCharge 或 total。

此時才提供本節所有功能的完整 target-local module，包含四項測試。它不是全局 library，不會讓本節早期命令載入後文：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_complete/dyn_charges/mod.rs"
```

完整答案根檔案公開該 module，並定義自己的 ExtraFee：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/dyn_complete/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example dyn_complete --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example dyn_complete --locked
```

輸出 `新的費用種類一起加總=240`，四項測試分別核對外部費用／空清單、borrowed object、closure 工廠、associated Output。下一節用同一種 borrowed dyn 介面，沿錯誤的原原因形成報告。

[下一節：Error 與原因報告](11-07-lesson.md)
