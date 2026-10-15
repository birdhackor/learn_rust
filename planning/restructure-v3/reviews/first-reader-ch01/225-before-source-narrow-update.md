# 13.2 thread、join 與 panic payload

上一節只檢查跨 thread 的型別能力，還沒有建立工作。現在要把 `[80, 120]` 的加總交給 worker（執行這份工作的 thread），主 thread 自己計算 `[1, 2, 3]`，最後取得兩份小計，合併成206。除了正常答案，我們還要分清：工作回傳自己的 `Err`，與工作發生 panic，分別從哪一層得知。

本節各程式沿用同一個 no-lib package，以下是完整 member 設定；共同欄位從第 12.1 節已介紹的 repo 根 workspace 採用。所有命令都從 repo 根目錄執行：

```toml
[package]
name = "concurrency-stages"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

```

## 建立工作，保留取得成果的 handle

`std::thread::spawn` 接收一份不需要呼叫參數的 closure，把它交到新 thread 執行，不等待這份工作完成就交回 `JoinHandle<T>`。T 是 closure 正常回傳的型別；handle 是一份 **等待這個 thread 結束並取得結果的 owned 權限**，不是已算好的 T。

以下只摘 API 簽名，沒有函式 body，不是獨立可執行程式。`where` 是11.6已教的 bound 寫法；F 代表工作 closure 的型別：

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

沿9.2與13.1的能力來讀：

- `FnOnce() -> T`：新 thread 呼叫這份工作一次，不傳引數，正常結束時得到 T；可以使用 Fn 或 FnMut 的 closure，因為它們也能符合 FnOnce。
- `F: Send + 'static`：整份工作要能移交，而且不能依賴短命外部借用。`move` 接手 owned Vec／String 可以滿足後一項；只捕捉一份局部 reference，則仍然受來源有效性限制。
- `T: Send + 'static`：成果也要能從 worker 交到等待它的 thread，不能夾帶短命借用。這是對回傳值的要求，與對 F 的要求分開。

符合 static 的 owned 值仍會正常清理，不是永久存活。這些是 spawn 的介面契約，即使呼叫端打算立刻等待，也不會取消 bound。

spawn 本身在作業系統無法建立 thread 時可能 panic。下面處理的是 **成功建立後** 的工作結果；不把 handle 當作建立失敗時也一定拿得到的值。

### join 消耗權限，等待工作結束

`handle.join()` 取得 handle 的 ownership，等待對應 thread 結束，再回傳 `thread::Result<T>`。若 thread 已結束，便不必繼續等；若尚未結束，呼叫 join 的 thread 會 **blocking（阻塞）**：暫停這次呼叫後面的工作，直到等待完成。

這個 Result 有兩種結果：正常回傳是 `Ok(T)`；工作因 Rust panic 結束、且採下面說明的 unwind 方式時，是 `Err(payload)`。payload 是 panic 攜帶的資料，沿用第 11.11 節隨事件交接內容的角色。

第 11.11 節的 Any 讓我們保存實際型別，之後用 downcast 檢查。這裡仍由 Box 持有資料，dyn Any 提供型別檢查介面；trait object 還可以加上第 13.1 節的 auto trait Send，要求內容能跨 thread 交回。加號再把 static 的借用限制寫出來，完整型別就是 `Box<dyn Any + Send + 'static>`。Any 配 Send 沒有指定 payload 一定是文字，稍後會處理這件事。

`thread::Result<T>` 是8.2的 type alias 加上泛型參數：固定錯誤型別，讓 T 隨工作成果改變。`std::result::Result` 是5.2所用 Result 的完整路徑。以下是標準函式庫 type alias 的寫法，供閱讀介面：

```rust
pub type Result<T> = std::result::Result<
    T,
    Box<dyn std::any::Any + Send + 'static>,
>;
```

