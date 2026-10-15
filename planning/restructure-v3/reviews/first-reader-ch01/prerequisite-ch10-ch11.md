# 第 10／11 章普通完整頁先備閱讀紀錄

已順序讀完合法 feed 的 **200–219，共 20 個完整頁**：200–208 為第10章九頁，209–219 為第11章十一頁。219 保存後再呼叫唯一 feed，實際收到：`STOP: no more frozen material has been released. Save current state and report to root; do not search later files.` 現在停在這個邊界；未讀第12、14或其他新章。

本輪是 **ordinary prerequisite page reading; not staged first-use evidence**，不是逐段盲首讀。每頁完整正文、inline source、manifest、fixtures、tests、折疊內容、負例與同頁練習答案均按釋出內容讀完，保存自己的四題、機制／用途分開的理解、引文依據、推論、小變化預測及當下未知，才取下一頁。記錄為 `records/200.md` 至 `records/219.md`。已同頁見到答案的地方明說，不宣稱隔離解題。

## 實際路線與限制

- 唯一正文入口始終是 reader_feed.py；未直接讀 repo 章節、source 檔、queue、作者 ledger、其他 review 或外部教材。
- 未安裝工具、未跑 tutorial 程式、Cargo、測試、shell 示例或 fixture 操作。本文輸出、診斷、測試通過均是讀到的教材展示；不能當成實測。
- 208 第一次工具輸出被截斷；沒有先存 record 或取下一頁，而是重取同一當前 feed 並加大輸出額度，讀完缺少的末段完整根檔才保存。這是同頁重讀，不是看到未釋出後文。
- 219 的 TypeId／Box receiver 段落使我停下重讀相鄰解釋與 caller；疑點和後來同頁理解都記在219，不用結論抹掉停頓。
- 沒有確認看到未釋出的內容。先前190的 TryInto、194的 impl Trait 自帶知識更正及原紀錄維持原樣；本輪沒有回改它們，也沒有把先前路線改稱盲讀。

## 實際看過的必要圖片

共 **12 張 PNG**，只在當包 metadata 列出後才 view，沒有提前開後圖。以下是親眼讀到的對象與關係，不只轉述 alt：

| 單位／圖檔 | 真正看到的必要對應與界線 |
| --- | --- |
|203 `ch10-buffer-prefix.png`|同一四格 buffer；第一輪前三格 ABC 有效，第二輪前兩格 DE 有效，後面仍有舊 C／0。只使用 `[..n]`，不是每次填滿，也不是實際 File 一定切成3、2。|
|206 `ch10-json-model.png`|借用輸入 &str 經 from_str 到 owned Notebook（String、Vec、bool），再借 model 經 to_string_pretty 產生新的 owned JSON String。資料往返不等原排版往返。|
|207 `ch10-fixture-sequence.png`|TempDir owner 藍色存活範圍涵蓋建立 fixture、啟動並等子程序、檢查 status/stdout/stderr；之後 close 消耗 owner 並可檢查刪除。PathBuf 只指位置，不延長資源存活。|
|208 `ch10-search-boundary.png`|args_os／Parser 到 owned Settings，再到 UTF8讀取與 owned Report、借用 render、write_all、main flush；錯誤虛線回 main 的 stderr／ExitCode。0/1/2責任及 stdout 只放資料。|
|210 `ch11-boxed-chain.png`|頂部 Link80 的 next Box 指到 heap Link120，再由下一 Box 到 End；箭頭是 ownership 管理，不是借用。頂層沒有被圖宣稱必定在 stack，也沒有 byte layout。|
|212 `ch11-deref-coercion.png`|&Wrapper<String> 沿 Deref TargetString 到 &String，再沿 String Targetstr 到 &str；對同一文字建立 borrow 視圖，不複製 owner。|
|214 `ch11-dyn-dispatch.png`|同型別 Box<dyn Charge> handles 分別指 Order80×1、Bundle120 與各自 amount 方法表入口；沒有據此推論實際 ABI 或永遠快慢。|
|215 `ch11-error-chain.png`|ReadCountError abc → source Some &cause → ParseCountError Integer → Some &error → ParseIntError → None；各層 Display 各述自己，連線是借用 cause。|
|216 `ch11-shared-tail.png`|tail、first.next、second.next 三條 Rc strong handles 共指 Link120，初始 strong3，尾端到 End；不是三份 Link120。|
|216 `ch11-strong-cycle.png`|A/B 在外部 owner 消失後各被對方 strong1 維持。這裡是明說的假設圖，並未假裝已看到建立循環的程式；217才讀到實際 Weak 父節點樹。|
|217 `ch11-guard-lifetime.png`|shared guard 建立、最後 println 後 guard 還在、try_mut 得 Err；drop(view) 後才可 try_mut 成功。時間軸區分最後讀取與 guard 的 Drop。|
|219 `ch11-owned-downcast.png`|藍色 Box<dyn Any> 內為 String Rust；請求 i32 的箭頭到橘色 Err(original)，仍擁有同一原字串；再請求 String 的箭頭到綠色 Ok(Box<String>)。箭頭交接 owner handle／結果，不是 clone 或內容轉換。|

