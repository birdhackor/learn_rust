# 第12章原判裁定與最小修正

唯一原稿為 `ch12-complete-prerequisite-v4`：六頁44 BODY／71來源。FIRST、TECH、CONT原判全部封存，ROOT全文讀完後修正；各自SHA、原分級與當時未知保留，不把本處置算另一份獨立發現。

| 原發現與分級 | 來源核對、理解收益及最小處置 |
| --- | --- |
| TECH N1／CONT N4必要：raw pointer Copy | 採用。12.3首次正常code前交代兩種raw pointer都可Copy，不要求T Copy；複製資訊不複製資料、不保活來源或增加free資格，mutable raw亦不放寬存取限制。後面的borrowed self按值取field可接已教Copy，不需新增unsafe操作。 |
| CONT N1必要：array reference到slice的轉換 | 採用最低介面說明。12.3第一個直接array caller前說shared／mutable array reference可在預期slice型別處轉換、長度來自原array、仍借同資料；與Deref coercion分清。保留原caller，避免更改多份正常／測試／答案。 |
| CONT N3必要：數值array的Debug／PartialEq | 在第一個whole array assert前說本例i32 array的能力、等長逐位置相等及Debug列元素；不用泛泛假設元素有trait就所有容器皆有。其他arraycaller可沿已教契約。 |
| CONT N2必要：sizeof具體語法／單位在第一Cblock後才教 | 將現有說明搬到code前，保留整array bytes／一項bytes→項數的理由、size_t及固定型別不讀值的界線。沒有擴寫C型別課。 |
| TECH N2必要：完整Result比較偷偷依賴Utf8Error PartialEq | 選縮小所需能力的修法：最後一個獨立owner test先expect取得固定ASCII的str，再比較字串。Utf8Error的Debug已有契約；不多教本工作不需要的錯誤相等能力。只重跑這項test，其他五項filtered。 |
| TECH O1可選：strlen圖的計入長度範圍與掃描終點 | 採用。正文與SVG一起明說上箭頭是計入長度的四項，判斷停止仍讀到有效終止零、回傳長度不含它。下箭頭仍依len訪三項。原有效範圍契約與4／5／3／2數值不改，未升成安全缺口。 |
| CONT O1可選：GAT入口粗體closing delimiter | 加closing後空白，讓Markdown正常突出原問題，不更動教學需求。 |
| CONT O2可選：FFI owner圖type與value拼寫 | 初框明標greeting的Greeting型別，cleanup標drop(greeting)，減少由型別標籤回找值的負擔；沿原同配置／借用／副本關係，不重畫多份owner。 |
| CONT O3可選：GAT桌面浮動回頂chip遮圖字 | 採用方向是第12章發布時移除navigation.top浮動元件，保留一般導覽；仍待在修後實頁核對，現在不宣稱已關閉。第11章候選與其已核nav不混入這項未完成共享UI變更。 |

FIRST原判沒有確認必要／可選缺口，保留cc receiver精確簽名等未知及沒有讀完整source／實跑的界線。TECH與CONT的raw Copy是兩份原始獨立發現；其他相互同意只算交叉後補查，不改算獨立命中。

修後正文目前保持原六節與44單位。正常C、header、ABI、allocator配對、所有unsafe實作均不改；Rust只改一個owned-cleanup test，SVG只改兩圖的文字及說明。array／Copy契約補前文也讓後面Pin／Future能沿順序取得這些能力，沒有靠後章反補第12章。

定向fmt、該test一項（另外五項filtered）與lib／tests Clippy均實際exit0，raw logs另存協調工作目錄。沒有因prose／SVG重跑C、其他正常例或未改整套。獨立cross、局部首讀／實頁與共享UI核對尚待完成，整章還未宣稱可發布。
