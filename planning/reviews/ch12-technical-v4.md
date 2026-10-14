# 第 12 章獨立技術原判：唯一 original v4

結論：完整閱讀後，列出 **2 項必要的先備補充、1 項可選的圖文精確性改善**。本次所選正常執行與編譯／連結負例皆符合正文；沒有發現所讀有限介面中的 allocator 配對、borrow 來源或 ABI 宣告錯配。執行成功不代表已證明全部 soundness。

## 身分、隔離與完整閱讀範圍

本人未撰寫第 12 章，曾撰寫第 2 章、4.4–4.5、第 5 章、9.2–9.4，並審閱其他章。先前 TECH14 已讀較早凍結的 12.3／12.4 與 contracts helper；這兩節有既有背景，本次不稱 blind 或全新讀者實驗。未讀第 12 章同行原判、FIRST12 答案、作者 ledger／source notes、root 處置、live12 或新版12，也未開啟未授權的第 14 章交叉材料。

唯一教材輸入是 `/workspace/work/tutorial-implementation/frozen/ch12-complete-prerequisite-v4`。manifest SHA-256：`668479e5d059a82f74bd3f713ca07470659d1bb0c3dd74212c025d6cda27ca98`。先順讀六頁 **44 BODY**，再讀 manifest 的 **71 sources**；不是只查疑點或目錄。archive 包含 39 Rust、15 expected、2 Cargo.toml、6 C、2 headers、7 SVG，所有正常例、完整解答、tests、helpers、build.rs、負例與設定均已閱讀。技術正文閱讀包含折疊答案的原始內容；實頁關閉／展開觀察則另行記錄，不能稱為未見答案的首讀。

本輪沿用本人已讀且指紋相同的 AGENTS、clear-tutorial、review-protocol、rust-terminology；四檔 hash 保存在 `work/ch12-technical-v4/instruction-fingerprints.json`。判準是好懂好學、先需求再機制、正文及所有配套不得用未教能力；沒有把教材大綱或 root precheck 當通過證據。逐頁筆記及 source 後自行提出的候選先保留原時點，後續核對結果另附，不回寫早期 unknown。

## 必要 N1：raw pointer 的 Copy 尚未交代

位置：12.3 `003-002` 型別介紹後、`003-003` 正常建立／檢查 pointer 前；後續實際依賴見 12.4 `004-005` ReadOnlyI32，以及 12.6 Greeting 的 getter／Drop。目前正文說 raw pointer 不保存借用 lifetime、離開 scope 不釋放 referent，但這些性質不能推出 Copy。

最小 source：`pub fn read(&self) -> i32 { unsafe { read_i32(self.pointer) } }`。read_i32 按值接收 raw pointer；從 shared self 取出欄位而未發生 move-out，是因 `*const i32: Copy`，不是因回傳的 i32 是 Copy。同樣地，pointer 的按值方法及重複使用並未拿走其值。

實際前文 2.2／4.2 介紹數值與需明確取得能力的 struct，2.6 明說 shared reference Copy；這不能概括所有 pointer 型別。全 canonical1–10 的正文搜尋與必要位置實讀，未找到 raw pointer Copy 已教依據；8.1 的 Box move 反而提示 pointer 類型各有不同能力。Std1.99 官方明列 `impl<T: ?Sized> Copy for *const T` 與 `*mut T`。

實際影響：讀者知道 move／Copy 後，會無法用已教規則解釋 borrowed self 欄位能按值傳出，以及 raw pointer 重複使用。最低修正是在首次正常碼前補一句：兩種 raw pointer 值都可 Copy，複製 pointer 資訊，不複製 referent，不保活來源，也不新增合法讀寫權限；後續 wrapper 可回指這一句。無需展開 pointer layout 或新增 unsafe 操作。本項是先備缺口，未判正常 source 不安全。

## 必要 N2：Utf8Error 的 PartialEq 在完整測試中被默用

位置：12.6 `006-006` CStr::to_str 契約與 `006-007` 測試說明，archive `examples/ch12-ffi/src/lib.rs:180`。

最小 source：`assert_eq!(second.as_c_str().to_str(), Ok("Rust"));`。這裡比較整份 `Result<&str, Utf8Error>`，不只比較成功字串；5.1 的實際 `001-005` 已教 Result 的 T 與 E 都需 PartialEq。Std1.99 Result 的正式 impl 也明列這兩項 bound。

