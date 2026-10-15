# 第12.4節獨立技術初判

封存：2026-10-15T09:27:05.582Z。必要問題0；可選問題1。

TC12T-O1（可選）：正文:52的「完整內容保存在上面的程式中」方向不符現在排列。TempDir材料建立實際在下方答案／測試，建議改成「下方的答案與測試程式中」。完整內容都在同頁，不影響source閉包或執行。

全文、34snippets、獨立workspace全部18owned檔、5runs與1個最後all-targets test、scan正常library依賴皆靜態核對；35 repo＋2cached blake3 hashes及完整閉包見JSON。18項Unix測試（其他平台17）、2SVG source均已讀。未讀作者驗證或其他角色結論，沒有本波Cargo/Rust執行。

按root提示後首次實讀的新稿7c6341...查核：完整scan／固定材料及streams全檔都在各自首次命令前；不把先前搬移算自己獨立發現。四階段只載各自source與已教path library，沒有上層ch09-content舊lib或未教後章能力。

短讀取先填同容量／EOF再比較，Interrupted重試與左右失敗保留正確。檔案前後path／handle觀察是盡力檢查，沒有承諾原子快照或抗替換。大小與digest只是候選；A/B/B依各組代表比較、same path去重、singleton完整讀取與按原生路徑排序均對source。

metrics使用TryFromIntError及checked算術，零情況與溢位驗收正確；hard link只作按路徑的重複bytes，不等於實際可回收量。答案受控同key不是找到真碰撞，1／2兩組算3 bytes。SVG僅讀XML/source，未做visual驗證。
