# 第15章 ORIGINAL TECH：A-prefix，v11

本份是未寫作第15章的技術原判，只處理15.1–15.6。讀完六頁正文與全部封存來源後，實作的主要資料、錯誤、ownership與出口契約可以對上。我列出一個很小的必要先備銜接項 N1，以及一個可選的工作量口徑項 O1；沒有把可編譯當成「已教」，也沒有把約千行變成硬上限。

## 身分、隔離與輸入

- reviewer：`/root/chapter01_foundations`。我曾寫第1／8章、11.4及14.4／14.5／14.7／14.8；**未寫15任何正文、source或答案**。因此這是非作者 TECH，帶有前章作者背景，不能冒充基本Python讀者的無記憶首讀。
- 唯一第15章輸入：`/workspace/work/tutorial-implementation/frozen/ch15-a-for-review-v11`；manifest SHA-256：`b0943a259e99dc064a7789964b5b21152ea50e632d6a37c98f29bde0e93c8c62`。
- 逐序全文讀完42 BODY units（6／7／8／8／7／6），以及101 archive paths，含A1～A4 starter／complete、完整答案、README、manifest、helper、unit／integration tests及真實材料。三份SVG的XML也讀完。
- 101 paths實際有63種完整檔案bytes。每種都全文閱讀；相同bytes副本由我自己的SHA及完整bytes核對對映。四份starter checks另證實是已全文讀完complete source的**精確完整suffix**。不是抽樣讀一個版本後就視其他不同版本已讀。
- 本人開始／結束比對148個frozen實體檔，完全未變。沒有修改student、shared配置、staging或original輸入。只在自己的work副本做preview與finite probes。
- 沒讀15作者notes／ledger／handoff、root gate／disposition、CONT15／FIRST15；沒有讀B1～B5正文/source、15.7–15.11或13作者材料。15.1授權頁面的B題目介紹已讀，但**不判B實作、網路政策或負擔**。
- 開始前實讀AGENTS、clear-tutorial、review-protocol及術語指引；指紋在本人的 `instruction-fingerprints.json`。術語以台灣正體正文與必要英文技術名核對，沒有要求heap／stack改中文。

## 先備核對範圍

本次重新全文實讀actual 9.3／9.4（`ch09-review-fixed-v6`）、7.6～7.9（`ch07-review-fixed-v13`）、8.6（`ch08-review-fixed-v9`）、14.4（`ch14-review-fixed-v10`）。早先CONT12工作曾實讀actual1～10所需正文；這次依真實source需求再核4.2／4.3、6.5～6.7的具體宣告。沒有把outline、章名、root摘要或作者實測紀錄當已教證據。

A2 checks的 `fixture(&[...])` 用到array-reference→slice coercion。我本人先前完整讀過actual12，這次再讀 `ch12-review-fixed-v5/003-005.md`：shared與mutable array reference可在預期slice位置轉成相同元素的slice，無元素copy。它已在15之前的實際正文教過；不能誤報成新coercion偷渡。此依賴與原先1～10的主要scope分開披露。

本題未用proptest／property策略；沒有將14.5當額外實作必要，也沒有讀其作者紀錄。所核14.4的 `pub use`／private module／non_exhaustive／`--manifest-path`，均在15使用前已有實際契約。曾寫14.4的背景保留，不把對該節的重新閱讀算另一份獨立QA。

| A階段實際需要 | actual或本地先教位置 | 結論與界線 |
| --- | --- | --- |
| literal Cargo欄位、library/module、unit tests、check/test/fmt/Clippy、`--locked` | 1.2／1.3／1.9／1.10；7.1的版本／feature／dev-dependencies | 各starter manifest不靠workspace繼承。15.1先教repo外 `+1.99.0` toolchain選擇、先解析建立lock再locked；空tests不冒功能成功。 |
| PathBuf／Path原生位置、snapshot、walkdir、途中Err保留 | 9.3完整BODY與對應source；15.2責任分工 | root不能開始和中途項目問題分開；兩種follow設定、Unicode報告政策與native排序不是事後靠答案才得知。 |
| BLAKE3、Read短讀／Interrupted、bytes相等、metadata讀前／後觀察 | 9.4完整BODY與support source；15.3來源與callback契約 | 舊9.4介面重用於本package自己的FileRecord；重建是本地先解釋的新組合，不假單次分組能自行恢復。 |
| FnMut、IntoIterator、filter／collect、slice索引／排序、owned與借用 | actual4／6；9.4；15.3模型及三個工作 | callback重複呼叫用closure借同一compare；`ContentGroup<'a>` references未逃出records，成功才保存owned paths。source用明確as_slice／as_mut_slice。 |
| 同一Report兩格式、Serde模型、完整ties與checked metrics | 7.7／7.9、6.7／9.4；15.4先教Option Ord／as_ref與新增欄位映射 | `Option<T>: Ord`、None在Some前、as_ref不move且不是AsRef trait，都在code之前。N1只涉及一項checks的reference相等形式，見下。 |
| 真CLI、TempDir／Command／Output／env!、write與flush、clap顯示請求 | 7.4～7.9；15.5明教complete與出口完成分開 | 不用process::exit，不把output()的Ok當child成功。fatal診斷自身失敗的 `let _ =` 用途本地先說明。 |
| 公有payload可命名、non_exhaustive、Error::source與borrow界線 | 8.6、14.4；15.6先核標準error能力 | 私有wrapper不全實作Error亦可；source直接借標準原因，reference仍借self，`'static`不表示永久reference。 |

