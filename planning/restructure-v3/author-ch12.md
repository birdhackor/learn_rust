# 第12章作者交接紀錄

## 實讀研究與來源隔離

本輪親讀AGENTS、clear-tutorial、writing-prompt與Rust術語指引；完整舊9.1–9.4及舊1.3、新1.3／2.6／2.7／10.1與必要實際source。原四個package的manifest、library/helper、main、answer、tests與fixtures均已親讀；舊engineering依賴的ch01-basics library閉包也另讀。55筆範圍及SHA256在/workspace/work/tutorial-implementation-20261015/ch12-author/local-read-ledger.json。新前綴另保存於prefix-read-ledger.json，不以舊稿／inventory／crossref提案當新前文已教證據。

外部21份body包含EN／繁體Book、Cargo Reference、真正中文RustCourse build與中文Cookbook，精確hash/range/逐份採用與不採見research/source-read-ledger.json及research-report.md。403、404、空索引和中文名稱repo的實際英文章另記，沒有算成中文實讀。source-isolation方案在snapshot-plan.md；12.4計畫分size／streams／checked-files／groups，避免第一個size runner就載整份content library。該方案不是已完成或已驗證的程式。

## 12.1作者freeze

root核准後，建立examples/ch09-engineering/lesson-workspace/獨立兩member workspace，package為lesson-subtotal與lesson-engineering。root生成這份snapshot lock並接正式check.sh；作者沒有改教材根Cargo／主lock／nav或原ch09-engineering package。

本節先教workspace.package、rust-version／publish、成員明選繼承、workspace.dependencies相對起點及profile，再展示完整小root manifest；小library只有乘法。app定義default=["batch"]與batch=[]，因此關掉預設功能會真的移除批次介面，保留單筆160。先說attribute移除與cfg!產bool，再載完整app library/main；default與明選batch得到280，答案的零數量變體得到120。integration test角色、功能組合與release驗收也在首次命令前說明。

小workspace後實際對照教材repo根的共同metadata／dependency／release三組設定，fixtures/repo-root-settings.toml與當下根檔suffix完全相同。另解釋根members／exclude與選package的角色；根目前沒有workspace.lints，本節不虛構這項設定。這讓後13+成員採用根metadata有實際前文，而不是只借小workspace的欄位名稱。

13個實際成功命令保存於validation-12.1.json：6次runs（主程式三配置、答案兩配置與release）、3組all-targets tests、2組all-targets Clippy且-D warnings、fmt檢查與release build。default／no-default／all-features的features.rs測試數為2／1／2。故意失敗的rustc診斷獨立保存，實際包含E0425，據此建立同名非空.expected。作者逐份讀真stdout，核對160／280／120與Unix bool；沒有用單一成功run冒充全部配置。

snippet-audit-12.1.json確認所有實際Rust與TOML完整同頁，含root、兩member manifests、兩library、main、tests、answer與故意失敗程式。沒有隱藏module或舊ch01依賴；後續正文小修沒有重跑未改Rust。未執行全repo scripts/check.sh，也尚未做新版網站原位桌面／手機檢查；獨立審查與版面由root安排。

13檔初次freeze位於/workspace/work/tutorial-implementation-20261015/ch12-author/freeze-12.1.json，保留原始hash作後續獨立審查基準：

- 12-01-lesson.md：`ec0a88e410fbee7c7ce4ca3278be446758483d5dbdf7121bd2f91f1a7fed6770`
- freeze-12.1.json：`d26b0df23a87a5fee4dae942d77091c6c86a75c1aa6ee4fe8bad21974159c26d`

12.2可沿root已放行的10.5實稿繼續；作者已親讀其全文與path_types／os_strings／working_directory及manifest。若使用11.5的static／11.7的Error，仍須先親讀必要全文及實際source。12.3／12.4的TempDir與CLI fixtures則待實際10.8交接，不把10.1僅宣告tempfile當API先備。

## 12.2作者freeze

作者親讀實際11.5全文與static_reference／manifest，並重讀實際8.1、8.3及format呼叫者／impl，新增hash/range於prefix-read-ledger.json。10.5全文及path_types／os_strings／working_directory的實際來源也已親讀；env!基本角色在本頁首次介紹，未把尚未交接的10.8名稱直接改成已教。另實讀std env!全文26行、option_env!全文23行、VarError 1–86及Cargo Profiles 1–96／150–190，詳research/api-read-ledger.json，不宣稱其餘長API已讀。

沿用原ch09-build-script的完整manifest、build.rs、library、main、integration tests及identity答案；沒有外部依賴／隱藏module或後文來源。新增完整optional_build_tag與missing_env負例，前者先說明Option<&'static str>的Some／None／非Unicode契約，後者保存真實編譯時缺值診斷與同名.expected。OUT_DIR／生成責任、build-dependencies分工及rustc-link搜尋／名稱責任全保留；C與ABI實作留15章。

