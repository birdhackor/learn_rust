# 第 9 章 TECH CROSS 與修後局部覆核：v6

`chapter10_concurrency`，2026-10-14 UTC。這是原初判封存後獲 root 開放的交叉與局部修後覆核，不是重新盲讀、不回寫原判，也不是全站共享樣式驗收。

原 TECH9 `ch09-technical-v5.md` 保持 SHA `38dee2d6274c72f72826743163d343db51e572e9873baef839eb3317207918fc`，原必要0、可選I1的分級不改。本輪才完整讀取 FIRST9、CONT9、root disposition 與修正範圍證據；未讀 peer10、作者 notes 或 live 第9章正文。

| 已開放紀錄 | 本人重新計算的 SHA-256 |
| --- | --- |
| ch09-first-reading-v1.md | `7feff159c1a6f23bc9e61c53bf0893fd261c7a624b11a39cbd7d32dc4e91740d` |
| ch09-continuity-v5.md | `5f3112ed7fc6db3e4b9192d4a095bf03556330190d8608c92af6425b818fa5e6` |
| ch09-technical-v5.md | `38dee2d6274c72f72826743163d343db51e572e9873baef839eb3317207918fc` |

## 交叉：原需與通過理由的範圍

FIRST 的 27 單位紀錄只使用當時開放正文，三份完整答案未開、未實跑 Rust、未親看完整桌面／手機頁；其理解證據不等同 TECH 的 source/API 查證。CONT 也有先前作者背景，另有完整 source 與實頁檢查；不能把兩個 nonwriter 專家角色合稱基本 Python 盲讀。

我對照同行的通過理由與自己的 P1–P7，核對以下具體關係；沒有只因三份必要項均0就宣稱一致或全通過。

| 關係與原需 | 同行提供的證據／與原 TECH 的交叉判斷 |
| --- | --- |
| 9.1 共用來源、此次可選介面、編譯設定 | FIRST `001-002–005` 分開回答 root定義/member採用、profile、feature/cfg!，CONT回收80×2與兩筆/零數量的不同用途。符合原 TECH P1：160/280只是計價材料，不能替共用設定、編譯排除或效能理由代答；未出現需提高入口背景的差異。 |
| 9.2 建置當下的設定要由成品固定回報 | FIRST `002-002/003` 明列 script runtime、cargo stdout交付、crate編譯env!、binary runtime的四時點，且Linux相同平台不能證明host/target同義；CONT同樣指出原需在方法前給出。支持原 TECH P2的用途與界線；本人的六項原probe僅驗特定行為，不補算成FIRST已跑。 |
| 9.2 OUT_DIR/build-dependencies 的導覽 | FIRST/CONT 都保留「本例只交文字、無生成檔案或新增第三方」的實際範圍。原 TECH 契約查證支持其角色；當前程式不依賴未教生成macro，故未補成必要缺口，也不宣稱已完成完整生成工作。 |
| 9.3 一般檔案與個別失敗並存、native身分 | FIRST `003-002–007` 分出root/內部link、metadata觀察、Option借來路徑轉owned副本、partial move及Some(&5)仍報issues。CONT核實原生key而非lossy display及受控Ok/Err/Ok。與原 TECH P3相符；打不開目錄的內容沒有被說成仍能取得，ScanReport無自訂Drop的適用條件保留。 |
| 9.4 候選縮工作、bytes才確認 | FIRST `004-001–004` 從原問題及5/5/6材料推得候選用途，另回答BLAKE3累積與slice相等；CONT分開機制與用途。符合原 TECH P4：結構分工有理由，速度仍未測；有限來源與Read契約是比較完成的前提。FIRST所說零上限是「不合法測試設定」，限short_reader明訂正上限政策，不能擴成一般Read返回Ok(0)皆違約。正文沒有這種泛化，不需改稿。 |
| 9.4 外部檔案觀察的有限保證 | FIRST `004-005` 指明path/handle查詢對象與非原子gap；CONT區分讀完再變長的受控測試與同步改每次read。與原 TECH P5相符：觀察能揭露差異不等於一致快照或惡意替換安全；未以同行文字支持本輪未做的race實驗。 |
| 9.4 A/B/B、單例驗證、排序與量 | FIRST `004-007–009` 在不開答案下可說出b1第二次比b2才進B組、0+3=3及按路徑量；CONT指出各設計的用途和單例多讀一次的代價。支持原 TECH P6/P7；手工同摘要鍵不是實測BLAKE3碰撞，hard link量不是實體回收量。完整繼續/CLI政策仍合理留最終專案，當前元件不偷用該實作。 |

上述同意是**封存後交叉覆核**，沒有新增第二份盲讀證據。FIRST重連時已開未存`004-002`的重取限制保留；不同角色實讀的1–8版本/局部範圍亦保留，不改成三人都重新看過全部最新前文。

## 發現來源與裁定差異

| 項目 | 原獨立發現與本輪判斷 |
| --- | --- |
| TECH I1／CONT O1：兩處literal `**` | 我與CONT各在原實頁獨立看到同一關係與呈現雜訊，均列可選；本輪才互看，屬獨立重現。FIRST未看整頁，未報此現象不構成技術/理解裁定矛盾，不能事後回填它的原發現。root採加空白，收益與原最小建議一致。 |
| CONT O2：fold code較一般source小 | 本人的原TECH明記未驗全部fold UX，沒有報出這項關係。**本輪由CONT與root處置提示才量測**；只能記提示後核實有收益，不能新增到原TECH獨立發現或其SHA。可選分級成立：折疊外必要規則已足夠，改善的是參考完整答案時的字級一致性。 |

## 本人重建修正範圍

