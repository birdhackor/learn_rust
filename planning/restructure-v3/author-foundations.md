# 1.7 作者交接：計算機基礎

作者：author_v3_foundations。根代理已確認實際 1.1–1.6 穩定後才開始 canonical 正文與 SVG。交接凍結指紋另見本文件末尾；作者自查不代替獨立技術、銜接或首讀審閱。

## 已親讀的實際前綴

`docs/chapters/01-01-lesson.md` 至 `01-06-lesson.md` 全文，及其中實際引用的入門 workspace manifests、入口、數值／mutability／函式／block 範例與答案、四個 compile-fail fixtures。它們已教編輯／編譯／執行、package／crate／workspace／example 選擇、型別／數值運算、`println!` 佔位、mutable／shadowing、函式參數／回傳、expression／statement／block／unit／scope。

1.7 不把 1.8 的 tuple／array、後文 struct、`String`、`Box`、reference／raw pointer 語法、`Copy` trait 或任何配置 helper 當先備。Root 授權接管實際 `memory.rs`／`memory_answer.rs` 後，在 `result` 與尾端回傳之間加已教過的 `println!`，讓 B 觀察點及局部變數都有可見用途，避免 Clippy 的 `let_and_return`。

## 首次需要與實際教法

| 必要概念／轉換 | 先備的實際位置 | 1.7 首次解釋位置 | 本節可查的理解結果 |
| --- | --- | --- | --- |
| 原始碼檔、執行檔、一次執行的區別 | 1.1「三個工具各做什麼」；1.2「先檢查，再真的執行」 | 「檔案留在儲存裝置，執行形成一次 process」與 `program-process.svg` | 重新保存 source 不會自行更新已有執行檔；執行時變數不會自動寫回 source |
| CPU／RAM／process 的角色 | 基本 Python 使用經驗；前文只教操作，未假設硬體知識 | 同上，先給全名與當下工作再讀箭頭 | CPU 執行乘法、RAM 保存本次工作資料、儲存裝置保存檔案 |
| bit／byte、i32 的 4 bytes | 1.4「其他單一值的型別」把型別名中的數字定為 bit 數 | 「值、大小與位置是三件事」及 `i32-value-size-location.svg` | 32÷8=4；120 與 80 同為 i32，表示大小不因值變小而減少；相同值不代表相同位置 |
| 呼叫中與回傳後的資料 | 1.6「呼叫把資料交給參數」與實際 `functions.rs`；尾端 expression 已教 | 「沿著 A、B、C 追蹤一次函式呼叫」先完整 source，再定義 frame／stack 與 `subtotal-frames.svg` | A 尚無已取得初始值的 first；B caller 等待、callee result=360；C caller first=360、callee 那次 frame 結束 |
| scope 與函式 frame 的差別 | 1.6「內部 block 也能交出結果」明說 block 沒有函式呼叫 | frame 圖後相鄰段落 | 不把每組大括號畫成新函式 frame；不把圖中槽當成編譯器必定使用的實體存放 |
| heap 配置、位置資訊與內容 | 前面數值／函式工作概念；本節承擔新概念介紹 | 「另一種需要：內容量不同的訂單明細」與 `heap-location-content.svg` | H1／H2 是不同案例的兩項／五項內容；pointer 表示去哪裡找，內容不是存進定位框的總和 |
| 呼叫結束、資料仍使用、最後歸還的時序 | 同一 H1 明細已在前圖建立 | 「呼叫結束與內容歸還，分開追蹤」與 `heap-lifecycle.svg` | T1–T4 追蹤同一 H1；歸還後空間可再配置，不是清成零；heap 不保證跨呼叫存活 |
| 變值後仍用相同表示大小與呼叫規則 | 本節前三段實教 | 「練習與完整答案」及實際 `memory_answer.rs` | 80×2=160，B result／C first 同為160，i32 仍4 bytes |

## 真實 source 與同頁完整性