本節沿用目前專案的 Cargo 設定，在 panic 時採 **unwind**：沿呼叫 stack 往回離開，清理遇到的局部值；工作 thread 的邊界取得 panic 資料，讓 join 回傳 Err。若改用 **abort**，panic 會直接終止整個 process，不能靠 join 取得這份 Err。這是讀取下面結果時必要的執行條件。

## 分工完成，再合併小計

先沿用已懂的 slice 加總。這裡的輸入採很小的 i32：80、120與1、2、3，總和都在 i32 範圍內。helper 不建立 thread，回傳的只是這份 slice 的小計：

```rust
pub fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    for value in values {
        total += *value;
    }
    total
}

```

完整程式用 `use std::thread` 取得 module 名稱，再寫 `thread::spawn`。本節仍使用上一節的 no-lib package 與完整 manifest。這個 target 用 `pub mod workers;` 載入自己的 workers/mod.rs，裡面只有上面的 sum，沒有 panic helper 或其他節的模組。完整根程式：

```rust
pub mod workers;

use crate::workers::sum;
use std::thread;

fn main() {
    let values = vec![80, 120];
    let handle = thread::spawn(move || sum(values.as_slice()));

    let local_total = sum(&[1, 2, 3]);
    println!("主thread小計={local_total}");

    match handle.join() {
        Ok(worker_total) => {
            let total = local_total + worker_total;
            assert_eq!(worker_total, 200);
            assert_eq!(total, 206);
            println!("工作小計={worker_total}，總和={total}");
        }
        Err(_) => println!("工作panic，沒有完整總和"),
    }
}

```

```bash
cargo run -p concurrency-stages --example thread_join --locked
```

```text
主thread小計=6
工作小計=200，總和=206
```

worker 接手 values 的 ownership，借用自己的 Vec 取得 slice，再得到200。主 thread 的陣列是另一份資料，算出6；它不需要讀取已移走的 values。join 的 Ok 分支才把 worker 的小計交到這裡，讓我們合併答案。Err 分支只報告未完成總和，沒有冒用一個不存在的200。

下圖是兩條路徑的關係示意，箭頭表示流程或成果交接；縱向距離不是量測時間。右邊可以先完成，也可以在左邊計算後才開始，**唯一用來取得完整成果的等待，是 join**：

![主thread與worker各自計算小計，join等待worker完成後取得200，再合併成206](../assets/ch10/thread-join.svg)

先看左邊：spawn 之後，主 thread 還可以計算自己的6，接著到 join。再看右邊：工作計算200並結束；join 等到這條 thread 的工作及相關清理結束，左邊才繼續使用回傳的小計。這使等待之後的流程有明確先後關係。

這個小例子讓 ownership 的移交與成果交回可見，沒有量測加速。只有一份很短的加總，建立 thread 的成本也可能比工作大；兩條路徑能各自推進，不代表一定同時在兩個處理器上跑。

若把 join 移到主 thread 計算6之前，主 thread 會先等待 worker，再做自己的加總。206不變，但原本放在等待之前的獨立工作，現在改到等待之後。**join 放哪裡，是在選擇哪個位置需要已完成的成果。**

### ownership 錯誤與成果能力，仍然在編譯期檢查

下面是完整的故意編譯失敗 fixture。closure 只讀長度，預設會借用局部 values；即使下一步立刻 join，spawn 要求的 static 仍不成立：

```rust
use std::thread;

fn main() {
    let values = vec![80, 120];
    let handle = thread::spawn(|| values.len());
    match handle.join() {
        Ok(length) => println!("長度={length}"),
        Err(_) => println!("工作panic"),
    }
}

```

實測診斷比對字串：

```text
E0373

```

編譯器以 `E0373` 指出 closure 可能比局部來源活得久。改用 move 接手 Vec，可以修這個問題；接手之後，原本的名稱就不能繼續讀取同一份 owned 值：

```rust
use std::thread;

fn main() {
    let values = vec![80, 120];
    let handle = thread::spawn(move || values.len());
    println!("原數列長度={}", values.len());
    match handle.join() {
        Ok(length) => println!("工作回傳長度={length}"),
        Err(_) => println!("工作panic"),
    }
}

```

