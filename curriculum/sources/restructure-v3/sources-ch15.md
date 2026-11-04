# 第15章v3來源實讀 ledger

只把真正讀過的range算實讀，fetch/cache取得不等於讀過。HTML來源text行號為本次抽取非空行（含網站導覽），raw Markdown保留空行；SHA256是原始cache bytes。所有URL/取得時間/cache path/read ranges與取捨在work/ch15-v3/research/read-manifest.json；本機前置在local-read-manifest.json。

## 15.1–15.2寫前實讀

| 原文 | 實讀範圍 | SHA256 | 採用／不採用 |
| --- | --- | --- | --- |
| [book-unsafe-en](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html) | 1–170 | `23c91b352a66dbfd80e839a147eae74be041406492048bd93c1ee2a2fb83a6ab` | 建立raw pointer與unsafe呼叫；不採任意整數造pointer、不以相信我替代契約；剩餘段落寫15.3/4前再讀。 |
| [book-unsafe-tw](https://rust-lang.tw/book-tw/ch19-01-unsafe-rust.html) | 1–329 | `a94f187cbd670fc5eb5786b9d2a43a3270d49a13a1c4257d17c6a5e6fc981911` | 繁中完整章含導覽；採建立/dereference/呼叫契約/安全抽象分工；舊unsafe fn內免block與extern/no_mangle語法不沿用edition2024；static mut/union不擴課。 |
| [nomicon-variance-en](https://doc.rust-lang.org/nomicon/subtyping.html) | 1–267 | `6e326c02c3838ab8647ffc6b9a7b8bcc1f837440934f4dfb12b825f09d7f0124` | EN全文，保留三種variance、&mut各參數、Cell寫回與fn輸入/輸出；不照抄thread_local!/UnsafeCell/後段raw pointer表。 |
| [nomicon-variance-zh](https://nomicon.purewhite.io/subtyping.html) | 1–223 | `8e3ec0209720be29c129282c9ef717d939736302d9090c08b32c00b38dd91e0b` | 中文對應全文，採相同替換關係；不用lifetime=一段固定lexical code的過度概括，不將型變誤作資料活更久。 |
| [gat-en](https://rust-lang.github.io/generic-associated-types-initiative/explainer.html) | 1–50 | `e923e3547ef1672cf7c9032c66499317ab42f3ecff7847ad08e63e6fb863bfed` | GAT入口導覽，實質內容再追三篇；舊未穩定狀態不當目前版本事實。 |
| [reference-associated](https://doc.rust-lang.org/reference/items/associated-items.html) | 150–383 | `7a4a92cee2a4adac50eec641ea4ba410c0771dfad4e2ae330a6179c1f3d7b76c` | 實讀associated type/GAT宣告/definition/required bounds與後段constants；採本例必要Self:a，拒絕把它概括為所有GAT一律需要；不新增associated const課。 |
| [gat-motivation-en](https://rust-lang.github.io/generic-associated-types-initiative/explainer/motivation.html) | 1–123 | `73203ab75f3e4dd9e2f0e73cd551edf9ebb12c44691cd354de6b5c2a79b1daaf` | 全文，採ordinary associated type不能連method本次lifetime的動機；本書改用兩種BorrowView，不擴寫Iterable。 |
| [gat-iterable-en](https://rust-lang.github.io/generic-associated-types-initiative/explainer/iterable.html) | 1–109 | `9a4b284be349613036ab4bd5621b3ef5e10fb376a11d2f8ba2d453780bca30f1` | 全文，採帶lifetime的associated type、receiver帶入與不同次borrow；只用已教有限視圖，不照抄split_first/未知helper。 |
| [gat-required-en-real](https://rust-lang.github.io/generic-associated-types-initiative/explainer/required_bounds.html) | 1–124 | `373b678498b5f686ba5ce10e6a3e6682c0ecc60c8e92cf8270a5197837b5617e` | 全文，採Self outlives與receiver implied bound的理由；此頁MVP歷史與std::vec::Iter舊路徑不照抄，用當前Reference核對。 |
| [gat-zh-real](https://raw.githubusercontent.com/sunface/rust-course/main/src/basic/trait/advance-trait.md) | 1–95 | `ac4f342da58dc361bb8736e6c5b1527c7f3eda1701ce0d3d4372f8d963e8f9be` | 真正中文RustCourse associated type段，對照普通Self::Item與泛型參數分工；此原文沒有GAT，不冒稱中文GAT規則來源，GAT精確規則採官方EN Reference。 |

舊12六節全部正文、兩Rust package與C-basics全檔、七份SVG實際全文親讀；舊tests與負例保全，不以region代替完整source。當前12.1/12.2 hash核對root給定5f7ab379…、846d2893…；13.1–13.3亦親讀必要完整source，沒有用研究大綱代替正式教材。11.5/11.9的outlives、Cell/RefCell/guard與11.6 where clause精確範圍保留，不聲稱本輪11全章全文已讀。

Source isolation以實際loaded source為準。沒有為引用source使用grouped use，沒有將old fullunsafe library當15.1只選example的捷徑，沒有把中文普通associated type原文冒稱含GAT。GAT required bounds的目前規則採官方Reference實讀，歷史Initiative文本僅取理由。失敗404（舊GAT URL與猜測path）保留fetch-manifest.json，實際追到有效英文正文與中文RustCourse後再採用。

## 15.3–15.4實讀 append

| 原文 | 實讀範圍 | SHA256 | 採用／不採用 |
| --- | --- | --- | --- |
| [book-unsafe-en](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html) | 1–540 | `23c91b352a66dbfd80e839a147eae74be041406492048bd93c1ee2a2fb83a6ab` | 全文實讀完成；採建立/解參考/unsafe fn/安全抽象/unsafe trait/ABI分工；不採任意地址UB、static mut/union/未固定nightly、不把多次PASS或unsafe標記當安全理由。 |
| [std-ptr](https://doc.rust-lang.org/std/ptr/index.html) | 1–225 | `d38c61b5fbaaddd7c21fa6879924caf3fec3fb2e60cf37e5093b16913cad71f0` | 實讀Safety/alignment/轉reference/allocation/provenance；只採本節必要來源/範圍/權限與同配置責任，不擴strict/exposed APIs/UnknownCell。 |
| [std-pointer](https://doc.rust-lang.org/std/primitive.pointer.html) | 740–780, 1170–1196, 1710–1775 | `99f78131867f19aa488a8e37d4362dff27336d1052bc9384a1d9b21abaa22334` | is_null與const/mut add原簽名/全部Safety，元素單位、同配置/onepast/isize與地址overflow；不採byte_add/offset衍生課程。 |
| [std-slice-mut](https://doc.rust-lang.org/std/slice/fn.from_raw_parts_mut.html) | 1–36 | `631094759d13dfa8724fc8b1ef28d59163a1d05cab21a6bf4e346a4c188fa148` | from_raw_parts_mut完整簽名/Safety，特別零長仍non-null/aligned、單一配置、初始化、獨佔與borrow有效性。 |
| [reference-ub](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) | 1–146 | `2a999a9465128b0c2aa1bddc241def35ffdde13cb5f3f6ff396d6e6891142506` | soundness/aliasing/data race/錯ABI/invalid value完整原文；UB不等panic/crash，已建立reference即有義務；不抄MaybeUninit或UB例子執行。 |
| [std-send](https://doc.rust-lang.org/std/marker/trait.Send.html) | 1–85, 188–199 | `4f1c6beb69e6cb3cea37f0af6965b5e7b94f67c3a2a7b243e6f3c4c241e440e8` | unsafe auto marker trait與rawpointer負實作，移交能力；不複製Arc/未教原語。 |
| [std-sync](https://doc.rust-lang.org/std/marker/trait.Sync.html) | 1–110, 236–247 | `211d6eef06d382f4be667c372f9fd84812012dde032e16c0b59d32d81677b71d` | shared reference等價、i32共享讀與rawpointer負實作；不引入transmute/UnsafeCell/鎖API。 |
| [align-api](https://doc.rust-lang.org/std/mem/fn.align_of.html) | 1–20 | `2223f50f730d98a4656ab356165ab457a0854e43399ef45183188c599dadb892` | ABI-required alignment與usize/byte單位；i32=4只本target，非跨平台保證。 |
| [null-api](https://doc.rust-lang.org/std/ptr/fn.null.html) | 1–24 | `242ffc799463471537a689c253afe81b809260c2c35ef94dd0aaf4c62ca6777f` | null建*const T、不配置T；具體i32用法，不引入Thin/MaybeUninit/整數轉pointer。 |
| [raw-borrow-reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html) | 90–205 | `6c4a4f2b560bba54edd67cf2c63b543a3732a9cce459478ef14569c2e86ba9be` | raw borrow operators/place/context與dereference；不把&raw當先造reference，不新增packed/MaybeUninit實例。 |

Source-isolation-plan原列13.4等待條件已由root正式gate滿足；本輪實讀stage04新增source並記hash，沒有拿13.8/14/16作先備。已讀完整Book EN/TW說法仍以目前std ptr及Reference校正：from_raw_parts_mut建立reference當時就要符合條件，不用「之後使用才可能UB」或「unsafe fn可免block」舊措辭。

## 15.5 C基礎實讀 append

| 原文 | 實讀範圍 | SHA256 | 採用／不採用 |
| --- | --- | --- | --- |
| [c-en](https://beej.us/guide/bgc/html/split/pointers.html) | 1–163 | `4b5bc429bfb256fbdbce4e2ffe66c3013003df3bc4587f05d3cb4fe1a4e40289` | Beej pointer全文，採型別星號/解參考星號、值傳遞仍指原資料與sizeof型別；不抄未教%p/void* cast或NULL dereference。 |
| [c-zh-raw](https://raw.githubusercontent.com/wangdoc/clang-tutorial/master/docs/pointer.md) | 1–181 | `f71363ac469cbea5bbfae5ad8938fe775befe3f2a159cff72a70f65dc8de198c` | 中文pointer全文，採宣告與運算式區分、傳pointer值副本修改原資料；拒絕pointer本質無號整數、任意地址算術/減法、未初始化可讀隨機值、NULL讀取會確定報錯等說法。 |
| [c-zh-memory](https://raw.githubusercontent.com/wangdoc/clang-tutorial/master/docs/memory.md) | 1–205 | `5c09df7f42e354528a5c936ce9c379e4b23413c5ee57541955fc3922df7ca40b` | 中文memory的void*/malloc/free及後段部分；只採C objectpointer轉換/NULL/初始化/一份配置一次清理；拒絕全域變數須手動free、所有局部必在stack、未初始化資料可當普通值讀及省略NULL檢查的片段。 |
| [c-zh-array](https://raw.githubusercontent.com/wangdoc/clang-tutorial/master/docs/array.md) | 1–170 | `7c77496215e08ffeff8dc201e2d5d5a101c5da23887ce1700ff99a5f97a6d59d` | 中文array宣告/初始化/sizeof/len；拒絕越界正常執行固定改下一個變數、sizeof(size_t)以%zd或%d混印等；本書只%zu且不跑UB。 |
| [c-zh-string](https://raw.githubusercontent.com/wangdoc/clang-tutorial/master/docs/string.md) | 1–170, 182–230 | `32909cb2a08cc78a3c2f133d2e75eeb1edb47838027636af821d746b2aeff8f8` | 採literal末尾NUL、array副本、相鄰literal合成、strlen/sizeof差異；拒絕char s[5]="hello"必編錯、char陣列未初始化已是合法字串、strlen結果直接當int/%d等概括。 |
| [c-memory-en](https://beej.us/guide/bgc/html/split/manual-memory-allocation.html) | 1–250 | `2821a5ef147d569d6b6246013d31579c460a3d7469ee333a1843433a73b2cc20` | Beej memory全文，採malloc/free/NULL/初始化/份數與清理；不新增calloc/realloc/aligned_alloc/FILE；不照抄部分示例省NULL檢查或malloc對所有overaligned型別皆足夠概括。 |
| [c-array-en](https://beej.us/guide/bgc/html/split/arrays.html) | 1–230 | `fbbb6d421a297733370365b49d960cacf1134036e63163ddf6a28fad1b339120` | 採array初始化/sizeof除項大小/函式參數傳第一項pointer與另外長度；拒絕arrays其實就是pointer的過度概括，不執行越界展示。 |
| [c-string-en](https://beej.us/guide/bgc/html/split/strings.html) | 1–135 | `8517dee42caeb7f2e9f2241542abd5bc6f7e5d7ae80e10445fe341af52a2da99` | Beej strings全文，採NUL/stringliteral/array可寫副本/strlen與真複製；不把所有buffer當字串，不新增strcpy。 |

來源有用的教學安排與技術正確性分開判：借鑑由原資料/指標副本/函式修改的動作開始，不沿用只因normal output就保證pointer有效的論述。C sizeof byte、exact-width可選型別與target條件均在正文先說；malloc的alignment只針對本節int32_t，不宣稱涵蓋任意擴充alignment。GCC14.2是實測compiler版本，不按crossref packet誤改成第17.2節。

## 15.6 FFI實讀 append

| 原文 | 實讀範圍 | SHA256 | 採用／不採用 |
| --- | --- | --- | --- |
| [nomicon-ffi-en](https://doc.rust-lang.org/nomicon/ffi.html) | 1–210, 345–390, 423–475 | `31e6a4e9aaf2b00eb802f64887f1e2319537b8e890f6bd89901eaa093dafc554` | 實讀本地外部介面、wrapper/析構、link與ABI/CString/佈局；不採snappy/set_len/外部C++依賴、grouped use、只去unsafe簽名就宣稱安全、舊unsafe fn內免block，C→Rust與callbacks不擴課。 |
| [nomicon-ffi-zh](https://nomicon.purewhite.io/ffi.html) | 1–180, 287–385 | `518e49df16c891ec0f77cfb394d9fcac682ec490c0f54458c0b86e08018bfab0` | 中文對應實讀介面/安全包裝/析構與連結/ABI/佈局/字串；相同取捨，不把Box原始pointer當任意C配置owner，不抄過時slice::raw、任意as長度轉型。 |
| [cc-api](https://docs.rs/cc/1.8.0/cc/struct.Build.html) | 169–184, 330–440, 535–555, 587–633, 780–815 | `22783dba501efab9a34d9d094beb4a77f42d64e9f447a435cb0f47d3a5b4c7f3` | cc1.8.0真實new/file/std/warnings/warnings_into_errors/compile、target/HOST/OUT_DIR/default cargo_metadata與link協定；只用所選methods，精確=1.8.0不upgrade，開發警告政策不是所有compiler的穩定承諾。 |
| [std-cstr](https://doc.rust-lang.org/std/ffi/struct.CStr.html) | 110–166, 229–265, 284–333 | `c54489c847ffc805e970900a80dcf0e6545ebf22958b2cf65c82d92dddca9469` | from_ptr完整Safety/borrow綁owner、as_ptr readonly/owner不可早清、to_bytes兩種範圍、to_str UTF8借用；不照抄dangling temporary UB、lossy Cow或nightly APIs。 |
| [std-cstring](https://doc.rust-lang.org/std/ffi/struct.CString.html) | 95–180, 310–335 | `32a6315eac80f11ddbbb70f9893cd8bf3b74aabb184b5271afbd23e5436b6620` | CString owner/Into<Vec<u8>>對String及&str的具體轉換、拒絕內部NUL/補末尾NUL、as_c_str借view；不引入C literal、into_raw/from_raw或Box轉換，舊extern例以edition2024校正。 |
| [reference-layout](https://doc.rust-lang.org/reference/type-layout.html) | 130–170 | `9a0c4ee32b9dabedad764bc13b93ca4a800cf911481e9c5587eb48464c9c49aa` | Rust預設只基本對齊/不重疊而未保欄位次序；repr(C)struct依宣告順序alignment/padding/整份大小；不宣稱內部String/Vec自動C相容。 |
| [reference-external](https://doc.rust-lang.org/reference/items/external-blocks.html) | 45–118 | `e8cdee35c3c411c3a8aa54b00358f51ba3c32aa3c7cbe1b9cce3afc6f57c859a` | 當前edition2024 unsafe extern、預設unsafe函式、ABI選C target約定與宣告namespace；不抄system/跨語言unwind或safe override API。 |

所選C與Rust實作都用單一路徑use。沒有將Nomicon的libc/snappy安裝、Vec::set_len、callbacks、no_mangle或C++ unwind協定帶入本章；明確採本地C version、uint32 counter、C string與五-byte工廠，從C body與header核對每次unsafe前提。所有API、helper與測試在首次所需處先教，不靠外書補正文。
