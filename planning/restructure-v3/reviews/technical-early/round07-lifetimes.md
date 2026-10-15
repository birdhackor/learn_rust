# 第7章獨立技術初判：7.1–7.4

依基本Python與實際已讀第1–6章，按7.1→7.4讀正文、全部actual selected source／helper／tests／manifest／fixture及expected，另讀borrowed-field SVG原文。封存前未讀作者驗證、continuity、reader、其他review或歷史問題；第8、9章尚未讀，不作先備。

**必要0項，選讀0項。** 本輪為整頁技術審閱及靜態推理，未執行Cargo、rustc、Clippy、Rust測試或瀏覽器，未把作者執行當作本角色證據。

no-lib intro-lifetimes的11個run目標各只載入自身完整程式／明列module；source_basic沒有後段函式、source_complete沒有struct、view_basic沒有回傳reference方法、receiver_only沒有TextPair。到7.4才另載intro-lifetimes-complete：正常library的lib.rs／source.rs／views.rs、main及兩個examples全已展示。label_answer即使未use library也按正常Cargo閉包計入，沒有漏查。兩份manifest均僅既有三欄，無dependencies／build script；兩個member都在intro workspace中，後續member只有metadata而未讀其source。

7.1先說唯一輸入elision與回傳借用，再定義lifetime parameter／&'a str。共同參數代表使用輸出所需區間可由兩來源支持，不要求兩個owner同時生滅，也不延長資料。byte長短與lifetime長短分開；交換參數走過兩分支，ab／CD等長且內容不同，能識別選第一段。source_too_short依簽名拒絕，沒有用本次實際較長內容放寬契約。

7.2只連first、owned String、placeholder與outlives均在首次相關source前給足角色。copy_text明確建立獨立內容，可在來源結束後使用並與original修改分離；不是複製視圖。'_依elision，不能因兩處同字面就當共同具名參數。'b:'a方向、非嚴格涵蓋與borrow區間／整體scope的差別正確；Some空文字與None保持不同。

7.3的struct參數、欄位reference與impl參數可接上已教泛型；TextView<'_>明指唯一輸入來源。模型持有reference而非原String，byte_len交出owned usize，答案留下4而不把內層模型帶出去。pub struct／欄位pub回指3.1、欄位簡寫回指3.6皆核對到實際前文。

7.4在方法片段前分開receiver借模型與欄位借文字：elided text結果連receiver，original明寫原來源'a；不是把&self固定成'a。同body可交回reference依據3.8已教&str本身Copy。成功例與E0597反例確實分別保留／結束模型，反例被限制的是view而非仍在外層的source。TextPair兩來源獨立命名，使first_source輸出只依賴第一來源；無須'b:'a。13+4=17、note與外層Rust tutorial均相符。

14個run目標的輸出與assertions、source_complete的4項test、complete library的4+2項test，皆由實際source核對。6份非空expected為E0106、E0597、E0515、lifetime may not live long enough、E0597、E0597，與各完整失敗例吻合，未宣稱重新執行。兩個test命令都選精確target，沒有額外早載入來源。

SVG的上方TextView欄位、下方source owner及借用箭頭，和title／desc／alt／正文一致；XML可解析，不等同畫面可讀性驗證。同名JSON保存50份SHA-256、54個snippet、16條命令、逐目標完整閉包與四題證據。本結論限第7章技術／先備／來源。
