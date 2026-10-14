# 8.3 用 Deref 借用包在裡面的值

8.1已經能用 `&*boxed` 借出Box裡的值。現在把問題換成我們自己的資料模型：有一個struct保存String，而既有函式只接受 `&str`。我們不想交出String，也不想為了呼叫函式複製文字；希望能從外層模型借到裡面的內容。

直接讀欄位並呼叫 `as_str()` 已經能完成這件事。這一節再學一種介面：讓外層型別告訴Rust「借用我的時候，可以接著借到哪一種內容」。它可以讓既有函式和method使用同一份資料，而不必在每個呼叫處重複寫出欄位路徑。

## 先分清楚外層值與借出的目標

本節的 `Wrapper<T>` 擁有一個 `value: T` 欄位。它只是一般struct，沒有自己把T另外配置到heap；若T是String，文字的heap儲存仍由String管理。Wrapper也沒有自訂Drop，所以之後仍能像6.2的Holder一樣，接手外層後交出欄位。

```rust
--8<-- "examples/ch08-smart-pointers/src/views.rs:wrapper"
```

**`Deref` trait** 位於 `std::ops`，名稱來自dereference。它讓型別提供「從shared borrow借到內容」的介面。本節先用以下契約理解它，再看實作：

| 介面 | 在本節的工作 |
| --- | --- |
| associated type `Target` | 指定借出的內容型別。一個Deref實作選定一個Target。 |
| `deref(&self) -> &Self::Target` | 借用外層，回傳指向Target的shared reference，不取得外層或內容的ownership。 |

`Self::Target` 的角色和6.2的 `Self::Output` 相同，都是這個trait實作選定的associated type。標準Deref允許Target是 `?Sized`，因此Target也能選擇8.1的DST，例如str或slice；不是所有Target都必須有固定型別大小。

回傳reference的有效性仍依4.5的receiver省略規則，接到這次 `&self` 借用。Deref不是替內容延長存活，也不是在method裡新建一份短命資料再把reference交出去。

對Wrapper，Target就是欄位的T，實作只借出 `self.value`。先用 `use std::ops::Deref;` 取得trait名稱；下面這一段也以 `impl<T>` 宣告泛型參數，和6.2相同：

```rust
--8<-- "examples/ch08-smart-pointers/src/views.rs:deref"
```

這個body沒有clone或move欄位。`&self.value` 找到原欄位，建立shared borrow；如果原欄位是String，仍由原Wrapper持有那份String。

## 星號語法連到借用介面

有了Deref，對Wrapper使用 `*wrapped` 時，Rust能透過Deref借到Target，再對這個reference做dereference。理解shared讀取時，可以把它連到 `*wrapped.deref()`：先取得 `&T`，再存取它指向的T。這是理解操作的對應，不表示每個星號都會取得owned T。

所以 `&*wrapped` 借到欄位，而讀取 `*wrapped` 中的i32可以得到Copy數字。若T不是Copy，能否取得ownership仍是另一個問題，後面會用String直接檢查。

要允許mutable借用，再實作 **`DerefMut` trait**。它也在 `std::ops`；實作它的型別必須已經有Deref，沿用同一個Target，不另選一個不同的內容型別。它的method是 `deref_mut(&mut self) -> &mut Self::Target`，把對外層的mutable借用連到欄位的mutable借用：

```rust
--8<-- "examples/ch08-smart-pointers/src/views.rs:deref-mut"
```

這沒有取消2.4的規則：呼叫需要mutable的外層值，取得的mutable reference不能與同時使用的shared borrow衝突。回傳reference依然借自receiver。

下面把直接method呼叫與星號借用放在一起。`deref()`／`deref_mut()` 是trait method，所以呼叫處也 `use` 對應trait，沿用4.1的method名稱可用範圍。Wrapper則從本章library的views module引入：

```rust
--8<-- "examples/ch08-smart-pointers/examples/deref_views.rs"
```

```bash
cargo run -p ch08-smart-pointers --example deref_views --locked
```

```text
deref借到的數字=80
*語法借到的數字=80
修改後的原欄位=100
```

前兩項都讀同一個欄位。後面的mutable reference加上20，最後直接讀欄位得到100，說明修改留在原資料上。這個關係來自實作回傳 `&mut self.value`，沒有另建一份數值容器再修改它。

## 函式要借內容，Rust可以接著轉換reference

**deref coercion** 是Rust在需要某種reference型別的位置，透過Deref把reference轉成借用內容的形式。這裡coercion的意思是依型別要求進行轉換；它不代表複製內容或交出ownership。

例如函式參數要求 `&str`，呼叫時給 `&Wrapper<String>`：Wrapper的Deref先提供 `&String`，String本身也有 `Deref<Target = str>`，因此能繼續借成 `&str`。前文的 `String::as_str()` 是顯式借文字的方法；這裡則把相同借用需要交給型別轉換規則處理。

下圖每個向下箭頭都表示繼續借用裡面的內容。它們不是move、clone，也沒有逐層新建文字。

