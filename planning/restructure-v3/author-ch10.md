# 第10章v3作者交接 ledger

最新狀態（全章交接）：10.1–10.9正文、10個最小階段package與完整永久CLI均完成作者自查並freeze。以下保留最初prefix交接與歷次append；獨立審閱及實際桌面/手機頁面驗證仍待root，不宣稱全書驗收。

最初prefix交接時的狀態：

狀態：僅10.1–10.2完成作者自查並freeze，交root獨立審閱；10.3–10.9未完成，不宣稱全章ready。clear-tutorial已讀並套用。沒有修改shared Cargo/workspace/lock/nav/scripts；三個最小member由root接線並產生lock後才執行Cargo。

## 真正first-use位置

| 能力 | 正文首次必要解釋位置 |
| --- | --- |
| dependency、path起點、workspace與dependency分工 | 10.1:7 |
| registry、crates.io、第三方與std | 10.1:17 |
| feature、features/default-features、dev-dependencies、TOML行內表格 | 10.1:28 |
| SemVer、相容要求與精確=字串 | 10.1:42 |
| docs.rs與API文件角色 | 10.1:50 |
| cargo add/@/--features/--dev | 10.1:77 |
| lockfile、tree/--depth/(*)、update package spec | 10.1:90 |
| ///、rustdoc、Markdown fence、doc test及自動main | 10.2:7 |
| 閱讀signature/behavior/limitations/examples | 10.2:23 |
| cargo doc --lib/--no-deps/--open與--doc | 10.2:29 |
| integration tests獨立crate、外部名稱與--test | 10.2:47 |
| 同檔測試回收2.7契約 | 10.2:80 |
| 獨立答案package與新增完整doc test | 10.2:136 |

前提：親讀現行2.6/2.7/2.8與modules-basic manifest/lib/pricing；數字、module/use/pub、assertions/attributes已成立。沒有Parser、tempfile檔案呼叫、format!、?、Iterator adapters、Box<dyn Error>或Error trait。10.2原始與答案library分package保存，不讓10.1載入///或tests。第三方內部實作不當成本書own source。

## 完整來源閉包與設定

`/workspace/work/ch10-v3/prefix-source-closures.json`逐target保存本機manifest/source/hash；另加root管理的intro workspace/lock與固定toolchain，當下共享設定hash在prefix-freeze.json。10.1自有package僅main，10.2僅lib與public_orders測試，答案僅lib；三個member均明寫name/version/edition。唯一自有以外本機library是modules-basic的lib/pricing，沒有編它的main或其他example。本頁所有載入的本機來源、設定與答案均以real source snippets完整內嵌。

10.1設定clap4.6.7+derive與dev tempfile3.27.0，都是相容版本要求；實際tree本次為clap4.6.7/tempfile3.27.0。正文明確區分字串4.6.7與=4.6.7、manifest/lock、設定與實際呼叫。無自有失敗fixture，因此此prefix沒有缺.expected sidecar。

## 實際驗證與限制

所有Cargo新shell先source scripts/env.sh，CARGO_INCREMENTAL=0、DEV/TEST_DEBUG=0、JOBS=2。只選三個member必要target，未執行根cargo/check.sh。

- prefix-verification.json保存13條實際命令與stdout/stderr：setup check/run/tree，原始unit1/integration2/doc1，答案doc2/unit1，cargo doc --lib --no-deps，3個selected Clippy -Dwarnings，三package fmt --check。全部成功；setup stdout逐字等於兩筆訂單總額=240。
- 原始總4項測試、答案3項，合計7次測試執行；答案unit與原始同規則，非7項獨立規則。
- copy-verification.json：在work獨立練習副本實際執行兩個cargo add、cargo update clap，manifest與正文來源逐字一致；再把doc預期240改200，實際doc test非零並呈left240/right200。沒有修改repo lock。
- 副本第一次未複製rust-toolchain.toml，Cargo因無default toolchain而失敗（copy-first-error.json）；補上相同toolchain後完成上述檢查。這是驗證環境準備問題，保留原紀錄。
- Markdown+pymdownx.snippets(check_paths=True)兩頁實際展開，HTML與markdown-verification.json保存在work/ch10-v3。沒有遺留snippet標記。
- 實際cargo doc成功產生API HTML；未執行--open，未以桌面/手機看實際站點頁面，版面與閱讀路徑留root後續核對，不以生成成功聲稱視覺驗收。

