# 第9章獨立銜接與實頁初判

封存 2026-10-15T09:11:34.670064+00:00；起始來源 2026-10-15T08:59:27.927160+00:00。角色非作者、technical、firstreader。只用實際1–8前綴，9.1→9.7順讀，未讀10/11或他角結論。9.1首14行在snapshot前剛讀，已記scope；無已知9問題提示。

必要0，可選2。82檔含14相鄰expected、3圖與全targets/answers/manifests/default main、workspace metadata。末尾變動0筆；章稿、Rust、圖、member manifest皆穩定。

## 發現

### C9-01 — 選讀改善／可選

docs/chapters/09-04-lesson.md:107

> | 建立工作 | 借用或接手什麼 | Item | 結束後原 Vec |

390x844 table402px、scrollwrap client375/scroll434/max59；scroll0裁右側，scroll59失左方法標籤；1280正常。 橫捲增加同列比較負擔，但內容可捲讀，前後文字與程式足供主線理解，故非必要缺口。

首次需要：9.4同列比較方法與後果/回傳；實際前文：9.4此前已教Iterator/IntoIterator/ownership；9.6表前已教Entry借map接key及兩狀態。表後正文與完整程式均另解讀，必要語義無缺。

收益：手機同時核對建立API、Item與原Vec可用/改動/移出後果。最小方向：把借用/接手與結束後原Vec合欄，保留三條路徑及所有方法/Item/後果。

### C9-02 — 選讀改善／可選

docs/chapters/09-06-lesson.md:54

> | method | 有值時 | 缺鍵時 | 回傳 |

390x844 table402px、scrollwrap client375/scroll434/max59；scroll0裁右側，scroll59失左方法標籤；1280正常。 橫捲增加同列比較負擔，但內容可捲讀，前後文字與程式足供主線理解，故非必要缺口。

首次需要：9.6同列比較方法與後果/回傳；實際前文：9.4此前已教Iterator/IntoIterator/ownership；9.6表前已教Entry借map接key及兩狀態。表後正文與完整程式均另解讀，必要語義無缺。

收益：手機同時辨識Entry→Entry或&mut值，以及callback何時執行。最小方向：將有值/缺鍵合為有明標記的兩段，保留三method/兩狀態全部動作/回傳。

## 四題與首次必要

引文直接抽取frozen原始行，自己的解釋另列。Q2機制與需要逐項獨立判已足夠。

### closure與capture

首次必要：9.1 fee28前。

docs/chapters/09-01-lesson.md:3

> 第 1.6 節已把計算整理成函式。這次想先保存一個運費計算工作，之後交給不同訂單使用；免運門檻已經保存在旁邊的變數，不想在每次呼叫時再傳一次。

docs/chapters/09-01-lesson.md:5

> 普通函式也能完成計算，只要把門檻列為參數。本節要學的是另一種組合：把這段工作與它使用的周圍資料收在一起，形成一個可以保存、之後呼叫的值。

docs/chapters/09-01-lesson.md:9

> **closure**（閉包）是沒有另取函式名稱的函式形式，可以保存到變數中。它除了收到呼叫時的參數，還可以使用定義位置周圍的變數。這項保存所需資料或借用資訊的行為叫 **capture**（捕捉）；「環境」在這裡指那些外部變數，不是電腦的作業系統。

docs/chapters/09-01-lesson.md:23

> closure 用兩個直線符號包住參數，例如 **`|amount: i32|`**。後面可以接 `-> i32` 與 body，輸入、回傳值和 block 的規則沿用第 1.6 節。將它存進 `shipping_fee` 後，以 `shipping_fee(280)` 呼叫。

docs/chapters/09-01-lesson.md:25

> 這次規則是金額至少 300 時免運，其餘收 60。下面的 `amount` 是每次呼叫才提供的訂單金額；`free_limit` 則在建立 closure 的位置就已存在，body 從環境取得它。只有門檻這項資料需要 capture：

docs/chapters/09-01-lesson.md:40

> 280 未達門檻，所以取得 60；320 已達門檻，所以取得 0。建立 closure 時還沒有計算某筆訂單的運費；呼叫時才執行 body。這兩個時點分開，才能把準備好的工作先保存，再處理後來到達的金額。

- Q1：函式形式可保存值，capture周圍資料/借用。
- Q2機制/性質（已足夠）：建立保存工作與門檻；呼叫才提供amount計算。
- Q2需要/用途（已足夠）：來源3明定每次不想再傳已存在門檻；普通fn也能完成，改變工作保存的分工。
- Q3：直線參數/回傳與body；shipping_fee(280)，free_limit外部而amount是此次輸入。
- Q4：280/320跨門檻、300 assertion支持規則及兩時點，非性能證據。

### closure推導固定型別

首次必要：9.1 inference49/fixture57前。

docs/chapters/09-01-lesson.md:44

> 編譯器常能從 body 與呼叫位置推導 closure 的輸入、輸出型別。單一 expression body 可以略去大括號，像下面的 `|number| number * 2`；若有多個 statement，仍用 block。

docs/chapters/09-01-lesson.md:46

> `let result: i32` 讓這次計算的型別要求清楚可見。推導成功後，同一個 closure 的參數型別不會在每次呼叫時任意更換：

docs/chapters/09-01-lesson.md:54

> 這與第 5.1 節的泛型函式有別。下面先要求 identity 接收、回傳 `i32`，再想用同一個 identity 處理 `bool`，會得到 `E0308`：

docs/chapters/09-01-lesson.md:60

> 省略註記讓編譯器補出型別，不代表宣告了泛型參數。初學時可以先寫完整型別，看清輸入／輸出契約，再省略容易辨認的部分。

- Q1：接5.1推導，但省略型別非宣告泛型。
- Q2機制/性質（已足夠）：body/呼叫/接收變數提供線索；同closure輸入不每次任意改。
- Q2需要/用途（已足夠）：減可辨認註記仍保契約，防把省略當泛型。
- Q3：double省略型別，result:i32；identity先i32再bool失敗。
- Q4：8/12成功和E0308，未需推導演算法。

### shared/mutable capture

首次必要：9.1 shared69/mutable91/fixture109前。

docs/chapters/09-01-lesson.md:64

> 接著把材料換成已教的 `String`，以便分辨 owner 與借用。工作只需讀取 byte 長度，不需要接手整份文字。

docs/chapters/09-01-lesson.md:66

> 無參數的 closure 寫 **`||`**，這裡兩個直線之間沒有參數；它出現在 closure 的開頭，不是在兩個布林值之間做運算。下面 body 只使用 `as_str()` 和 `len()`，因此編譯器可以用 shared borrow capture `text`。closure 保存這份借用，原 `String` 仍由 `text` 管理：

docs/chapters/09-01-lesson.md:82

> 讀原文字和呼叫 closure 都只需要 shared borrow，可以同時保持這些讀取用途。這不是複製另一份 owned 文字；若原文字過早結束，借用它的 closure 也不能繼續使用，仍遵守第 3.5 節 reference 使用時來源必須有效的規則。

docs/chapters/09-01-lesson.md:86

> 現在換一項工作：每次呼叫，將原計數增加一，並交回新數字。body 需要修改 `count`，因此 capture 改成 mutable borrow。

docs/chapters/09-01-lesson.md:88

> `count` 要宣告為 `mut`，才允許修改原數字；保存工作的 `tick` 也要宣告為 `mut`，因為呼叫需要透過 mutable 借用操作它保存的環境。兩個 `mut` 分別作用在原資料與 closure 值，不能只留其中一個：

docs/chapters/09-01-lesson.md:104

> 最後一次使用 tick 之後，這份 mutable borrow 不再需要，於是可以直接讀 `count`。第 3.5 節的最後使用點規則沒有因 capture 而消失。

docs/chapters/09-01-lesson.md:106

> 如果在建立 tick 與後續呼叫之間讀 `count`，兩項用途就重疊：closure 仍要持有 mutable borrow，格式化輸出又需要借用原數字。以下完整失敗範例得到 `E0502`：

- Q1：延3.3–3.5借用與最後使用；空直線是無參數。
- Q2機制/性質（已足夠）：只讀長度保存shared；改count保存mutable。count mut與tick mut各有作用。
- Q2需要/用途（已足夠）：讀長度不需接手String；新計數工作需持續改同一原狀態，換材料有需要。
- Q3：shared可並讀原text；tick最後用後讀count，仍將呼叫時先讀則E0502。
- Q4：兩次1/2與重疊失敗，無永久鎖/複製文字誤推。

### move capture與呼叫移出

首次必要：9.1 owned121/copy129/take139/圖152前。

docs/chapters/09-01-lesson.md:116

> 如果工作需要自己擁有那份文字，在參數直線前加 **`move`**，寫成 `move || ...`。它要求按值保存所使用的環境資料，和前面的借用 capture 不同。

docs/chapters/09-01-lesson.md:118

> 對沒有 `Copy` 的 `String`，這表示 ownership 從外面的變數交給 closure。下面 body 仍然只讀它保存的文字，沒有把文字交出去，所以能重複呼叫：

docs/chapters/09-01-lesson.md:126

> 對 `Copy` 資料，按值 capture 會複製值本身。下面的 `i32` 與 shared reference 都符合；特別注意，複製 `&str` 是複製借用資訊，並沒有取得原 `String` 的 ownership：

docs/chapters/09-01-lesson.md:132