validation-12.2.json分開保存11個成功命令及3個故意失敗：未設定／demo／practice的正常build/run、直接runtime改demo仍為practice；optional標記缺值／lab及直接runtime改值仍為lab；完整identity答案、all-targets測試兩項、fmt與Clippy --all-targets --locked -- -D warnings。wrong與非Unicode LESSON_MODE皆由build script使Cargo失敗，missing_env則由rustc編譯失敗；三者沒有混算成正常成功。Unix非Unicode驗收以本次實際byte255環境值執行，不依賴無法重現的外部配置。

完整檔案與第一次需要的說明逐項核對於snippet-audit-12.2.json。build-phases.svg先讀原XML並實際嘗試render；初次通用sans在工具中缺中文字形，未算視覺通過。新engineering-v3副本明選Noto Sans CJK TC後，實際render親看340與300px寬，中文字與TEACHING_MODE均完整，24px文字在300寬仍約21px。圖的四框與script／library／main時點相同，箭頭表示先後。尚未做新版網站原位桌面／手機驗收，不以單圖預覽代替。

11檔freeze位於/workspace/work/tutorial-implementation-20261015/ch12-author/freeze-12.2.json；12.1原freeze維持：

- 12-02-lesson.md：`d2cd09193df22ad0493d084fe7bb247a885377a9cc4930009b7ec7d67c9184c0`
- freeze-12.2.json：`91fd9be582bbcd97ceda6562bc923796bfff7703d81df2eda9c964fbf180e6be`

此版可交root親讀供15／FFI build gate；獨立審查另排。12.3／12.4接下來所需的10.8 fixtures仍須等實稿交接，11.6／11.7則root已放行，作者尚須親讀必要實際內容。

## 12.1技術初判後最小修正

root採納獨立technical可選TC12P-O1：all-targets命令前的「每份程式都已完整列出」改為明說完整來源包含下方答案，整包檢查會編譯它且僅用本節已教介面。只改這一句prose，Rust／原13檔freeze其餘指紋不變，未重跑Cargo；原freeze-12.1.json與獨立初判保留。新頁hash：`5f7ab3792ff83635cc74b2147311ae448cd914c74f18e86c96aa54c0e3cf61f0`。追加freeze-12.1-tc12p-o1.json、followup-tc12p-o1.json與technical-tc12p-o1.diff保存before／after與處置。

## 12.2 root首次載入順序小修

root親讀實際載入閉包後指出：embedded測試段的all-targets會先編譯下方才介紹CARGO_PKG_NAME的identity example。這是root作者階段檢查，不算獨立technical finding。該處改成`LESSON_MODE=demo cargo test -p ch09-build-script --test embedded --locked`，並將原all-targets整包指令放到完整答案之後。新selected命令已單獨實跑，兩測試通過，真stdout／stderr保存於validation-root-embedded.json；Rust未變，沒有重跑其他成功targets。

原freeze-12.2.json與原all-targets證據保留，追加root-first-use-embedded.diff、followup-root-embedded.json與freeze-12.2-root-embedded.json。修後12.2 hash：`846d2893b7c8c692a00efa7792cf4256cde6732e35f118d104585ed72bdfba7a`。

## 12.3作者freeze

root交接10.8後，作者全文親讀實際10.3／10.4／10.8與11.6／11.7，並親讀10.8全部四份manifest/main/tests、10.3 File來源／寫入範例、10.4 library／ShortReader／InterruptedThenFailure，以及11.7 leaf錯誤實際source。10.8正文hash確為301e423c4f1e2cd6541442c1bb47c0bf839b335031c6a9dec281ca57c1a4c2e7；範圍與hash追加prefix-read-ledger.json。必要API原文的讀取範圍見research/files-api-read-ledger.json（walker builder/entry/error、metadata、symlink_metadata、Path等），未讀的長頁不算全文。

12.3沿用ch09-files的完整scan元件，沒有為prose重寫Rust。新正文先列三份完整輸入（5／5／6 bytes），把symbolic link政策、metadata觀察、成功與錯誤資料、兩個link開關、DirEntry ownership、Option::map/is_none、原生Path身分與排序依序教清，再收完整scan.rs。manifest、lib根、main、三份輸入、八項測試與完整答案全部同頁。原來「定義下列資料」的等待對象措辭改為「用幾個型別分開保存結果」，保留各角色先介紹再接型別程式。

crossref按實際語意對到新10.5原生路徑／10.8 fixtures／11.5 static／8.3 Display／9.4 IntoIterator／9.5 map／9.7排序；Eq／Hash／Borrow實際回用6.5，沒有盲採proposal中不精確的6.6候選。partial move回用4.5。沒有為本節引入dyn Error；具體ScanCause仍依8.3提供Display，Result也不要求Error。

validation-12.3.json保存本節首次selected驗證的五個成功命令：主程式、path_identity_solution、八項scan測試、fmt及Clippy --all-targets --locked -- -D warnings。原Rust未改，這是本輪首次建立12.3執行閉包的證據，之後prose變更不重跑它們。八測試中的受控失敗有自己的assertions，沒有把它們列作三個或八個外部失敗命令；本節無新增compile/runtime-fail檔。真stdout為hidden 6／first 5／nested 5與掃描完整true；答案查得5 bytes並保留issues=0。

