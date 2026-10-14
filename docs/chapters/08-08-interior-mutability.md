# 8.8 用Cell與RefCell管理內部修改

上一節能讓不同持有者讀同一份資料。現在要讓幾個程式部分共同使用一個記錄器，各自記下80、120等數值，再讀回記錄。寫入輪流發生，但介面提供的是shared receiver，不讓某個持有者長期獨占整個模型。

先沿4.1定義Record trait：`record(&self, amount: i32)` 接收一筆數值、回傳unit。若Recorder直接保存普通Vec，push需要mutable borrow，不能從這份shared self取得。下面是故意收到 `E0596` 的完整fixture：

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/shared_record_vec.rs"
```

如果介面可以改成 `&mut self`，而呼叫者也能獨占借用整份模型，普通Vec就是清楚的選擇。本節保留的是shared receiver介面，並讓模型內部管理短暫的讀寫權限。

**interior mutability（內部可變性）** 是透過特定型別的安全API，在shared借用下修改其內部狀態。這不是讓所有 `&T` 都能任意修改T；容器要有自己的規則，限制如何讀寫。本節學兩種不同介面：Cell以交接／複製值操作，RefCell則提供受執行期檢查的借用。

## Cell以值操作，不交出長期借用

先把記錄縮成一個計數：共享呼叫者每次增加1。**`Cell<T>`** 位於 `std::cell`，保存一個T，讓shared receiver也能透過受控操作更新它；操作不交出讓呼叫者長期持有的 `&mut T`。

本節使用以下契約，先分清哪些取得副本、哪些轉交owned值：

| 操作 | 契約與結果 |
| --- | --- |
| `Cell::new(value)` | 接手T，回傳保存它的Cell。 |
| `cell.get()` | 在T是Copy時，回傳內部T的副本，不拿走原值。 |
| `cell.set(value)` | 接收新T、替換內部值，原值正常清理；回傳unit。receiver是 `&self`。 |
| `cell.replace(value)` | 接收新T、替換內部值，把舊T的ownership交回；不要求T是Copy。 |
| `cell.into_inner()` | 接手整個Cell，回傳其內部owned T，不再保留原Cell。 |

Counter的私有value欄位是 `Cell<i32>`。new接收起始數字；tick借用整個Counter，先get一份數字，算出下一個值，再set回去。使用小數字，不讓加法溢位：

```rust
--8<-- "examples/ch08-smart-pointers/src/cells.rs:counter"
```

下面的counter binding沒有mut，tick仍能依Cell介面更新計數。後半段用String檢查另一項區別：Cell本身能保存非Copy資料，只是不能用get複製它；replace與into_inner以owned交接完成工作。

```rust
--8<-- "examples/ch08-smart-pointers/examples/cell_values.rs"
```

```bash
cargo run -p ch08-smart-pointers --example cell_values --locked
```

```text
shared method第一次計數=1
shared method第二次計數=2
replace取回owned舊文字=Rust
接手Cell內的owned新文字=Rust 教學
```

前兩項從同一個Counter依序得到1、2。replace把舊String交回，Cell接手新String；最後into_inner再把新String交回。這些操作沒有讓兩個呼叫者同時持有內部資料的mutable reference。

若對 `Cell<String>` 呼叫get，便不符合Copy要求。下面fixture會收到 `E0599`，指出String沒有提供get需要的Copy能力；這不表示Cell只能保存Copy型別。

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/cell_string_get.rs"
```

## RefCell借出內容，也記住活躍借用

記錄器保存Vec時，我們需要借用清單、呼叫push，再把清單留在原模型裡。**`RefCell<T>`** 也位於 `std::cell`，擁有自己的T，同時保存目前內部borrow的狀態。它不是共享ownership容器；還沒有套Rc時，T仍由這份RefCell持有。

RefCell在執行期維持相同的讀寫互斥要求：可以同時有多個shared借用，或一個mutable借用，不能混在一起。這些borrow由容器的API建立、檢查與解除，不是把ordinary reference的編譯檢查全部關掉。

### 回傳的是guard，不是普通reference

**borrow guard** 是一個代表活躍借用的值；持有它時，RefCell記得這份借用仍在。guard正常清理時，才通知RefCell解除該借用。兩種guard先在這裡介紹，才使用對應method：

| guard型別 | 借用能力與內容存取 |
| --- | --- |
| `Ref<'a, T>` | shared guard。有 `Deref<Target = T>`，能讀內部T、使用其shared method。 |
| `RefMut<'a, T>` | mutable guard。有Deref與DerefMut，能透過可修改的guard借用內部T、呼叫其mutable method。 |

`'a` 仍是借自容器的有效性關係，不是「有guard就讓容器永遠存活」。這些guard本身是管理借用狀態的值，清理責任沿用8.2；借用內容與method查找沿用8.3。