## prefix凍結hash

- `docs/chapters/10-01-lesson.md`：`5aba346afa0a1c427f2da036edbd83b6b54d1bc75b7ede0c4bf68f5a9670296b`
- `docs/chapters/10-02-lesson.md`：`394da63f7182a1c977beae9f11180ce370e1efe18851f4fbb0964a5dca4fb79a`

完整9個owned檔案（2頁、3manifest、4Rust來源）的hash與當下共享設定在prefix-freeze.json。後續窄修需append保留原freeze；章10其他節的新增不改此prefix。

## 10.1文字位置窄修 append

保留prefix-freeze.json原hash。A0610-C01再查將行內表格解釋從dependency段移到完整manifest正前，沒有改Rust、指令或測試材料；不重跑Rust。新10.1 SHA256 `2d8fdc5ae52f16c2bce057f6b59f272f79af58d051e932a88ee8b1d552aa6564`，10.2保持原hash。最新快照prefix-freeze-after-inline-table.json。

## 10.3–10.4完成與freeze append

兩節已實作並作者自查；10.1/10.2不變，後續10.5–10.9仍未完成。root先接好io-text/io-bytes/path-cli最小members/lock後才Cargo。

新增真實first-use：
- 10.3:5：先準備
- 10.3:32：read_to_string
- 10.3:67：File 與 Read
- 10.3:97：Write：
- 10.3:115：保存內容
- 10.3:138：完整設定
- 10.3:162：練習
- 10.4:7：一個 byte
- 10.4:15：Read::read
- 10.4:31：同一個讀取
- 10.4:43：收集完整
- 10.4:61：刻意短讀取
- 10.4:73：為了建立原資料
- 10.4:81：中斷之後
- 10.4:99：短寫入
- 10.4:126：用可控制
- 10.4:150：完整設定
- 10.4:188：練習

I/O結果、ErrorKind具體規則、File/Read/Write方法與各caller政策在使用前說明；u8/array repeat/有效前段/EOF/Interrupted先於read_all，from_utf8/ToOwned byte slice先於ShortReader main，Error::from(kind)/is_empty先於模擬錯誤，Write/write_all/flush先於ShortWriter。沒有Iterator adapter、parse、Box/dyn/Error trait或未教helper。

io-verification.json保存17條命令：9個正常程式逐字stdout比對，5個selected testtargets共9項測試，兩package所有本節已完整教過targets的Clippy -Dwarnings及fmt，全部成功。write_note實際產生9-byte專用輸出、核對完整內容後移除；不依輸出給讀取測試提供輸入。io-markdown-verification.json兩頁snippets實際展開成功。沒有失敗fixture，沒有需補.expected的.rs。

io-freeze.json逐檔hash；io-source-closures.json逐target記真正本機source與三個fixture。io-text沒有lib；io-bytes只有read_all lib。byte_count example在Cargo中也會建這個本節lib，沒有載未教後續CLI。空檔與invalid UTF8原始byte完整描述於兩頁，note文字real-source snippet完整內嵌。

沿用原有340px直式18px字級buffer-prefix.svg，未修改既有asset。已讀SVG內容與圖文對應；尚未在桌面/手機實際頁面看圖，不將程式通過或SVG存在當視覺驗收。

- `docs/chapters/10-03-lesson.md` SHA256 `0ef86d599bc6a201968cededd1b0971904d3b0a957545558e6654170f626deca`
- `docs/chapters/10-04-lesson.md` SHA256 `3fc73a9915ecedebe2589e981615aed0271a906fdfafe7bd8741367a20bf2171`

## 10.5完成與freeze append

親讀EN/TW Book Arguments、EN/中文RBE Path及目前std選定API，再實作完整10.5。root已開9.4且作者親讀正文/實際Iterator來源；此節沒有用9.5–9.7新API。

真實first-use位置：
- 10.5:7：PathBuf
- 10.5:40：OsString
- 10.5:65：工作目錄
- 10.5:77：用 args_os
- 10.5:101：先驗證
- 10.5:119：stdout
- 10.5:147：實際觀察
- 10.5:168：完整設定
- 10.5:200：練習

