# 新第11章來源：11.1–11.2實作交接

日期2026-10-15。本次正式freeze範圍是11.1/11.2；其他節來源研究與snapshot計畫仍留在work的ch11-preparation，不把下載過或知道標題說成已讀／已教。

## 實際英文／中文來源與採用界線

| 來源 | 實讀範圍 | 採用與未採 |
| --- | --- | --- |
| [Rust Book15.1 EN](https://doc.rust-lang.org/book/ch15-01-box.html) | cache book-en-box.txt單元1–51全文；兩張原SVG也已實際看過。 | 採遞迴直接內嵌→大小循環→Box handle；不採Box總在stack、沒有成本、精確enum大小等概括。 |
| [繁中Book15.1](https://rust-lang.tw/book-tw/ch15-01-box.html) | book-tw-box.txt單元1–57全文。 | 同樣的owned間接存放關係；用本書已教的named fields/match/loop，沒有引Lisp API或未知derived顯示。 |
| [Book advanced types EN](https://doc.rust-lang.org/book/ch20-03-advanced-types.html)／[簡中](https://kaisery.github.io/trpl-zh-cn/ch20-03-advanced-types.html) | EN單元46–61、ZH49–64 Sized/DST段；較早newtype/alias選段也已讀，但不在本輪採用。 | 採預設Sized與?Sized、str/reference分開；Rc/dyn、closure alias等後文內容不帶入11.1。 |
| [Rust語言聖經：泛型原始Markdown](https://github.com/sunface/rust-course/blob/main/src/basic/trait/generic.md) | 實讀本次原文行353–479；353–440對應const泛型，442後nightly段只讀不採。 | 採不同array長度是不同型別、const N是值參數；不用nightly const expressions，不複製矩陣／效能歷史結論。 |
| [Rust語言聖經：Sized原始Markdown](https://github.com/sunface/rust-course/blob/main/src/advance/into-types/sized.md) | 行1–140全文。 | 採Sized分類與放寬預設bound；不採把reference限定stack／內容限定heap、不合法的runtime array等同DST、Box<str>沒有length metadata等推測。 |
| [Reference DST](https://doc.rust-lang.org/reference/dynamically-sized-types.html) | cache reference-types.txt單元1–10全文。 | 核對slice／str與其pointer metadata、泛型Sized預設；不帶trait-object或unsized尾欄位細節。 |
| [Reference generic parameters](https://doc.rust-lang.org/reference/items/generics.html) | 單元17–36，const generics及其值／型別位置。 | 本輪只教usize N與`[T; N]`；不教generic_const_exprs等未需要機制。 |
| [std size_of](https://doc.rust-lang.org/std/mem/fn.size_of.html)／[size_of_val](https://doc.rust-lang.org/std/mem/fn.size_of_val.html) | 新cache文字1–47／1–15；採用size_of的1–33基本契約。 | 核對array n×elementbytes、i32/bool、資料與reference查詢；repr(C)、raw pointer、struct layout演算法不進正文。 |
| [std boxed module](https://doc.rust-lang.org/std/boxed/index.html) | 新cache文字1–70，intro/examples/memory layout前段。 | 核對owned heap、取出與Sized單pointer；不採「所有遞迴都只能Box」的概括，不教allocator/FFI/raw API。 |

原文、原始HTML／Markdown／SVG與SHA-256保存於 `/workspace/work/tutorial-implementation-20261015/ch11-preparation/research/`。fetch-manifest.json是先前完整研究頁，original-markdown-fetch-manifest.json保存兩份實際中文原始Markdown；sized-box-primary-fetch-manifest.json保存新std契約；book-box-images-manifest.json保存兩張原圖。course.rs實際回403，改從該教材官方GitHub原始檔取得200，不把失敗頁說成已讀。

## 舊8.1的完整保全與隔離

| 舊來源 | 新位置 |
| --- | --- |
| 8.1:7–116；sizes.rs/fixed_arrays.rs及caller | 11.1 Sized/DST/?Sized/const泛型完整教學；size_support與array_support分開，無Box；新增同頁array_bytes完整答案。 |
| sized_default／const_generic_length負例 | 11.1完整fixture＋expected，同頁解釋E0277/E0308。 |
| 8.1:1–5、117–211；boxes.rs | 11.2先建立遞迴資料需要，再教Box owned/borrow/move-out與完整caller。 |
| recursive_inline／box_after_move | 11.2完整fixture＋expected，同頁E0072/E0382。 |
| chain.rs型別／into_total／兩tests；chain caller | 11.2 chain_support明寫module，只載這兩項功能與當下tests。保留空鏈、單項、兩項、零與負數。 |
| box_solution.rs | 11.2 chain_answer.rs，同頁完整main，完整helper已於同頁提供。 |
| boxed-chain.svg | 新types-v3路徑保留同圖，正文直接對應80/120/End與owned箭頭；作者實際看過渲染圖。 |

Book之外的來源用來核對舊章各自的Sized/constgen責任，沒有以Book15.1取代它們。11.1不靠11.2的動機或Box練習才能成立；11.2不載舊ch08 umbrella library的Rc/RefCell/dyn/Error等後文。

其餘舊8.2–8.10的152檔實讀證據、Book15.2–15.6 EN/TW完整讀取與advanced traits/traitobjects/RBE/Reference選段，保留在preparation/bookmark與source-notes。後續正式寫作需再依actual gate補必要std契約；本次不宣稱11全章已完成。

## 11.3–11.5 來源追加

- 11.3保全舊8.2全文責任：Drop Scope／early／fields／完整答案，各自獨立完整target；四個Drop負例及expected同頁。Book15.3 EN/TW已全文實讀，單元1–29／1–34；不把正常scope輸出說成所有結束途徑的保證。
- 11.4保全舊8.3的Wrapper/Deref/DerefMut/coercion/method lookup/owned取出／兩負例。Book15.2 EN/TW全文實讀，單元1–61／1–72，EN亦在正式寫作時重讀；不照搬Box move-out作為所有自訂Deref的能力。basic不載完整views；完整views及兩tests到答案才載入。
- 11.5保全舊8.4的Borrowed、T:'a、implied bound、static literal、type static bound、owned含borrow反例、普通block收尾、Wrapper答案與三bounds tests。RBE static EN/ZH各單元1–21全文實讀；Reference trait-bounds單元41–51的implied-bounds段亦實讀，核對struct與impl推導。RBE概括「any owned data always passes static」未採用；Box<borrowed>與TextView直接作反例。
- root確認Cow並非核准outline內容，已撤回新增需求。原先發現舊稿/source無Cow的紀錄保留，不另外撰寫Cow。

三節實測與凍結紀錄見author-ch11追加段；版本不覆寫11.1/11.2原freeze。後續dyn/Error/Cell等必要契約仍按實際使用處補讀，不以Book15涵蓋所有責任。

## 11.6–11.8 來源追加

- Book trait objects EN 1–39／TW 1–53 全文親讀；Reference traits 21–35 核 dyn compatibility（Self:Sized、generic、Self回傳及associated type）；Reference coercions 1–49 核 unsizing；不把 Deref 當成 owned unsize。
- Reference lifetime-elision 15–29 親讀：object default bound 從 containing type／trait 推導，Box dyn 無其他線索時為 static，借用 outer 與 inner bound 分開；`+ '_` 回一般省略。
- std Error 1–15（supertraits/default source/Display 與 source 避免重複）、82–85（From<E> for Box<dyn Error+'a> where E:Error+'a）親讀；RBE boxing errors EN/ZH 各1–6親讀。沒有引入 deprecated/nightly methods。
- Book15.4 Rc EN1–33/TW1–36、15.6 cycles EN1–60/TW1–68已全文親讀；std Weak 1–12 親讀，分清不保活T但配置仍可留存的契約。
- 對應舊8.5、8.6、8.7全文、全部 examples/helpers/fixtures 再核實保全：dyn六階段、Error五階段、Rc四階段的編譯閉包各自只載當下完整source。沒有引用舊umbrella library。

std Cell／RefCell在11.9前另親讀：Cell new/set 11–17、replace/into_inner/get 23–32；RefCell 1–51（new/Ref/RefMut/borrow/borrow_mut/try variants與guard結束）。Cell 66/101、RefCell75/112核對 !Sync 及 T:Send 才Send，避免誤說永遠不能移往別執行緒；這些跨執行緒trait只供技術核對，不在11.9提前教13章機制。cached原文及hash沿preparation fetch manifests。

## 11.9–11.11 來源與取捨追加

| 對應責任 | 實際親讀來源 | 採用／未採用 |
| --- | --- | --- |
| Cell與guard、shared mock介面 | Book15.5 EN單元1–69／TW1–74全文；舊8.8正文243行、cells.rs111行、7個caller與全部負例重新親讀。std Cell 11–17、23–32，RefCell1–51另核。 | 保全Counter／String replace／多guard／try variants／Recorder／RcRefCell／Weak parent；Guard回原資料，不宣稱clone。Send/Sync精確contract只作技術核對，留後章。 |
| Add與newtype動機 | Book advanced traits EN18–36、67–88／ZH20–41、83–109；advanced types EN1–28、46–61／ZH1–30、49–64已親讀。舊8.9全文與newtypes.rs158行、六caller／七fixture重新親讀。 | 保全alias、不同單位、owned Add、混合方向、有限介面、orphan、uncovered、fundamental、coherence；不把newtype說成自動驗證、透明繼承或heap配置。 |
| Add精確介面／orphan例外 | std Add新取原文單元1–17；Reference implementations27–38親讀。 | 分清Rhs預設argument與method body、associated Output、self型別先於trait arguments、Box不cover T但可傳遞本地性；沒有只靠Book15概括。 |
| Any確切型別／static／借用與owned | std Any EN1–20、65–68、103；中文Any1–13、37–46；std any module1–11親讀。舊8.10全文、payloads.rs94行、五caller／三fixture重新親讀。 | 較舊中文頁的safe方法與新版英文交叉核對；allocator／unsafe unchecked／SendSync額外impl不放教材source，不靠它們當先備。 |
| owned失敗回原owner | std boxed/convert.rs原始碼333–335實讀，確認不符合時Err(self)。 | 只採safe downcast公開契約與ownership；不將實作內部unsafe帶進11正文。 |
| TypeId邊界 | std TypeId新取原文1–8、57、71、82親讀；std any module的Box/內容對照7–8。 | 採opaque identity、static bound、PartialEq/Debug、layout/hash/ordering跨版不保證；不提前教variance或用TypeId設計不安全抽象。 |

新增原文cache與HTML SHA：std-en-add-fetch.json、std-en-typeid-fetch.json、std-boxed-convert-source-fetch.json；其餘原文hash在既存fetch-manifest.json。實讀範圍與原文檔可獨立回核，不以bookmark或摘要代替原文閱讀。英文Book與繁中Book均實際讀過，不只記URL。

本次Cow沒有新增：root已撤回誤列要求，11.10照原核准newtype/Add/orphan/coherence撰寫。所有新章仍在intro no-lib package分階段，不提前繼承12章root工程設定。完成不等於通過獨立審閱。
