# 18B 六節獨立銜接與實頁初判（sealed）

初讀snapshot：2026-10-15T10:36:09.838198+00:00；封存：2026-10-15T10:52:14.879504+00:00。結論：**必要2、可選0**，請root逐項裁定。角色是continuity/actualpage，非作者、technical或firstreader。未讀peer18B結論、作者validation、known audit、root dispositions。18.7→18.12全部正文與實際來源依序讀完；前綴actual1–14已讀，18.1全文及literalfixture、17.4全文與25actualinclude為實際回指先備補讀，不替17宣稱pass，也不以未讀16/18A來源補能力。

## Findings

### C18B-01｜必要｜第一次由borrowed result取出opaque Id前，補Id具備Copy

位置：docs/chapters/18-10-lesson.md:67 record導讀至:263完成lib前。首次必要時點：18.10完成快照b4-complete/src/lib.rs:130–131 match &result後，id是&Id，用*id按值取出；:136仍接手整份result。18.9的next先接手match result取得ownedId，沒有這一步。

原文：

> `docs/chapters/18-10-lesson.md:67` 要自行完成的`record(&mut self, Result<(Id, Check), JoinError>)`不等待：從成功Id或error.id找到Input、移除登記；成功保Check，is_cancelled保存Cancelled，其他JoinError仍保存TaskFailure。把Entry推進entries，**這裡不fill**。登記的expect沿B3檢查作者自己的不變條件，不把漏登記當正常HTTP問題。

> `docs/chapters/18-10-lesson.md:259` 

> `docs/chapters/18-10-lesson.md:263`     --8<-- "examples/ch15-projects/b4-complete/src/lib.rs"