> 執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example closure_move_copy --locked`，第一行兩個數字都是 `300`；第二行仍能直接讀 `view`，closure 讀到 `4`。原 `text` 仍要在這份 reference 使用期間有效。`move` 不會把借來的文字變成獨立 owned 文字。

docs/chapters/09-01-lesson.md:136

> 最後把 body 改成直接交回保存的 `String`：`move || text`。這次先在建立時把文字交給 closure，再在呼叫時把文字交給呼叫者，讓後者可以獨立管理、追加內容。

docs/chapters/09-01-lesson.md:150

> 圖中第一條箭頭對應建立 `take`，第二條對應執行 `take()`。文字的 owner 從 `text` 轉到 closure，再轉到 `saved`；不是先 clone 兩份文字。

docs/chapters/09-01-lesson.md:154

> 這裡交出的 `String` 沒有 `Copy`；呼叫 take 會消耗這份 closure，不能再要求同一份 take 交出第二份文字。若重複呼叫，下面的 fixture 得到 `E0382`：

docs/chapters/09-01-lesson.md:160

> 所以要分開追蹤兩件事：建立時，環境按值或借用保存；呼叫時，body 讀取、修改或移出什麼。前一個 `move` 只讀範例能呼叫兩次，這個移出範例則消耗 closure，差別在 body 的工作。

- Q1：move按值保存；String move、i32/sharedref Copy；body另決定移出。
- Q2機制/性質（已足夠）：只讀自有String可多次；交回nonCopyString的body消耗closure。
- Q2需要/用途（已足夠）：需要自有文字或交給caller獨立追加，非僅換語法；move ref仍依賴原文字是當下ownership界線。
- Q3：byte_count兩次4；limit/view仍可讀；take()->saved追加教學。
- Q4：圖text→take→saved兩箭頭兩時點、同一文字無clone；E0382二次take對照只讀成功。

### FnOnce callback

首次必要：9.2 callback_once26前。

docs/chapters/09-02-lesson.md:3

> 上一節已能保存並呼叫 closure。接著希望把「收到一份工作，再按約定執行」整理成共用函式：有的只執行一次，有的重複執行，有的只借用原工作。

docs/chapters/09-02-lesson.md:5

> 這些共用函式不需要知道 closure 使用哪些外部變數，但必須知道它能接受什麼參數、交回什麼，以及用哪種方式呼叫。第 6.4 節的 trait 與 associated type 關係，現在可以用在這份契約。

docs/chapters/09-02-lesson.md:9

> 把其他程式提供、由共用函式負責呼叫的工作稱為 **callback**（回呼）。例如提供一份「交出 owned 文字」的工作，共用函式接手後執行它，取回文字交給呼叫者。

docs/chapters/09-02-lesson.md:11

> **`FnOnce`** 是 Rust 的可呼叫 trait，提供按值接手這份工作的呼叫能力。closure 的 body 可以在呼叫中移出保存的 non-Copy 內容，像上一節的 take。只知道 `F: FnOnce` 時，共用函式不能假定同一份 F 還能再用。

docs/chapters/09-02-lesson.md:13

> `FnOnce` 與稍後的 `FnMut`、`Fn` 都由 `std::ops` 提供，也已在 Rust 的 prelude 中，因此本節可以直接在 bound 使用這些名稱。這是這三個標準 trait 的名稱來源；自訂 trait 仍沿第 6.1 節以 `use` 引入。

docs/chapters/09-02-lesson.md:15

> 這裡的 bound 採專用語法 **`FnOnce() -> T`**：空括號表示不接收參數，箭頭表示回傳 T。回傳型別對應這組 trait 的 associated type `Output`，和第 6.4 節一樣是該實作的輸出選擇；不用手寫標準函式庫內部的呼叫 method。

docs/chapters/09-02-lesson.md:17

> 每個 closure 表達式都有自己的具體型別，通常不能在原始碼中寫出該型別的名稱。因此用泛型 `F` 表示工作型別，再用 trait 說清楚它能做什麼。下面 `T` 是結果型別，`run_once` 接手 task 並呼叫一次，交回取得的 T；沒有保存供後續第二次呼叫的 task。

docs/chapters/09-02-lesson.md:35

> `FnOnce` 是最低呼叫能力，**不是每個符合它的 closure 都必定只能呼叫一次**。上面的 `|| 7` 沒有移出需管理的資源，也符合契約；共用函式選擇只依承諾使用一次。

- Q1：callback由共用fn呼叫，FnOnce專用括號bound、Output沿6.4。
- Q2機制/性質（已足夠）：按值呼叫，已知FnOnce只承諾一次，不是實作都只能一次。
- Q2需要/用途（已足夠）：共用工作只執行一次，無需知道環境形狀但需輸入輸出與能力。
- Q3：run_once<T,F:FnOnce()->T>接F後task()交T；標準prelude來源先說。
- Q4：takeString與無捕捉7都可，後者防互斥分類誤讀。

### FnMut

首次必要：9.2 mutable44/repeatfail58前。

docs/chapters/09-02-lesson.md:39

> 如果要讓計數工作連做兩次，只知道 FnOnce 就不夠。**`FnMut`** 提供透過 mutable reference 呼叫自身的能力，可以重複呼叫，並讓 body 修改保存的環境。

docs/chapters/09-02-lesson.md:41

> `FnMut() -> i32` 表示每次不接收參數、交回一個 i32。本例的 `call_twice` 接手工作後，參數寫 `mut task`，允許借用 task 作兩次呼叫；結果按照第一次、第二次的順序放進 tuple：

docs/chapters/09-02-lesson.md:51

> 輸出 `兩次結果=1、2，原計數=2`。呼叫程式中的 tick 沒有直接被呼叫，只被交給函式，因此它的 binding 不需要 mut；接手之後要實際呼叫兩次的 task 才需要 mut。

docs/chapters/09-02-lesson.md:53

> call_twice 結束時沒有交回 closure，保存的 mutable borrow 也不再使用，因此外面可以讀 count。這裡是重複使用同一份計數狀態，不是建立兩份從零開始的工作。

docs/chapters/09-02-lesson.md:55

> 下面故意要求同一份工作回傳兩個 String，但提供的 take 會移出唯一保存的文字。它只能提供 FnOnce 的按值呼叫，不能滿足 FnMut，所以得到 `E0525`：

docs/chapters/09-02-lesson.md:61

> 把兩次呼叫需求改成一次，或重新設計成每次都能產生一份文字，是兩種不同的工作。只替參數加 mut，不能替已移出的資料補出第二份。

- Q1：mutable receiver可反覆呼叫，沿tick與3.6。
- Q2機制/性質（已足夠）：mut task供接手後兩次呼叫，外tick未直接呼叫毋須mut；fn結束borrow完。
- Q2需要/用途（已足夠）：連做兩次共用計數狀態，FnOnce最低承諾不足。
- Q3：call_twice回tuple1/2，外count2；移出唯一String不能滿FnMut。
- Q4：常數7也可、E0525對照；不是補mut就造出第二份資源。

### Fn與能力包含

首次必要：9.2 shared72前。

docs/chapters/09-02-lesson.md:65

> 如果共用函式只收到對工作本身的 shared reference，它不能用 mutable receiver 呼叫。**`Fn`** 提供透過 shared reference 呼叫自身的能力，也能重複呼叫。

docs/chapters/09-02-lesson.md:67

> 本例的加法工作只讀取保存的 base，適合這項要求。**`Fn(i32) -> i32`** 表示呼叫時收到一個 i32，回傳一個 i32；不是宣告名為 i32 的環境欄位。

docs/chapters/09-02-lesson.md:69

> `apply_shared` 收到 `&F`，保留呼叫者的工作，而不接手它。這裡選 Fn 的理由是只有 shared access；只是「需要呼叫多次」的工作，通常 FnMut 已足夠：

docs/chapters/09-02-lesson.md:79

> | trait | 呼叫如何使用這份工作 | 這份契約允許共用函式做什麼 |

docs/chapters/09-02-lesson.md:85

> 符合 Fn 的工作也符合 FnMut 與 FnOnce；符合 FnMut 的也符合 FnOnce。因此 call_twice 能接受不修改環境的 `|| 7`，run_once 也能接受前兩種能力的工作。這是可提供能力的包含關係，不是三個互斥分類。

docs/chapters/09-02-lesson.md:87

> 還要保留上一節的區分：`move` 只規定按值 capture。擁有 String 但只讀它的 closure 仍可有 Fn；把 String 移出 body 才使這個例子只能有 FnOnce。Fn 描述 shared receiver 能力，不保證每次回傳相同值，也不把所有可能間接可達的資料都宣稱成永遠不變。

- Q1：Fn為shared receiver呼叫；Fn→FnMut→FnOnce非互斥。
- Q2機制/性質（已足夠）：&F可呼叫並保留；Fn不保同結果/一切間接資料不變。
- Q2需要/用途（已足夠）：只有shared access才需Fn，單多次FnMut夠，選擇理由非更多能力必好。
- Q3：Fn(i32)->i32，base10只讀，apply_shared&add得13後仍add4→14。
- Q4：table與常數7支持包含；間接資料限定不依賴Cell/RefCell細節，僅限制過度推論。

### function pointer

首次必要：9.2 pointer98/failure106前。

docs/chapters/09-02-lesson.md:91

> 有時工作已經是一個普通函式，不需要重新寫一個會捕捉環境的 closure。**function pointer**（函式指標）是保存可呼叫函式位置的值；型別以小寫 **`fn(i32) -> i32`** 表示，指定輸入與輸出。

docs/chapters/09-02-lesson.md:93

> `fn` 在這裡是具體型別，`Fn` 則是能力 trait，兩者不同。可以把普通函式名稱指定到這個型別，編譯器會轉成對應的 function pointer。不捕捉環境的 closure 也能作這種轉換；pointer 本身沒有位置保存任意外部環境資料。

docs/chapters/09-02-lesson.md:95

> 這種 function pointer 提供 Fn、FnMut、FnOnce，所以也可交給前面的共用函式。下面先直接呼叫兩個 pointer，再將其中一個借給 apply_shared：

docs/chapters/09-02-lesson.md:103

> 若 closure 需要 capture base，就不能只用同樣的 pointer 型別保存它；以下範例得到 `E0308`。這時使用接受 Fn 等 trait 的泛型介面，而不是把環境偷偷塞進 function pointer：

- Q1：小寫fn具體型別、Fn是trait；保存函式位置。
- Q2機制/性質（已足夠）：普通fn與無captureclosure可轉pointer，提供Fn系列，沒有任意環境儲存。
- Q2需要/用途（已足夠）：已有函式可直接共用callback介面，需base capture時用泛型Fn。
- Q3：add_one→fn(i32)->i32；double不capture；&pointer給apply_shared。
- Q4：4/6/4與E0308捕捉base反例；不要求ABI/位址內部。

### 參數與回傳impl Trait

首次必要：9.3 parameter24/owned34前。

docs/chapters/09-03-lesson.md:3

> 上一節讓函式收到工作並執行它。現在希望函式收到設定值，建立好一份工作，交回去讓呼叫者之後反覆使用。例如保存加數 10，之後把不同輸入各加 10。

docs/chapters/09-03-lesson.md:5

> 這個 closure 需要 capture 設定值，不能只用上一節的 function pointer 表示。它有具體型別，但通常不能在 source 中寫出型別名稱；回傳介面仍必須說清楚呼叫者可以做什麼。

docs/chapters/09-03-lesson.md:9

> **`impl Trait`** 在型別位置表示「符合指定 trait 的具體型別」。先看參數位置：`task: impl Fn(i32) -> i32` 接受一份有對應呼叫能力的工作。具體工作由呼叫者提供，函式 body 只能依已宣告的能力使用它。

docs/chapters/09-03-lesson.md:11

> 這裡保留上一節的輸入／輸出契約，但省略另取泛型參數 F 的步驟。它是匿名的型別參數；若要在多個位置表達同一個型別，仍可用原來的具名泛型。這項匿名參數也不能像第 5.1 節的具名參數一樣在呼叫處指定 turbofish。

docs/chapters/09-03-lesson.md:21

> 這組參數／回傳對照先使用同一種 Fn 契約，讓差別集中在誰提供具體型別。evaluate 本身只呼叫一次，單獨設計這項工作時，FnOnce 就已足夠；此處的 Fn 是本例選定的介面能力。

docs/chapters/09-03-lesson.md:27

> 把 `impl Trait` 放在**回傳位置**，選擇方向反過來：由函式實作選定具體結果型別，呼叫者取得符合這份介面的結果，不能自行改選另一個型別。

docs/chapters/09-03-lesson.md:29

> 這種隱藏具體型別的介面稱為 **opaque type**。這裡隱藏的是原始碼需要寫出的型別細節，不是讓結果沒有型別，也不是把型別改成執行期任選。

docs/chapters/09-03-lesson.md:31

> `make_adder` 回傳 `impl Fn(i32) -> i32`，承諾結果可接受 i32、交回 i32，且能透過 shared receiver 呼叫。body 選擇一個按值保存 base 的 closure；base 是 Copy 整數，工作不借用函式內會結束的區域變數：

- Q1：參數匿名泛型由caller提供；回傳opaque由實作選具體型別。
- Q2機制/性質（已足夠）：同Fn介面選擇方向相反，opaque隱名稱非無型別；匿名無F多處關係/turbofish。
- Q2需要/用途（已足夠）：設定→工作需capture而型別通常無可寫名，pointer不適用；介面仍須保留可做什麼。
- Q3：evaluate收implFn，make_adder moveCopybase回implFn；沒有借結束區域。
- Q4：6與13/14、方向表；21明說evaluate選Fn為同能力教學對照，非最低必需。

### opaque單一具體回傳型別

首次必要：9.3 fixture49/branches55前。

docs/chapters/09-03-lesson.md:44

> 同一函式的所有回傳分支，必須有相同的具體型別。每個不同 closure 表達式都有自己的型別，所以「它們都實作 Fn」不代表能作為同一個回傳結果。

docs/chapters/09-03-lesson.md:46

> 下面故意在兩個分支寫不同的 closure，得到 `E0308`：

docs/chapters/09-03-lesson.md:52

> 本項工作只需要依設定選擇加或減，不需要兩種資料形狀。因此把分支放進**同一個 closure 的 body**：建立的仍是一種 closure 型別，保存的 subtract 值決定每次執行哪項算術。

docs/chapters/09-03-lesson.md:75

> evaluate 將 3 交給乘二工作，得到 6；make_adder 保存 10，收到 3、4 後分別得到 13、14。subtract 為 true 時做 30 減 10，否則做 30 加 10，因此是 20、40。不同 base 值可以保存不同設定，但同一個 closure 表達式的型別仍相同；回傳介面保留它能做的工作。

- Q1：每closure表達式獨有型別，實作Fn不等同一型。
- Q2機制/性質（已足夠）：兩分支不同closure E0308，分支放同closurebody保一型。
- Q2需要/用途（已足夠）：工作只依設定加減，不需兩種資料形狀；保原算術選擇並符合介面。
- Q3：make_adjuster capture subtract/base，body依bool選+/-。
- Q4：30/base10→20/40與失敗對照，不暗用後章dyn補。

### RPIT lifetime capture與precise use

首次必要：9.3 borrowed84/implicit108/precise119前。

docs/chapters/09-03-lesson.md:79

> 現在要交回一份讀文字 byte 長度的工作，讀的仍是既有 String 的內容，不重建另一份文字。輸入使用第 7.1 節的省略形式 `&str`，但來源 lifetime 仍然存在。回傳 closure 用 move 保存這個 shared reference 值；第 9.1 節已說明，按值保存 reference 不會取得被借文字的 ownership。

docs/chapters/09-03-lesson.md:81

> 回傳位置的 `impl Trait` 簡稱 **RPIT**（return-position impl Trait）。本專案採 edition 2024：沒有明確限制時，這個 opaque 型別會自動 capture scope 內的泛型參數，**包含 lifetime 參數，省略名稱的輸入 lifetime 也在其中**。因此下面的隱藏型別可以依賴輸入 lifetime，保存借自輸入的 reference：

docs/chapters/09-03-lesson.md:87

> 這裡的「RPIT capture」是在型別契約上允許隱藏型別依賴哪些泛型參數；第 9.1 節的「closure capture」則是建立值時保存哪些資料或 reference。兩者相關，但不是同一項動作，前者不會配置或複製原文字。

docs/chapters/09-03-lesson.md:95

> 若把來源移到內層，卻在內層結束後才呼叫回傳工作，便不符合這項依賴，得到 `E0597`。回傳值有 opaque 介面，不會取消原資料的 lifetime 要求：

docs/chapters/09-03-lesson.md:103

> 有時隱含 capture 比這次實作真正使用的資料更廣。以下對照刻意保留一個 `&str` 輸入，但工作只交回固定數字 4，不讀輸入；它只為展示回傳契約的差異。

docs/chapters/09-03-lesson.md:105

> 在 edition 2024，省略明確限制仍會 capture 輸入的 lifetime，即使本次 closure 沒有保存該 reference。呼叫端仍依介面允許的依賴檢查，所以這個 fixture 也得到 `E0597`：

docs/chapters/09-03-lesson.md:111

> 要表達較窄的依賴，可用 **precise capture**（精確捕捉）的 `use<...>`。它放在回傳 bound，和第 2.6 節引入名稱的 use statement 是不同用途：

docs/chapters/09-03-lesson.md:113

> - **`use<'a>`** 明確允許隱藏型別依賴 `'a`。真的保存輸入 reference 的 reader 需要這份來源關係。

docs/chapters/09-03-lesson.md:114

> - **`use<>`** 不 capture 泛型參數。本例沒有其他型別參數，固定工作也沒有保存輸入 reference，才可以這樣排除來源 lifetime。

docs/chapters/09-03-lesson.md:116

> `impl Fn() -> usize + use<>` 中的加號，把呼叫能力與 capture 約定放進同一個回傳介面，不是數值相加。下面第一個函式保留來源，第二個明確排除不需要的來源：

docs/chapters/09-03-lesson.md:128

> `use<>` 沒有延長 inside，也不能用來消除一份真實保存的 reference。若實作仍使用那份借用，隱藏型別就需要 capture 相應 lifetime。現行語法若有具名型別參數，也要求 capture 清單列入 scope 中全部型別參數；不能把這個只有 lifetime 的示例推成任意省略 T 的規則。

- Q1：完整縮寫81；型別契約依賴參數與值保存資料分開；接7章來源有效。
- Q2機制/性質（已足夠）：2024預設含scope泛型/lifetime及省略輸入；use<a>允a，use<>排本例唯一lifetime，+組介面。
- Q2需要/用途（已足夠）：79讀既有文字不重建所以借用；103固定4不讀輸入的對照揭預設依賴妨礙inside結束後用，精確排除回應具體限制。
- Q3：move&str reader需來源有效；fixed_reader use<>可獨立；counter答案moveownedstate FnMut。
- Q4：真借用E0597、未借但契約E0597、use<>成功；128不延長真borrow/有T須列全只防泛化，不需泛型capture內部。

### Iterator/next/lazy

首次必要：9.4 countdown32/圖39前。

docs/chapters/09-04-lesson.md:3

> 第 5.5 節的清單整理器用索引迴圈，每次要處理下一項，同時維護索引與結束條件。現在希望把「怎麼取得下一筆」收在來源中，讓讀取端不必知道來源是否有索引、是否已先保存全部資料。

docs/chapters/09-04-lesson.md:5

> 本節先建立一個只保存剩餘數字的倒數來源，再將同一套讀取方式用在清單。索引迴圈仍然能完成原工作；新的分工是讓來源負責推進，讀取端只處理交來的每筆資料。

docs/chapters/09-04-lesson.md:9

> **iterator**（迭代器）是可逐筆交出資料的值，在 Rust 中由標準 trait **`Iterator`** 描述。它保存推進所需的狀態，例如下一個數字或清單目前位置。

docs/chapters/09-04-lesson.md:11

> 這個 trait 的 associated type 叫 **`Item`**，表示每次交出資料的型別，沿用第 6.4 節的型別關係。Item 可以是 owned 值，也可以是 reference；不能只由「走訪」一詞判斷 ownership。

docs/chapters/09-04-lesson.md:13

> 核心 method 是 **`next(&mut self) -> Option<Self::Item>`**：透過 mutable receiver 更新 iterator 狀態；有下一筆就交回 Some 包住的 Item，這次走訪結束則交回 None。沒有額外的輸入參數，也不以數字零當成結束訊號。

docs/chapters/09-04-lesson.md:15

> Iterator 的一般契約允許某些實作在 None 後再次交出 Some；後面的 for 會在**第一個 None** 結束。這節的有限倒數與 slice iterator 則在結束後持續回傳 None，不把這份實作的性質推成所有 iterator 的必要保證。

docs/chapters/09-04-lesson.md:19

> 倒數來源從 `start` 開始，每次先保存目前正整數，把 remaining 減一，再交回剛才保存的數字。start 小於或等於零，直接交回 None；沒有需要先建立一份倒數清單。

docs/chapters/09-04-lesson.md:29

> `type Item = i32` 選定每筆數字型別。count_down 這次直接回傳具名的 Countdown，方便對照 remaining 保存在誰身上；它建立的值再依下面的 Iterator 實作逐筆交數字。

docs/chapters/09-04-lesson.md:35

> 建立工作只保存 start，尚未取得第一筆。**lazy**（惰性）在本例表示數字由 next 推進時才逐筆交出，不是在建立時就先算出並保存整份清單。建構值本身仍然有發生，不表示任何初始化都沒有成本。

docs/chapters/09-04-lesson.md:37

> 下圖追蹤從 3 開始的前四次 next。remaining 是保留在同一份 Countdown 中的狀態，Some 中的數字則交給呼叫者；第四次沒有下一筆，剩餘狀態仍是零。

- Q1：Iterator保存推進狀態，Item associatedtype可owned/ref；next&mut→OptionItem。
- Q2機制/性質（已足夠）：先current後remaining−1交Somecurrent；<=0 None；lazy在next逐筆產生非零初始化成本。
- Q2需要/用途（已足夠）：來源可不索引/不先存全部，倒數只remaining；來源推進與reader處理分工，原索引法仍能做。
- Q3：mut stream next3/2/1/None/None；0/-4 None。
- Q4：圖同工作四列state3→2/2→1/1→0/0→0對Some3/2/1/None；一般None後可Some限定正服務for首None結束。

### IntoIterator與共同來源

首次必要：9.4 for74/NumberList86/sum92前。

docs/chapters/09-04-lesson.md:65

> **`IntoIterator`** 描述「這種輸入如何轉成 iterator」。它與 Iterator 分工不同：前者建立走訪工作，後者負責逐筆交出 Item。

docs/chapters/09-04-lesson.md:67

> 它也有 Item，另外有 **`IntoIter`**，指定所建立的 iterator 型別。對 IntoIter 的要求是 **`Iterator<Item = Self::Item>`**：工作交出的元素，必須和這份 IntoIterator 宣告的一致。associated type 也可以有這種能力要求，不是只放一個沒有條件的名稱。

docs/chapters/09-04-lesson.md:69

> **`into_iter(self) -> Self::IntoIter`** 按值接收輸入，交回工作。若輸入本身是 reference，接收的是那份 reference；若輸入是 owned collection，接收的才是該 collection。方法名稱中的 into 不會讓所有情況都自動取得原資料的 ownership。

docs/chapters/09-04-lesson.md:71

> `for value in source` 會先依 source 的 IntoIterator 建立工作，再反覆呼叫 next。Some 的內容進入這次迴圈，第一個 None 結束。所有 Iterator 本身也有 IntoIterator，轉換時直接交回同一份 iterator，所以 Countdown 可以直接交給 for：

docs/chapters/09-04-lesson.md:77

> 執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example iterator_for --locked`，依序取得 `3`、`2`、`1`，合計 `6`。這裡 for 接手 stream，自己以可修改的方式推進，原 binding 不必寫 mut。沒有重新建立另一個從 3 開始的 iterator。

