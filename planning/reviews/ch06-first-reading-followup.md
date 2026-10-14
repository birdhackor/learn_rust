# 第6章有提示局部修後複查

性質：原第6章50單位初判已封存後的局部followup，不是新盲讀、不是技術/實跑角色。背景沿原actual1–6及第5章followup，原理解與未知不回寫。

原紀錄 `ch06-first-reading-v1.md` 複查前SHA256：`5f2d9b4bef1a9a7c2dc59735f5ade18f2b76989e52f99c6fc39d318180b87bda`；不編輯它。

提示時點：本輪root任務已先告知修法範圍，於2026-10-14 15:06:02 UTC校記後才開第一個正文。提示含Fn名稱prelude、evaluate指定Fn介面理由、adaptor按值Copy/move且不主動clone、Vec Debug移到negativefixture前與一般Vec<T> FromIterator。這些提示是在原初判封存後收到；沒有看peer報告或author notes。原first6只提出Fn名稱來源必要缺口及evaluate能力需要可選疑問，沒有抓到Vec Debug位置問題，也沒有把adaptor按值說明列成缺口。此輪後兩项是收到修訂提示後的局部核對，不假冒原盲讀發現；root提及其他角色採納的流程不等於我看過其報告或自行執行驗證。

唯一actual：`/workspace/work/tutorial-implementation/frozen/ch06-review-fixed-v10`。只依序開003-002、004-002、006-002、006-004；各保存Q1/Q2a/Q2b/Q3/Q4、原句與hash後才开下個。答案折疊先遮蔽，raw bytes只作hash，外連未開；原位圖只到遇到單位才取引用SVG，不以root PASS當自己實跑。

## 003-002（本輪1）

SHA256 `41c8d19787adfb2a3ae4024a846617a7b03924de259f042f6fdb36b033378bad`。新句引文：「FnOnce 與稍後的 FnMut、Fn 都由 std::ops 提供，也已在 Rust 的 prelude 中」。

Q1：補的是三個標準可呼叫trait的名稱來源與預置scope，未改FnOnce的call能力。Q2a：std::ops提供、prelude已放名稱範圍，bound可直接寫三名；自訂trait仍須4.1的use，沒有把所有標準名稱都推為預置。Q2b：原需獨立寫callback契約並能判斷名稱在哪裡、何時須匯入，原示例缺此關係；並非因補文才產生新名稱需要。Q3：run_once的FnOnce不用另use，稍後兩名亦可；自訂介面仍照其路徑引入。Q4：新句位於第一次bound前，足以解原003-002–004的scope未知，不宣稱工具已驗證std路徑。原FnOnce最低承諾/non-Copy移出/Output關係保持。局部可判斷能力已足，無須猜prelude；新句只有已教prelude與use的明確對應，未增加需另查的概念負擔。此unit沒有答案折疊/圖。

## 004-002（本輪2）

SHA256 `b146cb9214d7829455247dc3cc5920e322f0389e981d88b832a7a2f47857ae16`。新句引文：「evaluate 本身只呼叫一次，單獨設計這項工作時，FnOnce 就已足夠；此處的 Fn 是本例選定的介面能力」。

Q1：補的是例子為何選同一Fn介面的教學對照理由，不改impl Trait/opaque型別機制。Q2a：parameter caller供型別、return body選型別保持；Fn更強承諾仍足以一次call，原一次流程本身只需FnOnce。Q2b：此例先要看清參數/返回「誰選具体型别」的差異，保持同種Fn契約是控制對照項；若獨立設計evaluate，業務一次需要不會导出Fn。Q3：讀者可分清API有意承諾與最低工作需求，不再把一次call當成必選Fn理由；不需改動make_adder shared能力或範例才能懂選型方向。Q4：補句緊接evaluate引入、在code前，直接解除原可選疑問；没有新增trait/語法或隐含性能宣称。其能力使用與方法選擇都可判斷，不須猜；不是我技術驗證更換bound成功。此unit無答案折疊/圖。

## 006-002（本輪3）

SHA256 `a4f30717a33e9cc2177d96f3cca703521b7e1b2d8a073eeeed57c815cf6a8613`。新句引文：「新工作按值接收原 iterator……不主動呼叫 clone；交接仍依來源型別的 Copy／move 規則」。

