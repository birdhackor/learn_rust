# 第5章 5.1–5.5：獨立銜接與實頁初判

封存時間：2026-10-15T07:42:23.228646+00:00。接續實際已讀1–4，依5.1→5.5順序讀actual body／snippets／全部sources／答案／tests／fixtures／最小設定。本角色是continuity與page reviewer，不是作者、technical、盲首讀或全書通過聲明。未讀作者ledger、technical/firstreader結果、歷史issues/audits、root dispositions、6/7稿或Rustsource作提示。

**結論：沒有新必要缺口；兩項可選窄修。** 保存37份初始來源與hash；本輪所有第5章文字、Rust、fixture、SVG、兩package設定均穩定。審查中workspace/lock新增trait-stages、lifetimes，另記end-config；只讀這兩個member四欄manifest作設定閉包，未讀後章source，也未當先備。原始frozen37份逐一驗hash吻合。

## C5-01（可選／低嚴重度／高確定性）：5.5的「上一節」應明指5.3

5.5:69「text 保留上一節已教的文字取得函式」、:109「`data/src/text.rs`與上一節text-only版本相同」。實際上一節5.4教Vec；文字函式selected_byte_len、text-only module、manual_map與四項測試在5.3:84–107。5.5已保留完整同頁source，實際前面也真的教過，所以不構成必要先備缺口；但回找說明的指引落錯節。

建議兩處改「第5.3節」或明確連結，讓讀者可直接找到原文字契約與測試，也理解finished package如何重用已學程式。來源與展開後實頁都重現該字樣，兩份text檔案逐byte相同。證據為desktop/mobile-data-complete-sources-open截圖及初始5.3/5.4source。

## C5-02（可選／低嚴重度／高確定性）：prelude首次定義露出Markdown星號

5.2:11「Rust 的 `**prelude（預先匯入的常用名稱）**會`自動把一組名稱帶進一般 module」。1280×800與390×844actual頁都顯示原樣**，JSON的rawAsteriskParagraphs也有此段。此段完整說明仍先於第一個Some/None使用，讀者能讀懂，故是呈現與新詞辨認問題，不是補教需求。

可改成 `**prelude**（預先匯入的常用名稱）會…`，只調strong邊界，複查actualrenderer。收益是移除新名詞周圍可見markup，恢復清楚的重點。實看desktop/mobile-option-prelude截圖有證。

## 正向證據：新觀念在當前需要前已交代

### 5.1

位置：3/7/9;26/28/34;44;54;79/81/87/89/95;112/126。

Starts from saving either i32 or bool with same one-field rule. Type parameter is explicitly a type input, unlike120/true value input; Holder<T> declares T, table gives two concrete distinct types. hold<T> preserves input/output type, uses known field shorthand, assumes no arithmetic/display capability. Inference and explicit turbofish are explained before calls. Generic impl declares T separately from Holder<T>, Self means this instantiation; owned into_value may move any T out, with String example already taught3.8. Same-T Pair restriction is stated before E0308, two-parameter answer explains comma and each field mapping.

generic_holder prints/asserts only concrete i32/bool; generic_impl number is consumed and not reused, String moves label→Holder→saved_label; generic_answer uses only numeric/bool assertions. No derive/trait/generic equality hidden.

### 5.2

位置：7/9/11;19/23/29/31;41–50/56/62;94–105。

Option shape and bare prelude names precede construction; Some0 is present, None absent. Inference has explicit context. Pattern extraction uses prior match/if let, and if arms yield same Option<i32>. Result T/E roles, nested DivideError and zero-denominator branch are defined before divide; Err is data and does not itself panic/terminate. Nested Err pattern explained before handle. Caller decides response, small-integer contract avoids claiming all i32 overflow cases.

Whole source contains DivideError, all helpers/main; division occurs only nonzero branch; zero error still continues. Answer returns Result<i32,AmountError>, concrete i32/bool assertions avoid requiring generic PartialEq/Debug. Choice table visible in both viewports. prelude emphasis issue is C5-02, not missing explanation.

### 5.3

位置：3/7/9/11/13–25;29/31/33/39;72/74/80;84/86/88/92/94/98–112。

Separates byte position, Unicode scalar, glyph and char storage. Per-scalar bytes supplied for this exact material, with end-exclusive ranges and boundaries0/1/4/8 before get. get on&str returns Option<&str>, legal bounds/order/scalar endpoints, Some empty8..8 vs None2..2 explained before code. Shared view validity and as_str chaining use prior borrowing. Direct range indexing has separately stated panic contract, no integer char-index promise. Answer returns owned usize, preventing new returned-reference lifetime requirements. Module layout, named manual_map and deliberate known match form are explained before source. Test branches ensure unexpected None fails assertions.

utf8 source prints char and bytes, whole same-page manifest/helper/main. utf8_answer text module has4tests covering length3, empty0, partial1..2 plus2..2, out-of-bounds4..9. Source uses only known match/bool/usize assertions and module paths. Necessary UTF8 SVG actual400×680 desktop,342.984×583.094 mobile; served SHA equals frozen SVG. Labels/positions/ranges and success/failure blocks remain readable, 24px text scales20.58px. Table row data remains accessible, image immediately below repeats full mapping.

