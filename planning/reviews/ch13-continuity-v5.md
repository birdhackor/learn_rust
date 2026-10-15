# 第 13 章 ORIGINAL 銜接審閱：凍結 v5

本輪原判：**必要修正 0 項，可選改善 1 項**。已讀 5 頁、32/32 BODY、44/44 archives；必要前文、全部正常範例／完整答案／helpers／tests／負例／Cargo／SVG source 都納入。主要通過的是：這份來源能由已教內容接起「地址問題 → Pin 契約 → 手動 poll 與通知 → 外部事件同步 → 通知安排再次 poll」。這不是 Rust 執行、任意 Future 的安全性、通用 runtime 效能或真實讀者學習成效的認證。

## 角色、版本與獨立性

- 審閱者：`chapter10_concurrency`。我撰寫過第 10 章，做過第 9、11 章非作者 QA；沒有撰寫第 13 章。這是獨立來源判斷的銜接審閱，**不是沒有背景的盲讀**，也不把自己的技術經驗當成基本 Python 讀者的先備。
- 唯一第 13 章來源：`/workspace/work/tutorial-implementation/frozen/ch13-complete-prerequisite-v5`。manifest SHA256：`b4a997a6f10c76039c1a1a0e60a6ad60a395b1c4641b789756ffe4a46704f772`。
- 按 reading order 全文讀 `001-001`–`001-006`、`002-001`–`002-005`、`003-001`–`003-008`、`004-001`–`004-007`、`005-001`–`005-006`；來源先保存於自己的 `observations.md`，才收尾官方 selected contracts 與實頁。完整答案的補讀不能倒填成未開答案時的正文解釋。
- 未讀 FIRST13／TECH13／其他 peer13、作者 notes／ledger／handoff／rootgate、live 第 13 章或第 15 章。未修改教材、共享 Cargo、lock 或 nav。沒有執行 Rust、UB、無界等待或作者整套檢查。
- 依實際已教順序使用前文；第 12 章取最新 `ch12-review-fixed-v5` 的必要部分。前文選讀、目錄與術語表不代替教學。
- 我的 CONT11 cross 原封存 SHA256 `1e5425d212ba204076580f07c8821e15c12b46b642975ff5ede4a73354485305` 保持原 bytes；沒有回寫舊判。

各頁原始 SHA256：

| 頁 | SHA256 |
| --- | --- |
| 13.1 self-reference | `7db483dcf73b09387856ce8775885d3b9dc0b536d4a588bcbc1d12eff15bcdb5` |
| 13.2 Pin／Unpin | `b175ae272a34d7d54b47b0944f899b395659fae38888ed3849a14bd5932957e8` |
| 13.3 poll／Waker | `a0399b3ed78ebf61dbe476a9cabd263295bc244b296239c3301eb401cb2b4ff8` |
| 13.4 外部事件 | `2984f8e3e82e500df96d97b2b72bb5279b6d31aedbde0e429ed40f06f598cae3` |
| 13.5 小型 executor | `b67b7c3cd39646999ce4416533b1e6809e24f6a9a1ae2b7c1f2829ad69729c24` |

本報告的單位／行號指凍結 BODY，不是 live 正文。所有 unit、archive、圖片及前文指紋另存 own work，可由封存清單重核。

## 實際前文與 first-use

本輪全文重開下列 actual 單位，不宣稱重讀 1–12 所有頁：

| 當下必須理解的關係 | actual 來源與已教內容 | 本章第一次需要／收益 |
| --- | --- | --- |
| 借用、move、有效資料與地址是不同問題 | 4.5 borrowed model（`ch04-publication-target-fixed-v8/005-002`，本輪 selected 重開、先前 QA 全文背景）；12.3 latest v5 全 7 units，raw pointer 的建立、null、有效性、alias、provenance、reference 轉換與 UB | 13.1 `001-002`–`004`；不能把 lifetime 有效或地址相等代答內部記錄正確／可讀權限。 |
| unsafe 能力與 safe abstraction 的整體責任 | `ch12-review-fixed-v5/004-001`–`006` 全文：Safety／SAFETY、所有 safe 介面、私有欄位與整體不變量；有限測試不能代證明 | 13.2 `002-004`；new、setter、getter 與 Drop 一起納入，不只看 unchecked block。 |
| Box owner／heap 中的 T／可取出 T | `ch08-review-fixed-v9/001-005` 全文 | 13.1 `001-005`；交換 Box owner 與交換整份 T 的對照有已教 ownership／解參考依據，且普通 Box 不禁止取出 T。 |
| 正常清理、Drop receiver、顯式 drop | `ch08-review-fixed-v9/002-002`、`002-003` 全文；Drop 是 `&mut self`，`drop` 接手 owner；Copy／shared reference 的副本清理不等於清理來源 | 13.2 Drop 契約、13.4 離鎖再清理／通知、13.5 返回時清理。不借最後使用推定 guard 已解除。actual10.6 `006-002:12–13,39–54` 也已明教自由函式 `drop(guard)`。 |
| associated type、typed receiver 與 trait method | `ch06-review-fixed-v10/002-003` 全文，`Self::Output` 是型別，與 instance 欄位不同；此前實際 trait／impl／borrow 語法 | 13.3 `003-003` 的完整 `Future::poll` 簽名；typed `Pin<&mut Self>` 在13.2當場補教，`self: Arc<Self>` 在13.3當場補教。 |
| channel 訊息與消耗 owner 的事件不同 | `ch10-review-fixed-v15/005-002` 全文：buffer、send 借 Sender、recv 等待、關閉／Err；不是無限記憶體 | 13.4 control i32 channel／一次 `AmountEvent::send(self, value)`；13.5 通知 unit channel，資料仍在 State。 |
| Arc／Mutex 分工、guard 結束、等待循環與 Send／Sync | `ch10-review-fixed-v15/006-002` 本輪 selected 重開；第10章原作者 actual 全文背景 | 13.3計數觀察器、13.4同鎖檢查／登記與解鎖回呼；不是把 Arc 當 synchronized State。 |
| 保存工作與等待借用其內部資料 | `ch11-review-fixed-v15/005-003` 全文，actual `Job`／`job.release.recv().await` | 13.1 `001-001:3` 的原需可對到真實前例，不由 self-reference 名稱自行編出問題。 |