目前 `006-006` 只明說「Utf8Error有Debug供expect使用」；7.8 原有 std::str::from_utf8 說明也是 Debug／Display。7.4 的 FromUtf8Error 是另一個型別，並未提供此缺口的先備；本節 CString::new 的 NulError 雖另明列 PartialEq，也不能移給 Utf8Error。官方確認 Utf8Error 確有 PartialEq，六項測試也通過，因此問題是首次契約不足，而非該測試無法編譯。

實際影響：正文聲稱全部 helpers／tests 沿已教介面，最後一項卻需要未交代的錯誤型別比較能力。最低修正可選一種：在 to_str 的錯誤型別契約補 PartialEq，並回指 Result 比較；或先沿既教 expect 取得固定 ASCII 的 &str 再 assert_eq。後者保留兩份 C 配置獨立清理的目的，縮小所需能力。無需改 C factory 或重做六項測試設計。

## 可選 O1：strlen 圖把計入長度與判定終點的範圍混在一起

位置：12.5 `005-005`，c-string-buffer.svg。原句「下面兩個箭頭都表示本次走訪範圍。word的掃描停在終止零之前」；圖中文字「遇到終止零，停在零之前」，箭頭覆蓋 Rust 四個非零字元。

長度4、storage5、一般 buffer 的零仍是資料，全部正確；同段也先要求有效可讀的終止零，所以不列必要安全修正。C11 §7.1.1 定義 string 含 first null，長度則不含它；§7.24.6.3 的 strlen 回傳 preceding null 的字元數。若箭頭代表「計入回傳長度的範圍」便完全一致；若稱實際掃描停止在零之前，初學者可能以為判斷不需讀到終止零。

最低改善：把上方箭頭說成「計入長度的四項」，補一句仍需遇到有效終止零才能決定停止；下方仍是 len 指定全部三項的走訪。桌面與手機原位實看都確認該語句／箭頭確實存在，並非只由 SVG source 想像。本項不要求重畫配置或更改 strlen 的實作假設。

## 全章通過項的理由與限度

| 範圍 | 真實需要與實際前文 | 機制、例子用途與本次判斷 |
| --- | --- | --- |
| 12.1 七 BODY／完整 GAT source、解答、tests、二負例 | 普通 Vec 已能借 slice；要用同一 trait 保留 slice／Ref 各自型別與本次來源。6.2 Extract／Self::Output、4.4 lifetime、8.4 T:'a、8.8 guard 均有實際正文支持。 | 先教帶參數 associated type、兩個各自宣告的 'a、Self::View<'a>、本 trait 的 required Self:'a，再 code。generic helper 只保 View，不冒稱有 index／Deref。ordinary borrow 最後使用與 guard 清理分開；正常2→push40→3、主動 drop、missing bound／source 已結束兩錯因各自對應。mutable 練習先補 as_mut_slice 契約與 is_err，沒有把 GAT 說成延長來源。 |
| 12.2 七 BODY／所有 variance helpers、解答、三負例 | 已教 reference 與 type lifetime bound；需求是合法替換滿足全部介面。 | 'long:'short、shared／外層 mutable lifetime covariance，mutable T invariance、Cell 的 shared set 寫入能力、fn 參數 contra／結果 co 分開。generic helper 的 body 核所有允許 lifetime；一次 literal 輸出不被當一般證明。另建 Cell 複製 reference，保留原容器 static 契約，不聲稱 Cell 變 covariance。假設破壞契約的箭頭不實際執行失效存取。 |
| 12.3 七 BODY／三正常例、練習答案、負例 | reference 已能修改數字；raw 是理解後續安全抽象與外部介面的責任。 | 星號型別／dereference／binding mut 分開；raw borrow、null、is_null、align_of、as_mut_ptr／add 先契約後 code。來源、初始化、alignment、權限、單 allocation、byte offset／isize 限制、one-past 不讀、provenance 不只地址均給出。80→100、120→140／總260與第三項30→50只示範條件內結果。缺 raw Copy 另列 N1，不以結果代全部 UB 排除。 |
| 12.4 六 BODY／完整 contracts、六 tests、正常例／答案／四負例 | caller 的前置與作者維持 invariant 各有工作；std 已有安全切分介面，手寫用來學責任。10.1 Send／Sync、10.4 scoped thread、10.8 data race 與合法 atomic 更新差別有前文。 | unsafe fn／block、Safety／SAFETY、2024 lint 先教。split mid 檢查與原 slice 保證分工，空段／one-past 的 non-null、alignment、初始化、exclusive access、範圍、不重疊與唯一輸入 lifetime 均核完整 helper。ReadOnlyI32 private constructor 維持 pointer/source 同一 i32；無寫入介面、i32 Sync、無 thread 專屬清理各支持 unsafe impl。兩個 join 先等完，沒有用 const 名稱、host 一次值或 atomic 旗標代 safety 理由。 |
| 12.5 九 BODY／全部 C、header、答案與 compile-only negative | Python 背景未假定會 C；先學 C 資料範圍、長度與配置責任，才接 FFI。 | C11、header／prototype／body、標準型別／平台限制、sizeof bytes／元素數、array 轉 pointer 不帶 len、條件／for／+=、pointer 副本仍指同物件先教。const 不阻止他路寫，非空有效範圍／len0 NULL 不做 arithmetic、signed overflow、malloc fail／初始化／matching free 一次均有限定。printf 格式與 inttypes macro、cc flags、object／linker 區別先命令；缺 body 是 link fail、缺 len 是 compile fail。獨立配置240／260證修改的是副本；示範不冒稱完整輸出錯誤政策或 malloc failure 驗證。 |
| 12.6 八 BODY／全部 FFI C/header、build.rs、library／sys、examples／答案、六 tests、四負例 | 有限本地 C API，全部能純 Rust 做；目的不是宣稱更快。9.2 build script 協定、12.5 C/compiler、5.4 TryFrom、7.4 ToOwned、8.2 Drop／owned 清理、10.2 literal suffix 各有實際支持。 | extern "C"／2024 unsafe block／unsafe call、cc Build 的 file/std/warnings/compile、host script 與 target object／link metadata先交代；repr(C)、u32／uint32_t、c_char／c_int、checked len 不截斷先 code。CString 禁內零／append NUL／named owner、CStr 有效範圍／借 owner／未保證 UTF8、C allocator 對應 free、private unique Greeting、from_ptr／mut→const、view 與副本、Drop 責任均核完整 C/Rust。count len0不讀、上限counter數學、ASCII expect 理由、兩份配置獨立、錯宣告／owner escape compile-only 均明確；隱藏 Utf8Error PartialEq 另列 N2。不泛推此 factory 的五-byte invariant 到任意 C API。 |