package維持no-lib。path_types/os_strings/working_directory/arguments/argument_pair_solution均各自單檔；早期example不編src/main。arguments完整自足parser/tests；晚期binary才保存相同parser與streams/ExitCode。原root ch07-path-cli library/source未改，快照複製不刪原內容。

path-verification.json共16條實際檢查：5examples精確stdout、2測試targets共4次測試（同兩項規則各執行一次）、CLI成功/空檔/缺參數/多參數/缺檔/非法UTF8六路的stdout+stderr+exit、Linux原生非UTF8檔名成功保留9-byte文字、Clippy -Dwarnings、fmt；全部符合預期。預期狀態2/1不是harness失敗。

初次Clippy發現tests位於檔中段，移到末尾，不新增allow；work/path-initial-failures.md保留摘要。其後驗證script曾因Python bytes無法JSON序列化而停，已保存13條結果後只補尚未記錄三條；沒有把harness失敗當Rust成功。path-markdown-verification.json實際展開snippets成功。未實際桌面/手機視覺驗證。

10.5 SHA256 `4e6a04dc914b90e51355003dc6bfc0f7734dfa2f3a3ff3e003733be2d1fe55e8`；完整owned/輸入/共享設定hash在path-freeze.json，逐target單檔無lib閉包在path-source-closures.json。沒有自有故意失敗.rs fixture，因此無缺.expected。

## 10.6完成與freeze append

實讀Book EN/TW Arguments、Cookbook EN全文/中文實際clap正文，以及clap4.6.7 Parser/derive/Error API，再放入selected source。root接最小member/lock後才Cargo。早期只選--lib，完整library在這一步之前已教Settings/Parser/attributes/try_parse_from/Error methods；main的print/streams只在後段編譯。

真實first-use位置：
- 10.6:9：先決定
- 10.6:27：用 derive
- 10.6:50：try_parse_from
- 10.6:75：接上真正
- 10.6:104：完整 library
- 10.6:114：練習

完整manifest（含name/version/edition/dependency）、library/main/答案都inline。try_parse_from泛型輸入逐項說IntoIterator、Into<OsString>、Clone、完整首筆程式名稱；clap Error的help/version Err≠用法錯誤、出口/exit_code/print I/O均在使用前解釋。A0610-O01將練習本節模型改為Settings設定資料。show_settings末尾檢查flush，沿10.5已教政策。

clap-verification.json：7個正常或預期狀態CLI/答案路線，stdout/stderr/exit核對；--lib四項測試、Clippy -Dwarnings通過。初次fmt只要求import排序，對本package fmt後僅重查fmt成功，沒有重跑已通過的Rust行為。原始fmt差異保留clap-verification-before-format.json。11筆總記錄含這次已修排版失敗，不宣稱11筆全綠。Markdown snippets成功展開；未桌面/手機視覺驗證。

10.6 SHA256 `1a66bbad509d726af4d6c6b8697c854ce1878fba9dd23b3311f22d471130a99c`；clap-freeze.json完整hash與clap-source-closures.json三種選定target閉包。沒有JSON/final CLI lib穿透，沒有故意失敗.rs或缺.expected。

## 10.7完成與freeze append

實際親讀7全章與必要views source，Book/Cookbook EN/中文JSON實際正文、Serde官方raw及serde_json1.0.154選定API。serde.rs三頁403保留失敗紀錄，實際原文改讀官方GitHub source，沒有只讀摘要。

真實first-use位置：
- 10.7:9：先看要保存
- 10.7:29：Serde 的能力
- 10.7:56：from_str
- 10.7:114：先看要拒絕
- 10.7:156：用具體錯誤
- 10.7:180：檢查模型
- 10.7:196：完整 helper
- 10.7:222：練習

no-lib隔離：basic_json只載json_core root/model/codec；晚段file_errors與json_solution才多載json_files。沒有早期lib自動載NotebookError。pub module沿2.6已教規則，正文說明不因此建立library。所有manifest/source/8份fixture（7 JSON全檔+bin完整單byte255）/答案同頁，失敗資料在第一次error範例命令前完整列出；missing.json刻意缺檔。

json-verification.json六條實際驗證：三個examples（正常/答案精確stdout、error四類分類）、file_errors selectedtest4項、全本節targetsClippy -Dwarnings、fmt，全部符合預期。初次Clippy發現私有shared core中encode在file_errors target未用，改公開共享core介面，不加allow；摘要json-initial-clippy.md，原完整工具輸出保留於turn。snippets實際展開通過。