實測診斷比對字串：

```text
E0382

```

這次是 `E0382`：values 已被移走。兩份 fixture 分別檢查「工作依賴局部借用」與「呼叫端再用已移走的值」，不是同一個錯誤。

回傳值的 Send 也會單獨檢查。下面的 closure 沒有捕捉任何 Rc，卻想在 worker 裡建立並回傳 Rc：

```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let handle = thread::spawn(|| Rc::new(80));
    match handle.join() {
        Ok(value) => println!("值={}", *value),
        Err(_) => println!("工作panic"),
    }
}

```

實測診斷比對字串：

```text
E0277

```

`E0277` 缺的是成果 T 的 Send。13.1已教 Rc 的單 thread 契約：在 worker 裡建立它，不會讓這份 Rc 成為可跨 thread 交回的成果。

## 兩層 Result 回答兩個不同問題

現在把工作改成「檢查一個金額」。沿第 4.1 節用 enum 表達可預期的拒絕：負數回傳 `Err(WorkError::Negative(...))`，零或正數回傳 `Ok(amount)`。這裡只檢查單一 i32，不做加總：

```rust
#[derive(Debug, PartialEq)]
pub enum WorkError {
    Negative(i32),
}

pub fn check_amount(amount: i32) -> Result<i32, WorkError> {
    if amount < 0 {
        Err(WorkError::Negative(amount))
    } else {
        Ok(amount)
    }
}

```

check_amount 正常回傳的 T 就是 `Result<i32, WorkError>`。兩邊的值都由 i32 組成，符合 Send 與 static。join 外面還有一層 `thread::Result<T>`，所以要先問「工作是否正常回傳」，再問「回傳的金額判定是否接受」。

```rust
pub mod workers;

use crate::workers::WorkError;
use crate::workers::check_amount;
use std::thread;

fn main() {
    for amount in [200, -1] {
        let handle = thread::spawn(move || check_amount(amount));
        match handle.join() {
            Ok(work_result) => match work_result {
                Ok(value) => println!("工作正常回傳，接受金額={value}"),
                Err(WorkError::Negative(value)) => {
                    println!("工作正常回傳，拒絕負金額={value}");
                }
            },
            Err(_) => println!("工作panic，沒有金額判定"),
        }
    }
}

```

```bash
cargo run -p concurrency-stages --example thread_result --locked
```

```text
工作正常回傳，接受金額=200
工作正常回傳，拒絕負金額=-1
```

每次迴圈建立一份工作，等待它，再處理下一個金額，因此這份程式的兩行順序確定。200得到外層 Ok、內層 Ok；-1得到 **外層 Ok、內層 Err**。第二份工作完成了自己的檢查，沒有 panic。

把外層 Ok 當作「業務必定成功」，就會漏掉負金額；把內層 Err 當作「thread 崩潰」，則會把可預期的拒絕誤判為 panic。match 保留這兩個界線，沒有用一個 unwrap 把兩種失敗混在一起。

## 取出 panic 攜帶的資料

第 8.1 節的 `panic!` 會離開正常回傳路徑。一般 `panic!("訊息")` 的文字 payload 可能是 String，也可能是 `&'static str`；不應依某次執行剛好出現的型別，只處理其中一種。

我們沿11.11的 owned downcast，先試 String，再試 static 文字 reference。`Box<dyn Any + Send>` 也有 downcast，使用方式相同：成功交回 `Box<T>`，失敗交回 **原來的 Box，仍帶 Send 能力**，可以接著再試下一個型別。

這份 workers/mod.rs 先用 `use std::any::Any;` 匯入 Any 名稱。下面的 helper 接手整份 payload，回傳 owned String。String 成功時用 Box 已教的 `*text` 取出 owned 值；static 文字成功時，`*text` 得到 reference，push_str 把文字存進新的 String。若不是這兩種型別，只回報「非文字panic payload」，不假裝已讀到未知型別的內容。