表中每列包含正常、失敗與練習／完整答案的首次使用核對，不等於每個 target 都已實跑。archive 的兩個 Rust packages 使用 workspace inheritance；獨立診斷 workspace 是本人建立的工具，不把 freeze 沒有 global lock 誤判為教材漏了 standalone lock。

## 正式契約與自行執行證據

在完整 BODY／source 之後，查閱並保存 Std／Reference **1.99.0** 的 selected 契約：[GAT required where](https://doc.rust-lang.org/1.99.0/reference/items/associated-items.html)、[variance](https://doc.rust-lang.org/1.99.0/reference/subtyping.html)、[raw pointer add／Copy](https://doc.rust-lang.org/1.99.0/std/primitive.pointer.html)、[from_raw_parts_mut](https://doc.rust-lang.org/1.99.0/std/slice/fn.from_raw_parts_mut.html)、[CStr](https://doc.rust-lang.org/1.99.0/std/ffi/struct.CStr.html)、[CString](https://doc.rust-lang.org/1.99.0/std/ffi/struct.CString.html)、[extern](https://doc.rust-lang.org/1.99.0/reference/items/external-blocks.html)、[repr(C)](https://doc.rust-lang.org/1.99.0/reference/type-layout.html)、c_char／c_int、[Utf8Error PartialEq](https://doc.rust-lang.org/1.99.0/std/str/struct.Utf8Error.html)與 [Result PartialEq](https://doc.rust-lang.org/1.99.0/std/result/enum.Result.html)。只稱上述 selected 範圍實讀，不稱所有文件／所有 API 已讀。

cc 選用正文固定 **1.8.0**，查實際 registry source 的 Build methods、TARGET／OUT_DIR／HOST、archive 與 cargo link metadata；本人的 own lock 解析到 cc1.8.0、find-msvc-tools1.0.14、shlex2.0.1。C 契約對照官方 [WG14 C11 N1570](https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf) selected §6.2.4、6.4.5、6.5.3.4、7.1.1、7.22.3／.3／.4、7.24.6.3；文件 SHA 為 `208969fec021e4f4ce87a1873765298c73e7bbb5209b1c5868967c7adfc8492e`。

官方擷取曾有 C11 formfeed 行號與 cc file 範圍定位問題，後來以正確 heading／source line 重新取出並親讀；採用證據是 selected-c11-02／selected-cc-02，不是初次錯位片段。pointer／CStr 擷取讀到額外方法段不被算作教材先備。原始下載、SHA、selected text 與修正後片段保存於 `work/ch12-technical-v4/official/`。

自行建立 own workspace，正常 members 只有 ch12-unsafe／ch12-ffi；64 非 SVG source 與 freeze byte 相同，fixtures 不加入正常 members。own lock SHA：`f1f19270d76961124fa426652f879a549c2d178f1e43a17cd80131a5bb45dae0`。未修改根 Cargo／lock／學生檔案。實際 rustc1.99.0 b940084d7、x86_64-unknown-linux-gnu、GCC14.2.0。

共 **16 個有限命令**，逐條 argv、purpose、exit、stdout／stderr 均保存在 `probe-results.json`：兩版號、own lock；GAT guard 一項 test（1 pass／8 filtered）；GAT missing bound、Cell invariance 兩 compile-only 負例；C 兩 object 編譯、缺 counts body 連結失敗、正常 link／demo、缺 length 編譯失敗、獨立配置答案 build／run；FFI 六 library tests；FFI 合併練習答案。四項負例 exit1 與指定診斷相符，其餘 exit0。沒有為 prose 疑點重新跑全 workspace 或未變全套。

正常 C demo 的五行逐字對上 amount100、local240／empty0、Rust文字bytes4／storage5、buffer3／nonzero2、owned240；C 完整答案逐字 `original=240; updated=260`。FFI 六項測試全部 pass，包括不配巨大 buffer 的 TooLong helper、內零拒絕、複製後 Drop、兩配置獨立清理；完整答案兩行逐字對上零0／非零4、相同標籤true／Rust。GAT probe 核 guard 未 drop 時借用仍活躍，正常 borrow 結果不能替代 runtime 管理契約。

## 自行實頁圖文與折疊範圍

本人從同一 v4 的44 BODY重組六頁，只調整本機資產 URL；七 SVG 逐一驗 served bytes 等於 archive。使用明確 extra_css 的同主題 own preview `127.0.0.1:8790`，public CSS 是先前已授權取得的本人副本，SHA及 config 保存在 preview-proof／visual-metadata；未讀 live12。Zensical build 無問題。Chromium／Playwright 自行定位，每張圖在 **1280×1200、390×1200** 各截一張，七圖14份本人全部親看，不借 root 圖像結果，也不以 bbox／source 算視覺。

GAT圖同一source、兩次 borrow 的資料2→3；Cell圖假設替換的禁制符與 set箭頭；raw三格索引／元素起點／one-past邊界與120→140；split兩range無交集、原資料100／45；C字串4／storage5與一般buffer len3／nonzero2；C header→兩object→linker→demo；FFI借原配置→另建String→matching free→副本仍用，均與相鄰正文及source對上。全圖文字、框、箭頭無缺字或碰撞；前三張在手機縮到約343px，約16.5–17.2px字仍能直接讀，其餘約21px以上。本次未因 preferred SVG 寬度門檻機械列缺陷；C字串箭頭的語意精確性保留 O1。

另在同兩種viewport分別先截關閉的 C／FFI 最後練習提示，再展開答案截圖，共8張，全部本人親看；總22份。關閉狀態已給任務、步驟、cleanup與預期值，不靠選讀答案補主文契約。答案展開後仍用已交代介面；手機 code 長行使用既有橫向容器，這些截圖只證所見區域，不宣稱全部 code 已橫捲逐行看。完整 code 的閱讀證據來自先前 source／BODY。

## 未驗範圍與原判封存

未本輪執行全部 Rust examples、所有14 Rust compile-fail、C checks.c、全套 clippy／fmt／根 workspace；這些 source／expected 全部已讀。本輪未跑 Miri，也未在普通 host 執行 intentional UB、invalid pointer、double-free 或 ABI mismatch；未模擬 malloc fail、巨大 u32::MAX buffer、cross-compilation／其他 C ABI。有限正常結果、編譯診斷、source 契約與圖文各有範圍，不能互相代替。

未發現需擴增測試才能定級的其他候選；上述未驗項保留為未驗，不當通過或必要缺陷。原需→機制→資料結果與主文／解答門檻在已讀範圍內可銜接；兩項型別能力缺口阻礙「所有 code 只用已教」的嚴格標準，最低補充已列。這是 ORIGINAL 技術判斷，沒有任何事後同行提示；本次不修改教材，也不進行 cross。

封存時保存同 byte initial、副本 SHA、完整124輸入指紋、71來源對映、閱讀路線、官方selected、有限命令、own lock／64來源指紋、22已看PNG與preview設定。封存完成後停止，等待 root 授權交叉；既有 TECH14 original 及其他 originals 保持原 byte／原分級。