前述前提的 SHA 與讀取範圍保存在 `actual-prerequisite-fingerprints.json`。原有1–10作者／QA背景支持基本語法、Option／Result／match、泛型、trait、Cargo／module／use；並未把「Outline 有提」計作已教。12.1 GAT、12.2 variance、C／FFI 不承擔這一章必要操作的推理，未為本輪而無差別重讀。

在首次實碼前，本文已補的 selected 契約包括 mem::swap、ptr::eq、Pin getters／Unpin、PhantomPinned 的 unit struct、typed receiver、Poll、Context、Waker clone／wake、Wake 的 Arc receiver、From<Arc<W>> bound、Waker／Arc／Sender 的選定能力、Option::is_none／Copy、`--no-run` 與 wake_by_ref。下面把機制與用途分開核對；後文解說不當成前面已教。

## 逐概念四題與主要通過範圍

以下 Q1 是身分與前文關係，Q3 是輸入／操作／輸出；各自列出，不用用途代操作。Q2M 為機制／性質，Q2N 為原需／用途，分別記依據與判定。Q4 限定材料支持的關係。每項依本輪 BODY、已教前文與實讀 source 判斷；實碼對照支持閱讀／關係核對，不是本輪執行結果。

### 13.1：從資料有效，走到位置記錄

**A1 self-reference 與 async 等待狀態。** 首次 `001-001:3` 問「內部保存的位置資訊，之後仍對得上同一處資料嗎？」；`001-002:3–14` 緊接等待狀態示意。

- Q1：值內保存的位置資訊對到同值另一部分；actual11.5外層保存 job、內部 recv 等待借其收端，是引入線索。不是所有 Future 一定具有實際 self-reference。
- Q2M：保存資料和內部借用可能讓未來恢復使用依賴原位置；示意呈現這種關係，並沒有承諾 compiler 的精確 struct layout。**已足夠**。
- Q2N：原工作要在等待後繼續使用同資料，而「還活著」不能單獨回答內部位置是否仍對。先以數字／地址移除 channel 旁支，保留需要觀察的關係。需要來自原句與 actual11.5，不是由 raw pointer 方法反推。**已足夠**。
- Q3：先讀圖與問題，不必先自行寫自借用 struct／Pin。Q4：圖支持內部資料與借用的角色；未證明每個 async future 的實際佈局、地址或欄位。

**A2 lifetime／move／地址穩定三分，及 whole-value swap 的借用界線。** `001-003:3–51` 明說「ownership 交接不會自動修正值內的 pointer 記錄」及不能把每個 move 當搬動 heap 全部資料。

- Q1：lifetime 是借用有效性、move 是 ownership 轉交、地址穩定性是某段期間依賴同位置；`std::mem::swap` 接兩個同 T 的 `&mut T`，交換整份 T、回 unit，不當場 Drop。
- Q2M：whole swap 要可變借用整份值，與後續仍用其內保存 shared reference 衝突；`BorrowedNote<'a>` 的 negative 保留實際 field borrow／後用，所以是 E0502。**已足夠**，不是「Rust 禁止任何內部 reference」。
- Q2N：原問題要辨認 owner 是否仍有效與位置記錄是否可用。分開三問，才可公平理解下一個可編譯、只觀察地址的模型；沒有把 raw pointer 當成忽略借用規則便可安全讀值。**已足夠**。
- Q3：swap 的 reference／泛型與 Option reference 均沿 actual 前文；本節給所選交換契約後才用。Q4：負例只支持這個 whole mutable swap 與保存借用後用的衝突；未執行負例，也不泛化所有 self-reference 表示都不可能。

**A3 AddressNote 的位置記錄與 ptr::eq。** `001-004:3–103`、`src/addresses.rs`／`inline_swap.rs`。

- Q1：non-Copy AddressNote 保存 i32 及私有 `*const i32`；null 表示尚未記錄，remember 在 caller 已放定後記欄位地址；ptr::eq 是本例薄 pointer 的地址比較。
- Q2M：whole swap 搬數字與 saved，A／B 是固定欄位位置，因此 saved 隨整份值換位而不會自動重記；地址相等不授予存活、provenance 或解參考權限。**已足夠**。
- Q2N：需要觀察「值仍存在，但內部位置記錄失配」。80／120、以及80／80的對照排除用數值相等代答地址關係。這是原需的具體材料，並非新方法自訂成功標準。**已足夠**。
- Q3：new → 在定點 remember → whole swap → address_matches；從不透過 saved 解參考。Q4：四項 source tests 包含 refresh 與相同數值，支持已教位置關係的有限對照；未支持任意 unsafe 使用安全、固定十六進位地址或全部 move 都會換地址。