### 5.4

位置：3/7/9/11;21/23/25/31/33;37/39/45;49/51/57;61/63;73/79;108/114/118/120/139。

Motivates mutable-length list versus fixed array/slice; Vec owns homogeneous elements, len counts elements usize; i32 contiguous heap and empty no allocation are scoped correctly. push and mut prerequisite precede build. as_slice returns shared slice, indexed get returns Option<&i32>, *value Copy and element index vs text byte range explained before source. remove owned return/panic and shifted positions; pop owned Option and emptyNone are separate contracts. as_mut_slice can change existing entries only, last use permits later Vec read. vec! macro/array distinction precedes use.

Trace starts80/120/200, get1→120/get9→None, remove0→80 then index0→120, pop→200, remaining mutable item120→130. Fixture position=len violates remove contract after it is taught. Answer creates10/20,push30,removes10,adds5→25,pops30/25/None,endslen0; for_ is explained before code. No String move or allocator detail needed before5.5.

### 5.5

位置：7/9/30;34/36/42;46/48–59/65;69/71/77/79;85–125;139/141/143。

Owned String push/pop handoff explained before full example, shared as_str display remains known. Vec itself nonCopy and cleanup role stated without requiring future trait lesson. Possible element relocation plus overlapping Option<&i32> use explains E0502 before fixture; rule does not depend on actual capacity. New filter has independent need: preserve nonnegatives, zero, duplicates, order and original; trace and while-boundary algorithm precede function. Caller/test roles and emptyarray[i32;0] explained before tests. Finished intro-data exposes list/text roots using chapter2 module rules; all text was actually taught5.3 and repeated unchanged (locator wording C5-01).

data-steps has no lib/build.rs/dependency; each chosen example fully owns definitions/main. intro-data package-only manifest, pubmodlist/text, simple binarymain, complete list_demo, text/list and six tests all inline. text.rs bytes exactly match5.3 text-only module. list2tests check duplicate80,zero,input-5,empty/allnegative. list_demo uses safe0..len valid indices. Answer ceiling nonnegative contract stated and uses only while/if/Vec methods; original first120 preserved, result100/0/80. No hidden helpers, advanced equality/display/iterator/closure/trait.

## 例子與設定閉包

實際查data-steps目錄：只有package/name/version/edition的manifest、簡單default main、各自完整的example與四個fixtures；沒有lib、build.rs、dependency或未出現helper。5.3 utf8_answer target只載自身text/mod.rs。5.5 intro-data實際有pubmodlist/text的lib.rs、簡單main、list_demo/list_answer及兩module，與頁面說明相符。所選全部來源、兩份完整manifest都在同頁內嵌；每個fullinclude與actualDOM code比對吻合（僅正規化空白與snippet marker註解）。所有named局部段落均同頁另有完整檔案。

四份.expected分別為E0308、E0502、not a char boundary、removal index，與事前文字契約吻合。所有測試/答案只操作已確定的i32/bool/usize、已教pattern/module/testattr；沒有直接比較整個Option/Result/Vec、derive、closure、iterator或後章trait。沒有執行Cargo、build、Rusttests，也沒有改repo文字、程式或共用設定。此處不替代technical執行驗證。

## 實頁與必要圖解

實開5頁，1280×800與390×844；29張焦點截圖全部實際查看，確切檔名列於round05-ch05.json。唯一inlineimage是5.3的`docs/assets/ch03/utf8-boundaries.svg`，已在兩種actualpageviewport完整看過；servedSHA與frozen來源一致。桌面400×680、手機342.984×583.094，24px文字縮為20.58px仍易讀。三色方框以文字列A／中／crab、byte值／位置／下一分界，正文先說crab是第三項🦀；下面1..4完整中、1..2終點切入scalar的綠紅框另附文字，不依賴顏色。此圖沒有未解釋箭頭；必要關係由group位置和標籤明確建立。

5.3四欄table手機434pxwrapper超375px，初始可見所有rowdata與range但末欄表頭尾字截去，右捲會失去firstcolumn。這裡同一完整對應立即在343px清楚figure重列，前後prose也足夠，未另立finding。其餘主要比較table可完整讀；generic型別inlinecode偶爾換行但所有符號仍在。5.5完整text/list預設收合／展開都實看：新list規則與tests早已在主線完整展示，text整份5.3已教，panel是重複來源而非藏必要新知。

實頁URL、時點、HTML hash、所有fullinclude比對、rawemphasis、唯一SVG來源hash與尺寸在`round05-ch05-visual/metadata.json`；附表格尺寸與左右狀態在`tables-metadata.json`。本次是actual必要關係觀看與所有source／DOM完整核對，未聲稱每張長頁每一pixel均看過，partialnavpreview也不是最終strictsite驗收。
