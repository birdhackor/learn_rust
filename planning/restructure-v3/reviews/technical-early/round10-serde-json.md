# 第 10.7 節獨立技術初判

封存：2026-10-15T08:46:23.444Z。必要問題 0；可選問題 0。

按實讀1–9、10.1–10.6核對全文、20個snippets、三個run example、一個selected test（四項tests）、完整helper／manifest／答案，以及七份JSON和單一FF binary材料。28檔repo hashes與2個cached serde_json API source hashes見JSON。未先讀作者驗證或其他角色結論，不用11章補先備。

JSON欄位／escape／拒絕政策、Serde能力與格式實作、derive feature和deny_unknown_fields在使用前完整。Deserialize的輸入lifetime不強制結果借用；Notebook全owned，來源String結束後仍有效。from_str／to_string_pretty的精確借用契約與鎖定API一致。

完整no-lib closure準確：basic_json只載json_core的model／codec；file_errors和答案才加json_files。正常run不載cfg(test) body；file_errors test才含三個codec與一個json_files test，全部完整inline。default main與helper資料夾沒有被誤當selectedtarget載入。

具體NotebookError保留Io／Json原因，From與?順序正確，不要求未教Error trait。UTF8失敗先是Io/InvalidData，語法／欄位／範圍問題才是Json。答案只改archived並重新decode檢查全部材料。cwd和所有fixtures相對路徑一致，missing實際不存在，FF為原始255一byte。

沒有新增Cargo/Rust執行；輸出與tests為靜態核對。json-model.svg僅讀XML/source，不稱browser或讀者驗證。