**A4 Box owner swap 與 field-only swap。** `001-005:3–53`、`001-006:3–57`；`box_handles.rs`／`field_swap_solution.rs`。

- Q1：一個 T 是 Box<AddressNote> owner，另一個 T 是欄位 i32；並非同一層的交換。`&*`、解參考與 heap owner 由 actual8.1支撐。
- Q2M：Box owner 交換不搬 heap 中 AddressNote；只交換 i32 內容既不換整份 Note，也不搬 saved，因此兩者仍匹配。普通 Box 仍能取出 T。**已足夠**。
- Q2N：原地址需要不變，換交換對象以指出哪一份資料必須穩定，避免把任何 ownership 交接都說成破壞 self-reference；普通 Box 的能力又引出下一節的介面限制。**已足夠**。
- Q3：兩個 owner／兩個 i32 借用可由正文預測；完整答案放折疊但三種操作及關係在正文已有交代。Q4：whole／owner／field 三對照實際發揮不同分工；Box 試驗沒有證明 caller 不能稍後移出 T。小節末 `001-006:55` 回收地址問題並交接 Pin／Drop，未提前依靠其 API。

### 13.2：穩定位置成為共同介面契約

**B1 Pin<Ptr>、pointee 與 library／Drop contract。** `002-001:3–5` 原需「這份原值仍有效時，不能任意移出它所在的位置；收尾也要在該位置完成」；`002-002:3–18` 回應。

- Q1：Pin 包 pointer，固定約定的對象是 pointee。Pin<Box<T>> owner、Pin<&mut T> 可變借用與 Pin<&T> shared 觀察並列；wrapper 移動不等於 T 換位置。
- Q2M：safe API 限制能力，unsafe 作者維持契約；不是改 move 語意或自動修正 saved。位置與 storage 必須保持到 Drop 在原位返回或 panic，而不是保證一定最終 Drop。**已足夠**。
- Q2N：普通 Box 不限制後續取出；交給外部使用者時需要共同約定，不能每次猜不同 future 欄位。把單型別內部保護提升為可共享的標準介面；原需獨立於 Pin 型別名稱。**已足夠**。
- Q3：區分可移動 wrapper 與受約束的 T；具體 getter 稍後才用，當前不依賴全部 API。Q4：三種 pointer 身分與圖的單次 owner 轉交完成此分工；不承諾 Pin 一定配 heap、延長借用或完全阻止 unsafe 作者破壞約定。

**B2 Unpin、Pin::new／get_mut 與 PhantomPinned。** `002-003:3–44`、`002-004:5–24`。

- Q1：Unpin 是不依賴 pinning 限制的 marker auto trait；selected i32 可用。`!Unpin` 是解說 notation，並不表示尚未 pin 的值已不能 move。PhantomPinned 是沒有欄位的 unit struct，其欄位讓外層不自動 Unpin，`_pin` 不是特殊 pinning 語法。
- Q2M：Pin::new 與 Pin<&mut T>::get_mut 的安全通道要求 pointee T:Unpin；後者消耗借用 wrapper 交回 &mut T，owner 沒有離開 caller。**已足夠**。
- Q2N：讓同一標準介面處理不依賴固定位置及依賴位置的型別，同時把 unrestricted mutable 能力放在有條件的界線。i32 swap 的可行與 PhantomPinned 的 E0277 共同反駁「所有 Pin 都不可改／移」的誤讀。**已足夠**。
- Q3：80／120 的 Unpin 實驗有 T 與輸出；標記的建立方式先教才用，負例僅編譯失敗。Q4：這些對照支持 selected bound，不證明任何 !Unpin 型別具有真正 self-reference 或其所有 API sound。

**B3 PinnedNote 的 constructor、typed receiver 與 safe abstraction。** `002-004:26–106` API 表／完整實作；`src/pinning.rs`。

- Q1：private value／saved／PhantomPinned；new 回 Pin<Box<Self>>，as_ref／as_mut 分別借同一 T；get_ref shared 讀取；typed self 明說 `Self` 仍是 PinnedNote，非任意型別可當 receiver。
- Q2M：Box::pin 先安置整份 Self，再用 unchecked getter 記地址；setter 只改 i32 內容；沒有向 caller 交 &mut Self 或任意移出 T。private 欄位、所有 public 方法與 Drop 一起維持位置，不能只憑 SAFETY comment。**已足夠**。
- Q2N：要讓 caller 能觀測／修改數值但保持內部位置關係；新 unsafe 能力正用於安置後的記錄與受控內容修改。`002-004:49–59` 把此需要、較大能力和12.4整體證明責任相接。**已足夠**。
- Q3：API 表在調用前說明 wrapper 輸入／消耗、返回、T 條件與 unsafe 責任；Setter 接 PinMut 與 i32 回 unit，typed receiver 新寫法當場解釋。Q4：set140後 matches 仍真是內容與位置分離的有限觀察；不主張任意 field projection 安全，i32 field 沒有被移交為獨立 pinned field。