```rust
pub fn panic_message(payload: Box<dyn Any + Send + 'static>) -> String {
    match payload.downcast::<String>() {
        Ok(text) => *text,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(text) => {
                let mut message = String::new();
                message.push_str(*text);
                message
            }
            Err(_) => {
                let mut message = String::new();
                message.push_str("非文字panic payload");
                message
            }
        },
    }
}
```

我們故意讓一份宣告回傳 i32 的工作發生 panic；它會離開 body，沒有正常的 i32 成果：

```rust
pub mod workers;

use crate::workers::panic_message;
use std::thread;

fn main() {
    let handle = thread::spawn(|| -> i32 {
        panic!("工作故意panic");
    });

    match handle.join() {
        Ok(_) => panic!("這個範例應收到工作panic"),
        Err(payload) => {
            let message = panic_message(payload);
            assert_eq!(message.as_str(), "工作故意panic");
            println!("主thread收到panic：{message}");
        }
    }
    println!("主thread完成收尾");
}

```

```bash
cargo run -p concurrency-stages --example panic_worker --locked
```

主 thread 印出的兩行是：

```text
主thread收到panic：工作故意panic
主thread完成收尾
```

執行時還會看到標準 panic 診斷，包含發生位置。標準函式庫預設在 panic 發生時就把診斷寫到 **stderr（標準錯誤輸出）**；上面 main 的 println 則寫到 **stdout（標準輸出）**。join 收到 Err 不會回頭消除先前的診斷。這份程式處理了 worker panic，main 正常結束；不能光看到診斷就斷言 main 也發生未處理 panic。

為了不讓一次 panic 的實際型別替兩種文字契約代答，完整測試直接建立 String、static 文字與 u32 的 payload，分別核對文字與未知型別的報告。

這裡需要明選數字的型別。數字 literal 後可接型別名稱，例如 `7_u32` 是型別為 u32 的數字7；中間的底線可省略，`7u32` 同樣有效。這是 literal 的型別後綴，與1.4在變數名稱後寫 `: u32` 的註記位置不同。

```rust
use std::any::Any;

pub fn panic_message(payload: Box<dyn Any + Send + 'static>) -> String {
    match payload.downcast::<String>() {
        Ok(text) => *text,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(text) => {
                let mut message = String::new();
                message.push_str(*text);
                message
            }
            Err(_) => {
                let mut message = String::new();
                message.push_str("非文字panic payload");
                message
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::workers::panic_message;

    #[test]
    fn accepts_both_text_types_and_reports_other_types() {
        let mut owned = String::new();
        owned.push_str("owned文字");
        let message = panic_message(Box::new(owned));
        assert_eq!(message.as_str(), "owned文字");
        let message = panic_message(Box::new("literal文字"));
        assert_eq!(message.as_str(), "literal文字");
        let message = panic_message(Box::new(7_u32));
        assert_eq!(message.as_str(), "非文字panic payload");
    }
}

```

這份完整 workers 快照包含 helper 與同檔測試。它只由 panic_worker target 載入；沒有章節共用 library。`cargo test --example panic_worker` 選這份 example 的測試建置，會執行其 module 中的 `#[test]`，不執行一般 main。

執行 `cargo test -p concurrency-stages --example panic_worker --locked`。Box::new 在傳入 helper 時，依已教的 trait object 轉換保留各自的實際型別；測試不建立 thread，檢查的是 payload 的處理契約。真正的 thread panic 則由上面的完整範例核對。

## 丟棄 handle，不會替你等待

JoinHandle 沒有 Clone；等待權限只有一份。join 用掉它後，不能再等一次：

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| 200);
    match handle.join() {
        Ok(total) => println!("第一次小計={total}"),
        Err(_) => println!("工作panic"),
    }
    match handle.join() {
        Ok(total) => println!("第二次小計={total}"),
        Err(_) => println!("工作panic"),
    }
}