唯一修後材料是 `/workspace/work/tutorial-implementation/frozen/ch09-review-fixed-v6`，manifest SHA `8663faec8eb6fbe5931b1388880182f96d465910d1118dd6d442beb5a6e995f8`。我自行比較兩版27 BODY（先正規化凍結資產路徑），只得到以下兩筆實質差異，與root的scope proof相符；不是直接引用root結果當本人diff。

1. `002-001:5`：`編譯。**這支程式` → `編譯。** 這支程式`。
2. `004-001:3`：`相同？**檔名相同` → `相同？** 檔名相同`。

兩版原39個source hash全相同，包含Rust/helper/答案/test/fixture/Cargo/三SVG；修後所有27 chunk與40 source實際bytes亦核對manifest相符。唯一新增source是`docs/stylesheets/tutorial.css`，SHA `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`，內容只有註解與：

```css
.md-typeset details {
  font-size: inherit;
}
```

這個rule讓details繼承其parent字級；不是將所有code寫死成12.8px，也不是只作用本章。scope selector是全站`.md-typeset details`，因此root需要另核其他答案與代表性舊頁；本人不把一份答案的成功擴成共享CSS全面發布通過。

本人diff/hash/preview config證據保存於 `/workspace/work/tutorial-implementation/ch09-technical-preview-v6/own-scope-proof.json`。未修改原TECH或學生稿、sharedCargo/nav/lock；source/API行為與前置介紹沒改，本輪沒有再跑未變Rust或重讀未變全文。

## 本人獨立同theme預覽與實頁

在 `/workspace/work/tutorial-implementation/ch09-technical-preview-v6` 從fixed v6 BODY按manifest重組四頁，保留fold，僅去除閱讀包重複頁標題與映射三SVG到局部資產；theme/extensions沿用自己的原v5預覽，nav限定四頁。**明設** `extra_css = ["stylesheets/tutorial.css"]`，CSS從fixed archive複製，沒有讀live正文或借root畫面。

config SHA `7d5ecbe1d34294d0e369006871117314400d5070a4e6b21f628bbd79cc753846`。獨立 `zensical build --clean` exit0，config的strict=true；完整命令、exit與log另存`build-result.json`/`build.log`。HTTP/DOM實際載入`stylesheets/tutorial.css`，不是只放檔而未套用。

Chromium/Playwright：desktop1280×800、mobile390×844、device scale factor1；14張截圖**全部本人view_image親看**。清單與hash保存`visual-reviewed.json`；DOM/實際開關/字級及位置在`render-checks.json`，script為`render.py`。

### 修後 I1：兩處entry

桌面及手機的9.2/9.4 entry（4張）均將原句顯示為正確粗體；DOM每處有1個對應strong、font-weight700，該paragraph中literal `**`數0。句子角色與相鄰說明保留：9.2仍是新增編譯前工作的轉折，9.4仍是完整bytes相同的原問題。I1在本輪限定位置修妥，沒有追加概念或新先備。

### 提示後 O2 核實：9.4 完整答案展開

我自行點開原先closed的9.4「完整可執行答案」（desktop/mobile各1張）。修後fold code與一般source皆12.8px，details15px。為核對樣式因果，只在**同一fixed v6頁面probe**暫時disabled該CSS link，再恢複；不是把它說成重新盲讀原v5或原初判發現。

| viewport | archived CSS啟用：fold／normal code | 同頁停用CSS：fold／normal code | 恢復CSS：fold／normal code |
| --- | --- | --- | --- |
| desktop1280×800 | 12.8／12.8px | 10.9227／12.8px | 12.8／12.8px |
| mobile390×844 | 12.8／12.8px | 10.9227／12.8px | 12.8／12.8px |

這項controlled probe與親看支持CONT所說的字級關係及root採用目的：開答案參考時不再縮小code，normal code沒變。手機fold code可用寬度311px保持，最長內容scrollWidth由579變682px；較大字仍需水平捲動，改善字級不代表消除長行的捲動成本。本輪畫面/DOM中整頁寬375≤viewport390，沒有新增整頁橫向溢出；答案沒有變成必要規則的新入口。

### 必要圖與局部過渡

本人另親看三SVG的原位desktop/mobile（6張）：仍340px寬，四建置階段與LESSON_MODE/TEACHING_MODE交付、左右短read→四格比較段、forced digest桶→b1第二次比b2→真正兩組的標記/箭頭/相鄰圖說均保持可讀。SVG bytes完全未變；CSS僅選details，三圖在折疊外，不以hash本身代替此次視覺核對。

9.4答案展開後的「正常與失敗，都保留可控制的驗收材料」與收束（2張）仍能接到streams/files/groups用途、有限觀察、Unix限制、最後專案組合的既有分工。桌面/手機沒有把必要文字新藏入fold或造成資料角色變更。此處是本輪局部過渡檢查，不改稱重新看完四頁所有滾動位置。

## 結果與尚未查範圍

- 原TECH I1與CONT O1是原獨立同項重現；固定v6兩處修妥，維持可選呈現改善的原分級。
- CONT O2由同行提示後本人核實；本輪9.4一個答案的字級及對照normal符合採用目的，沒有回填原TECH或FIRST。
- source/介紹時序無實質改變，原技術判斷在相同hash範圍保留；本輪未確認新增必要待修項。
- 9.2、9.3另兩個fold、代表性舊頁、nested details、其他theme變體/深色模式、公開deployment和全站閱讀負擔本輪未親看，交root已安排的共享style覆核。本人未用root的DOM數字或照片代替自己的probe。
- 未重跑Rust、features矩陣、跨平台或race/性能實驗；原未驗範圍不因CSS修正或同行同意而改成已驗。

本followup與SHA另存，原初判保持。完成本輪授權交叉及修後局部覆核後STOP，不擴第10章或其他未授權範圍。
