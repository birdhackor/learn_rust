# 第18.1–18.2節獨立技術初判

必要問題：0。可選修正：0。

完整實讀兩頁、A1 starter／complete全部source與六份實體材料；未看作者驗證、peer或歷史問題，未用後續A2–A4答案補A1。

- 六份材料的bytes、隱藏目錄、空檔、大小相同內容不同與10 bytes的按路徑指標均吻合。
- repo外cargo +1.99.0的選擇、check解析lock後test、零測試不代表完成的說明正確。
- 三個stub確實未實作；checks不是自動test target，完整cfg(test) module須加入collection.rs後才有行為失敗。起始版拒絕有效root及observe目錄可能偶然通過的預測均準確。
- 完成版的八測試與starter驗收逐字相同；五通用、三Unix。root link拒絕、樹內link略過、非Unicode保留原生位置、Ok/Err/Ok繼續、觀察len／modified及原生排序都與source吻合，並未聲稱抵抗同時替換。
- 四個完整答案檔案先於命令；inventory明訂package cwd，相對root可解析到同一份六檔材料。A1只列收集資料，未冒稱完成最終CLI或重複判定。

10個package-owned檔案（8個source/config與2個README）、6個材料檔、15個snippets、9條literal commands、23個hash。清單與閉包證據在同名JSON。

版本：首讀18.1舊SHA a22da7d2…d982後，封存hash保護偵測root同時修訂，第一次寫檔因此中止；當時沒有initial檔。收到純版本通知後已全文重讀current7c5e1ff0…98d0，才作本次首封存。此為root更正B答案位置措辭，不算自己的獨立發現；未用後續B稿補先備。其他本波source未變。

未執行Cargo／Clippy／tests／inventory，未建立新專案或做檔案走訪／symlink實驗；以上為source查核。