Q1：此次補/收窄adaptor保存原iterator的交接規則，沿1–4章Copy/move/Clone，map的Item轉換與lazy没变。Q2a：adaptor按值保存source和callback，不主動clone；按值不是一律move，也不能一律說没有Copy，需看来源型别能力。原first6顺序37照舊句记「非复制」未标缺口，本轮收到修法提示后才明确把它收窄，不回改原判。Q2b：原流程要把原工作交给逐筆轉換，不需要另clone一份或立刻建完整清單；按值机制仍服从已有资料类型规则，並非由新句反推工作要複製。Q3：可按具体已知能力判断source交接后是否还能用；本例没有再用原iterator，无需猜这個slice iterator是否Copy，Vec仍借保留与visits最后使用规则可照读。Q4：新句足以撤掉「所有iterator不可能被Copy」的过广推论，未给每种来源Copy能力表，也无需为了此任务查标准资料。没有新增技术黑箱，仅复用既教规则；局部能力与用途可判，特定source Copy能力未宣則保留，不擅自验明。无圖/答案折疊、未實跑。

## 006-004（本輪4）

SHA256 `be52997350e070801727535bc54be1f36afd7644d13228d715f20745a98fd526`。新/移位句引文：「標準 Vec<T> 能從 T 元素依取得順序建立，例如 Vec<i32> 接收 i32 元素」；「標準 Vec<T> 當元素有 Debug 時也能 Debug……」。

Q1：補的是Vec一般FromIterator元素/目的關係，另把Vec Debug/PartialEq能力移到negativefixture首用前，不新建收集方法。Q2a：Vec<T>接T逐筆保順序，T可具體數字/char/reference；不能把Vec<i32>接&i32解成自動解reference。若T本身是reference，Vec拥有集合不自动owner原文字（已讀原007-002）。Debug须T Debug；PartialEq须T相應且長度各位置相等。Q2b：原需要把适型别每笔收成目標，後續char與文字视图也会使用同收集；negativefixture要顯示目的Vec，同样须先知道显示能力，不能用後放補文回填當初已教。Q3：collect目标/Item可独立配对判斷；第一次negativefixture的{owned:?}前已提供显示規则，读者无需从後段寻找其来源；Item mismatch仍是该fixture故意问题，未说修订让它编译成功。Q4：一般句在collect介绍处、能力句在圖後而fixture前，实际先后支撑补洞。原first6顺序39記了後段Vec能力但没有抓其首用位置問題；此轮是收到root提示後核位置，不能算原判發現。新增T一般化只复用泛型/ownership规则，没有增加目标自动转换猜测，未實跑。

原位圖：到本輪006-004才讀v10引用 `assets/adaptor-pipeline.svg`，SHA256 `9f76ea92d5a61b0c93cf91d4ee6f4db71714631afd93497fffd23b99eecbca4a`，與我原位看過v6內容hash相同；Inkscape390寬独立render並view，非教材實頁/整頁。四框iter Item&i32、filter callback&&i32→bool而保&i32、map callback&i32→i32取Copy乘10、collect Vec[0,30,30]；三下箭頭標一筆reference、true才下步、新Item i32，底collect反覆next带動前面。图支持逐笔型别/数据流、不是全中間清單，也不是宣称iterator永不Copy；字与线仍對應，無改圖需要。沒用未看實頁字級當passed。

## 本輪局部封存

四單位已按指定順序各保存後才開下一個。Fn名稱原必要缺口解除；evaluate原可選疑问有直接对照理由。adaptor明说按值且依Copy/move、不主动clone，收窄旧过广「非复制」；Vec一般元素建构與Debug/PartialEq在fixture首用前现在可判。這两項有提示覆核不回填成原初判自抓。四处补文没有新增必須查外部的語法/方法需要缺口，未知具体各iterator是否Copy仍应依其已知能力判，不为未涉及操作擅下结论。

只讀這四units與最後unit原位引用圖，無答案折疊實際標頭、过滤全程啟用，外連/author notes/peer报告/完整source/後章未取。没有执行Rust，root修法提示不是我實測证据。保存後停止等待，不重做全50初判。

封存校核：四單位記錄順序為003-002→004-002→006-002→006-004，四個raw hash均吻合指定v10。原ch06-first-reading-v1.md複查後SHA256仍為 `5f2d9b4bef1a9a7c2dc59735f5ade18f2b76989e52f99c6fc39d318180b87bda`，與開始前一致。