十檔freeze與snippet-audit-12.3.json存於ch12-author；全部package檔都有同頁完整snippet，首個執行命令前已教完scan所用API。未作新版網站原位桌面／手機驗收，獨立審查由root排定。

- 12-03-lesson.md：`35e30859aa3cfac9752493c2e76843cfcd36c65d674bfa37c79bd8dc0c7ea8b2`
- freeze-12.3.json：`2a66e6be7985b0cba19b1c77cc42e19b59ba58e8cc83e489fd122a69b56db13a`

## 12.4與全章作者freeze

本節四stage快照為examples/ch09-content/lesson-workspace/，root已生成24-package獨立lock並接check.sh。size只有scan依賴；streams的library只定Read摘要／比較及兩個已教來源；checked-files的library只定前後觀察，candidate helper留example-local module；groups才提供refine／metrics。未改原ch09-content根package，也沒有載它一次export五個module的library。四member與小root manifests、全部Rust/helper/test、scan dependency及固定輸入、A／B／B完整答案都同頁。

原六項streams與十二項files/groups測試全部保留。後者的兩項candidate測試搬到candidate example-local helper，groups保留其餘十項；沒有丟掉候選雙鍵、局部成功拒絕、singleton驗證、精確失敗者重建、可觀察變動、hard link與checked metrics。正文按實際新序回用9.5／9.7、10.4／10.8、8.2／8.4與7章單一來源關係，另外親讀實際8.4全文與TryFrom實作source，記prefix-read-ledger。

inventory末項寫「可控制碰撞callback／失敗來源」，但凍結舊9.4及實際refine_bucket沒有外部callback參數；root確認依實際來源，不新造API。對應控制完整保留為手工同digest key的A／B／B、不同正數短讀取上限、Interrupted／PermissionDenied／左右來源、確切FileIssue與受控前後變動。三份同摘要只作候選，最後分1／2兩組、3 bytes；沒有聲稱找到BLAKE3真實碰撞。

validation-12.4-initial.json保留初次真實結果：fmt與四個先前stage runners成功，groups binary因tempfile誤留dev-dependencies而得到E0432，整包test／Clippy也因同一原因失敗。原來答案是example，可用開發依賴；搬成獨立binary後必須明選一般dependency。修正groups manifest並在首次展示前說清這項分類，只重跑未通過的groups run／整包test／Clippy；其餘成功runners及未改Rust的fmt沒有重跑。validation-12.4-manifest-fix.json保留修後真輸出，--locked通過，作者沒有改root生成的lock。

validation-12.4.json整理最終八個成功命令：五個runners、18項all-targets測試、fmt與Clippy --workspace --all-targets --locked -- -D warnings；初次非預期錯誤仍單獨保存，不改寫成預期fixture。size候選數2、streams相等true、真檔案相同／不同true／false、候選桶2筆，以及A／B／B的1／2組與重複量3均已逐份讀真stdout核對。source-list-ch12.txt列全章精確凍結檔案。

兩張內容圖先親讀原XML，再以Noto Sans CJK TC副本實際render、親看300px寬（原340px／24px文字）；圖的四格buffer與實作4096格差別、a／b2／b1對應實際檔名都在正文交代。原始read切分／相同位置比較段、候選桶／內容群組與各組代表的關係保留。三張本章SVG XML均有效。snippet-audit-12.4.json保存全檔與module閉包核對；仍未做新版網站原位桌面／手機驗收。

12.4新增21檔freeze為freeze-12.4.json：`6839c8fbf6c6f1382287e8acb06432abdb128e98c1bf024e0c5b099e0539d220`。全章最終55檔freeze-ch12.json：`cb55556e3c505d18a6273685ba7e31ab05cd43ff14f90c643adb7537228871f8`。它合併12.1 TC12P-O1、12.2 root embedded修正版與12.3／12.4作者freeze；早期各版freeze、原始初判與diff均保留，不覆蓋。

最終四頁hash：

- 12.1：`5f7ab3792ff83635cc74b2147311ae448cd914c74f18e86c96aa54c0e3cf61f0`
- 12.2：`846d2893b7c8c692a00efa7792cf4256cde6732e35f118d104585ed72bdfba7a`
- 12.3：`35e30859aa3cfac9752493c2e76843cfcd36c65d674bfa37c79bd8dc0c7ea8b2`
- 12.4：`eaf2e0526c1979e1601af0ff3f91ade1d716f0a6ac88fcfa78f676abe0820a37`

本輪最終共38個成功命令（21次runners、7組測試命令、5組Clippy、4次fmt、1次release build），四個故意失敗命令另記；Linux當前prefix共30項不同測試定義，功能組合及root新增selected命令另有重複執行。這是第12章selected驗證，不稱全repo checks或全書獨立驗收。作者全章交接後依root指示停止，後續18A等另行指派。