**B4 Drop、owner 移交、短 reborrow 與跨 crate 負例。** `002-004:59,148–209`、`002-005:3–44`，`pinned_owner_swap.rs`／`pinning_solution.rs`／`pinned_get_mut.rs`。

- Q1：Drop 仍收普通 &mut self；as_mut 在擁有 PinBox 的 caller 建短 PinMut wrapper，使 consuming method 後仍能再次操作 owner。
- Q2M：Drop 只在原位觀察，所有可訪問路徑維持約定；交換 PinBox owners 不搬 pointee。通過 as_mut 重新借用不是 clone T，不能把私有 unchecked getter 變成 caller 任意移動的許可。**已足夠**。
- Q2N：原需要含「收尾也在該位置」；正文主動審 Drop 並解釋 storage reuse 前先收尾。短 reborrow 則讓同一受限界面連續改／讀而保留 owner。**已足夠**。
- Q3：移 first→next、set100、drop next 在完整練習答案中均為已教操作；tests 多次 as_mut 有主文說明。Q4：正常 Drop 輸出不是整體安全證明；跨 crate get_mut E0277 與 safe 能力邊界相符。小節末把共同 PinMut receiver 接往 Future，不要求此時自寫 pin projection。

### 13.3：通知既不是再次 poll，也不是完成

**C1 poll、Poll<T> 與完成界線。** `003-001:3–5` 原需問通知發生後誰繼續；`003-002:3–19` 定義。

- Q1：poll 是檢查／推進狀態的動作；Pending 沒有 T，Ready 交 T；T 可以是 Result，Ready(Err) 仍是 Future 完成。
- Q2M：poll 應儘快返回，不能等授權、緊密重試；通知只提示應再檢查，新 poll 仍可能 Pending。完成後不能因 safe 再 poll 造成 UB，但一般 Future 不保證重複 Ready 可用。**已足夠**。
- Q2N：原問題需要把「資料獲授權／通知／poll／答案」分開，才能回答誰實際推進工作；不是把 wake 名稱換成執行即可。**已足夠**。
- Q3：driver 調 poll 並 match，Ready 停止；selected ManualAmount 選擇再次 poll 時 panic。Q4：手動兩 poll／先授權直接 Ready／postReady 負例分別檢驗狀態關係，不能推所有 Future 必經過 Pending 或都有同一種完成後行為。

**C2 Context、owned Waker clone 與完整 poll signature。** `003-003:3–33`。

- Q1：Context<'a> 借這次 Waker，'a 不是 Future 年齡；Waker owned handle 只通知同目標，不是工作副本或答案。Future::Output 由 actual6.2支撐，PinMut receiver 由13.2支撐。
- Q2M：from_waker 借原 Waker，waker() 返回 shared 借用；要留到 Pending 後需 clone 成 owned handle，原 Context 結束仍保有通知目標。**已足夠**。
- Q2N：原需要是條件稍後改變時仍能找到應通知的目標，不能延長短借用代替存儲通知權；這種 ownership 轉換在第一次保存前完成。**已足夠**。
- Q3：簽名的 outer &mut Context borrow 與 Context 內部 lifetime 分開；use std::future::Future 在 trait method 實際使用前給出。Q4：context變換練習及 clone 生命週期 test source呈現 owned 關係；不承諾 clone Future／延長工作資料借用。

**C3 Wake、Arc typed receiver 與 Waker::from selected bound。** `003-004:7–18` 與 source `WakeCounter`。

- Q1：Wake 的 required callback 是 `wake(self: Arc<Self>)`，接一份保活 callback object 的 Arc、回 unit；計數器只加通知數，沒有 scheduler。
- Q2M：From<Arc<W>> 要 W:Wake+Send+Sync+'static；不是 Wake trait 自己定義的全部要求。Mutex<usize> → Arc 的能力由 actual10.6和本文 chosen type 推得；static 排除短局部借用，並不永遠存活。**已足夠**。
- Q2N：先把不可見通知變成可讀計數，保留工作與通知的分工，才能看出 wake 後沒有自動 poll。計數器為這項觀察服務，不把新計數當作原業務答案。**已足夠**。
- Q3：new／from／snapshot／wake 短 lock；caller 不持計數器 guard 調 wake／poll，poison expect panic 而不修復。Q4：小次數 count 支持本例同步通知發生，未證明公平排程、任意次數不溢位或通用 thread-safe 泛稱。

**C4 ManualAmount 狀態、release 順序與 latest Waker。** `003-005:3–97`、`003-006:3–91`、`003-007:3–69`。

- Q1：五 fields 區分 value、released、completed、polls、Option<Waker>；Waker／Option 的 selected Unpin 在 get_mut 前說明。
- Q2M：Pending 保存最近 Context 的 owned clone；release 先改狀態、take waiting、再 owned wake。release 不 poll；第一次前 release 可直接 Ready 不需通知。**已足夠**。
- Q2N：條件改變後應通知當前使用端，且不能把授權當完成。正文第二 Context 練習顯出只記首次目標會通知舊端；兩計數對象是獨立觀察器，改 Context 不授權。**已足夠**。
- Q3：main 先 poll→Pending，release 後通知1、polls仍1、completed false，再 poll→Ready80；最新目標例額外一 Pending是受控檢查，不忙等。Q4：released/completed/polls 屬於 work，通知數屬於 counter；diagram 三框有實際 driver 第三步。先 release120 firstReady 的材料解除「必有 wake」誤解；不是新 Context 導致未來自動執行。