沿用未修改340px/18px json-model.svg，已讀圖文與箭頭角色；沒有實際桌面/手機頁面視覺驗證。10.7沒有Box/dyn/Error trait、format新語法、未教helper或未freeze9能力。

10.7 SHA256 `4199ad8b74b917794bfa2cdea3fc92c95a7ecf582b3d606393fb463f6a93526e`；json-freeze.json完整owned/figure/config hash；json-source-closures.json區分各target真正source及normal/test runtime fixtures。無故意失敗.rs，無缺.expected。

## 10.8完成與freeze append

先實讀EN/中文Cookbook External Command實際正文與tempfile3.27/std/Cargo選定原文，再保留10.5 standalone CLI快照。本package只有dev tempfile，沒有path-cli dependency，沒有lib或未知source穿透。

真實first-use位置：
- 10.8:9：固定要驗收
- 10.8:32：TempDir
- 10.8:64：Command：
- 10.8:90：讓 Cargo
- 10.8:104：從完整 CLI
- 10.8:132：完整tests
- 10.8:142：練習

TempDir/path/close、fs::write、to_path_buf、try_exists先於directory測試；Command new/current_dir/arg/output、Output三欄、ExitStatus success/code、env!/CARGO_BIN_EXE、str::from_utf8先於cli helper。starts_with診斷契約先於失敗測試，空檔答案與完整tests/cli.rs皆同頁。Drop只作11.3後續實作預告，這節已完整教TempDir具體契約，不需11章先備。

fixtures-verification.json四條selectedcheck：directory1測試、cli5測試（含完整空檔答案）、本package Clippy -Dwarnings、fmt全部成功。cli正常啟動5個child（成功、缺檔、非法UTF8、缺參數、空檔），另檢查不存在執行檔的外層NotFound；每個case持有獨立TempDir直到output結束，再close。沒有重跑已包含的名稱filter答案。snippets實際展開；未實際桌面/手機視覺驗證。

10.8 SHA256 `301e423c4f1e2cd6541442c1bb47c0bf839b335031c6a9dec281ca57c1a4c2e7`；fixtures-freeze.json完整owned/figure/config hash；fixtures-source-closures.json包含selectedintegrationtarget與Cargo自動建的本packagebinary。全部runtime材料由完整測試原始碼建立（文字Rust\n中\n、單byte255、空文字），沒有隱藏fixture或故意失敗.rs/缺.expected。

## 10.9完成與freeze append

先實讀EN與繁中Book 12.3、12.5全文、當前std選定API、實際9.5–9.7正文與必要iterator source，再遷移舊完整CLI。10.9保留永久完整library/binary/tests/答案；10.1–10.8仍是獨立最小snapshot。

真實first-use位置：
- 10.9:9：用材料固定
- 10.9:33：把設定
- 10.9:79：純搜尋
- 10.9:111：先建立
- 10.9:133：main
- 10.9:149：完整source
- 10.9:191：分層驗收
- 10.9:233：練習
- 10.9:85：contains具體&str契約
- 10.9:86：ASCII轉換與owned比較表示
- 10.9:87：從0起的usize與原Item
- 10.9:115：push(char)／LF／unit
- 10.9:178：第一個Cargo命令，在完整能力與source之後

搜尋表示與原文字分開，原行號對全部lines增加，非ASCII不折疊；owned Report不借已結束輸入。read_report先validate再讀取，render先完成再write_all，main再明確flush，讀/編碼/輸出/用法維持具體AppError與1/2狀態。無Box/dyn/std::error::Error先備。

A0610-O02改公開module與run各自工作，移除作者流程；O03以句號收測試介紹，必要Rust字串雙引號escape仍先於測試。所有manifest、7份src、2份integration tests、1份完整答案與唯一固定fixture均同頁snippets；所有runtime測試材料在來源內完整建立。

search-verification.json九條實際檢查全部成功：三種正常CLI資料/診斷/狀態（JSON解析整份stdout並對全部欄位）、答案精確stdout、--lib7項、--test writer1項、--test cli5項、全package Clippy -Dwarnings、fmt。沒有故意失敗.rs、沒有缺.expected；未為已涵蓋的search名稱filter另重跑。snippets成功實際展開。

