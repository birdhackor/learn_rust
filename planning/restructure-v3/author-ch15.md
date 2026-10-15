# 第15章v3作者交接 ledger

最新狀態：15.1–15.6全章作者自查並freeze；以下保留最初prefix與逐節append。獨立審閱及實際桌面手機頁面仍待root，不宣稱全書驗收。

最初prefix紀錄：

狀態：15.1–15.2作者自查並freeze；15.3–15.6未完成。clear-tutorial、writing-prompt、術語指引已讀並套用；source-isolation-plan已root親讀同意。Root接四members並更新lock後才Cargo，作者未改shared設定。

## prefix實際first-use與閉包

| 能力 | 正文必要解釋位置 |
| --- | --- |
| 來源留下來，結果借自來源 | 15-01-lesson.md:7 |
| associated type 也可以接收參數 | 15-01-lesson.md:34 |
| 為什麼還要 `where Self: 'a` | 15-01-lesson.md:42 |
| 每份實作選自己的借用型別 | 15-01-lesson.md:56 |
| 共用函式保留輸出關係 | 15-01-lesson.md:70 |
| 兩種失敗，仍由不同原因造成 | 15-01-lesson.md:105 |
| 練習：借出 mutable 視圖，再還回使用權 | 15-01-lesson.md:135 |
| 晚段完整快照的三項測試分別核對原 slice 資料、guard 清理前後的寫入權限，以及兩種空清單的視圖。測試使用 **`Result::is_err(&self) -> bool`**：結果是 Err 時回傳 true，Ok 時回傳 false；它只檢查 variant，不取得內部值。這讓 assertion 能直接確認「guard 仍在時，取得 writer 的嘗試被拒絕」。這時才把上述helper與三項測試合成gat_complete；完整module與根程式都在下面。它的main沿用已教的視圖觀察，測試模式不執行main： | 15-01-lesson.md:153 |
| 長借用可以滿足較短的讀取需要 | 15-02-lesson.md:17 |
| variance 看的是包住參數後的替換方向 | 15-02-lesson.md:55 |
| 為什麼 `&mut T` 不能任意改換 T | 15-02-lesson.md:69 |
| Cell 的 shared 介面仍有寫入能力 | 15-02-lesson.md:89 |
| function pointer 的輸入與輸出有不同方向 | 15-02-lesson.md:113 |
| 練習：讀出 reference，建立另一個 Cell | 15-02-lesson.md:160 |

15.1 gat_views helper只含模型/trait/impl/轉交，不含variance/contracts或is_err測試；gat_complete在is_err介紹後才載原三tests。15.2 reference helper只含兩個shorten，callbacks晚段才加入兩adapter。mutable GAT答案與Cell答案都是完整獨立example。同頁所有manifest/root/helper/test/fixtures/.expected/答案均real source引用，泛型使用code span。

prefix-verification.json共14筆：五正常example精確stdout、GAT完整三tests、五compile-fail逐份rustc及.expected真實substring比對、六selectedexampleClippy -Dwarnings成功；初fmt僅兩個尾端多空白行失敗，修正後只重查fmt成功。不重跑已通過Rust行為。所有fixture只編譯不執行，沒有UB host執行。

prefix-markdown-verification.json兩頁snippets成功；prefix-source-closures.json逐target全source/manifest/hash；prefix-freeze.json保存所有引用檔案與當時shared設定hash。未實際desktop/mobile頁面視覺驗證；作者自查不當獨立首讀/技術/銜接驗收。

crossref packets只是提案：15.5把GCC14.2中的14.2誤認章號，其實是compiler版本，不採該替換；15.4原13.8 atomic回指也不直接使用未freeze能力，將依本節已建立的只讀/衝突存取契約說明，保全data race責任。13.4 gate現已root放行，15.4前另親讀完整實稿與source。

## prefix freeze hashes