以下方法的receiver都能使用shared `&self`，差別是容器替內容建立哪種借用：

| 操作 | 結果與失敗條件 |
| --- | --- |
| `RefCell::new(value)` | 接手T，建立尚無活躍borrow的RefCell。 |
| `cell.borrow()` | 回傳 `Ref<'_, T>`。可和其他shared guard共存；若已有mutable guard，這次呼叫panic。 |
| `cell.borrow_mut()` | 回傳 `RefMut<'_, T>`。需要沒有其他活躍guard；已有任何borrow就panic。 |

這裡panic是執行這項呼叫時中止正常流程，不是編譯器提前拒絕。後面會把正常、執行失敗與可處理的失敗放在一起觀察。

下面先同時建立兩個shared guard，再顯式drop它們，取得mutable guard追加120。`let mut writable` 讓guard能使用DerefMut；外面的RefCell binding仍沒有mut。

```rust
--8<-- "examples/ch08-smart-pointers/examples/refcell_guards.rs"
```

```bash
cargo run -p ch08-smart-pointers --example refcell_guards --locked
```

```text
兩份shared guard讀同一值=80、80
修改後有2項，總額=200
```

兩個shared guard都借同一Vec，因此讀到相同80。它們都結束後，mutable guard才取得修改權限。push把120加到原Vec，writer清理後再借shared guard讀回兩項；不是clone另一個Vec再修改它。

## 最後一次讀guard，不等於解除borrow

2.4的ordinary reference可以按最後使用判斷借用是否還需要。RefCell的執行期狀態則跟著**guard的清理**；8.2已說明最後使用不會自動把owned管理值立即drop。

所以讀完一個Ref，若它還留在scope裡，不能立刻再borrow_mut。下面是故意執行失敗的fixture：它能編譯，但在borrow_mut處panic，訊息包含 `already borrowed`。main沒有處理這項panic，因此程式非零結束。

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/runtime-fail/guard_used_then_conflict.rs"
```

最後的shared讀取雖已完成，view仍持有活躍guard。要結束這份借用，讓view離開它的scope，或用std::mem::drop交出view。這裡drop的是借用管理值，不是把RefCell內的i32清理掉。

### 用try方法把衝突當成Result處理

若借用衝突是工作流程可能遇到的狀態，不想讓那次取得借用直接panic，可以用以下方法：

| 操作 | 契約與結果 |
| --- | --- |
| `cell.try_borrow()` | 回傳 `Result<Ref<'_, T>, BorrowError>`。成功交回shared guard；與活躍mutable borrow衝突時回Err。 |
| `cell.try_borrow_mut()` | 回傳 `Result<RefMut<'_, T>, BorrowMutError>`。成功交回mutable guard；已有任何borrow時回Err。 |

BorrowError／BorrowMutError是標準函式庫表示這兩種取得失敗的型別；本節只沿3.2的Result作match，Err(_)代表不讀取其欄位。成功取得的guard仍依相同規則清理。

下圖的Err／Ok是這兩項方法的Result結果；向下箭頭表示本例依程式順序繼續，shared guard最後一次讀完仍活躍，drop後才解除這份borrow。

![guard建立後仍維持shared借用，println最後讀取後try_borrow_mut仍回Err，drop guard之後才可取得mutable guard](../assets/ch08/guard-lifetime.svg)

下面先在guard讀完但尚未drop時試一次，處理Err後drop原view，再試成功的mutable借用。之後反過來，在mutable guard仍活躍時試shared借用：

```rust
--8<-- "examples/ch08-smart-pointers/examples/try_borrows.rs"
```

```bash
cargo run -p ch08-smart-pointers --example try_borrows --locked
```

```text
shared guard最後讀取=80
讀完還沒drop，mutable借用被拒絕
drop shared guard後更新=100
mutable guard仍在，shared借用被拒絕
guard釋放後讀回=120
```

第一次try得到Err，沒有建立新的mutable guard，也沒有改動80。第二次成功的guard加20，離開match分支即清理；後面再借一個writer加20。writer活躍時shared取得被拒絕，drop它之後才讀到120。

## 回傳guard，把借用的責任交給呼叫者

回到記錄器。Recorder的私有values現在保存 `RefCell<Vec<i32>>`；Record的shared receiver保持原樣，record裡取得短暫mutable guard、push一項，函式結束便清理guard。

新增兩項使用介面：**`Recorder::empty()`** 建立空記錄；**`view(&self) -> Ref<'_, Vec<i32>>`** 回傳shared guard，讓caller能讀已有記錄。view的 `'_` 依receiver來源推導，所以caller必須維持Recorder有效，並負責在要繼續寫入前結束取得的guard。

```rust
--8<-- "examples/ch08-smart-pointers/src/cells.rs:recorder"
```