docs/chapters/09-04-lesson.md:81

> `Vec<i32>` 的 owned `into_iter()` 會接手 Vec，從頭到尾逐筆交出 i32。回傳工作有標準型別名稱 **`std::vec::IntoIter<i32>`**：std 是標準函式庫 crate，vec 是其中的 module，IntoIter 是 iterator 的型別名稱，不是 IntoIterator trait。

docs/chapters/09-04-lesson.md:83

> 現在以 NumberList 包住 Vec，讓它也能直接交給 for。它的 into_iter 只把欄位中的 owned Vec 轉成工作，沒有重建元素。實作的 `IntoIter` 必須寫出具體型別，所以使用剛才的標準工作型別：

docs/chapters/09-04-lesson.md:89

> 共用的加總函式只要求 **`S: IntoIterator<Item = i32>`**，不需要知道欄位名稱，也不需要來源能依索引存取。它接手 source、逐筆相加，空來源交回零：

docs/chapters/09-04-lesson.md:95

> 第 2.3 節的 range 與剛才的 Countdown 都符合 i32 Item 的要求，所以同一個函式能接收三種來源：

docs/chapters/09-04-lesson.md:101

> 執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example iterator_sources --locked`，三種合計都是 `6`。清單與 range 交出 1、2、3，Countdown 交出 3、2、1；同一份加總工作都依序加進 total。空清單沒有下一筆，因此保留初值 0。共同的逐筆操作不再綁住特定清單欄位；它仍沒有自動證明資料或加總的商業規則正確。

- Q1：IntoIterator建立工作，IntoIter具體Iterator且Item相同，分工不同Iterator。
- Q2機制/性質（已足夠）：self接收到的reference或owned值各自ownership；Iterator本身into_iter交同工作。
- Q2需要/用途（已足夠）：共同加總不依欄位或索引，同一工作接NumberList/range/倒數。
- Q3：std::vec::IntoIter具名；NumberList交ownedVec；S:IntoIterator<Item=i32>for加總。
- Q4：不同順序三來源sum6、空0；StepDown先next5再for3/1 sum4證同進度不重設。

### slice/Vec三種ownership

首次必要：9.4 table107/ownership120/borrowed142/fixtures152/158。

docs/chapters/09-04-lesson.md:105

> 資料改成 `Vec<String>`，才能看清楚是否複製文字、借用文字或交出文字。依工作需要選以下建立方式，Item 型別也跟著不同：

docs/chapters/09-04-lesson.md:107

> | 建立工作 | 借用或接手什麼 | Item | 結束後原 Vec |

docs/chapters/09-04-lesson.md:109

> | `texts.as_slice().iter()` | shared borrow slice | `&String` | 仍可用；沒有交出 owned String |

docs/chapters/09-04-lesson.md:110

> | `texts.as_mut_slice().iter_mut()` | mutable borrow slice | `&mut String` | 借用結束後可用，內容可能已改 |

docs/chapters/09-04-lesson.md:111

> | `texts.into_iter()` | 接手 owned Vec | `String` | 原 Vec 已交出，不能再用 |

docs/chapters/09-04-lesson.md:113

> **slice 的 `iter`** 從頭到尾交出各元素的 shared reference。**`iter_mut`** 從頭到尾交出各元素的 mutable reference，讓每個元素能個別修改。建立這些 iterator 本身不會走完所有元素；空 slice 的下一筆就是 None，不會因為空清單越界。

docs/chapters/09-04-lesson.md:115

> 借用來源必須在 iterator 與交出 reference 的使用期間保持有效；mutable 路徑也仍遵守其他借用不能重疊的規則。本節顯式使用 as_slice／as_mut_slice，讓取得的視圖與其方法可辨認。

docs/chapters/09-04-lesson.md:117

> 下面分成三段：先借來讀，再借來追加 ending 保存的驚嘆號片段，最後接手每個 String，追加 saved 並保存在新清單。這裡 `for mut item` 讓第三段取得的 owned String binding 可以修改；第二段則透過 mutable reference 修改原元素：

docs/chapters/09-04-lesson.md:135

> 沒有呼叫 clone。前兩段的 String owner 都在原清單中；第三段把元素交回、再 move 進 saved。最後的結果擁有這些文字，原 texts 已不能使用。

docs/chapters/09-04-lesson.md:139

> 標準函式庫也為 `&Vec<T>` 和 `&mut Vec<T>` 提供 IntoIterator，Item 分別是 `&T` 與 `&mut T`。因此可以直接將 reference 交給 for，不先寫 iter；這是 reference 型別自己的 IntoIterator 實作。

docs/chapters/09-04-lesson.md:145

> 執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example iterator_borrowed_forms --locked`，先讀到 1、2，再將原值改成 11、12。for 接手的是借用資訊，原 Vec 沒有被移走；第二段用第 3.4 節的 `*item` 修改 reference 指向的數字。

