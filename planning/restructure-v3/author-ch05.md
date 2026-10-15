# 第5章作者交接：泛型與常用資料型別

狀態：5.1～5.5作者稿、兩個member的完整source與限定檢查已完成，供root核對；沒有聲稱獨立首讀或實際網站版面通過。第6、7章是否開寫由root依實稿交接。

## 親讀的實際前綴

完整重讀現行1.1～3.8全部24頁與本章必要source，證據在`ch05-preparation/read-evidence.json`。root正式交接第4章後，另親讀4.1～4.7全文與16份必要完整source，涵蓋2024 borrowed binding、if let、while let、let else；指紋在`ch05-authoring/actual-ch04-read.json`。沒有把outline、作者ledger或先前來源研究當成已教實稿。

依AGENTS與clear-tutorial撰寫，先前親讀Book英文／繁中原文的範圍、取捨見`sources-ch05.md`；本輪的原則是必要契約在首次圖／snippet之前，結果觀察可以在後。所有答案、helper與fixtures均用真實snippets列於當頁。

| 本節必要能力 | 真實先備與當頁首次解釋 | 當頁要完成的判斷 |
| --- | --- | --- |
| 5.1型別參數／泛型函式 | 1.6參數／推導、3.1欄位、3.2 move；先說型別角色，再Holder與hold | 相同形狀可供不同具體型別；每份欄位型別固定 |
| 5.1泛型impl | 3.6 impl／Self／self／欄位簡寫；先說impl<T>宣告、Holder<T>使用及owned receiver，再完整impl | generic new保存T，消耗self交回T，不假定Copy或trait能力 |
| 5.2 Option／Result／prelude | 4.1 enum、4.2 match、4.5 binding、4.6 if let及5.1泛型；先說一個／兩個型別參數及直接名稱來源，再簽名／程式 | 有值與缺值、成功與失敗各存哪些資訊；零仍是有效值 |
| 5.3 char／UTF-8／get | 1.4 char、1.7 bytes、3.8文字、3.5借用及5.2 Option；先單位與分界，再圖，再get具體回傳契約 | 有效空段與無效空段不同；byte界內不代表切法有效 |
| 5.3 manual_map／測試 | 2.7 cfg／test／局部allow、4.2 match；理由在完整text source前 | 保留Some／None兩路，無須closure；測試不因意外None略過檢查 |
| 5.4 Vec操作 | 3.7 slice、5.1泛型、5.2 Option；new/push/len/as_slice/get/remove/pop/as_mut_slice/vec!各在首次snippet前教 | 每項API的借用權限、結果ownership與失敗契約 |
| 5.5 ownership與完整整理 | 3.2 move、3.5區間、4.7 let else、2.4 while及5.4 APIs；先String進出與借用失敗，再整理規則／迴圈 | 借讀輸入、建立新Vec，保順序／重複／零與原資料；最後才接完整library |

## Source與manifest隔離

- `intro-data-steps`位於`examples/intro-workspace/data-steps`，無lib／dependencies；正常examples自足，只有utf8_answer載入自己的text/mod.rs。default main只println。5.1～5.4的執行不編入後文library。
- 5.5開頭的vector_ownership及vector_borrow_overlap fixture也放data-steps，先獨立觀察元素ownership。這是相對寫作前snapshot-plan的小調整：若先選data中的example，會連編尚未在該頁教完的list，所以把此入口留在no-lib成員。
- `intro-data`位於`examples/intro-workspace/data`，只在5.5完整版本使用。lib公開text與list；全部helper、六項tests、default main、list_demo、完整list_answer均在同頁。text與5.3快照相同，不能經此lib倒灌早期章節。
- 兩個member均先以明寫name/version/edition與println-only main建立；root加入intro members／更新lock並通知後，作者才執行Cargo。沒有改root workspace／lock／nav／shared checks或其他作者檔案。
- 沒有workspace繼承／全域lint設定。只有兩份text快照的selected_byte_len帶精確manual_map allow，理由與scope在5.3先教；不以map／closure／Iterator API解文字。
- assertion只用已教i32／bool；usize長度用assert!(相等條件)，不直接比較Option、Result、Vec或&str。沒有derive Debug／clone／unwrap／expect／?／parse／ToOwned等後文API。
- 每份fixtures/compile-fail、runtime-fail的.rs都附同名非空.expected，已以實際穩定診斷片段驗證。

## 舊內容保全

Holder的i32／bool、turbofish、同T兩欄失敗與T/U答案保留；泛型impl新增純owned保存／交回版本，不搬Extract。Some/None、Some(bool)、由bool選零元報價或缺值、Result完整函式與零除數保留；練習直接回傳Result。

文字保留A中🦀分界0/1/4/8、有效空段8..8、無效空段2..2、切斷1..2、越界4..9及四項測試。Vec完整保留as_mut_slice、get借元素、remove移動後續索引、pop交出末尾及空結果。清單保留順序、重複、零、空輸入、全負數、原輸入未變與非負上限整理；完整library仍有六項測試。

## 實際檢查與限制

先source scripts/env.sh，使用固定toolchain。早期14項selected checks與後續27項均通過，共41項分階段記錄；沒有為後來的純Markdown修正重跑Rust。

- 12個examples、2個default binary，各自run／Clippy通過。只選own packages／targets，沒有全intro workspace all-targets。
- utf8_answer target的4項測試、intro-data library的6項測試通過，相關cfg(test)來源另以Clippy檢查。
- generic_same_type得到E0308，vector_borrow_overlap得到E0502；兩份runtime-fail可編譯且分別產生not a char boundary、removal index。四份.expected齊全。
- 兩個package的fmt檢查通過。五頁真正pymdownx.snippets／details展開成功，11組text輸出逐行匹配run logs；折疊內完整text/list與測試確實展開。UTF-8 SVG以XML核對，圖文材料／byte位置一致。
- **未驗證：**全站建置、實際網站桌面／手機版面、獨立首次閱讀。這是作者自查，不是讀者驗收。

新增vector_answer初版用get(0)觸發get_first；改成對已知仍有兩項的slice直接索引，保留本章既有接口，沒有新增first方法或allow。修後該target Clippy通過。

## audit新位置與實際引文

- A0105-007：`docs/chapters/05-01-lesson.md:112`：「下面的 Pair 宣告一個型別參數 T，並讓兩個欄位都使用這個 T。」接著再以T/U表示兩個不同角色。
- A0105-006：`docs/chapters/05-02-lesson.md:5`標題「Option：有值與缺值」；第9行：「Some(120) 是有報價 120，Some(0) 是有報價零，None 才是沒有報價。」實際Markdown以code span標記variant。

作者未改shared issue-dispositions，供root核實處置。

## 版本與證據

5.1～5.2先有`early-freeze.json`供root讀，原文／原判保留。作者在真實Markdown展開後發現普通段落的泛型型別可能被當HTML tag，只補code span；修前各頁、原early freeze與前後hash在`revisions/round01-generic-code-spans/`。語意、Rust與上述audit引文未變，修後HTML已確認沒有T/i32/bool等誤作tag。

工作證據根目錄：`/workspace/work/tutorial-implementation-20261015/ch05-authoring/`，包含early-validation、remaining-validation、每target logs、兩階段render、修訂前ledger與最終freeze。全章固定版本以`freeze.json`及`validation.json`為準；原early證據不覆寫成事後通過。