## 第10章實際恢復的規則

200：dependency 與 workspace 分工、manifest 相對 path、registry/std 邊界、features 和 dev-dependencies、版本需求與 lock 實際解出版本的差別。知道 `--locked` 不代替自行決定升級；沒有實查本機 lock 或 registry。

201：rustdoc 的 Rust code block 可成 doctest，產文件不等於執行測試；integration test 是獨立 crate，只由公開 library API 進入。完整來源與同頁答案都已讀；計價範例的零金額仍有每筆 shipping 的預測依實際 API，不把0金額當0筆。

202–203：文字讀取的 UTF8 成功條件、IO Result 與 kind 分類；File handle／位置、String append 的新 bytes 與總長分別；u8／固定 buffer、短讀與有效前綴、Interrupted、EOF這次回傳0的條件。write_all 追蹤尚未寫部分，失敗不自動 rollback，flush 不等於持久化。所有 custom reader/writer 及測試完整讀，但不是實際 File 批次行為保證。

204–205：PathBuf／&Path 和檔案資源不同，native OsString 不必 UTF8；display 是人讀用途。args_os 的完整 argv 與 skip first 的責任不能重複。clap 的 owned Settings、parse try_* 結果、help/version 成功退出和 usage error 分流；Command::arg 一個參數不是 shell 片段。stdout、stderr、ExitCode 與保留 scope cleanup 的 main 路徑能對上。T 的 Into<OsString>+Clone 是已讀 API 需求，未猜 Clone 內部算法。

206：Serde trait 與 serde_json 格式層，owned Notebook 不依附輸入文字；JSON schema欄位與拒絕未知欄位、字串／數字差異、read UTF8 error 在 JSON parse 前。序列化往返不保證原排版，新增 archived 的例只回傳新 JSON，沒有憑空寫回檔。

207：TempDir owner 活多久與 PathBuf 路徑分開；Command.output 才啟動並等候，外層 IO 成功和 child exit 成功是兩層判斷。Compile-time env!／Cargo binary path 與 Output byte buffer 的 UTF8借用能恢復；close 是可檢查的清理界面，Drop清理錯誤不能當測試已檢查。

208：完整搜尋 CLI 的 args→owned Settings→read_report→owned Report→render→write／flush 責任。contains 是 substring、每原行只一筆匹配、ASCII忽略大小寫仍保留原文、原行號包含空行；empty query 在 IO 前拒絕，無匹配仍成功。讀完全部 modules、lib/main、fixtures 與 tests。錯誤退出和主程式確實 flush 只能說讀到有這個路徑，未跑。自己的 CRLF／空行／同一行重複詞變化預測已留208。

## 第11章實際恢復的規則

209–210：Sized 是型別大小可知，不等於資料一定在 stack；String/Vec 自身 Sized、str/slice DST、reference 可作固定大小 handle。`?Sized` 放寬 generic 隱含 Sized，const N 是陣列型別的一部分。遞迴 inline enum 的型別大小不能靠「實際資料會到 End」解開；Box 提供固定 indirection，owned、非自動 Copy，且 Box 的 owned move-out 有特別責任。