- 完整 runnable source：`examples/intro-workspace/steps/examples/memory.rs`；1.7 同頁整檔 snippet 及已教的 `cargo run --manifest-path … -p intro-steps --example memory --locked`。
- 完整答案 source：`examples/intro-workspace/steps/examples/memory_answer.rs`；同頁整檔 snippet、命令與預期輸出。這份答案只有一筆 80／2 的呼叫，不依賴外部 helper。
- `memory.rs` 第一筆以 caller `price=120,count=3` 呼叫同名參數的 `subtotal`，B 的 result=360 先印出，C 的 first=360 之後印出；第二筆80／2使新的 B result=160，可驗證每次呼叫使用本次材料。
- heap 圖是明示的訂單明細機制示意；數字 source 沒有為那些數值額外配置 heap。正文一次交代後直接教內容，不反覆插入維護式保證。

## 圖與檢查範圍

五張原創手寫 SVG 均有 title／desc、336 px 直式 viewBox、最小文字18 SVG px。A 分檔案／一次 process／RAM／CPU；B 分值／尺寸／位置；C 沿真實A/B/C；D 分定位資訊與兩個案例的內容；E 追蹤同一H1的建立／交出／內層結束／歸還。

作者以 Inkscape 實際渲染並親看五張336px寬PNG，未見遮字或出界；圖中必要資訊不依賴點擊放大。檢查原始 SVG 的 XML 與所有文字尺寸。尚未在 Zensical 實際頁面做1280×800／390×844原位檢視；不宣稱頁面版面通過。未執行 Cargo、root configuration、導覽或git操作，依root分工由整合者執行編譯與網站檢查。

已避開：所有 locals 必在 stack、固定大小不能 heap、heap 一定比函式活得久、move／Copy 由 stack／heap 決定、pointer 一律在 stack、釋放等於清零、內部block等於函式frame。

來源實讀與採用限制見 `sources-foundations.md`。實際局部驗證與 SHA256 由凍結清單補入。

## 凍結 SHA256（2026-10-15）

作者局部靜態檢查通過：五張 SVG 的 XML、title／desc、336px viewBox、最小18px文字；正文兩個完整 snippet 及五個圖片路徑存在，code fences 成對。未執行 Rust 工具。

| 檔案 | SHA256 |
| --- | --- |
| `docs/chapters/01-07-lesson.md` | `017a1fa288e5545af8878f752fc7b4730ca639df1025ba8a90c4e5ffc41fc49d` |
| `docs/assets/foundations-v3/heap-lifecycle.svg` | `e53c8727d4f10625fe015d1836372806985c432ebd1271dc29711916e419cd9a` |
| `docs/assets/foundations-v3/heap-location-content.svg` | `cbf0c1a2d8e130303093a37246f15096a1e08b70ed4d15175e5e1b3315b074ab` |
| `docs/assets/foundations-v3/i32-value-size-location.svg` | `00c15a22127e47bef1c326c613aeb9ed9e521830f16aae6723041e576a48e891` |
| `docs/assets/foundations-v3/program-process.svg` | `03367eb5f7d3bef3aaa637ba242b0e90a2a96d8cd0ef2536ca94132fb64db561` |
| `docs/assets/foundations-v3/subtotal-frames.svg` | `4d20deebd597037398e621e19d50132de691b599646e6d4f65bbbf7ef54ce37c` |
| `examples/intro-workspace/steps/examples/memory.rs` | `54dc40c93a46f7a0e3af6a4e8ebd78da172e037035bb9373c1f026e97b5e3661` |
| `examples/intro-workspace/steps/examples/memory_answer.rs` | `11d2fde89c13aa7aceb49d6a8e114d5f6ac3cbef5eede68bcf27be6f27775c7b` |

Root 首讀包釋出前的局部修正：在完整 `memory.rs` snippet 前，先介紹 `//` 到行尾的註解意義與本節 A／B／C 觀察點用法。僅更新 1.7 正文與其凍結指紋，source／SVG 未變。