- docs/chapters/15-01-lesson.md: `4d4b83c88f8ba6ccfcac7866d71b7a0730e55a2f5afed39f4e3d7f7ce8ef3b4b`
- docs/chapters/15-02-lesson.md: `e87a0ee7d3577ac6d9d472acb519b57ac60eac6e23d52cb2e8caa17eddba1cc0`

## 15.3–15.4完成與freeze append

13.4正式stage04 gate後親讀完整正文及新增scoped正常/負例closure；11.1/2/3/4正文與必要size/Box/Drop/Deref完整source也實讀，hash/ranges追加local-read-manifest.json。

| 首次必要概念 | 位置 |
| --- | --- |
| 型別的星號，與讀取時的星號 | 15-03-lesson.md:17 |
| 建立與檢查 pointer，可以先用 safe 操作 | 15-03-lesson.md:34 |
| dereference 前，先交代必須成立的條件 | 15-03-lesson.md:58 |
| 在同一 array 中移到下一個元素 | 15-03-lesson.md:107 |
| provenance：來源資格不只是地址 | 15-03-lesson.md:141 |
| raw pointer 轉成 reference，還要守 reference 的契約 | 15-03-lesson.md:149 |
| 練習：從同一 allocation 修改第三項 | 15-03-lesson.md:155 |
| unsafe fn宣告呼叫者還要滿足哪些條件 | 15-04-lesson.md:17 |
| safe介面必須接手安全責任 | 15-04-lesson.md:74 |
| 要把同一份mutable數列切成兩段 | 15-04-lesson.md:82 |
| 新操作：由pointer與長度建立mutable slice | 15-04-lesson.md:104 |
| 檢查輸入，再建立兩個互不重疊的borrow | 15-04-lesson.md:120 |
| 跨thread時，unsafe impl必須有自己的理由 | 15-04-lesson.md:156 |
| 用真實reference保存來源關係 | 15-04-lesson.md:176 |
| 分別核對移交與共享 | 15-04-lesson.md:206 |
| 練習與驗證：結果檢查，不等於證明全部soundness | 15-04-lesson.md:247 |

raw例子均獨立單檔，不載contracts。unsafe_read只有read_i32；safe_split/答案只有split；readonly_threads才合併已教三種介面/unsafe impl，最後contracts_complete才載原六tests。逐target完整來源閉包保存在raw-contracts-source-closures.json；同頁所有manifest/root/helper/完整測試/fixtures/.expected/答案均展開成功。

raw-contracts-verification.json共16筆全部符合預期：八正常examples逐字stdout、contracts_complete原六tests、五獨立compile-fail逐份rustc比對sidecar、九selectedexampleClippy -Dwarnings與fmt。只編譯失敗fixtures，沒有執行UB；array/emptyVec/onepast只作已證前置成立的安全存取。

15.4去除對尚未freeze13.8 atomic的先備依賴，依已教13.1+本節普通i32說明data race；unsafe impl Send/Sync兩項責任與原scope局部來源皆保全。impl處重新說明placeholder放在型別位置，不錯稱7.2已教impl。

raw-contracts-freeze.json保存兩頁/全部引用檔案/沿用未改圖/當時config與13.4gate hash。尚未實際桌面手機頁面視覺驗證；作者自查不等獨立驗收。
- docs/chapters/15-03-lesson.md: `8dab364a2465746176fc776a154b507efc35ba3c4e0b89b69d9b19a96512f18e`
- docs/chapters/15-04-lesson.md: `3f60eb8974d39620c01dd5719914f728739bfbca6fe7507ce091d712b78ecd4f`

## 15.5完成與freeze append

寫前親讀Beej C pointer/memory/string與array必要實際正文、中文wangdoc對應pointer/array/string/memory；沒有用研究摘要或只讀API清單。取捨詳sources/read manifest，明確拒絕原文中的pointer=integer、全域變數手動free、越界固定正常執行與錯誤格式化等說法。

