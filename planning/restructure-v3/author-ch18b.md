# 第18章B作者交接

所有權：18.7–18.12 canonical正文、B1–4與最終工具的既有獨立快照、必要配圖，以及本檔／sources-ch18b.md。root負責workspace／lock／nav／整合與獨立審閱；本作者不宣稱獨立review。來源研究在work/tutorial-implementation-20261015/ch18b-preparation；正式實讀與限定驗證在ch18b-authoring。

## actual前綴與安排

本人已讀14.1–9完整正文、HTTP及Fixture完整normal library閉包（包含http-single的config/error/reply）、9.4–7、10.1–9、17.4與18.1，以及必要selected完整source，共46檔，range/hash見ch18b-authoring/actual-prefix-read.json。18.1在本次閱讀後由root窄修B入口；本人再讀57–65，保存當時未即時hash的限制，不捏造舊hash。14.7/8的root prose窄修不改教學契約或Rust。先前12.1實讀證據保存在ch13-authoring。

| 必要能力 | actual前文 | B當下首次需要與解釋 |
| --- | --- | --- |
| final headers／status／URL／body／proxy／Client pool | 14.7–9 | 18.7先固定只等headers與分類，再完整起始source；fixture ready另讀小body |
| Duration／timeout／虛擬clock | 14.5、14.9 | 18.7分工作10秒、entered觀察5秒、結果觀察20秒，先解借用handle及cleanup |
| owned行、enumerate／排序 | 9.4–7、10.9、14.6 | 18.8先編號再trim判空，原文與duplicates不改；18.9分兩種順序 |
| JoinSet／Id／slots／取消安全 | 14.3–6 | 18.9先Id→完整Input、完成待收仍占slots、先回收再補工 |
| 停止與signal owner | 14.6 | 18.10先new／record不補工、先查停止、abort+drain、Unstarted及真ready |
| 參數／Write／JSON／TempDir／ExitCode | 10.3–9 | 18.11先逐欄輸出映射、數字／Option編碼、default_value_t及helpers，再整合三步 |
| 公有Error／non_exhaustive／re-export | 11.7、17.4 | 18.11完整final root前回收其角色，typed分類不依opaque Display |

18.11為最後引導整合，18.12為所有core／tests／fixture／manifest／lock／使用方式的完整答案。保留現有10份永久package，不由早期B載最終future library。B1–4 dev path fixture的transitive normal source相鄰完整折疊同頁列出。命令明選已教target；root全workspace all-targets屬維護者工作。

## 逐節作者驗證與freeze

### 18.7 已完成

- docs/chapters/18-07-lesson.md：單筆政策→owned分類／時間→起始函式→完整起始source與Fixture轉依賴閉包→六checks與有限觀察→三步實作→完整完成版／demo。
- 8項限定檢查按預期：starter check／零tests；工作副本追加checks六項全部預期失敗（含entered期限後cleanup）；complete六項全過；真loopback demo；兩package fmt、lib strict Clippy與example strict Clippy。輸出／命令見7-validation.json及分檔stdout/stderr。
- freeze-18-07.json：15個頁／source／asset檔hash、18 snippets；samepage全檔及Markdown/details展開通過。未做browser或獨立review；未改Rust或root設定。
### 18.8 已完成

- 原行0-based索引→trim僅判空→owned Input／Entry→順序await→完整六checks→兩步實作→本節完整快照／真混合demo。先給全部lib／single與Fixture閉包，不靠前節檔案路徑猜缺件。
- 8項限定檢查按預期：starter編譯／零tests、工作副本六項預期失敗、complete六項全過、真HTTP混合demo、fmt、lib及example strict Clippy。命令／輸出見8-validation.json。
- freeze-18-08.json：17檔、20 snippets，全檔同頁及Markdown/details展開通過；未claim視覺或獨立review。
### 18.9 已完成

- 先Id→完整Input、Check／JoinError兩層、S1的slots含完成待收→new/fill／先回收再補工→排序；完整lib/input/single、manifest、全部七checks及helpers、真混合demo都同頁。
- 8項限定檢查按預期，包含工作副本1通過／6預期失敗、complete七項全過、真loopback demo、fmt與lib/example strict Clippy。9-validation.json保存stdout與實際狀態。
- freeze-18-09.json：19檔、27 snippets；同頁全檔及Markdown/details展開通過；沒有加入B4停止或最終CLI為B3前提。