```rust
--8<-- "examples/ch08-smart-pointers/examples/recorder.rs"
```

```bash
cargo run -p ch08-smart-pointers --example recorder --locked
```

```text
shared介面記錄2項，總額=200
釋放view後新增，第3項=40
```

這完成開頭的記錄需要：shared method實際保存80、120，caller借讀兩項取得200；view清理後，record再追加40。view不是免費的snapshot副本，它維持原Vec的shared借用；若保留view又呼叫record，record中的borrow_mut就會衝突。

guard也不能讓函式內短命容器跨越結束。下面故意建立局部RefCell，再回傳它的Ref；即使寫出任意 `'a`，仍收到 `E0515`，和4.4不能借回局部String的原因相同：

```rust
--8<-- "examples/ch08-smart-pointers/fixtures/compile-fail/refcell_guard_escape.rs"
```

## Rc負責保活，RefCell負責讀寫權限

現在組成 **`Rc<RefCell<T>>`**。Rc的Deref找到RefCell，呼叫RefCell的borrow／borrow_mut時只需要shared receiver，因此不要求Rc提供DerefMut。外層Rc保證共享容器仍有效，內層RefCell檢查T目前可以怎樣被借。

這是兩份不同的狀態：Rc strong計數追蹤共享owner，RefCell追蹤活躍guard。clone一份Rc不表示取得mutable borrow；borrow一份guard也不表示clone一份Rc。

```rust
--8<-- "examples/ch08-smart-pointers/examples/shared_cells.rs"
```

```bash
cargo run -p ch08-smart-pointers --example shared_cells --locked
```

```text
另一個Rc看到同一清單=80、120
Rc strong數量=2
只剩second仍有2項
```

second取得writer、追加120，在內層block結束清理guard。first接著借到同一份Vec。兩個owner並存時strong為2，但這沒有允許兩個writer同時存在。最後只移除first的Rc，second還保活容器，仍能借讀兩項。

### 現在能建立不以strong互相保活的父子連結

8.7已用假設圖說明循環。現在有受控修改API，就能建立真實的雙向關係：Node的children保存子節點的Rc；parent欄位保存 `RefCell<Weak<Node>>`，讓建立父節點後，才能補上子節點回看父的Weak。

children在本例建立後不修改，所以使用普通Vec即可；只有parent欄位需要RefCell。父節點保活子節點，子節點找父時要upgrade，並不反過來保活父。Node的Drop仍只用8.2的id輸出觀察清理。

程式用短block中的mutable guard，把空Weak替換成downgrade結果；後來借shared guard再upgrade。這份guard的Deref連到Weak，所以能呼叫既有upgrade，成功取得的Rc也隨match分支結束。

```rust
--8<-- "examples/ch08-smart-pointers/examples/weak_parent.rs"
```

```bash
cargo run -p ch08-smart-pointers --example weak_parent --locked
```

```text
父節點有子節點=1
子節點找到父id=2
branch的strong=1
leaf的strong=2
清理節點id=2
父結束後leaf的strong=1
子節點的父已結束
清理節點id=1
```

temporary父Rc已結束後，branch只有自己的一份strong；leaf則有外層變數與branch.children共兩份。branch離開block時清理id2，再移除children持有的leaf Rc；leaf剩外層一份，parent的Weak仍存在，但upgrade已得到None。最後才清理id1，沒有靠父子的strong循環彼此保活。

Cell／RefCell提供的這些shared receiver修改方式，不是不同thread同時讀寫資料的同步工具。本節操作都在單一thread順序執行；跨thread的容器與保護規則，會在並行章節建立。

## 練習：兩個持有者共用記錄器

用Rc保存Recorder::empty，再clone給另一個持有者。第一份record80，第二份record120；透過第一份view，確認兩項與總額200。顯式drop view後，第二份再record40；重新view，確認三項與總額240。

先指出哪份Rc維持Recorder有效、哪個guard阻止同時寫入。完成後可對照 [完整參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch08-smart-pointers/examples/cells_solution.rs)，或執行 `cargo run -p ch08-smart-pointers --example cells_solution --locked`。

library測試另外檢查shared caller操作同一Counter、兩份shared guard必須都結束才能取得writer，以及記錄中的零值與順序在view釋放後仍保留。選Cell時注意以值交接的介面；選RefCell時注意guard的存活。這兩種型別提供不同的內部修改安排，使用哪個仍依讀寫需求。

延伸閱讀：[英文Book15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)、[中文Book15.5](https://kaisery.github.io/trpl-zh-cn/ch15-05-interior-mutability.html)、[RefCell官方契約](https://doc.rust-lang.org/std/cell/struct.RefCell.html)、[Cell官方契約](https://doc.rust-lang.org/std/cell/struct.Cell.html)。
