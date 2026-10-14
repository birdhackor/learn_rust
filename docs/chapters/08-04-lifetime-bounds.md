# 8.4 型別內的借用與 'static

Box擁有它保存的T，但T本身仍可能保存reference。例如 `Box<&str>` 擁有一份reference值，並沒有因此取得reference所指文字的ownership。外層換成heap儲存，也不會讓原文字活得更久。

4.4／4.5已經能描述reference來源；現在還要判斷：**一個泛型T裡面的借用，能不能滿足所要求的有效範圍？** 這是type lifetime bound的工作。本節也會把容易混淆的 `&'static str` 與 `T: 'static` 分開。

## T: 'a 限制T裡面的借用

4.4的 `'b: 'a` 是兩個lifetime之間的bound，表示 `'b` 至少涵蓋 `'a`。新的 **`T: 'a`** 則把限制放在型別T上：T內所包含的借用不能短於 `'a` 的要求。這不是命令某個T值一直存活到 `'a`，也不會替原資料延長存活。

例如：

| T | 什麼情況符合 `T: 'a` |
| --- | --- |
| i32 | 沒有借用欄位，不受外部資料lifetime限制。 |
| String | 擁有文字，沒有借來的文字欄位，不受外部文字lifetime限制。 |
| `&'b str` | 所借文字的 `'b` 至少涵蓋 `'a`。 |
| `Box<&'b str>` | Box仍保存借用；一樣需要 `'b` 涵蓋 `'a`。 |

最後一列正好回應開頭的問題。Box的owned儲存不會消除T裡面的借用。泛型型別能不能符合要求，要看實際T，而不能只看到外面有Box就決定。

把4.5的文字視圖擴成借用任意T的模型，命名為Borrowed。沿用7.6的bound組合符號 **`+`**，表示各項要求一起成立；`T: ?Sized + 'a` 一方面沿用8.1、允許T是DST，一方面要求T裡面的借用至少符合 `'a`。

`Borrowed<'a, T>` 的欄位是 `&'a T`。它只持有reference，原T仍有別的owner；讀取模型時，也要維持那份T及T所需的借用來源有效。

```rust
--8<-- "examples/ch08-smart-pointers/src/lifetime_bounds.rs:borrowed"
```

這裡把 `T: 'a` 明寫出來，是為了看清楚關係。在這個特定定義中，`&'a T` 本身就要求T符合 `'a`，Rust也能推導這項 **implied bound（隱含要求）**；所以移除 `+ 'a` 並不會使這份struct變成不合法。不要把所有省略的bound都看成缺漏，也不要以為其他trait要求都會一併自動推導。

下面在同一份Rust文字上建立三種借用。第一項的T是String，第二項的T是DST str，第三項的T本身是一份借來的 `&str`，因此欄位是對這份reference的另一次借用：

```rust
--8<-- "examples/ch08-smart-pointers/examples/type_bounds.rs"
```

```bash
cargo run -p ch08-smart-pointers --example type_bounds --locked
```

```text
T是String=Rust
T是str=Rust
T是借來的&str=Rust
```

三種情況都合法，因為讀取時需要的來源仍有效。nested的T包含borrow，也不表示所有含borrow的T都不能用；只要那份borrow涵蓋本次要求的範圍即可。`*nested.value` 取得的是可Copy的shared reference，沒有取得原String的ownership。

## &'static str 說的是被借資料的有效性

**`'static`** 是Rust預留的lifetime名稱，不需要像 `'a` 那樣在泛型參數裡宣告。當它寫在reference上，例如 **`&'static str`**，表示被借資料可以在程式剩餘的執行期間保持有效。

前文的文字literal，例如 `"Rust"`，就有 `&'static str` 型別。literal的資料不是某個局部String管理的內容；保存這個reference的局部變數結束，不會使literal文字跟著消失。reference值的scope，與它指向資料能有效多久，要分開看。

較長的有效性也能滿足較短的借用要求。下面的choose_label回傳candidate，或回傳固定literal Rust；簽名仍沿用4.4的單一reference來源省略規則，回傳契約接到candidate的借用。literal在else分支可以提供符合這項較短要求的文字：

```rust
--8<-- "examples/ch08-smart-pointers/src/lifetime_bounds.rs:choose-label"
```

下面先把內層reference值交到外層，再使用這個函式：

