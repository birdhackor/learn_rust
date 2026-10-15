# 第16.3–16.5節獨立技術初判

必要問題：0。可選修正：0。

完整順讀三頁及所有selected source／normal library／path dependency／fixtures閉包，未看作者驗證、peer或歷史問題；另已審17／18沒有拿來補16先備。

- Poll／Output、Context與owned Waker clone、Wake receiver與From bounds先於用法。ManualAmount每次Pending更新Waker，release只通知；已通知但未poll不算完成，先到事件可首次Ready；完成後panic是本型別政策。四測試與完整runtime-fail吻合。
- 事件Future同guard「取答案或登記」，事件端同guard存值取通知，解鎖後wake；舊Waker Drop與新clone也在鎖外。先事件／先poll、最新通知、同State鎖回呼讀0與main再取得0皆吻合；按值send一次性與E0382正確。未誇稱Mutex完全nonblocking，也未替供值端drop捏造結果。
- executor把recv留在driver，unit通知buffer可保留先到wake；Box::pin不要求F有Unpin，來源也沒有F的Send／static bound。StartAmount先登記才送控制值；Recheck不遺失poll內通知；PinnedAnswer直接Ready。Ready停止、worker另join、供值端消失仍可能等、沒有一般取消／timer／公平排程的界線都正確。
- 完整normal library到五module全教才使用，18tests與所有helpers完整；16個保留入口全讀。兩個獨立失敗workspaces的manifest／lock與真正path library closure已核；E0277／E0382及--no-run後故意integration panic原因吻合。

73owned檔、83snippets、21Cargo commands、16copies／prefix checks、85hash。18normal＋8stage正常test functions與1故意失敗integration、7sidecars、4SVG XML原文均已核。完整清單在JSON。

未執行Cargo、rustc、Clippy、tests、examples或thread/executor；未做browser／圖像視覺／讀者實驗。輸出與錯誤原因為source推導。