docs/chapters/09-04-lesson.md:149

> 以下 iterator 還要繼續交出 mutable reference，卻先讓 Vec 再作一次 mutable 操作。兩份 mutable borrow 重疊，得到 `E0499`：

docs/chapters/09-04-lesson.md:155

> owned into_iter 則是另一項限制：整個 Vec 已被接手，不能再用原值。即使元素是 Copy 整數，Vec 本身也沒有 Copy；以下 fixture 得到 `E0382`：

- Q1：iter &String、mut &mutString、owned String；reference自己的IntoIterator。
- Q2機制/性質（已足夠）：前兩借原Vec，第三接手；for mutitem使ownedbinding可改而非改mutref規則。
- Q2需要/用途（已足夠）：換Vec<String>為看清複製/借用/交出；只讀、改原項、接手另存各有需要。
- Q3：shared讀Rust/Python，mut追加!，owned追加saved後push；&Vec/&mutVec i32以*讀改。
- Q4：無clone、E0499/E0382，元素Copy非VecCopy；手機表C9-01可選。

### map adaptor/lazy callback

首次必要：9.5 map20前。

docs/chapters/09-05-lesson.md:3

> 第 5.5 節的整理器保留非負數；現在多一項需要：保留大於或等於門檻的數字，再乘上指定倍率，保存結果供後面的工作使用。材料 `[-2, 0, 3, 3]`、門檻 0、倍率 10，應得到 `[0, 30, 30]`，保留原順序與重複值。

