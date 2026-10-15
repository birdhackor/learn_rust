# 18B 最後兩組修改：root提示後窄技術複核

兩組均已解決，三頁沒有新增問題。此為root在B獨立初判封存後指定的修改複核，不算本人獨立發現；原initial與hash保留。只讀root指定差異定位紀錄，未開啟其指向的peer報告，也不以其中分類作API證據。

**Id／result交接**：Tokio 1.53.3 `runtime/task/id.rs:31`明確derive Copy。B4完整record先match &result，依4.5已教的match ergonomics，成功分支id為&Id；*id只複製Id，不移出Check。借讀結束後match result接手Check／JoinError，符合3.5最後使用後結束借用的規則。18.10新增段落位於record導讀與完整答案前，說明與真實source一致。

**精確fixture重建**：18.11及18.12新增JSON字串均實際解碼成42 bytes，與各自原始invalid-urls.txt逐byte完全相同；另外用五行內容＋LF在記憶體獨立重建，也完全相同。檔首LF、第2行一個ASCII空白、最後LF皆保留，非空白原索引仍為1／3／4。10.7:27已教JSON換行跳脫，18.8:31已教lines與LF；新段落本身又明說\n、單空白及外層引號不屬TXT，原snippet明標逐行預覽，不需新API或未提供helper。

三份root before均與本人原B初判頁hash精確相同。47份相關owned Rust／manifest／lock／真fixture與原初判一致，三頁Cargo命令及snippet引用沒有改。current hashes：

- 18.10：`80c9c6fc9cb984b8964b1cd9efe9042d1be109a126224ff51bc8a207f686757b`
- 18.11：`2c5ddcff7574478104eeed31ec1e79998b2f5696d9168c29c4611eb00841bc00`
- 18.12：`0dd7448bc31cf09d1d64409c31a4b102fb6ed50a6fc041118c0fab04132b0ba1`

14份證據hash、逐byte十六進位與差異在JSON。只做source與記憶體中的JSON解碼／byte核對，沒有跑Rust／Cargo／HTTP或瀏覽器，也沒有在repo重建或修改檔案。完成STOP。