![呼叫text_bytes需要str的shared reference，Wrapper的Deref借到String，String的Deref再借到原文字的str](../assets/ch08/deref-coercion.svg)

同一規則也能用在清單：`Vec<T>` 有 `Deref<Target = [T]>`，因此 `&Wrapper<Vec<i32>>` 能借成 `&[i32]`。這回到2.6的slice函式；函式只需要讀這段資料，不需要知道外面經過哪幾層owned模型。

mutable形式透過DerefMut：需要 `&mut T` 的參數能由 `&mut Wrapper<T>` 提供。mutable借用也可以依需要提供shared內容；shared借用則不能憑空變成mutable內容。這和2.4的借用權限方向一致。

下面三個helper都只做前文已教的工作：text_bytes取得byte長度，item_count取得slice長度，add_one透過mutable reference加1。主要變化在呼叫時給出的reference：

```rust
--8<-- "examples/ch08-smart-pointers/examples/coercions.rs"
```

```bash
cargo run -p ch08-smart-pointers --example coercions --locked
```

```text
顯式借出文字=5
自動借用轉換=5
文字仍由wrapper持有=Rust!
借來的清單長度=2
透過mutable借用修改=101
```

第一個呼叫明寫 `wrapped.value.as_str()`，第二個呼叫寫 `&wrapped`；兩者借同一份Rust!，所以byte長度都是5，而且呼叫後仍能從原欄位讀文字。清單同樣只借出兩個項目的slice。最後的number先透過星號加20，再經mutable參數加1，原欄位由80變成101。

**method查找也會沿著Deref尋找可用的method，並依receiver需要借用。** 因此對 `Wrapper<String>` 寫 `wrapped.as_str()`，可以找到String的既有method；對mutable的Wrapper寫 `wrapped.push_str(" 教學")`，則能透過DerefMut修改String。這兩項操作仍使用3.4已教的String API，只是接收者外面多了一層Wrapper。

這種安排適合外層確實要提供「像內容一樣借用」的介面。若外層代表不同的業務概念，只暴露幾項具名method可能更容易理解。加入Deref會讓內容method也能被查找到，因此它是介面選擇，不是所有單欄位struct都必須加入的慣例。

## 借用式轉換不會交出owned內容

8.1的Box能用星號move出它擁有的非Copy值，那是Box有特別支援的操作。**一般自訂Deref只提供reference，不因此得到相同的move-out能力。** 對 `Wrapper<String>` 用 `let taken = *wrapped;`，仍是想從借來的位置move出非Copy的String。

下面是完整的故意失敗fixture，錯誤碼為 `E0507`。它沒有實作Drop；失敗原因是Deref給的是借用，不是8.2的「有Drop不能直接move欄位」限制。

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/wrapper_move_out.rs"
```

若需要交出String，就另外提供owned介面。`into_inner(self) -> T` 接手整份Wrapper，再move出value。和6.2的owned Extract相同；這裡Wrapper沒有自訂Drop，所以可以直接交出欄位：

```rust
--8<-- "examples/ch08-smart-pointers/src/views.rs:into-inner"
```

呼叫後非Copy的Wrapper不再由呼叫者持有，但拿到的T已是owned結果，能獨立交接。borrowed介面與owned介面各有自己的工作，不需要讓星號同時承擔兩種責任。

### Target的trait也不會自動變成外層的trait

4.2的Copy是型別自己的承諾。即使Wrapper的Target是i32，Wrapper也不會因為有Deref就自動實作Copy。下面的 `copy_value<T: Copy>` 要求傳入的owned型別本身符合Copy；傳入Wrapper會收到 `E0277`：

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/wrapper_copy_bound.rs"
```

如果只需要一份數字，可以讀取 `*wrapped`，把Copy的i32交進函式。若希望Wrapper自己能Copy，則要另行決定並實作它的Copy契約。Deref支援借用式轉換與method查找，沒有讓所有Target的trait bound自動套用到外層。

## 練習：借用修改後，再交出文字

先把內容為Rust的String交進Wrapper。不要直接寫 `.value`：透過Wrapper呼叫已教的push_str，加入空白與「教學」，再透過as_str確認文字為 `Rust 教學`。最後用into_inner取得owned String，顯示 `取回owned文字=Rust 教學`。

先說明哪兩步只是借用，哪一步接手整份Wrapper；完成後可對照 [完整參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch08-smart-pointers/examples/deref_solution.rs)，或執行 `cargo run -p ch08-smart-pointers --example deref_solution --locked`。

`cargo test -p ch08-smart-pointers --locked` 還檢查mutable借用轉換是否修改原欄位，以及借出文字後仍能取回owned String並追加內容。這些結果分別支持「借用連到原內容」與「借用沒有拿走ownership」，不把輸出正常解讀為所有自訂Deref都會有相同的設計。

延伸閱讀：[Rust Book15.2](https://doc.rust-lang.org/book/ch15-02-deref.html)、[繁體Book15.2](https://rust-lang.tw/book-tw/ch15-02-deref.html)、[Deref官方介面](https://doc.rust-lang.org/std/ops/trait.Deref.html)。