## 必要項 N1：A3 checks的 `&String` 相等，缺最低使用契約

**位置：**封存 `sources/examples/ch15-projects/a3-starter/checks/report-tests.rs:203`，及 `a3-complete/src/report.rs:420` 的 `text_escapes_control_characters_and_json_round_trip_keeps_data` 內：

```rust
for path in &recovered.groups.as_slice()[0].paths {
    if path == &name {
        found = true;
    }
}
```

此處path與右邊都是 `&String`。4.2實教String內容比較與 `.as_str()` 得到的 `&str` 比較；4.3具體實教的是**數值reference**的比較。這次可核actual正文沒有在這個first-use之前交代 `&String` 的比較會沿所指String的PartialEq；一般 `==` 會借用operands（4.2）也不等於已教reference型別的這項標準實作。`for &Vec` 的IntoIterator則已由6.5教，並非本項問題。

我查固定Std1.99的PartialEq宣告，`impl<A, B> PartialEq<&B> for &A where A: PartialEq<B> + ?Sized, B: ?Sized` 確實支持它。因此**不是程式錯誤，不是位置或字串address比較**；但使用者的「測試／helper也只能用已教能力」要求仍有一個小缺口。完整source和checks作為讀者驗收材料，不能因正文只展示部分tests就免除首用契約。

最小修正可直接寫 `path.as_str() == name.as_str()`，沿已教的借用文字比較；或在15.4安裝checks之前，短說shared reference的PartialEq沿所指值。本次不要求介紹完整泛型blanket impl，不需要添加鏡像tests。相同report source另出現在A4 starter／complete（本人exact-byte已證實），應一起保持一致。若cross能指出更早actual正文已教這項契約的精確位置，應依那份正文重新處置；此原判不讀未授權材料來猜它存在。

**重要程度：**必要的局部先備銜接；不阻止認定核心演算法、JSON或CLI行為可用，也不升級成整章重寫。

## 可選項 O1：入口工作量口徑與最終答案對齊

15.1開頭說約800–1200行，**包含程式與測試**；15.6及final README明說核心995、測試／fixture662、合計1657，含空行／註解／文件測試，不加stage副本。我自己的逐檔splitlines計數完全吻合。

最終頁已誠實交代tests另供、不需一次重寫，也把工作拆成A1三函式、A2三函式、A3三函式、A4兩函式及最後API整理。這仍符合使用者「約千行、不用精準、不要負擔太大」方向，**不是超硬行數而必須砍功能或tests**。

可選把15.1入口改成「核心約千行；驗收另供參考，總量在完整答案列出」，或保持800–1200的目標但先說目前A的core/tests口徑。讓讀者一開始就知道自己要實作哪些、新增tests是否必寫，而不用到15.6才換計數口徑。B尚未審，不將A的1657套到B。

## 主要通過項：需求、機制、用途及材料支持