```

實測診斷比對字串：

```text
E0382

```

這份完整失敗 fixture 得到 `E0382`。第一個 join 接手 handle，第二次呼叫再用已移走的值。

11.3的 `drop(handle)`，或讓 handle 離開 scope，則會 **detach（脫離等待）**：丟掉這份等待權限，不等待工作完成，也不要求工作停止。這個 process 還活著時，worker 可以繼續執行；main 正常結束、process 結束時，尚未完成的工作也可能來不及完成。不能用「handle 已清理」推得「工作已完成」。

若需要完整成果，保留 handle 並 join；若建立兩份工作，也要安排兩份的等待。單純延後 main、猜 worker 大概已結束，沒有替我們建立這個保證。本節的可執行範例因此都保留並等待自己的工作。

## 練習：兩份工作都要收尾

把第一份數列 `[80, 120]` 與第二份 `[1, 2, 3]` 分別交給兩個 worker，主 thread 等待兩份成果後印出206。兩份工作都使用本節已教的 sum，資料仍採上述的小數值。

1. 先建立兩個 owned Vec，分別用 move closure 接手；保留兩個 handle。
2. 先把兩次 join 的結果各存進變數，兩個 thread 都等過之後，再用 tuple 與 match 判斷。
3. 只有兩份結果都是 Ok，才合併數字；至少一份 panic 就報告沒有完整總和。不要因第一份 Err 就提前跳出，留下第二個 handle 被丟棄。

你應能說明：兩個 spawn 都在第一次 join 之前，但兩個 worker 先後完成仍由排程決定；先等第一份，不代表第二份一定比較晚完成。一般執行不應用 worker 印出的行序證明這件事。

??? example "完整參考答案"

    完整的 target-local `workers/mod.rs`：

    ```rust
    pub fn sum(values: &[i32]) -> i32 {
        let mut total = 0;
        for value in values {
            total += *value;
        }
        total
    }
    
    ```

    ```rust
    pub mod workers;
    
    use crate::workers::sum;
    use std::thread;
    
    fn main() {
        let first = vec![80, 120];
        let second = vec![1, 2, 3];
        let first_handle = thread::spawn(move || sum(first.as_slice()));
        let second_handle = thread::spawn(move || sum(second.as_slice()));
    
        let first_result = first_handle.join();
        let second_result = second_handle.join();
    
        match (first_result, second_result) {
            (Ok(first_total), Ok(second_total)) => {
                let total = first_total + second_total;
                assert_eq!(first_total, 200);
                assert_eq!(second_total, 6);
                assert_eq!(total, 206);
                println!("兩份工作都正常回傳，總和={total}");
            }
            _ => println!("兩份工作都已等待，至少一份panic，沒有完整總和"),
        }
    }
    
    ```

    執行 `cargo run -p concurrency-stages --example thread_join_answer --locked`，輸出 `兩份工作都正常回傳，總和=206`。兩次 join 都在 match 之前完成，因此走失敗分支時也已等待兩個 thread；沒有把一份成功的小計冒充完整答案。

## 來源與延伸閱讀

- [Rust Book 16.1：threads、join、move](https://doc.rust-lang.org/book/ch16-01-threads.html)，[正體中文對應章節](https://rust-lang.tw/book-tw/ch16-01-threads.html)。
- [`thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html)、[`JoinHandle::join`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join)、[`thread::Result`](https://doc.rust-lang.org/std/thread/type.Result.html)。
- [`panic!` 的文字 payload 與預設診斷](https://doc.rust-lang.org/std/macro.panic.html)、[`Any` 的 owned downcast](https://doc.rust-lang.org/std/any/trait.Any.html)。
- [Cargo 的 panic 策略](https://doc.rust-lang.org/cargo/reference/profiles.html#panic)。

[下一節：HRTB 與當次借用](13-03-lesson.md)