沿用未修改340px/18px search-boundary.svg，已全文讀圖source、圖說與箭頭角色。尚未實際桌面/手機頁面視覺驗證，獨立首讀/技術/銜接驗收由root安排，不將作者自查當驗收。

10.9 SHA256 `65e47b59a1715e2928ed498294e693de554e246be011eeb00f191d674d295c55`；search-freeze.json保存完整13份owned檔案與figure/config hash，search-source-closures.json區分六份library source、main、integration Cargo自動建binary及example。

## 全章10.1–10.9最終交接索引

chapter-final-freeze.json彙整75份owned檔案（9正文＋10package共66檔），逐一比對各節原freeze全部hash一致；10.1採文字位置窄修後hash，原prefix-freeze.json保留。沒有改shared設定、根Cargo/nav/scripts或舊教材/source；全10個members由root接線與產生lock。

| 頁面 | 最終SHA256 |
| --- | --- |
| 10-01-lesson.md | `2d8fdc5ae52f16c2bce057f6b59f272f79af58d051e932a88ee8b1d552aa6564` |
| 10-02-lesson.md | `394da63f7182a1c977beae9f11180ce370e1efe18851f4fbb0964a5dca4fb79a` |
| 10-03-lesson.md | `0ef86d599bc6a201968cededd1b0971904d3b0a957545558e6654170f626deca` |
| 10-04-lesson.md | `3fc73a9915ecedebe2589e981615aed0271a906fdfafe7bd8741367a20bf2171` |
| 10-05-lesson.md | `4e6a04dc914b90e51355003dc6bfc0f7734dfa2f3a3ff3e003733be2d1fe55e8` |
| 10-06-lesson.md | `1a66bbad509d726af4d6c6b8697c854ce1878fba9dd23b3311f22d471130a99c` |
| 10-07-lesson.md | `4199ad8b74b917794bfa2cdea3fc92c95a7ecf582b3d606393fb463f6a93526e` |
| 10-08-lesson.md | `301e423c4f1e2cd6541442c1bb47c0bf839b335031c6a9dec281ca57c1a4c2e7` |
| 10-09-lesson.md | `65e47b59a1715e2928ed498294e693de554e246be011eeb00f191d674d295c55` |

| 範圍 | 驗證／freeze／逐target source閉包 |
| --- | --- |
| 10.1–10.2 | prefix-verification.json、copy-verification.json、prefix-freeze-after-inline-table.json、prefix-source-closures.json |
| 10.3–10.4 | io-verification.json、io-freeze.json、io-source-closures.json |
| 10.5 | path-verification.json、path-freeze.json、path-source-closures.json |
| 10.6 | clap-verification.json、clap-freeze.json、clap-source-closures.json |
| 10.7 | json-verification.json、json-freeze.json、json-source-closures.json |
| 10.8 | fixtures-verification.json、fixtures-freeze.json、fixtures-source-closures.json |
| 10.9 | search-verification.json、search-freeze.json、search-source-closures.json |

上述檔案都在/workspace/work/ch10-v3。所有選定target最終成功；驗證JSON合計80筆執行記錄，包含副本刻意錯誤doc期望造成的101（符合練習預期）及clap初次fmt失敗1（排序後同檢查通過）。其他已解決的初始環境/Clippy/Python harness失敗另存前述原log或摘要，不能將80筆說成80次初跑全綠。各package的完整已教targets都通過Clippy -Dwarnings，沒有引入額外allow。

全章9頁最終snippets在chapter-markdown-verification.json再做一次文件片段展開核對（沒有重跑Rust），HTML保存於10-01.html至10-09.html，包含10.6 fmt後import次序。讀過4份沿用SVG的實際source與圖文箭頭；未改圖。真正desktop/mobile頁面、root全書harness、獨立首讀/技術/銜接審閱未由本作者宣稱完成。

所有source所需manifest、載入helper、fixture與完整答案都在對應同頁；10.5/10.7以no-lib隔離早晚targets，10.9完整library只在最後載入。沒有自有compile-fail/runtime-fail.rs，故無新增.expected sidecar。所有新first-use與局部首次命令位置保留各節append。

可核對的已知作者未完成項：實際桌面/手機頁面視覺驗證；獨立審閱與根harness屬root整合。未留下已知未修的Rust或snippet失敗。