1. **A1收集的是待判材料，還不是重複答案。** 先固定六檔含hidden／nested、同大小不同bytes、空檔，再讓root驗證、observe與walk逐項處理。RootError／ScanIssue各保存不能開始與可繼續問題。complete的八tests包含有效空root、不存在／一般檔root、directory observe、Ok/Err/Ok受控來源、Unix root／inner links與nonUnicode native位置；不能把第一項根Err吞掉或把後續成功漏掉。metadata.modified失敗未受控、非Unix未執行與一致快照不保證，正文有說明。
2. **A2把候選與相同bytes分開。** size>=2才hash，digest桶還需各組代表真比較；強制同digest的A/B/B材料不是宣稱找到真hash collision。單一路徑去重、正常false與FileIssue區分、左／右讀取錯誤保確切offender，均可由support／complete／tests對上。短讀以fill_chunk對齊，Interrupted重試，不把read分批方式當內容差異；binary／empty／9001bytes與尾端改動各有明確結果。
3. **失敗重建有必要且有有限進展。** callback規則先教；refine Err不交暫時群，confirm從全部剩餘重新判定，移除exact native path後數量嚴格下降。a代表讀錯後B/B仍找回的材料直接支持重建用途，不是假想其比原分組快。違約panic防作者接錯callback；正常I/O保留問題。真公有入口不暴露callback。
4. **A3保持資料身分到排序完成。** Path按components排序的a/b、a.txt例子與String排序有不同答案；group／members與全部issue ties要排序完整，HashMap／walker到達序及stable sort不能替此需求作保。暫時native key最後才去掉，不用Debug／lossy文字回開檔；Option None排序與action／reason／debug的ties也有受控模型。JSON與text取同owned Report，`deny_unknown_fields`、u64／Vec／Option-null映射本地先說；控制字元能讀回，text按Debug維持一項一行。乘／加溢位用小型u64::MAX模型，不造巨檔；hard links按不同路徑計數，不承諾回收實體空間。
5. **A4把掃描完整和成功交付分開。** write_all與flush兩步成功才選complete的0／1，任何輸出失敗回2；stdout已交部分bytes不能假稱能回收。可控制writer明示不是OS buffer，拒write／拒flush的狀態要求各自有tests；真正child另驗完整JSON／text相同Report、部分結果1、empty0、用法2、help／version0、root fatal2。native OsStr argument不再經shell拆分，TempDir owner保到child完成。一般問題在Report，fatal進stderr，沒有混入JSON進度。
6. **完整答案收好公有邊界，沒有偷改stage相容性。** private modules＋root pub use讓caller只用Path與owned Report，也可借typed Error標準原因。RootError／ReportError／MetricError公有payload都有root名稱；Report公開欄位未被說成外部任意修改仍保證有效掃描。non_exhaustive外部caller留其他分支、public_api真integration caller、TempDir空root doctest與README起點/lock/CWD都可對上。這是另一artifact，不聲稱刪A4公有module仍保原caller相容。
7. **引導型練習有可操作起點，關鍵要求不藏在提示內。** 各節先列成功／失敗與模型、給可編譯但明示未完成的stub，再把checks手動加入的方法教清楚。驗收先失敗，按最小責任逐個寫，三層提示由資料去向到已教能力再拆一分支；末尾答案摺疊及同階段完整snapshot都保留。提示關閉時主文仍有機制／先備／判準，答案不靠未來stage才能執行。本角色已見全部答案，不把這項結構核對冒充無提示首次自寫體驗。

## 本人執行的有限驗證

在 `work/ch15-technical-a-v11/probe-workspace/` 複製A1 starter、A2 complete及final。root只含這三member，沒有B；保留literal package manifests，使用自己的lock。final source bytes保持封存不變；A1只按正文把整份checks附到自己的collection.rs，已保存原檔與附加suffix證據。並未修改repo正常tests讓它們變紅。

工具鏈固定 `cargo +1.99.0`、Linux x86_64。最初我在repo外漏寫toolchain selector，rustup因沒有default而拒絕；更正成已教的selector後才算執行。這是本人probe設定問題，不是教材失敗。lock的selected版本為walkdir2.5.0、BLAKE3 1.8.7、clap/clap_builder4.6.7、Serde1.0.229、serde_json1.0.154、tempfile3.27.0。

| 有限驗證目的／target | 本人結果 |
| --- | --- |
| A1貼checks後 `--lib --no-run` | 編譯成功。 |
| A1 `six_material_files_include_hidden_and_empty_entries` | **預期失敗**：exit101，`有效根目錄: NotDirectory`；確定是stub行為失敗，非編譯錯誤。7項filtered，不冒全starter實測。 |
| final `failed_left_reader_is_removed_and_remaining_b_pair_is_rebuilt` | 1 PASS，7 filtered。 |
| final `writing_or_flushing_failure_overrides_complete_or_partial_status` | 1 PASS，7 filtered；各controlled組合在同一test內。 |
| 真CLI `real_cli_text_and_json_preserve_the_same_complete_data` | 1 PASS，6 filtered；整份JSON可解析，兩群／10bytes／complete與text同資料。 |
| 真CLI `non_unicode_item_makes_a_real_partial_report_and_exit_one` | 1 PASS，6 filtered；partial1、仍有成功兩群、問題保存native Debug位置。 |
| integration `public_root_error_keeps_typed_reason_and_shared_source` | 1 PASS；typed NotFound與借出的source能對上，leaf None。 |