211–212：Drop 的 scope順序、struct欄位順序、std::mem::drop 消耗 owner 與丟 reference/Copy 的差別；最後使用 borrow 不是 owner 自動 Drop。Copy／Drop不相容，Drop owner 不准任意 partial move。Deref／DerefMut提供借用與 coercion，沒有讓 wrapper 的 trait bound 或 Copy 身分自動等於 inner。能分辨 `*` 嘗試移出借用內容和從沒有 Drop 的 owned wrapper 合法取欄位。

213–214：`T: 'a` 約束內含借用，`T: 'static` 不讓值永不清理；literal reference 的 static 和當前 reference 的存活分開。dyn 共同操作的靜態／動態 dispatch、Box factories 與 heterogenous handles、object lifetime 預設和顯式 `+'_`；dyn-compatible trait 的泛型方法／Self return／associated Output／where Self:Sized 邊界。這些分成各概念記，並非只用一份總體通過替代。

215：Error 的 Debug+Display、source 的預設 None、具體錯誤分類和 boxed reporting 邊界；source 返回借 self 的 `&(dyn Error+'static)`，不是 reference 永遠有效。保有 owned cause 與只 borrow chain 有不同角色，各層 Display 避免重複 cause；`Error` trait 本身並非要求所有實作 Self 都 static。

216–217：Rc clone 共用同一值、strong count／weak observer／upgrade新 owner、最後 strong 才清 T；shared ownership 不給 arbitrary mutable access。Cell值交接與 RefCell執行期 guard 互斥不同；guard 活到 Drop，不因最後讀取自動釋放；try_* 只回 Err不等待或釋放舊 guard。借出 guard 仍受編譯期來源 lifetime；Rc owner count 和 RefCell borrow state 不是同一計數。Weak 父節點樹是實際首次讀到的反向連結，和216假設循環分開。未延伸到多執行緒同步。

218：alias 不建立單位身分，tuple struct newtype 才建立；公開 i32 欄位未驗證非負。Add 的預設 Rhs 型別參數與先前 default method 不同，Output可另選，非Copy operands消耗，異單位只授權所寫方向。local wrapper 選公開介面，沒有自動繼承 push。orphan 需看定義 crate，generic 還看 self→trait arguments 的順序與 uncovered T；coherence 還要求不與 blanket impl 重疊。fundamental Box／普通 Vec 的差別只有文字解釋，保留尚無法熟練獨立判斷全部複雜組合，沒有把能重述當成已完全會用。

219：Any 的 static 型別需求與借用此次有效期分開，確切型別辨識不是轉換或任意 trait／欄位反射。is、downcast_ref、downcast_mut、owned downcast 的 bool／Option借用／Result owner 分別明確。失敗 owned downcast 返回原 Box，成功取具體 Box後可沿既讀 Box move-out；原 owner已 move。TypeId 要先選 Box本身或內部 view，不能由「同值」推出同型別，也不能將 TypeId 當跨版本穩定的外部 id。

## 留下的未知及本報告能支持的結論

當前能使用教材明列的介面規則進行小變化預測；這支持後續以「實際讀過這些完整頁」作先備，不支持把第10／11章稱為逐概念首次使用已驗收，也不支持宣告全書或這兩章全面通過。

仍未知或未驗證的界線：本機真實依賴解析／平台IO／子程序執行與錯誤訊息、trait object ABI與效能、Rc/Any標準庫底層、複雜 generic orphan/fundamental 所有組合、超出例子的小整數溢位政策。後幾項沒有被當成每個例子的必要缺口，因為目前可用已明列契約閱讀；若後文真需要更深規則，仍須等真正讀到，不能用自帶知識補過。

所有當時疑問、初判與其後同頁或後頁釐清均保留在原 records；未倒改舊答案。已到明確 STOP，等待 root 另行合法釋出。