**C5 tests、`--no-run` 與 postReady fixture。** `003-008:5–48`。

- Q1／Q3：`polling::tests` 是 test name filter，`--no-run` 是僅編譯不執行，在指令前已解釋；manifest-path 是此前已教 fixture 路線。四項 library tests 與 chosen panic negative全source已讀。
- Q2M：tests 檢查上述狀態與 owned handle 行為，postReady fixture選擇普通 safe panic。**已足夠**。
- Q2N：讓讀者驗證通知／執行／完成的界線，而不是只接受 printed labels；不能從一項 chosen panic 推標準要求所有 Future panic。**已足夠**。
- Q4：本輪只是 source／命題對照，沒有運行；小節末 `003-008:48` 回到 await 的工作／通知／driver 分工，並給外部同步的新問題。

### 13.4：外部事件有結果，也必須有正確的通知次序

**D1 兩份 owner、一份 State、一次供值。** `004-001:3` 原需「事件卻在它登記通知前到達，之後還有人叫它再檢查嗎？」；`004-002:3–44`、`004-003:3–40`。

- Q1：EventAmount 保 future 狀態與 State Arc，AmountEvent 持同 State 的另一 Arc；不是 clone State。State 的 Some／None 表待取數值，非業務錯誤；取走後的 None 與 completed true 要合看。
- Q2M：amount_pair 創建兩權利、不啓動 worker／poll；AmountEvent::send 接 self 一次供值，無 Clone／公開 field 復造權限。Arc 清理一 owner不立即清另一。**已足夠**。
- Q2N：保持 main 有工作、worker 有供值能力，允許事件時序獨立，但 main最後取同結果；一次事件不是把 Sender::send 改成 owned。**已足夠**。
- Q3：tuple 拆 future／event，worker.consume event，main 保 future；drop事件無結果不等於Ready／Err／保證wake。Q4：所有 source 固定 finite producer；producer 消失可永 Pending 是正文邊界，本輪不執行這種路徑。

**D2 同一 guard 的檢查／登記，與兩種先後。** `004-004:3–24`、`004-004:45–87`。

- Q1：一段 guard 要麼 take已存在結果，要麼存 current Waker；並非只各 field 都用了某個 Mutex便足夠。
- Q2M：錯分兩鎖的四步可讓 event 看不到 waiting、future後來登記卻無新事件；同 guard 排除這個插入窗口。事件先來由結果直接Ready，poll先來已有Waker再通知。**已足夠**，來源給同步因果，不借「測試反覆過」代證明。
- Q2N：原需是不丟掉該再次檢查的通知；保持同業務數值，改變原子區段使兩種次序都可回到取結果。需要獨立於新的 Mutex設計。**已足夠**。
- Q3：clone 先在鎖外；guard 內 take value／舊 waiting、在無值時登記；owned tuple 出 block沒有帶 guard；Option::is_none 的最低契約在 source前教。Q4：兩欄時序圖的 coloured block 是完整同鎖區段，gray箭頭是順序，不是量測時間。受控先後加 source推理不等於 stress race proof。

**D3 離鎖再 wake／清理舊 Waker，及短 Mutex 的邊界。** `004-004:25–44,88`。

- Q1：event guard 內 store value／take waiting，block 外 owned wake；poll 把舊 owned Waker 取出後離鎖 drop；callback 是 driver提供的動作。
- Q2M：持 State guard調用可能讀同State的callback會形成循環等鎖；先更新／離鎖使 callback可見新值。舊通知資源的清理也不留在此鎖內。**已足夠**。
- Q2N：需要通知使用端重新檢查，不能為了通知反而卡在自己持有的 State 鎖。保持同結果／登記者關係，把可再入／資源清理動作移出同步區段；原用途有 `004-004:44` 明說。**已足夠**。
- Q3：只做短狀態操作，沒有 recv／join／事件等待／I/O／callback 在 guard 內；普通 lock競爭仍可能阻塞，poison不自動修復。Q4：此模型未宣稱 fully nonblocking、lock-free、通用executor latency／throughput；後面 callback test提供一條具體重取鎖路徑，不包辦所有callback。

**D4 跨 thread 能力、control channel 與 join。** `004-005:3–128`、`004-006:3–76`。

- Q1：Waker Send／Option Send／State Send → Mutex Send+Sync → Arc Send及 owned static，在首次spawn前給出；worker保event、main保future，控制i32訊息是另外channel。
- Q2M：recv等待在worker，join在main driver，均不在Future::poll。join只確認worker及同步callback返回，不能替future交Output；standardpanicpayload Debug在expect前selected。**已足夠**。
- Q2N：固定「先Pending登記，再準供80」或「先供120，再第一次poll」，使本例變化服務於兩個同步情況，不靠sleep猜時序。**已足夠**。
- Q3：第一條 join之後poll仍1／未完成，需要第二poll；反向第一poll可Ready且不需wake；latestcontext練習沿相同狀態機制。Q4：80／120是不同chosen工作，不是ID／時間測量；正常finite handles retained+join不泛化所有panic路徑都有完整cleanup。

