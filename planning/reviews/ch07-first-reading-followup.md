# 第7章局部修後核對

這是root提示後的followup，不是新blind首讀，不回填或更改prefix O1的optional原分級。原prefix／tail／entry及sidecar保持封存；本人原讀prefix v9全34與tail v10全28，沒有讀完整v11。

本輪只開frozen/ch07-output-fixed-v11的005-006 BODY，核Stdout完成位置與main成功後flush責任。使用ch07fix可見正文gate，closed答案先遮蔽；不讀其他第7章、peer／author／完整source或技術probe。四題仍分Q2a機制／Q2b用途，保存當下引文與hash後局部封存；未實跑／看整頁，不能將root提示或新技術測試當自己的原始證據。

## 指定單位

### 005-006 — 有提示核對O1：標準出口完成責任

本人原v9單位SHA-256：58bc881f617a32dc23d2aec5e49b283f55d9bb7fd33ef4ccfca44623d24bf9d6；舊O1保留optional，不改原紀錄。
Q1：Stdout可能使用行程共用buffer，Stderr預設沒有此緩衝；Write接受資料與完成推送是兩步，ExitCode仍正常回傳。
Q2a：新增「正常結束時的自動清理不能替 main 回報這一步的錯誤」，write_all後flush檢查推送，flush成功不保永久儲存。Q2b：原需保原筆記且最後可能無LF→不借加換行來觸發推送，改明確flush→完成兩步才選成功，診斷則直接核Stderr寫入。
Q3：write_all Err早返FAILURE；Ok unit後才flush，Ok SUCCESS／Err報「無法完成筆記輸出」FAILURE；舊parser／Read／report_error沿本人已讀。
Q4：完成位置說明先於code，新main確實在write成功後判flush；「兩步都成功才回傳成功狀態」與分支一致，不靠root技術probe補證據。
局部結論：O1在此指定正文單位已說清，沒有新必要未知；這是提示後核對，不是原判重現。未實跑、看整頁或核其他第7章改稿／完整source，因此只判本單位。

原始SHA-256：1adb24ee3bce71dc04cd63318a4871a88eba74a963fc15495b4e34fa548dcb77；visible SHA-256：1adb24ee3bce71dc04cd63318a4871a88eba74a963fc15495b4e34fa548dcb77。

## 局部封存

只核005-006可見BODY，O1完成位置與flush責任在本單位已解；不判其他修稿／技術測試。原初判、prefix／tail／entry與sidecar不變。先封存本record，再進入指定第6章兩段補讀。