```rust
--8<-- "examples/ch08-smart-pointers/examples/static_reference.rs"
```

```bash
cargo run -p ch08-smart-pointers --example static_reference --locked
```

```text
reference變數結束，literal仍有效=Rust
一般借用契約也能回傳literal=Rust
```

第一項合法的原因是literal資料仍有效；不是inside變數被延長到外層。第二項則說明 `'static` reference能符合一般較短的借用契約，呼叫者不需要因為某個分支用literal，就把整個函式改成只能回傳 `'static`。

反過來，短命String的borrow不能只靠annotation變成 `'static`。下面故意把內層String的as_str結果宣告為 `&'static str`，會收到 `E0597`：

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/static_from_local.rs"
```

來源inside仍隨block結束；改寫reference型別沒有改變它的owner。需要獨立保存文字時，依4.4交出owned String；只是繼續借用時，保留實際有效的來源和範圍。

## T: 'static 不表示這份值永遠存活

把 `'static` 放在型別bound中，**`T: 'static`** 要求T裡面沒有必須依賴短命外部資料的borrow。String和i32符合這項要求，`&'static str` 也符合；借用局部String的 `&str` 則不能符合。

這是一項型別契約，與值何時被drop是兩件事。接收者可以把符合契約的值保留，不會因為某份短命外部借用先結束而失效；接收者也可以立即drop它，沒有義務保存到程式結束。

先用一個刻意只檢查契約的helper觀察這項區別。`require_static<T: 'static>(value: T) -> T` 接手值後原樣交回；bound讓編譯器檢查實際T，函式不延長存活、不clone，也沒有儲存值的工作。這不是建議為所有普通函式加上 `'static`，只是讓不同輸入能否符合bound變得明確：

```rust
--8<-- "examples/ch08-smart-pointers/src/lifetime_bounds.rs:static-bound"
```

下面的Message擁有String與數字id，沒有borrow欄位，因此符合 `'static` bound。Drop只沿用8.2的收尾輸出；我們仍讓這份值在普通內層block結束：

```rust
--8<-- "examples/ch08-smart-pointers/examples/static_bound.rs"
```

```bash
cargo run -p ch08-smart-pointers --example static_bound --locked
```

```text
數字=80
literal=Rust
owned文字=Rust
收尾id=1，文字=Rust
block已結束
```

Message通過bound，仍在block結束收尾。它管理的String不需要永遠留著；`'static` 在這裡約束資料依賴，而不是安排延後清理。自訂Drop也沒有讓這份型別不再符合 `'static`。

### 擁有容器，也可能仍借著外部資料

若改成Box保存局部String借出的 `&str`，外層Box是owned值，但T裡面的borrow仍依賴原String。下面完整fixture會在要求 `'static` 時收到 `E0597`：

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/boxed_borrow_not_static.rs"
```

這不表示 `Box<&str>` 本身非法。只在原String有效期間使用它，可以合法借讀；失敗的是本例要求它符合 `'static`。修正也要依需求：接收介面若能跟著原來源借用，就保留相應lifetime；若需要不依賴原String，就把owned String交進Box。

4.5的 `TextView<'a>` 也有同一個區別：我們可以own這份模型，但它內部仍借文字。說「owned值都符合 `'static`」會漏掉這種情況；需要檢查的始終是整個型別內的借用關係。

## 練習：保留owned模型，通過bound後仍正常使用

建立內容為Rust的String，把它交進8.3的Wrapper，再交給require_static。用Wrapper已有的借用介面追加 `" 教學"`，顯示 `owned模型通過static bound=Rust 教學`。不要改成把 `text.as_str()` 交進Wrapper；先說明兩種T在外部借用上的差別。

完成後可對照 [完整參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch08-smart-pointers/examples/static_solution.rs)，或執行 `cargo run -p ch08-smart-pointers --example static_solution --locked`。

library測試還檢查literal能滿足一般回傳借用、含借用的T能符合局部要求，以及Box擁有String後能通過 `'static` bound、再交出String修改。把這些操作和收尾輸出放在一起，就能分辨「型別不依賴短命borrow」與「某個值實際活多久」。

延伸閱讀：[Rust By Example：Static](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html)、[中文版對應章節](https://doc.rust-lang.org/rust-by-example/zh/scope/lifetime/static_lifetime.html)、[Rust Reference：lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html#lifetime-bounds)。
