# 第9章發布候選檢查

2026-10-14。原稿ch09-complete-prerequisite-v5為4頁27BODY與39來源。獨立FIRST、非作者TECH、非作者CONT各先封存初判，ROOT完整讀三份，再开放互看，原SHA／分級／實際背景不改。原必要項皆0，兩專家獨立發現相同兩處粗體delimiter雜訊；CONT另外發現答案字級較小，保留可選呈現改善的性質，不事後升級或回填FIRST未看實頁的範圍。

採用最小修改：9.2與9.4導入只加空白讓原strong正常呈現，另以共享details font-size:inherit使展開答案沿正文大小。固定ch09-review-fixed-v6的原39source皆同byte，僅多1CSS；所有Rust/helper/tests/negative/答案/SVG保持，故不為文句／CSS盲目重跑Rust。Root自建差異proof；處置見ch09-root-disposition.md。

TECHcross909945a0…及CONTcross7f0e9627…各在封存後FULL讀同行，再自己diff／hash與建立同theme固定preview。ROOT FULL讀兩份followup，判同意理由各回原需、重要機制／用途／資料身分及證據界線，不以必要項皆0代答。CONT reviewer更正FIRST把integration-test位置簡稱1.10：actual7.2才明教獨立test crate，但已可用的7.2正文足夠，沒有學生成本新增，也不改FIRST original。

FIRST局部1566df08…只在兩個fixedunits依原closed filter先保存四問再開下一段，理解無阻塞；沒有展開原答案或補成原獨立發現。ROOT FULL讀local。兩專家各自實頁核fixedentry與三必要SVG，CONT另核三fold同12.8px、mobile code可橫捲與12次真copy取得完整內容。ROOT親看10張fixedentry／三fold兩尺寸，另4張7.8／7.9旧fold樣式：code與正文同12.8px，字可讀、長行仍在code內捲動，整頁沒有水平溢出。ROOT的pre量測誤取外框寬未當作內容不長的證明；CONT正確code scroll量補完整行互動證據。不同本人／同行／未查範圍保留，未宣稱全theme/nested fold真手機全站驗證。

三類原判、cross及受影響local均關閉。獨立Git index候選只開1–9，4pages＋40sources共44studentpaths精確等於v6，未帶入10–15活躍作者Cargo依賴；lock僅解析9新增三package，另原engineering既有member沿用。ROOT預覽導覽完整恢復；正执行candidate完整scripts/check.sh，未先宣稱PASS或公開發布。初次候選shell漏載ABS ROOT env，拿到系統舊uv被required-version拒絕；保留attempt-missing-env log，修正shell入口後重跑，這不是教材建置錯誤。


獨立候選完整scripts/check.sh退出0：strict文件建置、fmt、全target Clippy、workspace全targets tests、doctests与90指定故意失敗fixture皆通過。共有256 test-target結果、151個個別tests／doctests通過、0失敗；合計包括前章，沒有當成第9章獨立數量。證據release9-check.log／release9-check-counts.json。ROOT再核44student候選與v6完全同byte及三份originalSHA保持後，可發布；遠端Actions／HTTP結果另追加。