> `examples/ch15-projects/b4-complete/src/lib.rs:129`     fn record(&mut self, result: Result<(Id, single::Check), JoinError>) {

> `examples/ch15-projects/b4-complete/src/lib.rs:130`         let id = match &result {

> `examples/ch15-projects/b4-complete/src/lib.rs:131`             Ok((id, _)) => *id,

> `examples/ch15-projects/b4-complete/src/lib.rs:132`             Err(error) => error.id(),

> `examples/ch15-projects/b4-complete/src/lib.rs:134`         let input = self.inputs.remove(&id).expect("每份task已登記完整Input");

> `examples/ch15-projects/b4-complete/src/lib.rs:136`         let completion = match result {

前置位置：

> `docs/chapters/14-03-lesson.md:50` **`handle.id()`** 取得 task 的 **Id**，可用於辨識與顯示；它不是我們的輸入索引，也不應假定某次固定印出幾號。Id 也支援相等比較與 Hash，稍後可用作 HashMap 的鍵。**`handle.is_finished()`** 查詢 task 此刻是否已結束，沒有等待它；false 只描述當下。

> `docs/chapters/06-03-lesson.md:39` 再回到只有兩個 i32 欄位的摘要。第 3.2 節說過：整數已有 Copy，自訂 struct 不會因欄位都是整數就自動 Copy。

> `docs/chapters/06-03-lesson.md:41` **`Copy`** 允許在原本會 move 的動作中，隱含複製值本身的資訊。對這個數值摘要，`let second = first;` 之後兩份摘要都能使用，不必另外呼叫 clone。這項複製不能由實作者插入另一段任意工作。

> `docs/chapters/06-03-lesson.md:57` second 的數量加一，不會修改 first 的數量。這次保留的是兩份可獨立修改的數字欄位；Copy 的判準是型別的複製契約，不能從實際資料放在 stack 還是 heap 推出。

判斷：以前14.3明說的是Id辨識/顯示/相等/Hash，沒有Copy；*reference讀i32的例子不能替opaque Id建立Copy契約。按既教ownership規則，不能從shared reference搬出一般非Copy值。答案這一步當下真需要型別能力，不能由編譯成功倒猜。

最小建議：在18.10 record導讀補一句Tokio Id具Copy；這裡先借讀result，以*id複製Id後借用結束，仍可接手result作分類。不需Id內部表示、poll/Pin或新增Rust。

具體收益：讀者可以把答案的先借後move兩個match連成已教規則，理解為何沒有從借用中偷搬值。

### C18B-02｜必要｜固定輸入的全檔區塊丟掉檔首空行，複製後原索引會偏一

位置：18.11:123/199–202與18.12:103–106固定fixture全文區塊；首次實際run於18.12:131。首次必要時點：18.11請保存完整起點fixtures；18.12依展示全檔保存完整目錄後首次run預期index1/3/4。兩個actual8110 code blocks首字已是ftp，DOM沒有原檔開頭LF。

原文：

> `docs/chapters/18-11-lesson.md:81` 先保存完整`b-final-starter`為自己的工作目錄，包含Cargo.toml、Cargo.lock、src、checks與fixtures。起點的舊排程、單筆工作與signal流程都已提供；新題目只有build_report、Report.status、write_report三份未完body。下方列出全部起始檔、checks與材料，片段只協助定位。

> `docs/chapters/18-11-lesson.md:123` `fixtures/invalid-urls.txt`第0行空、第1行ftp、第2行只有一個一般空白、第3行/notes、第4行另一份ftp，最後有LF；完整內容在折疊中，空白與重複原樣保留。lock保存本目錄在repo外的解析版本，並非要求手填1774行設定。

> `docs/chapters/18-11-lesson.md:199`     `examples/ch15-projects/b-final-starter/fixtures/invalid-urls.txt`：

> `docs/chapters/18-11-lesson.md:202`     --8<-- "examples/ch15-projects/b-final-starter/fixtures/invalid-urls.txt"

> `docs/chapters/18-12-lesson.md:23` ??? example "展開完整設定、全部 Rust 來源與測試材料"

> `docs/chapters/18-12-lesson.md:103`     `examples/ch15-projects/url-checker/fixtures/invalid-urls.txt`：

> `docs/chapters/18-12-lesson.md:106`     --8<-- "examples/ch15-projects/url-checker/fixtures/invalid-urls.txt"

> `docs/chapters/18-12-lesson.md:119` 隨附fixtures/invalid-urls.txt是有空行、兩份相同ftp網址與一份`/notes`的真材料。下表明示原檔每行的內容；執行時使用隨完整目錄提供的檔案，最後LF不增加一份輸入。

> `docs/chapters/18-12-lesson.md:122` | --- | --- | --- |

> `docs/chapters/18-12-lesson.md:123` | 0 | 空行 | 略過，仍占原行索引 |

> `docs/chapters/18-12-lesson.md:124` | 1 | `ftp://host/item` | 保留為輸入 |

> `docs/chapters/18-12-lesson.md:125` | 2 | 一個空白字元 | 略過，仍占原行索引 |

> `docs/chapters/18-12-lesson.md:126` | 3 | `/notes` | 保留為輸入 |

> `docs/chapters/18-12-lesson.md:127` | 4 | `ftp://host/item` | 保留為另一份輸入 |

> `docs/chapters/18-12-lesson.md:131` cargo +1.99.0 run --quiet --locked -- fixtures/invalid-urls.txt --json

> `docs/chapters/18-12-lesson.md:136` 這份材料不送HTTP；報告原index為1／3／4，三筆kind=input，各自保留original，時間與HTTP欄位null，退出1。換成自己的http／https清單才是真網路檢查，結果依環境與當時服務而定；可用`--jobs 2 --timeout-secs 5`選兩個正數設定，或省略--json取得同份資料的文字表示。一般取消操作是按Ctrl+C，等報告與130；不要用強制終止代替這份收尾。

前置位置：

> `docs/chapters/18-08-lesson.md:17` 

> `docs/chapters/18-08-lesson.md:25` | 3 | 第二份相同 `/ok` URL | index3，另一份HTTP200結果 |

> `docs/chapters/18-08-lesson.md:29` ## 先編號，只用空白視圖決定略行

> `docs/chapters/18-08-lesson.md:31` 沿9.6，`text.lines()` 借用文字、逐行交回 `&str`。LF與CRLF分隔符不在行內容中；尾端換行不多產生一行，單獨CR沒有跟LF時仍是內容。這裡保存的「原行」指這份內容，不含已由lines取走的分隔符。

判斷：兩原檔均有五行，第0空、第2是一個空白；actual DOM在桌面/手機均只保四行，第0已是ftp、第1空、第2/notes、第3ftp。按已教lines/enumerate/trim，從顯示全檔複製會得到0/2/3，與教材要求1/3/4不同。忽略空白的normalized include比對會漏掉這項。

最小建議：讓這兩份固定fixture的可複製表示保留檔首LF和單空白行；或在區塊旁給精確escaped全文（含\n與\x20/明指單空白）及重建方式，明指不要把被trim區塊當原檔複製。驗actualDOM或copy文字後索引仍1/3/4即可，不需改Rust/fixture真檔或全站設定。

具體收益：直接由同頁完整答案保存出的材料與原行索引/預期報告一致，不把呈現失真當成讀者寫錯prepare_inputs。

已有補償證據：18.11 prose與18.12五列表格確實已精確交代所有行，可以依它手動重建；不報成材料未交代。本項是宣稱完整全檔的code block本身不可如樣複製，需讀者額外察覺矛盾再補LF。

實頁證據：round18b-fixture-render-proof.json含四個viewport/page組合raw DOM、原文與逐行索引；18.11/12 fold0-open-tail雙viewport已親看。

C18B-02的原檔精確escaped文字是`"\nftp://host/item\n \n/notes\nftp://host/item\n"`；actual code DOM是`"ftp://host/item\n\n/notes\nftp://host/item\n"`。原檔保留index1／3／4；複製顯示block會是0／2／3。這是實際8110兩頁兩尺寸重現，與原檔/正文表格正確不同；沒有把source本身報成錯。

## 完整scope與正向證據

72個唯一include檔，40個named regions，六canonical、五SVG與root metadata共86檔初讀至封存hash全穩。96次fullfile include每尺寸的normalized DOM內容皆找到；其中兩個whitespace-sensitive fixture另逐raw核，不能用normalized通過掩蓋C18B-02。24筆四題證據見JSON，Q2機制與獨立需要分開，逐字引文取frozen行。

B1保原文、parseUrl與finalURL三種身分；Header工作/讀body、None／Some0、Input／Request／Timeout界線清楚，wrapper包GET準備與redirect，client沒有競爭的另一期限。5秒entered／產品10秒／pause後20秒結果觀察分開；借用JoinHandle Future的owner/取消/二次Output限制已在測試前說清。完整normal Fixture依賴閉包同頁，不要求外找helper。

B2先enumerate再用Unicode trim視圖判空，原文不trim，LF/CRLF/loneCR說清；來源String結束後ownedInput仍可用，invalidonly不等Empty，duplicates各請求。current18.8在本段自教DurationCopy，未再誤回指14.5已教。B3 caller唯一遞增原index、Id完整Input登記、Client共享poolclone與額外原文String成本先說；tracked含完成未收，不由jobs假稱總memory上限/效能。next具體取消安全與同步分類/補工邊界說清，受控及合成tests先教完再列完整checks。finished-not-collected實驗有currentthread/noawait限制，當下不需16的poll/Pin深層內容。

B4用同五份材料分gate0/1與原index1/4，停止只在觀察後不fill，先前HTTP200保留；new/record不fill，run先查停止，by_ref先解釋再走pending。測試保存原tracked/登記數後才fallbackdrain，清理不補綠燈；觀察helper同task持兩owner，取消等待還能自己回收。真CtrlCdemo改四筆材料的理由、有限5/15秒觀察、10秒自完、signal/listener/server雙方cleanup與flush後退出都交代，並非靠sleep或外部強制殺服務。

B5（18.11）先固定所有退出政策，再做ownedRow映射、Reportstatus、真正writer三小步；u16/u128/Option Serde/null、text以JSON引用防LF破行，已先說明。17.4實際pubuse/nonexhaustive/Error契約補讀後才讀相關source；Clap defaulttyped值/正數優先/read/Empty和help流分開，use_stderr其實已在10.6:62，fold在9.5:109，不發假新API缺口。兩份完整答案與原九checks逐bytes一致，不偷載最後七batchtests；Fixture全檔已列但root不宣告，所以本階段不載。

18.12正式root才cfgtest載內置Fixture，16lib＋2Parser integration分開，所有Rust/helper/manifest/lock/README/固定輸入同頁fold。新batchtest原文helper及真HTTP後mappedreport斷言逐diff實讀，其餘已讀core/測試內容逐bytes確認。兩份lock183package解析metadata與directgraph，rawhash保存；不把registrytransitive名字當已教API。最終實計core825、同檔tests842＋integration56=898、Fixture198，共1921Rust行，與正文數字吻合。stage副本沒有累加成作業。

## 實際頁面

8110六頁於10:43:01–10:43:56 UTC，1280×800與390×844，58對pair／116張原圖全部view_image親看；精確檔名列JSON。五必要SVG文字、顏色支路、箭頭與同一材料各步逐一核對：18.7原文旁路及Input早退、18.8索引不壓縮、18.9完成未收占位→回收補位→兩順序、18.10停止分類與整批理由、18.11資料/出口/fatal邊界。三張高圖另看bottom；340/360nativewidth，mobile最大343px，18/20native字清楚。

9表手機全343px、wrapper375=scroll375，全部親看，沒有新增表格改稿建議。36details各尺寸初始closed；必要fullstarter/fullchecks/normaldeps/fullanswer逐一closed/open-tail實看，提示留為三層，18.12 README另開tail。所有程式長行pre獨立橫移，沒有誤報成正文裁失。固定輸入copy差異單列C18B-02，不因全文存在就放過。

## 版本、曝光與限制

18.7正文首次讀後立即snapshot，18.8–12在snapshot後依序讀。root先告知18.8 currentf71c7493回指窄修及18.9/10doc/link postfreeze，用此current初判，非舊版獨立重現。parent在本輪中告知14三doc已採納；未把修後新文字倒算原18B先備或回寫原14。18B86來源hash封存時全穩。

定點查找時曾rg既讀章＋17多頁尋舊use_stderr/u64，意外看到17.5 RngSeed與17.8配置metrics的u64三行，未讀其餘正文或拿來作已教。其後只為pubuse/nonexhaustive完整讀17.4；未讀任何他角finding/作者validation/ledger。README提ledger沒有跟往。未執行Cargo、教材命令、test/build/strictsite；使用parent已建8110只核actualrender。不是blind首讀/整書pass；所有必要/可選交root裁定，後續窄核另存，原initial保留。
