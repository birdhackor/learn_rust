# 11.5 型別內的借用與 'static

Box 擁有它保存的值，但那個值仍可能是一份 reference。`Box<&str>` 擁有借用資訊，沒有因此取得原文字。第 7 章已能描述 reference 的來源，現在要問：泛型 T 內部的借用，是否能支持指定的有效範圍？

## `T: 'a` 約束型別內的借用

第 7.2 節的 `'b: 'a` 表示 `'b` 至少涵蓋 `'a`。**`T: 'a`** 則要求 T 所包含的 lifetime 關係至少滿足 `'a`；它沒有命令某個 T 值一定存活到哪裡，也不延長來源。

| T | 如何符合 `T: 'a` |
| --- | --- |
| i32、String | 沒有依賴外部資料的借用，不受短命來源限制。 |
| `&'b str` | 需要 `'b` 至少涵蓋 `'a`。 |
| `Box<&'b str>` | Box 裡仍有借用，一樣需要 `'b` 涵蓋 `'a`。 |

把第 7.3 節的 borrowed field 改成借任意 T。**`?Sized + 'a`** 用加號同時列出要求：允許 T 是 DST，且 T 內的 lifetime 要能符合 `'a`。欄位自己則是 `&'a T`：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/static_complete/lifetime_bounds/mod.rs:borrowed"
```

在這個特定定義中，`&'a T` 本來就要求 `T: 'a` 才合法，Rust 能推導這項 **implied bound**（隱含要求）。明寫 `+ 'a` 是為了看清關係；刪掉它不會使這份 struct 不合法。這種推導針對相應 lifetime 要求，不表示任意 trait bound 都能省略。

先只使用 Borrowed，不載入稍後的 static helper。完整設定與程式如下：

```toml
--8<-- "examples/intro-workspace/type-stages/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/type-stages/examples/type_bounds.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example type_bounds --locked
```

```text
T是String=Rust
T是str=Rust
T是借來的&str=Rust
owned Box仍借原文字=Rust
```

前三項分別借 String、DST str，以及一份 `&str` 值。nested 的欄位再次借用 reference，因此 `*nested.value` 取得可 Copy 的 shared reference。最後的 Box 也合法，因為原 text 仍有效；owned 外層並沒有消除內部借用。

## `&'static str` 描述被借資料的有效性

**`'static`** 是 Rust 預留的 lifetime 名稱，不需在尖括號宣告。寫在 reference 上時，表示被借資料能在程式剩餘的執行期間保持有效。

前面使用的文字 literal 本來就有 `&'static str` 型別。保存這份 reference 的局部變數結束，不會使 literal 文字跟著消失。

較長的有效性也能符合較短的要求。下面 choose_label 回傳 candidate，或固定 literal Rust；簽名依第 7.1 節的唯一輸入省略規則，仍把回傳契約連到 candidate。literal 足以提供這項較短的借用，不必把整個函式改成只能接受 static 來源。

以下是這個階段的完整程式，只包含 choose_label 與 main：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/static_reference.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example static_reference --locked
```

```text
reference變數結束，literal仍有效=Rust
一般借用契約也能回傳literal=Rust
```

第一行是把 reference 值交到外層，不是延長 inside 變數。反過來，局部 String 的借用不能只靠標註變成 static；以下完整負例執行 `cargo check` 得到 **E0597**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/static_from_local.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/static_from_local.expected"
```

## `T: 'static` 不要求這份值永遠存在

型別 bound **`T: 'static`** 要求 T 不依賴短命外部借用。String、i32、`&'static str` 都符合；借自局部 String 的 `&str` 則不能符合這項要求。

接收者可以持有符合條件的值，而不會因短命外部來源先結束就失效；也可以立即清理它。這是資料依賴的契約，不是延後 drop 的指令。

用一個只檢查這項契約的 helper：`require_static<T: 'static>(value: T) -> T` 接手後原樣交回，不 clone、不儲存、不延長值。下面完整例子中的 Message 擁有 String 與 i32，沒有借用欄位；它通過 bound，仍在普通內層 block 結束時執行第 11.3 節的 Drop：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/static_bound.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example static_bound --locked
```

```text
數字=80
literal=Rust
owned文字=Rust
收尾id=1，文字=Rust
block已結束
```

Message 在「block已結束」之前收尾，清楚區分通過 bound 與實際存活位置。自訂 Drop 不會令這種 owned 資料不再符合 static。

### owned 容器不能替內部借用通過 static 要求

下面把局部 String 的 `&str` 放進 Box，再交給 require_static，會得到 **E0597**。這次不是 Box 不能保存 reference，而是裡面的借用不能符合 static：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/boxed_borrow_not_static.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/boxed_borrow_not_static.expected"
```

第 7.3 節的 `TextView<'a>` 也可能由呼叫者 own，內部卻仍借文字。判斷 `T: 'static` 要看型別內的借用，不能只看目前是否擁有外層值。需要不依賴原 String 的內容，就交出 owned String；只需局部借用時，則保留實際來源關係。

## 練習與完整答案

建立 Rust 的 String，交給第 11.4 節的 Wrapper，再交給 require_static。透過 Wrapper 追加「 教學」，確認內容 `Rust 教學`。說明為何放 owned String 能通過，而改放 `text.as_str()` 會依賴原文字。

以下是本節完成後的完整 lifetime_bounds helper，包含三項測試：literal 分支、owned Box 取回文字、含借用的 T 在局部範圍合法。

```rust
--8<-- "examples/intro-workspace/type-stages/examples/static_complete/lifetime_bounds/mod.rs"
```

這是答案實際載入的完整 Wrapper；保留已教的兩項借用／owned 取出測試，不會載入其他章的 library：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/static_complete/views/mod.rs"
```

完整答案根檔案如下：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/static_complete/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example static_complete --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example static_complete --locked
```

最後輸出 `owned模型通過static bound=Rust 教學`，五項測試應通過。下一節把具體型別的共同能力借成一份介面，保留這裡已分清的來源依賴。

[下一節：用 dyn 保存共同能力](11-06-lesson.md)
