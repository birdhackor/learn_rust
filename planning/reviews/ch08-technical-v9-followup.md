# 第8章 TECH 交叉與修後局部覆核：v9

五項原可選建議的最小修正已解決對應問題，本輪沒有新增必要技術缺口。這是獨立 TECH 原初判封存後、受同行提示的交叉與局部複查，不重新執行或改寫首次閱讀，也不宣稱全書發布驗收。

本輪實讀 FIRST、CONT 原始報告與 root disposition；修後重新實讀六個實質受影響 units 及相鄰過渡，共17個 BODY，查兩份真正改動 source 及相關原 library／fixture，核對全部66 BODY／150 archived source hashes。自行實跑一個改過輸出文字的 normal target，另親看固定 dyn 原位頁面的桌面／手機圖文。沒有因 prose／標籤座標改動重跑所有 Rust tests／negative fixtures。

## 角色、原判保留與唯一版本

- 本人不是第8章作者。先前為第12章使用過第8章先備，此背景已在 original 宣告；本輪也不是新 blind 或 first reading。
- TECH original：planning/reviews/ch08-technical-v8.md，SHA-256 **87dec72d3296ff2738b0b39d8a4cb64328d82effa3eda64edb2c1c282cd91b4c**。原結論為必要0、可選 I1 一項，原時點沒有驗證實頁；原文、sidecar 和 work 初判副本都保持不變。
- FIRST original：ch08-first-reading-v1.md，SHA-256 **5061c8bfd8266d4328da4e1f9bc50541aa1b2830c9ef9a7c13e718573d1a4eb2**。全66逐段四題，必要0、可選 O1／O2；沒有自跑 source，也沒有完整實頁證據。
- CONT original：ch08-continuity-v8.md，SHA-256 **4fc7fb3691c31e4bcf6a6939a433eec120f2676a454ac4635bed0a8a94b99930**。必要0、可選 O1–O3；原scope已有150來源與八圖實頁，而不是我的原 TECH scope。
- 三份均封存後，才收到 root 本輪 cross 授權並讀同行。讀入 root 的 ch08-root-disposition.md 與 work/tutorial-implementation/ch08-fix-scope-proof.json；它們用於對照，未替代本輪逐 bytes／source／選定 stdout 與實頁核對。
- 唯一修後來源：**/workspace/work/tutorial-implementation/frozen/ch08-review-fixed-v9**。manifest SHA-256 **92d0d7718b079bd60039245b41166a056b4a815ed653cb69d6bbd06a142b5978**，實際10頁、66 BODY、150 sources，8個原位 asset 複本。
- 原v8 manifest仍是 **79036df9490cd5ad5874be7978062928ff185e1d25abc76adf610c2678eae9c1**。此前 original 全66／150的實讀與有限 probes只支持當時scope，不改標成v9新全章獨立審閱。

## 交叉：同一問題、不同發現時點