18.8原freeze後root全文讀稿指出link label9.7誤指lines；窄修9.6以吻合09-06target。原freeze-18-08.json保留，前後hash見18-08-postfreeze-prose-followup.json；source／命令未變，不重跑Cargo。
### 18.10 已完成

- 五份受控停止材料的input／gate、S1/T0/W1與opaque Id分開；new／record不fill、先觀察停止、abort_all後逐份回收真結果與Unstarted；signal demo另四份材料先說換組理由。
- 10項限定檢查按預期：starter／工作副本1通過6預期失敗、complete七checks、fmt、lib/example strict Clippy、named example build；真binary不送signal在期限內自然回1，另一份於flushed READY後真SIGINT回130，Cancelled／Unstarted及client/listener/server收尾已驗。10-validation.json及10-real-sigint-after-ready.stdout保留實際輸出。
- freeze-18-10.json：19檔、27 snippets，全檔同頁與Markdown/details展開通過；未claim視覺或獨立review。
### 18.11 已完成

- 引導整合保留三步與同一九checks：追加後3通過／6預期失敗；映射5／4；status6／3；write+flush9／0。作者在獨立工作副本逐步真跑，並另以同頁兩份真實全檔report.rs／app.rs直接替換，驗9／0及strict Clippy；其他來源使用本頁完整final starter，沒有載完整工具另七項batch tests或test_fixture module。
- 10項限定檢查均按預期，含starter編譯／零tests、四個checks階段、全檔答案測試與strict Clippy、starter fmt／strict Clippy。11-validation.json保存實測狀態與stdout/stderr。
- freeze-18-11.json：20檔、29 snippets；全起始目錄Rust/lock/材料、完整checks、兩份可替換答案全檔同頁，Markdown/details展開通過。Row數字／Option-null、clap default_value_t、u64秒數、pub use／non_exhaustive、app helpers／main／writer契約均先說再載入。18.12另提供正式工具整目錄與16+2測試，沒有合併兩節。
### 18.12 已完成；B全六節交接

- 正式完整答案同頁15份全檔snippet：manifest／1774行lock、10份src（含198行fixture）、tests/config、invalid材料、README。normal／cfg(test)／integration三種來源閉包分清。完整Rust計1921行，沿原口徑核心825（含lib root兩行test-module接線）、tests898、fixture198；line-counts.json逐檔列計法，不累加stage副本。
- 9項限定Cargo按預期：16 lib tests、2 config integration、fmt、lib/bin/config strict Clippy、debug build、quiet真run材料退出1、release build、隔離install、安裝版真run材料退出1。安裝驗證用work內--root保護共用Cargo binary目錄，工具內容與教材--path/. --locked相同。
- 14項真正binary案例全過：help/version0；缺檔／非法UTF8／空有效清單／零jobs先於讀檔／零期限先於讀檔／用法2；真loopback全通過0、混合1、held逾時1、觀察兩份真handler進入後SIGINT130；checked-in及安裝版離線材料1。完整stdout以JSON解析、原索引／duplicates／null／kind核對。服務先真GET ready與body，entered用明確queue通知，不靠sleep；所有server及child結束後才assert。12-real-cli.json保留每次狀態、HTTP paths與cleanup證據。
- freeze-18-12.json：16個頁／source檔、15 snippets，全部全檔且Markdown/details展開通過。未做browser或獨立review。

## 原freeze之後的窄修歷史

18.8作者依root提示僅將lines link label9.7→9.6，Rust／命令未變；18.9由root刪「觀察期限s」的s，18.10由root將實測404的course.rs連結換成已親讀的Github原文。原7–12所有freeze檔皆保留，前後hash與來源見postfreeze-followups.json、18-08-postfreeze-prose-followup.json及root-followups-0944差分；不重寫成全數零drift，不重跑不受影響的Cargo。

B十份README僅改新章號、自然用詞／觀察期限及cargo run --quiet。全部50 Rust檔、10 manifests與2 locks及既有材料保留，old-B-preservation.json逐檔核對74檔。root Cargo／lock／nav／checkers未改。總53項限定Cargo檢查按預期（包括刻意不完整工作副本的預期失敗）與14項真binary案例；validation.json彙整。全章交接後作者STOP，後續窄修另由root指定。