| 新能力／材料 | 本文位置 |
| --- | --- |
| 先讀懂型別、變數與 array | 15-05-lesson.md:9 |
| C 的函式與 pointer：修改的是哪一份資料？ | 15-05-lesson.md:42 |
| pointer 加長度：讓函式知道讀幾項 | 15-05-lesson.md:66 |
| 字串的終止零，與 buffer 的零值 | 15-05-lesson.md:93 |
| malloc 與 free：清理責任不隨 pointer 副本增加 | 15-05-lesson.md:121 |
| 把結果印出來，再整理完整程式 | 15-05-lesson.md:146 |
| 從 source 到 object file，再連結成可執行檔 | 15-05-lesson.md:184 |
| prototype 能檢查什麼？ | 15-05-lesson.md:231 |
| 有限檢查與練習：真的複製資料後再修改 | 15-05-lesson.md:253 |

保全完整C型別、array→pointer傳遞/長度、NUL與buffer、malloc/free、printf/PRId32、header/prototype、source→object→link。新增counts.c完整檔與missing_length.expected同頁，全部C例子、checks、完整答案都在當頁；既有C原始檔未改。

c-verification.json共11條全符合預期：cc版本確為Debian GCC14.2.0、兩C source各自編object、正常link、demo精確五行、只給main.o故意link失敗實測undefined reference、missing_length只編譯診斷匹配、checks與答案各自build/run（空stdout與精確original240/updated260）。產物只放work/ch15-v3/c-build；沒有host UB執行，沒有假裝測過malloc失敗。

c-freeze/source-closures/markdown-verification.json保存正文/完整C閉包/沿用圖hash與實際snippets展開。未實際桌面手機視覺驗證。
15.5 SHA256 `d7a229d638429477507f1ca2beb0d742c008be9380f724d527f8ec583d15f04a`。

## 15.6完成與freeze append

寫前親讀EN/中文Nomicon FFI必要完整段落、目前Reference extern/layout與cc1.8/CStr/CString選定原文，實際11.3 Drop與12.1/2完整工程前置/source已親讀。stage三manifest新增build/cc依賴後由root更新lock（Locking0、cc仍1.8.0），才selected Cargo；作者沒有改shared設定。

| 能力／完整快照 | 必要解釋位置 |
| --- | --- |
| 先對齊一個沒有 pointer 的函式 | 15-06-lesson.md:9 |
| 用 build.rs 編譯 C，再連結到 Rust | 15-06-lesson.md:51 |
| Rust的private sys module管理Rust路徑，沒有替C symbol建立Rust module的命名空間。各快照與完整版分別編譯、連結；每個example只連到所選package的一份C實作，沒有將三份同名tutorial_version同時放進一個binary。 | 15-06-lesson.md:63 |
| buffer傳pointer與長度，結果struct傳佈局約定 | 15-06-lesson.md:142 |
| Rust-owned CString：只把字串借給C | 15-06-lesson.md:248 |
| C-owned配置：由專用owner管理對應的free | 15-06-lesson.md:345 |
| 本節library另外有六項正常測試：已連結版本、byte計數與子slice／空段、長度不能截斷、字串內零拒絕、move後複製文字再Drop、兩份獨立C配置先清一份。長度helper不用先配置巨大buffer；測試用 **`usize::BITS`** 取得這個整數型別的bit寬度、**`usize::MAX`** 取得最大值，在寬度大於32時確認helper回TooLong。這兩個是型別的標準associated constants，不是測量資料或改平臺。 | 15-06-lesson.md:402 |
| 檢查宣告、borrow與所選正常案例 | 15-06-lesson.md:455 |
| 練習：同一份 C 文字，兩種介面各做各的工作 | 15-06-lesson.md:517 |

