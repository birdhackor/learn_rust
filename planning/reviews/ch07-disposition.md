# 第 7 章問題處置

prefix 首讀34單位與tail首讀28單位均已封存，原始紀錄、SHA與分級保留；tail當時只讀original ch07-complete-prerequisite-v10，沒有取得本檔、root probe或live改稿。原始技術初判已封存；銜接角色仍核對original且未取得其他角色提示。以下是root已核實的局部處置，不是整章通過。

| 問題／來源 | 原分級與 root 查核 | 理解收益／最小修改／處置 |
| --- | --- | --- |
| 7.5 Stdout 的緩衝／flush 完成位置；prefix first O1 | 首讀原判可選：既有 7.4 教 flush，但 7.5 未連到標準出口。root 查原 main 只有 write_all；另以沒有換行的 Rust 檔、stdout 導向 Linux /dev/full，原 binary 返回0且無診斷，證實清理時失敗沒有交回 main | 必要採納。root 的必要性判斷是初判封存後的 source／實跑新證據，不能回算成讀者已自行發現技術錯誤。首次 Stdout 使用前教緩衝、明確 flush 與錯誤回報位置；write_all 成功後再 flush，後者 Err 也失敗，不改輸出 bytes、既有參數政策或新增未知 API。 |

probe 限 Linux 受控出口，不宣稱所有平台已跑。完整前文版本與修後實跑、技術／銜接交叉、首讀局部複查另行追加。root 的新發現沒有提供仍在獨立讀 tail 的讀者作提示。

修後 ch07-output-fixed-v11 封存整章62單位。root對ch07-path-cli作fmt／all-target Clippy／build，均通過；同一無換行材料到/dev/full現在回1、stderr有「無法完成筆記輸出」，一般capture回0、stderr空且完整bytes仍為Rust，沒有擅加LF。後續獨立角色仍核original，局部修後證據另存。

## 技術初判後的追加處置

技術角色的original初判SHA為`a55a6e33ffb9ec818ebff33beeb95afe6d6394e11e0ea5f6ca2db7c895373b32`，root已實讀完整報告。下列處置不改寫原首讀的可選分級，也不把後見同意算成獨立發現。

| 問題／來源 | 查核與理解收益 | 最小修改／處置 |
| --- | --- | --- |
| 7.5及7.8 stdout完成位置；技術N1 | 技術角色獨立以建立在child開始前的closed pipe重現original7.5短字串無LF漏報。7.8複用同一main，亦缺flush；這項範圍是技術角色新指出，root先前只修7.5 | 必要採納：7.8沿修後7.5明確flush並處理Err，source前交代同一完成檢查。root另以7.8真正binary、無LF Rust材料導向Linux /dev/full驗得1且診斷，正常capture驗得0、exact Rust bytes、stderr空。 |
| ToOwned的名稱來源；技術I1 | 7.4保留explicit use，後面用標準prelude帶入的同一trait；前面的能力已教，補名稱範圍可減少疑惑 | 可選採納：7.4首次介紹補一短句，說明prelude帶入名稱，explicit use用於展示來源；不重教trait或新增API。 |
| original凍結包缺9份靜態材料；技術E1 | 原bundle缺io空檔／非UTF-8及JSON七份材料，部分測試因此只能報未驗證，不能用live內容回寫original驗證 | 修正新的bundle完整性。ch07-review-fixed-v12保留全部12份靜態材料（原3份與補9份），root逐一比對live／archive bytes與manifest SHA；original v10不改動，初判限制保留。 |

7.8修後fmt／all-target Clippy／六項integration tests通過。root controlled probe在`stdout-flush-probe/fixtures-fixed-result.json`；修後前文與本節封存為ch07-review-fixed-v12，共62單位。首讀7.5局部v11已封存通過，不宣稱整章已驗收。

## 銜接初判後的追加處置

銜接original初判SHA為`65b13249772da61cd8f74cafd46b53952869ed41844293be2238f4dc144a8b06`。root實讀全部62單位的角色報告與source範圍；角色獨立重現7.5／7.8完成界線N1，不與root後見發現或prefix原可選混算。

| 問題／來源 | 查核與理解收益 | 最小修改／處置 |
| --- | --- | --- |
| Rust字串literal雙引號跳脫首用；銜接N2 | root比對actual1–6與7.7，後者介紹JSON檔案文字的escape，尚未建立Rust source中同一形式的用途。7.9 search tests首次使用；基本Python熟悉度不能代本課已教證據 | 必要採納：在第一份search tests前，教Rust literal的 `\"` 產生內容雙引號，`\n`仍是LF，再區分稍後JSON編碼的表示。不擴展整套跳脫語法。 |
| generic writer的完成責任；銜接O1 | write_report只借writer，write_all成功表示接受資料；caller仍持有writer。以已教flush明訂責任，可避免把任意buffered writer都當已確認出口 | 可選採納：library source前交代caller處理flush，7.9 main作為caller在run成功後checked flush，Err包進既有AppError::Output並回1。原LF輸出沒有被回判為已實證的無LF故障。 |
| 缺材料U1／缺root lock U2 | 補9材料在新bundle處理，original證據限制保留；精確registry解依賴與完整checks由release candidate固定root Cargo.lock | 未把封存外的執行當作original已驗證，也不宣稱Windows已實跑。 |

修後ch07-review-fixed-v13整章62單位封存，含更新main與相應SVG。root scoped fmt／all-target Clippy／search的13項tests通過；SVG與正文局部須另實看。銜接cross／局部已開，技術cross與首讀追加局部仍待，不宣稱整章通過。

## 修後交叉覆核進度

Root已完整讀取銜接與技術的交叉／局部報告。兩者均在original初判封存後才互看同行，另核v13的六個受影響單位與新材料；所有已報必要項、可選採納項及材料缺漏在各自受查範圍內已關閉，原始分級與漏查保留。技術角色另以隔離source執行10項tests、一個答案與兩份binary的四個出口對照；銜接角色的執行與畫面範圍依其報告，不合併冒充單一角色全查。

新版7.9 SVG的桌面／手機原位圖文已由root實看，詳見development-visual-checks.md；技術角色沒有做實頁視覺檢查。額外首讀局部複查與完整release candidate的固定lock、全套必要檢查尚待，故此處仍不宣稱整章已完成發布驗收。

後續五單位額外首讀局部已封存，SHA為`2c0183be4b9b61878d158b4a91825463883a530d0be3e66bd5e4d3d92471fcc5`；root完整讀取，writer接受資料／caller明確flush與Rust literal橋接都有第1–7章已教依據，沒有新問題。候選完整檢查也已通過，詳細範圍見ch07-publication-qa.md；此追加完成前一段所列待做，不回填初判。