docs/chapters/09-05-lesson.md:5

> 既有迴圈仍能完成這件事。這次把逐筆工作拆成可組合的 iterator 步驟，讓每一步的資料型別與操作可辨認。本節的算術都用不溢位的小整數。每個 example 仍是 no-lib package 的獨立完整檔，完整設定如下：

docs/chapters/09-05-lesson.md:13

> **iterator adaptor**（迭代器轉接器）是取得原 iterator、建立另一份逐筆工作的 method。新工作按值接收原 iterator，保存它與需要的 callback，不主動呼叫 clone；交接仍依來源型別的 Copy／move 規則。它也不是先把全部元素整理成另一份清單。

docs/chapters/09-05-lesson.md:15

> **`map`** 接收原 iterator 與一份 `FnMut` callback。每次需要下一筆時，它取得原 Item，交給 callback，再交出 callback 的回傳值。callback 因而可以改變 Item 型別；map 本身回傳的是另一個 Iterator，還沒有收集全部結果。

docs/chapters/09-05-lesson.md:17

> 先只將數字乘二。slice 的 iter 交出 `&i32`，所以 map callback 接收 `&i32`，以 `*item` 讀出 Copy 數字，再交回 i32；新工作的 Item 就是 i32。

docs/chapters/09-05-lesson.md:36

> 建立 map 後先出現「尚未走訪」，callback 的訊息則和 for 收到的資料交錯：每次推進才處理當筆。visits 被 callback mutable 借用，最後一次使用工作之後才讀它，沿用第 9.1 節的 capture 規則。

- Q1：adaptor按值收iterator/callback建新iterator，非先全收；map FnMut收Item回新Item。
- Q2機制/性質（已足夠）：slice&i32→map&i32 derefCopy→i32，推進才執行callback。
- Q2需要/用途（已足夠）：原需門檻篩選倍率保存；先隔離乘二觀察延後工作，拆可組步驟非舊迴圈不能做。
- Q3：visits/處理訊息在map，for收20/40，末使用後讀visits。
- Q4：尚未走訪先印、callback與for交錯2次，足見lazy不只最後值。

### filter Item與參數層次

首次必要：9.5 filter47/fixture55前。

docs/chapters/09-05-lesson.md:40

> **`filter`** 也建立新 iterator，但 callback 只判斷是否保留。它收到 **`&Item`**，回傳 bool；true 讓原 Item 繼續交出去，false 略過它，再尋找後續符合條件的資料。它不因判斷而將 Item 改成 bool。

docs/chapters/09-05-lesson.md:42

> 本例的原 Item 已是 `&i32`，所以 callback 參數會是 **`&&i32`**：外層 reference 借用這次取得的 Item，裡面的 reference 指向原數字。`**item` 連續取出兩層借用指向的 Copy 數字，才拿來比較。filter 之後的 Item 仍是 `&i32`，下一個 map 只需要一個 `*`。

docs/chapters/09-05-lesson.md:44

> 第二組改用 owned String：into_iter 的 Item 是 String，filter callback 才是 `&String`；判斷後保留的 Item 仍是 owned String。文字以第 8.5 節的 `to_owned` 建立，沒有新增字元走訪方法：

docs/chapters/09-05-lesson.md:50

> 執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example adapter_filter --locked`，數字依序保留 0、3、3，文字保留 Rust。`hi` 的 byte 長度不足 4，所以略過；不是把全部文字轉成長度數字。

docs/chapters/09-05-lesson.md:52

> 若對 shared slice 的 filter callback 只寫 `&i32`，和要求的 `&&i32` 就不同。以下 fixture 得到 `E0631`，指出 callback 的參數契約不符：

- Q1：predicate收&Item→bool，輸出仍原Item非bool。
- Q2機制/性質（已足夠）：&i32來源→&&i32 predicate，以雙dereference讀數；ownedString→&String判斷、String交下步。
- Q2需要/用途（已足夠）：保門檻、順序、重複是來源3獨立需求；保0/3/3，文字不是全改長度。
- Q3：true放行false找後續；String bytes>=4保Rust略hi，map後只單*。
- Q4：兩種ownership與E0631；正文code雙星是兩次dereference，非raw粗體bug。

### collect/FromIterator

首次必要：9.5 organize67/圖79/fixture86/impl96/full102前。

docs/chapters/09-05-lesson.md:60

> 篩選與轉換都只產生工作。要保存全部結果，使用 **`collect`**：接手 iterator，反覆取得 Item，依指定的目標型別建立產物。

docs/chapters/09-05-lesson.md:62

> 建立產物的能力由 **`FromIterator<A>`** 描述，A 是輸入元素型別。這個 trait 的 `from_iter` 接收能提供 A 的 IntoIterator，交回目的型別自身；和上一節的 IntoIterator 建立走訪方向相反。標準 `Vec<T>` 能從 T 元素依取得順序建立，例如 `Vec<i32>` 接收 i32 元素。

docs/chapters/09-05-lesson.md:64

> collect 沒有一個固定回傳型別。`let result: Vec<i32> = ...collect();` 指定結果是 Vec；在下面的 helper，回傳型別 `Vec<i32>` 也提供相同線索。map 已把 `&i32` 變成 owned i32，這份目標才能接住：

docs/chapters/09-05-lesson.md:70

> | 步驟 | 這一步的 callback | 交給下一步的 Item |

docs/chapters/09-05-lesson.md:77

> 下圖從 keep_and_scale 收到的 values 開始；這裡已是 `&[i32]`，所以直接呼叫 values.iter()。箭頭是逐筆資料與型別的流向；collect 的推進會帶動前面的工作，不是先建立圖中每一站的完整中間清單。

docs/chapters/09-05-lesson.md:81

> 標準 `Vec<T>` 當元素有 Debug 時也能 Debug，顯示元素清單；元素有 PartialEq 時，同長度且各位置相等才相等，因此本節能用 `{:?}` 顯示結果、用 assert_eq 比較 `Vec<i32>`。

docs/chapters/09-05-lesson.md:83

> 如果直接把 `&i32` 收進 `Vec<i32>`，collect 不會自動把 reference 變成數字。這個 fixture 以 `E0277` 指出目標沒有從這種 Item 建立的契約；先 map 取出 Copy 數字，才是前面正常版的安排：

docs/chapters/09-05-lesson.md:91

> 上一節的 NumberList 已能交出 i32。現在讓它也能**從 i32 元素建立**：實作 `FromIterator<i32>`，其中泛型 S 只要求 `IntoIterator<Item = i32>`，用原來的 for 與 push 保存資料，最後交回 Self。

docs/chapters/09-05-lesson.md:93

> 下面以 `use std::iter::FromIterator;` 明寫標準 trait 的來源；NumberList 與上一節的 IntoIterator 實作也直接保存在這份 example 同檔，完整檔會一併列出。這次增加的是從元素建立自己的能力：

docs/chapters/09-05-lesson.md:99

> 下面先執行整理，再用不同的目標表示法收集 range。`collect::<Vec<i32>>()` 在 method 指定結果型別，回用第 5.1 節的 turbofish；**`Vec::<i32>::from_iter`** 則先在型別路徑指定 Vec 的元素型別，再透過 `::` 呼叫這個型別提供的 from_iter。自訂 NumberList 以變數型別註記指定目標，選同樣的 i32 Item，但建立的結果型別不同：

docs/chapters/09-05-lesson.md:105

> 執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example adapter_collect --locked`，整理結果是 `[0, 30, 30]`，自訂目的型別的 values 是 `[1, 2, 3]`。FromIterator 不替本例做額外驗證；它依我們寫的建立方式保存資料。

- Q1：collect推進iterator，目標FromIterator<A>決產物；Vec按序存T。
- Q2機制/性質（已足夠）：let/return/turbofish選目標，不自動&i32→i32；自訂NumberList for/push。
- Q2需要/用途（已足夠）：整理結果需保存供後用，map/filter只工作；同i32可建不同目標。
- Q3：filter&&→map&讀數→collectVec；Vec::<i32>::from_iter路徑先說；NumberList另型目標。
- Q4：pipeline圖true門/Item型別、[0,30,30]順序重複、原值/空；Vec比較條件全碼前81，不暗驗證。

### fold accumulator

首次必要：9.5 fold116前。

docs/chapters/09-05-lesson.md:109