**D5 ReadinessObserver helper 與一次供值負例。** `004-007:5–87`，`readiness.rs`及`event_send_once.rs`。

- Q1：observer Arc連同一State，另一Mutex存 `Option<i32>` 副本；Option<T>:Copy when T:Copy 與 child test module可讀parent private規則在helper前說明。
- Q2M：callback取得原State鎖複製結果，離鎖後存自己的observed；不poll、不take結果。worker join後source檢查observer0和future下一poll同0。**已足夠**。
- Q2N：直接顯出離鎖通知的工作：callback能讀已供結果又不會奪走future結果，完成前述循環等鎖問題的一個實際chosen場景。**已足夠**。
- Q3：tests中unit／Option／match／assert都是已教；event再次send負例為E0382，非channelSender普遍一次。Q4：source與正文四tests用途相符；本輪未跑。丟future不強制停worker，保存handle並join；下一節仍需解決誰安排再次poll。

### 13.5：維持事件規則，改由通知驅動同一工作

**E1 executor、ChannelWake 與 unit notification。** `005-001:3–5`、`005-002:3–43`。

- Q1：一份工作的executor持future，ChannelWake只持Sender<()>；State仍存80及最近Waker，worker沒有future。一個已知工作無需在通知裏帶另一個工作或業務值。
- Q2M：wake送unit進buffer，既不poll也不取Output；Receiver留driver。Sender的chosen Send／Sync／static在From前給出，send不等receiver取走、不為buffer空位等待，不等於lock-free。**已足夠**。
- Q2N：原工作已能正確供結果，新增需要是把手寫join後重poll改成收到通知後安排重查；unit只承擔此安排，保留13.4 State／事件機制。**已足夠**。
- Q3：callback忽略driver已清理Receiver時的Err，既不panic／重試也不取消外部worker；活動driver仍保Receiver。Q4：分工表及圖可追 unit notification vs control80 vs State80，不能由二者都是channel推答案流經通知channel。

**E2 run_one<F> 的所有權、Pin與loop。** `005-003:3–45`、`runner.rs`。

- Q1：F:Future是傳入owned工作，F::Output是associatedtype；F留callerthread，不要求F:Send／static。Box::pin(F)可安置!Unpin F。
- Q2M：每輪as_mut短reborrowpoll；Pending後才recv等通知，再檢查；Ready直接返回不清完notifyqueue、不再poll完成工作。沒有把wake直接變答案。**已足夠**。
- Q2N：原需是通知發生後由driver推進同一future；保同owner／地址契約＋阻塞在poll外，避免緊密重試並連接await的頂層推進角色。**已足夠**，不承諾比任何runtime更快。
- Q3：returnOutput同時正常清futureowner／Context／Receiver；Waker仍持Sender，因此事件owner消失不讓通知recv自動closed。Q4：已講可能繼續Pending／永無事件，不執行無界例；普通expect不製造timeout或通知。不支持任意Future liveness、I/O／timer／fairness。

**E3 StartAmount controlled wrapper 的選擇與次序。** `005-004:3–96`。

- Q1：StartAmount持已教EventAmount、Option<Sender<i32>>，前者Unpin、後者selectedUnpin使chosenwrapper可get_mut。inner Pin::new 只依已教EventAmountUnpin，不是一般struct projection規則。
- Q2M：先innerPending完成登記，才take controlSender送80，然後outer返回Pending；worker可在outer返回前wake，但notifybuffer保留unit。Optiontake使啓動只一次。**已足夠**。
- Q2N：原需是看見通知安排再次poll，而事件若過早會合法firstReady使關鍵cycle不發生；正文主動指出chosen控制理由，而不是事後由讀者挑情境。保持同80／State規則，只控制一次啓動。**已足夠**。
- Q3：poll內無recv／join；controlsend是此前buffered send，等待仍在driver外；run返回80後另joinworker才完成例子收尾。Q4：圖首框是innerFuturePending，不誤承諾outerPending一定早於worker。藍點線追同future，綠buffer存unit，橘供80／callback另有角色；不是把通知順序或stdout當量測排程。

**E4 early wake_by_ref 與 !Unpin Future 對照。** `005-005:3–100`，`executor_early_notice.rs`／`executor_pinned.rs`。

- Q1：wake_by_ref shared借同Waker、不消耗；Wake默認方法cloneArc再調requiredwake，在首次code前教。PinnedAnswer帶PhantomPinned，沒有真實self-reference。
- Q2M：Recheck第一poll主動通知再Pending、第二Ready120，buffer保留早通知；PinnedAnswer經sharedas_ref/get_ref讀Copy i32直接Ready，driver無需F:Unpin也沒有unsafe。**已足夠**。
- Q2N：原需要分別含通知不能因「比recv早」丟失、標準PinMut接口能處理!Unpin工作；兩獨立對照各覆蓋一關係，不能用firstReady代通知cycle。**已足夠**。
- Q3：同一run_one入口兩target；shared讀取T無任意mutable移動能力，owned值pin前可move。Q4：標記只支持接口能力，不證明self-reference真實存在；noThread的早notify對照不是cross-threadstress實驗。

**E5 練習、normal tests與結尾回收。** `005-006:3–50`。

