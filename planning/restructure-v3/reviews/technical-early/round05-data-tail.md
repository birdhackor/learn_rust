# 第5章尾段獨立技術初判：5.3–5.5

依基本Python及實際已讀第1–4章、5.1–5.2正文，按5.3→5.5核對全部實際source、helper、tests、fixtures、manifests及UTF-8 SVG原文。先獨立保存作者發現的前綴code span窄複核，該項不算本角色發現；本尾段初判封存前未讀第5章作者ledger／validation、其他角色或首讀結果，也未用第6、7章補先備。

**必要問題0項；選讀建議1項。** 沒有執行Cargo、rustc、Clippy、Rust測試或瀏覽器。本輪以source靜態推理、hash／snippet檢查、Python文字編碼驗算與SVG XML解析為證據；未把作者已跑的結果當成自己的實測。

## TC05T-O1：兩處「上一節」指向不精確（選讀改善）

5.5:69說text保留「上一節已教的文字取得函式」，109又說與「上一節text-only版本相同」。實際上一節5.4教Vec操作；selected_byte_len及target-local text module在5.3:84–112。兩份text source確實逐byte相同，技術內容沒有錯。

讀者若按「上一節」尋找，可能多翻一次5.4。現有函式身分、5.3前文與5.5完整source已足以理解，不屬必要概念缺口。建議兩處只改成「第5.3節」，不新增API、段落或程式。這是本角色本輪獨立查到的局部交叉指涉精準度建議，並非Rust語義錯誤。

## 實際閉包與首次使用

- 5.3 utf8是單檔；utf8_answer的run與test只載入自己的main.rs及text/mod.rs，沒有Vec或intro-data library。5.4兩份清單example及5.5開頭vector_ownership仍是no-lib data-steps單檔。正文各自完整展示所需source／manifest。
- 5.5才轉到intro-data。list_demo與list_answer均會建置正常library，即使list_answer自己未use它；已逐一核對lib.rs、text.rs、list.rs。全部library source在124行的首個run之前已列出，text／list完整版本放在明確標為完整source的折疊區。兩份package manifest皆無dependencies／build script。沒有藉未發布source補先備。
- 5.3在get前已說明char／scalar、UTF-8 scalar的byte編碼與字形差別；get前交代Option<&str>、byte range合法條件與借用。合法空段Some空字串與None分開，直接range索引panic及不支援單整數文字索引的契約正確。
- manual_map的用途、刻意保留match的理由及精確局部allow在首份helper source之前已說明；沒有要求未教的較短API。test／cfg／module／pub／crate路徑沿用第2章，測試presence check不會在意外None時誤過。
- 5.4各方法均先說角色再顯示code：new／push／len、as_slice／slice.get、remove、pop、as_mut_slice、list形式vec!。get的borrowed reference與pop／remove的owned值、None與panic、元素數與文字bytes均分清。i32存放區在heap且連續的限定正確，沒有宣稱空Vec預配置元素空間。
- 5.5以String驗證push交入、pop交回，whole Vec即使裝i32仍move。借用失敗例確實在push後使用Option<&i32>內的reference；E0502及不可依賴某次未換存放區的解釋相符。最後使用結束後再修改的正例也符合既有規則。
- 整理函式先界定非負、零、順序、重複與保留輸入，再用表格／while推演。每次索引前有bounds條件；copy的是i32，push的是另份結果。空array語法在測試前已說明。cap答案先排除負數，再套用明定非負的ceiling，輸入沒有被改動；沒有偷偷引入trait、derive、closure、iterator或整份Vec比較。

## 結果與圖的核對

本範圍共7個成功執行目標、2個精確test命令。靜態推演的輸出與正文逐行一致；utf8_answer有4項test，intro-data library有4項文字加2項清單test，完整載入與數量相符。兩份runtime fixture的expected分別為`not a char boundary`及`removal index`；borrow fixture為E0502，均與實際source失敗契約一致，未宣稱本輪重新執行。

Python實際UTF-8編碼核對A=[65]、中=[228,184,173]、🦀=[240,159,166,128]。表格、SVG原文、alt與前置crab對應一致；合法分界0、1、4、8，1..2切斷中，8..8合法空段。SVG XML可解析。本角色沒有看瀏覽器或圖像畫面，這些source查核不能代替視覺驗證。

同名JSON保存36份SHA-256、28個snippet引用、完整selected closure、9條命令、四題概念依據與逐目標靜態輸出。本結論限於5.3–5.5技術及首次必要依賴。

封存檢查時另偵測到intro workspace新增trait-stages、lifetimes成員及對應lock entries；已實讀其metadata變更，resolver與本章package/source未變。原snapshot hash保留，另記封存時兩份metadata新hash；未讀新增member的source或當作先備。