> 如果只需要總和，不需要保存所有轉換結果，可以用 **`fold`**。它接手 iterator、初始值與 `FnMut` callback，按序把「目前累計狀態、這次 Item」交給 callback。callback 交回下一份累計狀態，最後整個 fold 回傳它；空 iterator 則直接回傳初始值。

docs/chapters/09-05-lesson.md:111

> 這份狀態叫 **accumulator**（累計狀態）。本例選 i32、初始值 0；map 之後的 Item 也是 i32。callback 的兩個參數以逗號分隔，寫成 `|total: i32, value: i32| total + value`。狀態與 Item 是兩個角色，不必在所有 fold 中選相同型別。

docs/chapters/09-05-lesson.md:113

> Clippy 的 `unnecessary_fold` lint 會建議把這種純加總改成更短的寫法。本節要逐次觀察初值與累計狀態，因此只在 scaled_sum 前使用 `#[allow(clippy::unnecessary_fold)]` 保留 fold，沿用第 2.7 節的局部具名允許方式：

docs/chapters/09-05-lesson.md:119

> 這是它與 main 組成的完整程式，空輸入與沒有符合值的材料也會檢查初值：

docs/chapters/09-05-lesson.md:125

> 執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example adapter_fold --locked` 得到合計 60。累計依序是 0 + 0、0 + 30、30 + 30；沒有建立 `[0, 30, 30]` 的中間 Vec。這也能用 for 累計，fold 的用途是將初值和每筆更新放在同一份介面，沒有承諾比原迴圈更快。

- Q1：初state和FnMut(state,Item)→nextstate，empty回初值；中文累計狀態。
- Q2機制/性質（已足夠）：i32初0、Itemi32，逗號雙參數；一般state可不同Item。
- Q2需要/用途（已足夠）：只需合計不需全結果，初值與更新共介面，非更快宣稱。
- Q3：0+0、0+30、30+30；局部allow沿2.7且現在觀察fold有理由。
- Q4：空/無符合值0、total60，未建中間Vec由程式直接可見非性能測量。

### Result collect短路

首次必要：9.5 parse_all142/full150前。

docs/chapters/09-05-lesson.md:129

> 接著把材料換成整數文字。第 8.5 節的 `as_str().parse::<i32>()` 回傳 `Result<i32, ParseIntError>`，現在希望把整批解析結果收成一份成功清單；遇到第一個失敗，就保留那個原因交回。

docs/chapters/09-05-lesson.md:131

> 標準 Result 提供相應的 FromIterator 實作，目標是 **`Result<Vec<i32>, ParseIntError>`**：

docs/chapters/09-05-lesson.md:133

> - 每筆 Ok 的 i32 進入 Vec；全都成功，交回 Ok 包住的 Vec。

docs/chapters/09-05-lesson.md:134

> - 第一次遇到 Err，就停止取得後面的 Item，交回那個 ParseIntError。之前的成功清單不作為部分結果回傳。

docs/chapters/09-05-lesson.md:135

> - 空來源沒有失敗，結果是 Ok 包住空 Vec。

docs/chapters/09-05-lesson.md:137

> 這份收集保留同一個錯誤型別，**不會像第 8.6 節的 ? 一樣替你套用 From 改成另一個錯誤型別**。它選擇的是這次整批工作遇錯就回傳的政策，不是把所有失敗一次收集起來。

docs/chapters/09-05-lesson.md:139

> 對 `&[String]`，iter 的 Item 是 `&String`；map 收到 `&String`、回傳 `Result<i32, ParseIntError>`；collect 才取得這些 Result，交出整批結果。本節錯誤型別以 `use std::num::ParseIntError;` 引入，沿用第 8.5 節的標準型別：

docs/chapters/09-05-lesson.md:145

> 下面額外計數，觀察第三筆 `30` 是否真的沒有解析。callback 每次先把 visits 加一，再 parse；把整數文字變成 owned String 的 to_owned 同樣來自第 8.5 節。

docs/chapters/09-05-lesson.md:147

> 完整檔同時保留 parse_all，並檢查全成功與空來源。標準 Result 在成功型別與錯誤型別都有 Debug／PartialEq 時，也能顯示與比較整份結果：同為 Ok 時比較成功內容，同為 Err 時比較錯誤；Ok 與 Err 不相等。這裡的 `Vec<i32>` 已符合本頁的條件，ParseIntError 則已由第 8.5 節介紹：

docs/chapters/09-05-lesson.md:153

> 執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example adapter_results --locked`，會顯示 ParseIntError 的失敗訊息，以及 `實際解析次數=2，原清單筆數=3`。第一筆 10 成功，第二筆 bad 失敗，第三筆沒有執行 callback。借用原清單不會把第三筆或原 String 消耗掉，但這次 collect 沒有交回第一筆成功值。

docs/chapters/09-05-lesson.md:155

> 如果需要逐筆報告並繼續，仍用第 8.1 節的 for／match 政策；不能在使用這項 Result collect 的同時，假定後面的每筆工作都已執行。

docs/chapters/09-05-lesson.md:159

> 寫 parse_total，接收 `&[String]`，回傳 `Result<i32, ParseIntError>`。重用本節 parse_all，成功取得 Vec 後再用 owned into_iter 與 fold 合計；失敗用第 8.6 節的 ? 交回。這裡兩層錯誤型別相同，保留原 ParseIntError，不需要新增轉換規則。

docs/chapters/09-05-lesson.md:173

> 0、3、−2 的解析全部成功，所以 ? 取得 Vec，fold 依序把 0、3、−2 加入初值 0，得到 Ok(1)。空 Vec 的 fold 保留初值，得到 Ok(0)。bad 使 parse_all 交回 Err，? 便從 parse_total 早退，不執行後面的 fold。兩邊錯誤都已是 ParseIntError，這裡用第 8.6 節相同型別到自身的 From 契約，不另包裝原因。

- Q1：Result<Veci32,ParseIntError>首Err回原因；與?的From轉換不同。
- Q2機制/性質（已足夠）：首Err不取後續、不回部分Vec；emptyOkempty，同錯誤型別。
- Q2需要/用途（已足夠）：希望整批成功清單、第一原因；逐筆全報需求仍8.1for/match，不混政策。
- Q3：iter&String→mapResult→collect；visits驗2/3；parse_total ?得Vec再fold。
- Q4：10/bad/30第三沒callback、成功/空/invalid；bad非零值。

### chars/lines與空行filter

首次必要：9.6 text_units24/lines45前。

docs/chapters/09-06-lesson.md:3

> 第 6.6 節已能按名稱計數，第 9.5 節又能把逐筆操作接成流程。現在輸入換成一段多行文字：每行是一個名稱，空行略過，相同名稱合併計數，結果自己擁有名稱。

docs/chapters/09-06-lesson.md:5

> 材料依序是 Rust、Python、Rust、空行、教學；要得到 Rust 兩次、Python 與教學各一次。名稱原樣保留，大小寫與前後空白都參與身分判斷。本頁先完成正確計數；固定報表順序由下一節接續。

docs/chapters/09-06-lesson.md:15

> **`str::chars()`** 借用文字，回傳 `Item = char` 的 iterator，依序取得 Unicode scalar。它沿用第 5.3 節的 char 意義，不是每次取一個 byte，也不保證每筆就是一個畫面上的完整字形。

docs/chapters/09-06-lesson.md:17

> 下面的文字 `é` 是字母 e 加上結合重音符號，通常顯示成一個字形，但有兩個 scalar、三個 UTF-8 bytes。char 已有標準 Debug，可以檢查它；Debug 把這個結合符號顯示為 **`'\u{301}'`**。這是 Unicode escape，用 scalar 編號表示一個 char，不是一串反斜線、字母與數字；U+0301 是該重音符號的常用編號。

docs/chapters/09-06-lesson.md:19

> 報表需要整行名稱，因此接著選 **`str::lines()`**。它借用文字，回傳 `Item = &str` 的 iterator，每筆是原文字中的一行，不會為每行建立 owned String。

docs/chapters/09-06-lesson.md:21

> 字串 literal 中的 **`\n`** 表示 LF（Line Feed，換行），**`\r`** 表示 CR（Carriage Return，回車）。lines 以 LF 或 CRLF（`\r\n`）分行，結果不含行尾；最後一個行尾不會額外產生一行，空文字產生零行。單獨 CR 沒有跟著 LF 時仍留在內容中，連續行尾之間的空行則是一份空 `&str`。

docs/chapters/09-06-lesson.md:36

> 第一份 collect 保存 char 值；第二份保存借自原文字的 reference。這是兩種不同的 Item 與 ownership 關係，不能只看都使用 collect 就認為結果都擁有原文字。

docs/chapters/09-06-lesson.md:40

> **`str::is_empty()`** shared 借用文字、回傳 bool，byte 長度為零就是 true，否則 false；它不修剪空白。lines 的 Item 是 `&str`，filter 參數因此是 `&&str`；`*line` 取得裡面的文字 reference，再以 is_empty 判斷，保留的仍是原 `&str`。

docs/chapters/09-06-lesson.md:42

> `nonempty_names` 回傳 `impl Iterator<Item = &str>`，使用第 9.3 節的 opaque 介面；輸出 Item 的 reference 依第 7.1 節單一來源規則連到輸入，回傳工作也依 edition 2024 capture 依賴來源。原文字仍必須在走訪時有效。這個 helper 和稍後的 count_names 都保存在本頁 text_counts example 的同一檔：

