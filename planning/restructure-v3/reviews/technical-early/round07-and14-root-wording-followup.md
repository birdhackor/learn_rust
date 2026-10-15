# 第7.4／14.7／14.8受提示後窄複查

三項文字修訂均與實際source吻合，窄範圍可關閉。

- 7.4的complete/src/source.rs確實收7.1與7.2各函式，明列第7.1–7.2節正確。這是後續crossref提示；我原7章初判未發現，原檔不改。
- 14.7四unit tests只解析／人工報告／writer，沒有發HTTP；精簡後的正面敘述準確。
- 14.8三core tests沒有驗真實網路；刪去隱藏server旁述不影響normal library／selected targets closure。

三句反向還原均匹配各自原始SHA。原範圍Rust／fixture無改動；intro workspace manifest／lock另有後續metadata變化，分開記錄。只讀指定root diffs及必要source，未跑Rust／HTTP／browser。完整before／after／hash與歸屬在同名JSON。
