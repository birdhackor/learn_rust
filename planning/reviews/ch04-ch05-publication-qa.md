# 第 4–5 章發布檢查

2026-10-14，root 協調者核對。發布正文為第4章v7局部修後、再調整target名稱的v8，以及第5章修後v7；各章原始首次順序閱讀、非作者技術與非作者銜接初判皆先封存，互看及局部修後覆核另外保存。所有已報必要與可選項都由disposition裁定，原角色分級與漏查保留，不以修後同意回填獨立重現。

第4章初始化前置、map查找線、複合鍵資料對照、4.2比較證據歸因與4.4練習範圍都有相應修正及三角色局部覆核。counts的Rust操作不改；新增different_region只使用已教型別與方法。Root實跑三組custom_keys結果吻合。第5章reference顯示、prelude、same-E From、全值binding及可選定位／主詞皆在首次需要前補最低契約。Root對改動的unwrap_error fixture實際編譯並觀察指定panic；其他未變來源由各角色核對，不偽稱全數重新執行。

Root親看1280×800桌面／390×844手機受查原位：4.1不同資料各次呼叫共用契約、4.3query與map的兩個借用來源、4.4兩候選輸出與4.5模型來源，以及5.6的Ok繼續／Err整函式回傳。4.3修後查找線避開標題；5.6箭頭拆為各自path後兩路方向可辨。圖文相鄰，手機所需對象／標記不靠放大，未見整頁橫向溢出或浮動TOC遮圖。這是受查畫面證據，不代表每個裝置或捲動位置。

發布候選只開第1–5章，workspace只加入已完成的第4章兩個package與第5章package。從Git index建立獨立候選，候選自己的Cargo.lock解析後，才執行完整scripts/check.sh；後續活躍作者的新增source與依賴不算在本批檢查內。

獨立候選第一輪完整檢查通過，35個故意失敗契約相符。workspace test另實際報第2／4章同名lifetime_solution輸出相撞；root將本批新增第4章答案target改名lifetime_annotation_solution，更新4.4連結與指令。Rust bytes、方法／練習內容不變，原審閱封存不改；新v8記錄這項整合修正。修後兩個target分別實跑及候選完整檢查結果另於完成後追加。

修後第4章答案實跑第一空／第一非空皆Rust，第2章原答案仍輸出3、4、8的借用／修改／owned結果；兩個target各自選定。候選完整check.sh再次通過，35個故意失敗契約相符，先前輸出名稱相撞已消除。此重驗由實际新整合修正觸發，未將第一次通過抹去或擴稱新首讀。遠端部署另於完成後追加。

提交 `93e147b3acbcc96131b66216e356d599a3f1d204` 的 [Actions 38577488663](https://github.com/birdhackor/learn_rust/actions/runs/38577488663) check／deploy均成功。Root實際請求公開首頁與第1–5章33節，全部200且H1相符；未發布6.1仍404。紀錄在work的release45-live-http.json。