- Q1：chars char scalar沿5.3；lines &str借來源，is_empty byte0。
- Q2機制/性質（已足夠）：e加重音一字形2scalar3bytes；Unicodeescape、LF/CRLF/空/尾/單CR在碼前。
- Q2需要/用途（已足夠）：原需整行名稱且身分原樣，不應拆scalar；先對照scalar/byte/字形避免單位混淆。
- Q3：Vecchar/Vec&str；lines filter&&str解一層後is_empty；opaqueItem&str依7.1/9.3。
- Q4：é3/2、空行、CRLF、單CR、尾LF/空 assertions；不把collect ref誤own文字。

### Entry條件更新與預設建立

首次必要：9.6 callback65/counts75前。

docs/chapters/09-06-lesson.md:50

> 第 6.6 節用 get_mut／match 區分已有鍵與缺鍵。現在用 **`HashMap::entry(key)`** 取得一份描述這個鍵位置的工作：它 mutable 借用 map、接手候選鍵，回傳 **`Entry`**，代表「已有對應值」或「目前缺鍵」。Entry 不會讓借用規則消失；它保存操作這份 map 的借用關係。

docs/chapters/09-06-lesson.md:52

> 對 `HashMap<String, usize>`，以下 method 接手這份 Entry：

docs/chapters/09-06-lesson.md:56

> | `and_modify(callback)` | 執行 callback，交給它 `&mut usize` | 不執行 callback | 同一個鍵的 Entry，供下一步使用 |

docs/chapters/09-06-lesson.md:57

> | `or_insert(value)` | 保留既有值 | 插入傳入值 | map 中值的 `&mut usize` |

docs/chapters/09-06-lesson.md:58

> | `or_insert_with(callback)` | 不執行 callback，保留既有值 | 執行 callback，插入它交回的 usize | map 中值的 `&mut usize` |

docs/chapters/09-06-lesson.md:60

> **and_modify 的 callback 要求 `FnOnce(&mut usize) -> ()`**：一份 Entry 至多執行它一次，修改參數指向的值後不另交回數字。**or_insert_with 要求 `FnOnce() -> usize`**，只有需要新值時才呼叫。即使提供的 closure 也有重複呼叫能力，這些 method 只需要最低的一次呼叫契約，沿用第 9.2 節的能力包含關係。

docs/chapters/09-06-lesson.md:62

> 先用計數觀察延後建立：新名稱建立值 1，舊名稱加一，另記 callback 真的建立幾個新群組。不是每一筆名稱都執行預設值的 callback：

docs/chapters/09-06-lesson.md:68

> 執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example entry_callbacks --locked`，建立新群組的 callback 次數是 2，Rust 次數是 2、Python 是 1。第三筆 Rust 走 and_modify，不再執行 or_insert_with 的建立工作。

docs/chapters/09-06-lesson.md:70

> 候選鍵的 `name.to_owned()` 已在呼叫 entry 前執行，因此重複名稱也先建立候選 String；延後的是值的 callback，不能把這份介面說成會自動省去所有鍵複製。

docs/chapters/09-06-lesson.md:72

> 完整計數函式需要的初值只是 1，不需要額外建立工作，直接用 or_insert(1)。name 是 borrowed `&str`，to_owned 建立獨立鍵，使回傳 map 不借用原文字：

docs/chapters/09-06-lesson.md:80

> 下面把原 String 放在內層 block，count_names 交出的 map 留在外層。每個鍵都有自己的 String，因此原文字結束後，仍能用 get 查到結果。全部 helper 與 main 如下：

docs/chapters/09-06-lesson.md:96

> 前兩筆輸入 Rust、Python 先各新增1；第三個 Rust 修改已有值成2，空行沒有進入計數，教學再新增1。顯示的是 get 回傳的 Option，所以保留 Some 外框；缺少的 JavaScript 仍是 None。空文字的 lines 沒有下一筆，因此額外 assertion 取得空 map。

docs/chapters/09-06-lesson.md:98

> 如果保留 or_insert 的 mutable reference，再借用 map 更新另一鍵，兩份借用仍會重疊。以下 fixture 得到 `E0499`；要先完成 reference 的最後使用，再操作 map：

- Q1：Entry借map、接候選key；三method以值接Entry，描述已有/缺鍵。
- Q2機制/性質（已足夠）：and_modify FnOnce &mutusize→unit；or_insert_with FnOnce()->usize只缺執行；最後&mut值。
- Q2需要/用途（已足夠）：6.6 get_mut/match同鍵兩工作接起；數new_groups觀察延後建立；常數1不需另callback。
- Q3：entry to_owned→and_modify加1→or_insert1；ownedkey使innerString結束後get仍可。
- Q4：Rust/Python/Rust groups2 counts2/1；70明說候選鍵先建非省所有copy；E0499 retainedref；C9-02表可選。

### HashMap三種走訪

首次必要：9.6 map_iteration117前。

docs/chapters/09-06-lesson.md:106

> 對本節的 map，Item 是 tuple，鍵與值各有自己的 ownership：

docs/chapters/09-06-lesson.md:108

> | 建立方式 | Item | 對原 map 的作用 |

docs/chapters/09-06-lesson.md:110

> | `counts.iter()` | `(&String, &usize)` | shared borrow，保留 map |

docs/chapters/09-06-lesson.md:111

> | `counts.iter_mut()` | `(&String, &mut usize)` | mutable borrow，可修改值，鍵仍只借來讀 |

docs/chapters/09-06-lesson.md:112

> | `counts.into_iter()` | `(String, usize)` | 接手 map，逐筆交出 owned 鍵值 |

docs/chapters/09-06-lesson.md:114

> 這些 iterator 都以任意順序走訪。mutable 路徑不交出可修改鍵的 reference，避免鍵改變後破壞第 6.5 節的鍵身分／Hash 一致性。下面先借來累計，再將各值加 10，用名稱查詢後，最後接手 map 取出 owned 名稱與數字：

docs/chapters/09-06-lesson.md:120

> 執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example map_iteration --locked`，各段的名稱輸出順序不保證。原本 Rust 2、Python 1，借改後分別是 12、11；最後 owned 走訪交出這兩份鍵值，合計為 23。前兩段仍由原 map 持有鍵，第三段將鍵 move 給每次迴圈，原 counts 不再可用。順序沒有影響這次逐值加10與加總；固定報表則留到下一節。

- Q1：tuple鍵值各有ownership，shared、值mut、owned各清楚。
- Q2機制/性質（已足夠）：mutable鍵仍shared防Eq/Hash不一致沿6.5；最後接手map失效。
- Q2需要/用途（已足夠）：先累計/加10/接手資料三工作；目前不需順序，固定報表接9.7。
- Q3：4.3tuplepattern、*讀改；owned合計。
- Q4：3→12/11→23，順序任意不當保證，無因加總相等偷推定序。

### Ordering/PartialOrd/Ord

首次必要：9.7 partial_order26前。

docs/chapters/09-07-lesson.md:3

> 上一節已得到正確的名稱次數，但 HashMap 的走訪順序沒有保證。現在要每次都顯示同一份報表：次數由多到少，同次數再按名稱由小到大。

docs/chapters/09-07-lesson.md:15

> 排序需要知道兩項資料的比較結果是較小、相等，還是較大。標準 enum **`std::cmp::Ordering`** 有三個 variant：**Less、Equal、Greater**。它們表示第一個比較對象相對第二個的順序；這個型別有 Debug／PartialEq，可顯示與核對結果。

docs/chapters/09-07-lesson.md:17

> **`PartialOrd`** 提供部分順序的比較，`partial_cmp` shared 借用兩個值，回傳 `Option<Ordering>`。能比較就是 Some 包住相應順序，不能比較就是 None；None 不等於 Equal。先前的 `<`、`<=` 等操作也由這項能力支持，判斷必須和 PartialEq 的相等規則一致。

docs/chapters/09-07-lesson.md:19

> **`Ord`** 進一步承諾所有同型別值都能依一致的完整順序比較，也要符合 Eq。它的 `cmp` shared 借用兩個值，直接交回 Ordering。反轉左右，Less 應變 Greater；相等對應 Equal，連續的小於關係也要一致。排序時不能依 callback 第幾次執行任意改變答案。

docs/chapters/09-07-lesson.md:21

> i32／usize 的 Ord 採數值順序，str／String 採內容 bytes 的字典順序：從第一個不同的位置判斷，前綴相同則短的先。這保留大小寫與內容差異，不是依台灣語言習慣作字典排序。Ord／PartialOrd 已在 prelude，Ordering 則以 `use std::cmp::Ordering;` 引入。

docs/chapters/09-07-lesson.md:23

> 浮點數則有 **NaN**（Not a Number，非數值），不能和一般值取得這種順序。**`f64::NAN`** 是標準 f64 提供的 associated constant，即型別提供的固定值，直接用 `::` 讀取，不加呼叫括號；本例用它觀察 None。一般 f64 因這種情況沒有 Ord，不能只因能用 `<` 就假定它滿足完整排序契約：

docs/chapters/09-07-lesson.md:29

