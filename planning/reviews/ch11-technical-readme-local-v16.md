# TECH11 v16 單 README 局部複查

通過：README 的型別已改為完整 inline code `Result<Option<usize>, Elapsed>`，與先前已完整讀過的 `next_started_within` 函式回傳型別一致。本輪只處理 v15 CROSS 新可選 C-I1；不是新 ORIGINAL、全章重審或新獨立發現。

## 固定來源與實查範圍

比較 `/workspace/work/tutorial-implementation/frozen/ch11-review-fixed-v15` 與 `ch11-review-fixed-v16` 的單一 `sources/examples/ch11-async/local-service/README.md`，全文讀完前後兩份README；另讀兩份manifest及root `ch11-readme-v16-scope.json`。程式以manifest逐項比較來源SHA、正規化BODY中的凍結絕對資產目錄後比較bytes，未重新語意閱讀whole BODY／其他來源。

| 材料 | SHA256 |
| --- | --- |
| v15 manifest | `6ffcc35ddcc04c2268c2d319f060df91550a142876d90f76091bec43ddbd9eb4` |
| v16 manifest | `564f274334c43f866892eccd33272c3fb5f7036bc5028f409526355b538f107c` |
| v15 README | `dc7cd5dd0a11312ad68212f457c85126ac1401ba9bdbdba50ce676d3f8959cbe` |
| v16 README | `4ab13b311a60c03dc494942e20fc0a2671cc01ccba1d4e75c587fa983917a3fc` |
| root scope檔 | `e89009ee96f819ea34dff26e98a3d0c767acb0d4b2f72f8aa095ad3d2076c0a6` |

兩份README實際bytes各自符合manifest。76個來源鍵一致，只有README SHA不同；8頁81 BODY單位的順序及頁面對應一致，正規化凍結asset根目錄後完全相同。scope明記原raw比較因新絕對資產目錄不同而失敗，沒有把這種路徑改變說成學生內容修訂。Rust及9 SVG來源指紋均未變。

## 修正與理解收益

README第5行原句為「取得Result<Option<usize>,Elapsed，」，缺最外層最後 `>`、未包code span。新句為「取得 `Result<Option<usize>, Elapsed>`，」。以精確字串替換驗整份README，除此一處及相關空格沒有其他文字改動。

此前已讀函式簽名是：

```rust
pub async fn next_started_within(&mut self, limit: Duration) -> Result<Option<usize>, Elapsed>
```

README現在保留外層Result成功／逾時、內層Option收到通知／channel關閉的型別結構，並以inline code避免把尖括號當HTML標記。v15正文008-006及真實函式本來就完整，故仍是可選來源定位改善；不後見升級為原教材必要先備洞。原C-I1發現者／交叉時點保持；root採納是後續處置，非再一次獨立重現。

## 保留的證據邊界

原TECH prefix `9d3d071f09f59016cb058983b816763d2da71782cd2d5b22b704384047a9b72d`、tail `fdefe62ca564961c7e9d2fe3b95c69b96776928c9af7468580f077acfabbfe9e`及v15 CROSS `13fd5004878ae6fab80491b8ac4b42a7f4531bb99a16fe6e43be4e2ad54664cc`再次核對未變；不修改原grade、C-I1記錄或26張v15實頁紀錄。

本輪未跑Rust、未新做probe／HTTP／全套checks，也未重建實頁或新增畫面；既有26張畫面仍只證v15的原局部視覺範圍。這次通過限單README source格式及版本邊界。未讀任何新peer或其他章scope；封存報告與sidecar後停止。