- Q1／Q3：改control業務值120，通知仍unit；完整答案保State／sameguard／driver／workerjoin分工，均當時已教。三runner測試支持正常供值、早notify、!Unpin接口三chosen情況。
- Q2M：複用相同工作機制與driver loop，只換供值；futurehandle不主動執行／延借用／修地址。**已足夠**。
- Q2N：回到11async／await如何推進，說明13Pin／Future／Waker契約連起來源原問題；並不以小executor宣稱替代TokioI/O runtime。**已足夠**。
- Q4：測試source與練習值變化為有限關係核對；沒有量測性能或目標讀者遷移效果。本章最後的下一步只作為範圍內過渡，未擴審14／15。

## 所有實碼、測試與失敗材料的範圍

44archives的組成是16normal examples（含5完整練習答案）、5helper modules＋lib.rs、4compile-fail、1cargo-test-fail、5expected、3Cargo.toml、2獨立fixtureCargo.lock、7SVG。所有文件全文實讀，完整source沒有只查主函數／作者說明。

- `addresses.rs`4、`pinning.rs`3、`polling.rs`4、`readiness.rs`4、`runner.rs`3，共18library tests：helper／斷言／typed receiver／trait method／模塊路徑皆和本節之前實際已教內容對照。無借未教Poll Debug／PartialEq替代狀態解釋；用match與primitive斷言。未把普通String和&str跨型別比較藏在答案中。
- 正常package Std-only，沒有第三方Future crate的隱形helper。3個Cargo manifests／2locks與actual9package/path／空workspace隔離方式相符，fixture回到本package；本輪不修改workspace／lock。
- 四個compile-fail原因按source分別為reference whole swap後續借用E0502、Pin::new標記E0277、跨crateget_mut要求Unpin的E0277、owned一次eventsend的E0382。第五chosenruntimefail是ManualAmount完成後再poll的panic。expected全文已讀、和所教規則對應，**未親跑重現**，因此不宣稱實際stdout／stderr一致或負例PASS。
- 每節可先理解正文機制，再決定自己練習／開摺疊答案；必要API契約沒有僅藏在答案。源碼以正文給予的選定能力使用，並沒有在後節才解釋source已依賴的新方法。

## 實頁、圖中身分與閱讀負擔

我用自己的snapshot preview，桌面1280×800／手機390×844，在正文讀完後的獨立版面時點**親看74張**：20張5頁entry/end、28張7圖context/element、20張5摺疊答案closed/open、4張13.2API表context/element，另2張正常向下閱讀的補查。沒有借root／同行畫面當本人的視覺證據。

重組只去重複H1、把凍結assetURI改相對路徑；原32BODY的code／heading／table／fold／順序不改，7SVG複製原archive同SHA。正確複製CSS並在ownconfig明確`extra_css = ["stylesheets/tutorial.css"]`，不僅寫設定：實際browser link所載HTTP200內容，10次page/deviceentry的SHA均為`fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`，與copy相同。own strict build真實exit0（0.23s），這是preview建置結果，不是Rust編譯結果。

圖文逐項支持：

| 圖／位置 | 實際可追的必要對象與關係 | 通過範圍／不能代答 |
| --- | --- | --- |
| future-internal-borrow／`001-002` | outer保存job，內部等待借收端；箭頭是內部關係 | 佈局示意，有當下正文解釋；不是compiler佈局／所有Future都self-reference。 |
| self-reference-addresses／`001-004` | A／B固定field位置，before/after兩整份value，saved cross箭頭 | 數值換位與pointer記號換位分開；相同數值不能代同對象。圖不授權raw讀取。 |
| pin-owner-transfer／`002-004` | first灰框為先前名稱，next藍框保同owner，heap只畫一份，V是value位置，purple是saved記錄 | 單次owner轉交對應正文，不是畫成兩份heap；沒有表示sharedclone／新增T。 |
| poll-cycle／`003-006` | work三field released/completed/polls，counter通知數，第三步driver再poll | 第二wake框未假裝已執行第三poll；箭頭是操作順序，不是時間測量。 |
| event-owners／`004-002` | mainfuture與一次event兩Arcs，指向同MutexState；polls/completed仍屬future | 兩owner不cloneState／worker不持future；值None要和completed對讀。 |
| event-order／`004-004` | eventfirst／pollfirst，各colored區段一guard，離鎖callback在區段外 | 最短因果可排除登記窗口；gray箭頭不代表platform排程公平／stress證據。 |
| executor-notice／`005-004` | innerPending登記、橘event供80／wake、綠bufferunit、藍driver再poll，藍點線同future | control業務值和notificationunit不同，Output來自poll；圖首innerPending不保證outer已返回。 |

7SVG原24px字體，手機343/400縮放後約20.6px；主要field／箭頭／legend可以在普通尺度辨認，必要機制不依賴點放大。長圖正常豎向滾動看底段；桌面巨大element捕捉有工具自動scroll後stickyheader覆蓋圖頭的artifact，context圖頭清楚、element補足底段，**沒有把一張被覆蓋的element圖宣稱全圖無遮擋**。

五folddefaultclosed，必要身分／機制／用途在主文；實際展開code12.8px與ordinarycode相同。長代碼橫向滾動在codeblock內，entry/end沒有整頁寬度溢出；這不等於每一行code在單截圖裏都已完全呈現。圖的首次需要位置、對應caption與轉場沒有靠讀者提前開答案補洞。