> 執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example partial_order --locked`，一般浮點比較是 Some(Less)，NaN 比較是 None。若要為浮點資料排序，必須另外決定 NaN 的位置與比較規則；本節報表的名稱與 usize 次數已具備 Ord，沒有這項未定政策。

- Q1：Ordering第一對第二三種；partial OptionNone非Equal；Ord完整一致且Eq。
- Q2機制/性質（已足夠）：數值/bytes字典序與反向/等值/傳遞；NaN使f64無一般Ord，associatedconstant讀法先說。
- Q2需要/用途（已足夠）：固定報表需完整比較非bool；1.4已有f64/有限表示，現在反例區分能<與完整序。
- Q3：partial_cmp借兩值→SomeLess/None；cmp→Ordering；標準來源明確。
- Q4：3/4與NaN、i32對照；不替浮點決NaN順序，當前String/usize已Ord。

### stable sort/by_key/by

首次必要：9.7 keys46/boolfixture68前。

docs/chapters/09-07-lesson.md:33

> slice 的 sort、sort_by_key、sort_by 都需要 mutable borrow，修改原排列，回傳 `()`；空或一項資料也能處理。它們保留比較相等項目的原相對順序，這叫 **stable sort**（穩定排序）。

docs/chapters/09-07-lesson.md:35

> | method | 如何決定順序 | callback 契約 |

docs/chapters/09-07-lesson.md:37

> | `sort()` | 元素自己的 Ord | 沒有 callback |

docs/chapters/09-07-lesson.md:38

> | `sort_by_key(callback)` | callback 取得排序鍵，鍵要有 Ord | `FnMut(&T) -> K` |

docs/chapters/09-07-lesson.md:39

> | `sort_by(callback)` | callback 直接比較兩項 | `FnMut(&T, &T) -> Ordering` |

docs/chapters/09-07-lesson.md:41

> callback 可能被多次呼叫，不保證每個元素只呼叫一次。本節將 Vec 明確借成 as_mut_slice，再使用這些 slice API。

docs/chapters/09-07-lesson.md:43

> 下面的 tuple 第一欄當排序鍵，第二欄只用來觀察同鍵資料是否維持輸入順序。標準 tuple 各欄位有 Debug／PartialEq 時，也提供相應的整份顯示／逐欄比較能力，因而 Vec 的 Debug 能列出這些 tuple：

docs/chapters/09-07-lesson.md:49

> 執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example sorting_keys --locked`，numbers 是 `[1, 1, 2, 3]`；rows 是 `[(1, 30), (1, 10), (2, 20)]`。第一欄相同的兩筆保留 30 在 10 前，沒有偷偷把第二欄也當排序鍵。

docs/chapters/09-07-lesson.md:65

> sort_by 要回傳 Ordering，不能只交回 bool。這個失敗範例以 `E0308` 指出輸出不符；完整順序還要區分兩項相等：

- Q1：slice mutable改排列回unit；stable保等鍵原序；各依Ord、keyOrd、Ordercallback。
- Q2機制/性質（已足夠）：FnMut可多次不保每項一次；tupleDebug/PartialEq條件全碼前。
- Q2需要/用途（已足夠）：rows第二欄為觀察等key原序，沒有默加第二key；報表後須完整比較策略。
- Q3：as_mut_slice；by_key row.0，sort_by兩ref→Ordering非bool；empty/1可。
- Q4：1,1,2,3與(1,30)先(1,10)、E0308，未需排序演算法。

### sort_unstable界線

首次必要：9.7 unstable56前。

docs/chapters/09-07-lesson.md:53

> 對只有數字的清單，這項工作只要升序結果，不再按原位置識別相等的數字，可以選 **sort_unstable**。它仍需要 mutable 借用 slice，按元素的 Ord 升序修改原排列，回傳 unit；排序工作本身不另配置 heap 暫存區，但不承諾相等項目的原相對順序。

docs/chapters/09-07-lesson.md:63

> 結果仍是 `[1, 1, 2, 3]`。這裡兩個1沒有附帶不同身分，沒有需要觀察誰先誰後；若像前面 rows 一樣要保留同鍵項目的原順序，就選 stable 版本。不承諾保序也不表示每次執行一定會重新排列相等項目。

- Q1：同mutable Ord升序/unit，不保等項原序，排序無heap暫存。
- Q2機制/性質（已足夠）：不保序不表示必定重排。
- Q2需要/用途（已足夠）：本段純數字升序不按原位置識別兩1，可以選少保證版本，未歪稱stable不能做。
- Q3：sort_unstable改numbers。
- Q4：仍1,1,2,3，文字承認兩1沒身分可觀察；如rows需原序選stable，無速度推論。

### 雙重比較與owned固定報表

首次必要：9.7 sort-counts83/report91/full95前。

docs/chapters/09-07-lesson.md:73

> 回到開頭的規則：次數降序，同次數才比名稱升序。先用 owned into_iter，把 map 的 `(String, usize)` 收成 `Vec<(String, usize)>`；排序 callback 借來的是兩筆 tuple，不會接手它們的 String。

docs/chapters/09-07-lesson.md:75

> tuple 的 `.0` 是名稱、`.1` 是次數。要讓次數大的左資料先出現，先比較 **`right.1.cmp(&left.1)`**。若左是 Rust 的 2、右是 Python 的 1，1 比 2 得到 Less，排序就把**左筆 Rust**放在右筆之前。次數 Equal，才用名稱正常的左右方向比較：

docs/chapters/09-07-lesson.md:79

> | Rust 2／Python 1 | 先比右次數 1 對左次數 2，Less | Rust 在 Python 前 |

docs/chapters/09-07-lesson.md:80

> | 教學 1／Python 1 | 次數 Equal，再比名稱，Greater | Python 在教學前 |

docs/chapters/09-07-lesson.md:86

> 只按次數排序，Equal 的名稱會保留來源順序；但 HashMap 的來源順序是任意的。因此**stable 不足以保證這份報表每次順序相同**。補上名稱比較後，不同的 map 鍵都有明確位置，這才滿足固定輸出的需要。

docs/chapters/09-07-lesson.md:88

> 把分行／計數／排序接成小函式，回傳 owned 的報表資料。完整檔同時列出上一節的 nonempty_names、count_names；後面的 assertion 另以不同插入順序核對相同報表，並確認來源結束後結果仍可用。名稱已由 to_owned 保存在 map，現在移入結果 Vec，所以原文字結束後，報表仍然有效：

docs/chapters/09-07-lesson.md:110

> 重用 count_names，把報表改為**次數由少到多，同次數仍依名稱升序**。先說清楚哪個比較方向要改，不修改分行、空行或名稱身分規則，也不要為排序 clone 整份名稱。

docs/chapters/09-07-lesson.md:130

> 次數改成 `left.1.cmp(&right.1)`，因此1在2之前；同為1時仍正常比較名稱，Python 在教學之前。各名稱由 map move 進結果 Vec，沒有為了排序 clone 文字；空輸入則交回空報表。

- Q1：mapownedtuple收Vec，callbackborrowtuple不接String；次數反向再名稱正常。
- Q2機制/性質（已足夠）：Rust2/Python1右1cmp左2→Less→左Rust先；同次數教學/Python→Greater→Python先。
- Q2需要/用途（已足夠）：來源3固定輸出；stable保任意map來源仍任意，第二名稱鍵給各不同key明確位置。
- Q3：sort_by matchOrdering，order命名pattern保留非Equal；report接count_names無clone。
- Q4：不同插入順序同報表、逐欄/innerString結束後/空，rare_first只改count比較左右，規則沒換。

## 完整閉包與依賴

全部例、答案、main assertions、14失敗fixture與相鄰期望全讀。每節manifest只package三欄，獨立單檔無library將後章能力提前載入。61完整include occurrences在兩viewport DOM與source逐一匹配，namedregion後另有完整檔；七頁details全0，不靠選讀或repo補主線。

實際先備查核：1.4 f64/浮點有限表示、1.7 heap配置；5.1推導/turbofish、5.3 Unicode scalar、5.4–5.5 Vec/slice；6.1 bound/6.4associatedtype、7章輸出reference/placeholder、8.5to_owned/parse與8.6?同型From。沒有以10/11補缺。9.3有T capture限制只防泛化，現例只有lifetime；9.4 None後可Some限定服務for第一None停止；9.2間接資料不全不變只是限制shared推論，無需當下教Cell等。9.5/9.7不承諾更快，不要求性能測試。

保存工作→callback→opaque工作→逐筆來源→adaptor/目標→文字計數→完整比較報表，每步新需要可定位。9.6刻意延後固定順序，在9.7確回收；不將暫待下一節誤判忘記目標。

## 實際頁面與圖

七頁各1280×800/390×844，HTMLhash/時點見visual/metadata.json。42原始截圖親看，19個pair內含38原圖、另4張窄圖單獨查看；完整名單見JSON。長頁9.3 precise全檔尾、9.4 StepDown答案、9.5 collect/parse_total答案、9.6含main計數尾、9.7報表assert尾都看。長碼手機橫捲是既有呈現方式，未用一屏不下假稱source省略。無fold隱藏必要內容。

實際親看圖僅3張，各兩viewport：

- 9.1 closure-transfer.svg：desktop400×610/mobile343×523。text白框String→第一向下箭頭建立take/move capture→藍take保存ownedString→第二向下箭頭呼叫take/body移出→綠savedString可追加；相鄰150明指兩時點，同一份文字無clone。文字、兩箭頭與角色皆可讀。
- 9.4 iterator-next.svg：400×680/343×583。四框同工作remaining3→2、2→1、1→0、0→0，對Some3/2/1/None；三箭頭同工作再next，底註for第一None停。正文37對state留工作與Item交caller，沒有四個新Countdown混淆。
- 9.5 adaptor-pipeline.svg：400×760/343×652。iter &i32→filter &&i32/bool但保留&i32→true門→map &i32→i32乘10→新Itemi32→collectVec結果0/30/30。正文77與圖底明說collect推進帶前方，箭頭為逐筆資料非完整中間清單。桌面上下視圖均看涵蓋底註；手機整關係可讀。

三served SVGhash皆吻合frozen初稿，精確指紋見JSON。其他四頁img0；receiver/impl方向/Entry狀態/比較方向以表與完整碼已夠，不強求補圖。兩個四欄手機表各列C9-01/02；其餘表合理可讀。9.5 code雙星是dereference，不是粗體格式bug。

未跑Cargo/build/tests，非技術實跑驗收，未驗final strict site與全書，非真人初學者測試。