| 來源／原位置 | 已教能力與原最小問題 | 本輪判斷、影響與修後核對 | 發現記錄 |
| --- | --- | --- | --- |
| TECH I1；8.1 001-002／001-005 | 原稿已完整介紹 Sized／DST／Box；5.4 已教 prelude，但這兩個 bare 名稱沒有明指其來源。 | **可選、已解決。** 各補 std::marker::Sized／std::boxed::Box 與 prelude，讓何時需要 use 的既有規則可對上；不改大小或 ownership 契約。修後為001-002／005最初定義處，沒有新API或篇幅旁支。 | 本人的 original I1。同行首次讀者已依既有預匯規則理解，沒有因此推高必要性。 |
| FIRST O1；8.5 005-002 | 前段 Charge impl 與既有 Vec 同T足以推共同trait不合併兩個型別；獨立 mixed_charge_vec fixture沒有 Charge 宣告／impl。 | **可選、已解決。** 原 fixture只能直接支持異型不能同Vec，原正文近旁的共同trait推論仍成立。修後把 fixture 的結果歸因限於不同型別，把共同amount能力明連回前段；沒有要求重寫helper或補新機制。原 fixture source未改。 | 我在 original 未另列此歸因改善；本輪由 FIRST 提示後補查，不能補算第二份 initial 發現。 |
| FIRST O2＋CONT O3；8.5 005-004／dyn-dispatch.svg | 圖已有兩份同型handle、各自資料與表入口的對應；原 method表字從x200起，右線在x290，同行實見穿字。 | **可選、已解決。** 兩份label各改到x160，四條箭線／資料框／表框／amount80與120均未改。本輪自己實頁看見label和右線之間留白，必要對應保留、沒有新增交碰。不是原計算／ABI或dyn機制錯誤。 | FIRST與CONT各自原initial已報同一關係，是兩份獨立重現。我的原 TECH只看source、實頁未驗；這次是提示後補查，不冒稱原圖可讀性已被本人驗過。 |
| CONT O1；8.4 004-002 | 必要前文7.6已有 Into<OsString> + Clone 同時要求；原句稱「本節新增」+，但本節大小／lifetime組合已解釋足夠。 | **可選、已解決。** 修後稱沿用7.6，仍先解釋T:?Sized+'a各條件，保留implied bound與三種T的差異。自行重讀 actual7.6 006-004確認前文，沒有重新宣稱新符號。 | original TECH未列這項歷史定位改善；本輪受CONT提示核對，不後見升級成缺少bound教學。 |
| CONT O2；8.9 009-005／newtype_interface.rs | 原正文已明定local按定義crate，BatchSize在library，example只use；binary印「本crate」容易讓讀者回找主詞。 | **可選、已解決。** 改為「library定義的trait」，與library宣告／impl及example imports一致；正文source與預期stdout同改。自行實跑修後target，三行與009-005完全相符。不是orphan本來未教或source違法。 | original TECH支持crate規則，沒有原始列出此文字歧義；本輪由CONT提示補查。 |

上述沒有回寫原必要0或可選分級。本人原initial獨立發現一項可選，這次多四項是同行提示後的核對；其中圖項原first／cont有獨立重現。root先前漏看交字的實頁範圍，也不能被本輪結果改成root最初已發現。沒有用位置相近、同行同意或命中數代替原缺少關係的證據。

## 候選解除與未報通過項的覆核

CONT原候選where首次、短片段imports、Box Error／Any Debug已在其獨立原報告保留否證路徑。本輪對照自己的original來源核對：

- where在8.5使用前已在005-007當下教；不把陌生clause名稱一律升為必要缺口，也不從未到的12章補它。
- Deref／Add／fmt短片段的完整library有明確std imports，且正文給API來源；既有use／module規則可對上，沒有需要發明黑箱helper。今回重讀newtypes.rs頭兩行與完整module。
- Box Error／Any的完整tests以match取結果，不把未知error Debug當作expect的隱藏前置；original已全面實讀150份source，fixed hashes亦證明這些source沒有改。沒有因同行候選而新增API教學。

此外，不只核對五項報告清單，也重新核對選定通過的原需與支持範圍：

1. **Sized／Box不是隻換名稱。** 001-001～006先提出自含型別的表示問題；array／DST分大小對象，const保型別長度，Box的固定間接next使Chain表示可收束。borrow／whole move／Box特殊move-out分開，表與正常／拒例都有同一資料對應；名字來源補句沒有把Deref提早當先備。
2. **T:'a沒有命令值活更久。** 004-001～003把Box內reference、內部來源要求、模型外borrow與literal有效性分開；新+引用不取消T:'a意義或implied bound。String／str／&str三種T仍真正不同，Copy reference不接手原String。
3. **dyn的理由仍是caller可擴充型別。** 005-001～005保持80＋120；KnownCharge可算200，Shipping caller新增後既有total算220，library沒有新增variant／match。圖是資料與入口關係，不是所有dyn必慢的證據；fixture歸因收窄沒有縮掉此原需要。兩個分支opaque不合一T的fixture仍另有真正Charge impl，不能倒填前一fixture原本也有它。
4. **newtype／orphan有兩種責任方向。** 009-004～006的owned Add保單位、MeterBatch只公開有限method、library local BatchSize可impl外部Vec、From uncovered順序與overlap各有不同拒例。改輸出主詞只讓既有規則清楚，不令import變local，亦不推所有foreign Self／local argument都非法。

這些與兩份peer的通過推論相符，沒有發現此輪修改引入新先備、機制錯誤或超範圍效果主張。其餘未改148份sources沿用原initial實讀與hash相同的支持，不冒稱本輪另行全文重讀150份或重跑整套。