沒有重跑workspace或各stage全tests／Clippy／Miri／benchmark。未執行其他case不寫PASS；例如forceddigest／overflow／hardlink／left-right短讀／changed與CLIhelp等，在此是**完整source及契約閱讀通過**，不是本人本輪額外執行結果。

## 固定官方契約實讀

直接讀快取registry的固定版本selected docs／實作；不是下載全文就宣稱全讀。逐檔SHA與具體範圍在 `selected-api-proof.json`：

- walkdir2.5.0：new／follow_links／follow_root_links的default與by-value設定、DirEntry path／into_path、Error.path的Option；核root link預設true，因此明確false有作用。
- BLAKE3 1.8.7：Hash Clone／Copy／Eq／Hash與PartialEq、new、update借bytes、finalize shared且可重複；只作候選，不把摘要相同等同全bytes。
- clap_builder4.6.7：try_parse_from的IntoIterator／IntoOsString+Clone、use_stderr的Help／Version分流與exit_code；不沿用parse／exit的立即終止。
- tempfile3.27.0：tempdir回io::Result、path借用、TempDir Drop嘗試移除且忽略清理錯誤，owner不能先結束。
- Serde1.0.229／serde_json1.0.154：u64與Option映射、pretty借Serialize回String、from_str帶輸入lifetime的Deserialize限制、Error標準原因／Debug／Display；未把通用Value／from_reader當必要。
- 固定Std1.99：PartialEq的reference blanket宣告、Path/PathBuf Ord經compare_components、io::Error與TryFromIntError的Error／source能力。Std與第三方source內部可能有別的語法；它們是review查契約，沒有放入學生範例，也沒有要求學生實作那些內部unsafe。

## 實頁與圖文核對

本人用42個expanded BODY按順序重組**自己的六頁preview**，只去掉凍結工具重複的合成H1，將frozen圖URL轉本地同bytes assets。strict build完成、No issues found。複製了通用 `docs/stylesheets/tutorial.css`，HTTP親核CSS與三圖body為200且同bytes；沒有重演之前CONT12原判的缺CSSfile限制。

使用1280×800及390×844，**36張全部親看**。範圍是三圖在各裝置的原位＋完整元素（12張），入口／工作量、A1檢查表與checks啟用段、A2主線及提示3開啟、A3驗收表、A4出口表／答案關閉和開啟、final公有入口／工作量（24張）。圖的操作先後、失敗path=a、B/B剩餘、native→String保序、兩步輸出後0／1與任Err2均可讀，箭頭及文字未改概念；表格可換行，fold可辨認主文與參考答案。code仍用自身橫向捲動區，不以窄螢幕截到長code右側推斷程式缺字。

此preview保留所用theme／extensions與複製CSS，只有授權六頁nav、**未啟用navigation.top**。這是明示的隔離配置，不是聲稱shared/global已更改或GitHub Pages實站已發布。只有light mode、兩種viewport；沒有全頁每個scroll位置、所有折疊、所有行動瀏覽器、dark mode、鍵盤／screen-reader或全外部link驗收。

## unknown、漏查與封存

- 本角色見過完整答案及前章作者背景，無法判定Python讀者首次獨立寫完需多久；A2回收多層組合的負擔由FIRST／CONT角色另判。此處只確認工作版可用、任務／材料／機制明示、hint關閉主線不缺要求，不宣稱真受試教學實驗。
- 沒跑非Unix／Windows／macOS，也不證明symlink對抗惡意替換、filesystem一致snapshot、同time同len無變動、磁碟空間回收、真OS輸出flush故障或所有stdout中途失敗；正文的有限承諾保持。
- N1的first-use判斷基於已實讀／明確可核的actual範圍；沒有把未讀13或B、peer/root資料拿來補先備。可在cross依新提出的精確actual位置處置，**本original不回改**。
- 五個PASS及一個expected failure僅本人所列finite probes。完整source／API核對不是alltests執行、發布signoff或實站QA。三圖36實見同樣不是所有sharedUI測試。
- 原始逐頁閱讀checkpoint保留早期unknown和後續核實進程，沒有由終判回填早期「已知」。停止於A-prefix；等B完整原稿另授權後才接獨立tail，不先cross。

證據根：`/workspace/work/ch15-technical-a-v11/`。包含 `input-fingerprint.json`、`source-byte-map.json`、`embedded-copy-proof.json`、`prerequisite-fingerprints.json`、`reference-equality-search.json`、`selected-api-proof.json`、`coverage-final.json`、probe logs/lock與36圖manifest。報告同bytes初判副本與SHA另存於 `original-seal.json`，以本份首次封存為準。