scalar的C/header/sys/lib只有version；buffer加入repr(C)/counts/長度檢查；label才加CString/strcmp。每份自己的manifest/build/C/header/Rust根/例子完整同頁。永久ch12-ffi在owner/Drop契約全部教完才載，六tests所需usize常數說明移到完整lib首次呈現之前；所有原tests未改。永久package全部五example、四compile-fail＋sidecars與完整答案也同頁。

Rust sys module namespace與C linker symbol namespace分開說明，各snapshot分別連結一份C實作；build.rs在host、C object配target與cross toolchain責任明說。每個unsafe呼叫均有逐項來源/型別/範圍/owner理由；不以C compiler或普通PASS作完整soundness證明。

ffi-verification.json共15條全部符合預期：stage三例＋永久owner/完整答案精確stdout、原六library tests、四compile-fail逐份rustc比對sidecar、四packages全已教targets Clippy -Dwarnings及fmt。newstages在首次behavior前只做格式正規化，未改原永久包/C/source/tests。未執行UB、未模擬malloc失敗或建立u32::MAX buffer；with_nul替換情況由已讀API/範圍說明，未宣稱另執行該替換。

ffi-freeze/source-closures/markdown-verification.json保存完整source/build/C/fixture與hash，snippets實際展開成功；沿用未改ffi-owner圖，未實際桌面手機頁面視覺驗證。
15.6 SHA256 `ed5a9511f205772cacba2f95e4c4b5b25744e59c039fdd4c656e78d090e713a7`。

## 全章15.1–15.6最終交接

六頁全部完成作者自查，所有階段／永久source／C／build／fixtures／原tests／答案均保全並完整同頁引用。prefix與15.3/4/5原freeze hash全部逐檔一致；當前全章aggregate為chapter-final-freeze.json。

| 頁面 | 最終SHA256 |
| --- | --- |
| 15-01-lesson.md | `4d4b83c88f8ba6ccfcac7866d71b7a0730e55a2f5afed39f4e3d7f7ce8ef3b4b` |
| 15-02-lesson.md | `e87a0ee7d3577ac6d9d472acb519b57ac60eac6e23d52cb2e8caa17eddba1cc0` |
| 15-03-lesson.md | `8dab364a2465746176fc776a154b507efc35ba3c4e0b89b69d9b19a96512f18e` |
| 15-04-lesson.md | `3f60eb8974d39620c01dd5719914f728739bfbca6fe7507ce091d712b78ecd4f` |
| 15-05-lesson.md | `d7a229d638429477507f1ca2beb0d742c008be9380f724d527f8ec583d15f04a` |
| 15-06-lesson.md | `ed5a9511f205772cacba2f95e4c4b5b25744e59c039fdd4c656e78d090e713a7` |

四組verification JSON共56筆命令紀錄；五個GAT/variance、八個raw/contract、三個C正常程式、五個FFI正常程式均有精確stdout核對。Rust測試共15項（GAT3、unsafe契約6、FFI6）。compile-fail（Rust14份＋C1份）及C缺body的link失敗共16個預期非零結果都實測匹配指定原因；另prefix初fmt1修後0保留，不宣稱56筆初次全綠。

每個新shell先source scripts/env.sh；Cargo使用INCREMENTAL0、DEV/TEST_DEBUG0、JOBS2，僅本章相關package/target，沒有root build/test/check.sh/format--all，沒有自改members/lock/CI/nav。沒有host執行UB。root17.6作者另跑的固定nightly Miri證據由root保存，本作者未重跑或算進自己的檢查。

所有新增Rust故意失敗.rs都有同名非空.expected，且每一份都以本輪實際stderr核對。原C缺參數.expected也核對；未新增runtime UB材料。七份沿用SVG已實讀source與圖文角色但未改，沒有實際桌面手機頁面視覺驗證。

剩餘需root統籌：獨立首讀／技術／銜接與實際頁面驗證、全書harness整合。本作者沒有已知未修Rust/C/snippet失敗，交接後STOP，等待審閱修正或下一任務。