## 自行核對的修後範圍與正式來源

完整重讀固定BODY共17units：001-001～006、004-001～003、005-001～005、009-004～006。另直接重讀 archive newtypes.rs／newtype_interface.rs／mixed_charge_vec.rs／dyn-dispatch.svg；核對 source diff 與原時點規則。回讀 actual5 ch05-review-fixed-v7 的004-002（prelude），actual7 ch07-review-fixed-v13的006-004（+同時要求）。其他必要前文為original已保存的實讀，不用大綱補教。

重新開啟正式 [std1.99 prelude v1](https://doc.rust-lang.org/std/prelude/v1/index.html) 的 Sized／Box re-exports，以及 [Reference：trait implementation coherence／orphan](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence) 的定義crate／uncovered／fundamental關係。官網顯示1.99.0 b940084d7，和執行版本相同；只用本輪選定契約，不把頁面其他nightly／後章內容作讀者先備。

自己驗66個chunk與150份archive SHA，8個asset複本對archive SVG bytes。原／新source key相同，只有以下兩份hash不同：

- docs/assets/ch08/dyn-dispatch.svg：兩個text x200→x160，其餘bytes相同。
- examples/ch08-smart-pointers/examples/newtype_interface.rs：只有println內主詞文字更換。

13個raw BODY差異分成五份prose改動、八份asset目錄prefix更新；其中005-004指到真正修改的dyn圖，因此**實質影響為六份unit，其他七份為同byte圖的純prefix差異**。這個區分由本輪獨立比較重建，與root proof一致；沒有把13份raw差異都誤算教學變更，亦沒有因005-004只是BODY路徑差異而漏查真SVG改動。

## 自己實跑與實頁證據

在work/tutorial-implementation/technical-ch08-v9-followup/isolated，完整package複製自固定archive，另建own workspace／toolchain／lock，不使用root lock。選定command array與stdout／stderr／status在selected-output-probe.json：

```text
rustc --version
cargo generate-lockfile --offline
cargo run --offline --locked -p ch08-smart-pointers --example newtype_interface
```

Rust1.99.0，target為x86_64 Linux；own lock SHA-256 **3315475cf952643ac73d9e2ffcbabba2e8e40a75bc54c45c0718773ce343a90c**，只含本package。修後target退出0，stdout完全符合009-005的三行：

```text
路程報告：2 段路程，共 200 公尺
再次借讀總路程=200公尺
library定義的trait能實作在Vec上，項目數=2
```

其餘normal／negative／library tests未重跑；原 TECH的6 selected tests／3 normal／7 compile-fail／2 runtime-fail只作原時點證據，root candidate checks也沒有冒稱為本人本輪執行。

實頁以固定8.5八個BODY重組單頁：去除chunk重複H1、把圖連結改為本地檔，內容、順序、SVG bytes不改。獨立strict preview成功；這是局部preview，不是live網站或完整全章頁面。

親看1280×900與390×844兩張原位圖文截圖：圖完整顯示，上方說明可讀，兩個method表label與右線分開；values[0]→左Order80×1／右Order表amount，values[1]→左Bundle120／右Bundle表amount都可定位。箭頭沒有改成move／clone，下面80／120仍對應各method結果。SVG intrinsic390×690，mobile343×607，24px字縮後約21px，必要理解不靠放大；document沒有水平overflow，沒有圖標／端點裁切。證據在screenshots/render.json與dyn-fixed-desktop.png／dyn-fixed-mobile.png，均由view_image親看，而不是隻相信DOM尺寸。

沒有全章其他七圖的新視覺複查、全站導航／fold／外鏈驗證、跨平臺執行或真人初學者試讀；這些不以同行或root的舊PASS補成自己的新證據。

## 封存、保留與停止

本followup另存work副本與SHA，不修改原initial。source-fingerprints.json保存v9 hashes／scope、實質差異、三份original報告hash、原TECH副本與sidecar的前後比對；全部原報告分級保留。

第12.5的10份學生材料最終fingerprints也再次核對相符，本輪沒有修改C正文／source／圖或sharedCargo／nav／lock。完成交叉後停止，12.6仍等root提供12.5 actual gate；沒有讀peer10／11／12的其他review歷史。