手機13.4的自動openfold捕捉曾看到navigationtop按鈕覆code：工具先把長fold居中又上跳至summary，會觸發該按鈕。沒有把猜測列為源碼問題。另做有限markup-only補查，從entry正常向下到closedsummary、click後不居中／上跳、再正常向下350px，兩張親看畫面沒有按鈕覆蓋controlsend／drop／join／count必要行。原72張保留，新增2張是對原觀察的補查，不是刪除不利畫面或再跑Rust。

正文轉場每次保留相同原需而增加一個主要責任：地址記錄→對外固定契約→通知與執行區分→併發檢查／登記→通知安排driver。normal前文複習雖包括12unsafe、10鎖、11await，但需要處給具體關係，非反覆跳外連補未教；五答案都可關閉讀主線。一個可選表內名稱拆行是本輪唯一具體呈現改善，未觀察到需要額外知識才能維持主線的累積缺口。

## I1：可選，手機API名稱保持可辨認

- 版本／位置：v5，`002-004:26–34`的selectedAPI表，第一列原文 **`view.get_unchecked_mut()`**。實際圖：ownpreview `mobile-pin-selected-api-context.png`／`mobile-pin-selected-api-element.png`；desktop同位置名稱完整。
- 已教部分：該行第二列準確說明unsafe接手PinMut→&mut T、不得移出／須維持Drop契約；後面的完整constructor／setter調用和SAFETY說明都在同一主文。不是先備遺漏、HTML吃掉泛型或新API無契約。
- 本人觀察：390px手機第一列顯示成`view.get_unc`／`hecked_mut()`；Box::pin(value)參數亦拆行。需要把identifier拼回再對應隨後code，略增查名稱的負擔。
- 分級：**可選呈現改善**。現行字仍可讀、同頁完整調用與語義已足，沒有阻斷當前判斷／安全責任；不把單一普通窄屏換行升級成必要概念缺口。
- 最小建議：對identifier保持不換行，或把表頭／操作標籤縮短並保完整methodname，第二列繼續保輸入輸出與unsafe責任。收益是名稱對照更直接；不增加Rust API、旁支語法或例子，不需要重跑Rust來證明文字佈局修復。
- 發現來源：本人original實頁觀察，保存於ownobservations之後整理；未受peer／root清單提示。未修改／複查修後版本；最終是否採用待協調者裁定。

## 官方契約核對與證據邊界

自己的`contracts/`保存fixed Rust1.99.0官方源與SHA；全文archive只是取證存儲，actualread是下列selected範圍，不宣稱全部8官方大文件全文實讀。源碼不執行，不引入官方內部RawWaker／unstableLocalWaker／額外trait作為學生先備。

| fixed來源 | 實際selected範圍／支持 |
| --- | --- |
| core/src/future/future.rs:37–113 | Pending最新Waker、通知後再次poll、可合併／spurious通知不必Ready、不tightloop、poll應快、完成後safe調用不能UB；`Pin<&mut Self>`／Context／Output。 |
| alloc/src/task.rs:90–124 | WakerequiredArcself、defaultwake_by_ref cloneArc再wake、From<Arc<W>>的Wake+Send+Sync+'static條件；不混作Wake定義。 |
| core/src/task/wake.rs:218–255,411–465 | Context借Waker，from_waker／waker、Waker Unpin／Send／Sync、wake接owned與wake_by_ref借用；通知可coalesce，不要求一wake一Ready。 |
| core/src/pin.rs:507–568,1159–1188,1360–1435,1555–1628 | Drop返回或panic前有效同位置storage；safe new／get_mut需TargetUnpin；as_ref／as_mut短reborrow、get_ref、unsafegetter不移動責任。Dropguarantee非保證一定drop。 |
| core/src/ptr/mod.rs:2460–2508 | ptr::eq比較地址／wide metadata；這裡薄i32selected只有地址比較，非存活／provenance／解參考權限證明。 |
| core/src/mem/mod.rs:808–834 | swap兩mutable同T、不deinitialize、不是當場drop；不自動刷新作者saved。 |
| alloc/src/boxed.rs:349–361 | Box::pin不要求T:Unpin。 |
| 1.99rustdocselectedimpl：Arc、Sender、PhantomPinned／Unpin | ArcUnpin不要求StateUnpin；Sender<T>Unpin及T:Send的Send／Sync；PhantomPinned明確!Unpin；支持本文chosenwrapper／跨thread能力。 |

本輪確認的是這些能力／邏輯在使用前獲得當下解釋、與官方selectedcontract一致，並有正文／圖／source的對應。沒有重新執行18librarytests、16normal、5negative；不借作者PASS當本人實跑。也沒有Miri／UB實驗／任意callback壓力競態／runtimeI/O／timing或實際初學者測試。正常有限例的join清理，不證明所有faultpath自動收尾；死等邊界是正文明確說明，未執行它。

ownwork保留：`observations.md`、`integrity-background.json`、`actual-prerequisite-fingerprints.json`、`preview-input-proof.json`、`contracts/fetch-manifest.json`、ownpreviewconfig／buildresult／render-metrics／screenshots／fold-flowprobe，以及原判seal與filefingerprints。後續cross只能追加另一報告，不覆寫本原判或把同行提示寫成原獨立發現。
