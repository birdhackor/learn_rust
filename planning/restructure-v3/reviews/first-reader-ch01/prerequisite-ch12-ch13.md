# 第12／13章普通完整頁先備閱讀

已循唯一 reader_feed 依序完整讀完 **220–231，共12頁**：第12章四頁220–223，第13章八頁224–231。每頁先保存當下四題、引文／依據、機制與用途分開的理解、自己的小變化推論與未知，再取下一頁。`records/220.md`–`records/231.md` 均已保存。231記錄後再呼叫feed，實際得到明確 **STOP: no more frozen material has been released**；現在停在該邊界。

這是 **ordinary prerequisite page reading; not staged first-use evidence**。完整正文、inline manifests、fixtures、source、必要折疊區、測試、負例和同頁練習答案都讀；不能稱第12／13章逐概念首次驗收，亦不將已看到答案的作業說成隔離解題。原先所有章節 reports／records及190／194誠實更正保留，沒有覆寫。

## 路線及實際限制

- 只有reader_feed.py給出的當前合法packet作正文來源，沒有直接讀queue、repo正文或source、作者ledger、其他review、外部教材或後章。
- 沒有執行教學、Cargo／rustc／測試、fixtures操作、thread／死鎖／panic程式，也沒有安裝。所有輸出／診斷和測試通過來自教材展示，非本機實驗。
- 223單頁很長，第一次工具輸出中段截斷。我未先存record／取224，而是重取同一未完成feed，將stdout的330–710、710–1120、1120–1510行分段顯示，補全缺失中段。原頭尾與全部測試加這三段完整讀完後，才保存223。沒有因分段直接打開底層source或讀未釋出單位。
- 224閱讀中root通知225暫停更新，隨後明確通知新版packet/hash完成可繼續；此時225尚未取得。我先完成224記錄，再取225，未看改動背景，也不把版本通知當技術提示。
- 226對外層a和body借用的差異有停下重讀，具體停頓及恢復理解保存226。231才讀到所有atomic仍能有邏輯lost update、data race至少一方non-atomic的更精確區別；原224初句保留，另追加231後釐清，沒有倒寫早已知道。
- 沒有確認暴露未釋出內容。第14章只見原文預告，未讀任何14正文；只等root正式釋出。

## 實際親看的必要PNG

共 **12張**，只按當包metadata原位置查看，未提前開其他圖。以下列必要對象、箭頭及不能推得的事：

|單位／PNG|實際讀到的對應|
|---|---|
|221 `ch12-build-phases.png`|四個縱框：host編譯build.rs→執行script讀LESSON_MODE交TEACHING_MODE→編譯主要crate由env!固定→成品執行不重讀。箭頭先後，不是ownership或每次Cargo必重跑。|
|223 `ch12-short-read-comparison.png`|同串ABCDE，左ABC｜D｜E，右A｜B｜C｜D｜E；各填4或EOF後比較第一段ABCD／第二段E。批次線不是內容，圖4格不等實作4096，也不保實際File批次。|
|223 `ch12-candidate-content-groups.png`|手工同digest aAAA、b2BBB、b1BBB全3bytes；b1先比a不同再比b2相同，真組a與b1/b2。不是實際找到BLAKE3碰撞，不能只比桶首。|
|224 `ch13-send-sync.png`|上threadA String綠ownership箭頭交threadB；下原owner保留String、threadB的&String藍借用箭頭指原文，&String Send等價String Sync。契約示意，不是helper已建thread。|
|225 `ch13-thread-join.png`|左spawn後算1+2+3=6再join；右接Vec80/120算200、工作結束含清理，綠成果回左6+200=206。縱向距離不量時間、不定哪邊先跑。|
|226 `ch13-hrtb-local.png`|F介面對任一合法借用有效期可輸入；灰with_local內80藍借給callback一次，綠owned200跨灰框交出。局部value仍在函式結束，HRTB不延長其生命。|
|227 `ch13-scoped-wait.png`|外values與total，灰scope內左callback建工→drop handle／callback返回→自動等body；右借values/獨佔total寫200→body結束，藍線使左可返回；底main才讀200。callback返回與scope返回有別，圖只到body，不包含可能未完thread-local清理。|
|228 `ch13-channel-drain.png`|main原sender clone兩份後drop；worker各送兩／三項並drop；同buffer在sender全無仍有待收，main收空才Err，底再join工作。權限關閉、待收訊息與thread完成分開。|
|229 `ch13-arc-mutex-guard.png`|main/worker兩Arc藍指同Mutex總額0；此可能次序main鎖+120/drop，再worker鎖+80/drop/bodyend，main join後再lock讀200。非兩份總額，guard解鎖與join收尾不同，main先非固定。|
|230 `ch13-rwlock-readers.png`|同Vec200兩綠readers，try_write灰WouldBlock無guard；drop first灰、second仍綠再Block；dropsecond後writer獨佔+30再釋、後read230。單thread操作序，不是並行速度證明。|
|231 `ch13-atomic-publication.png`|左P1DATA230Relaxed→P2READYtrueRelease；右R1讀trueAcquire→R2DATA230Relaxed；藍跨線明寫R1讀自P2才同步，底P1P2R1R2。不把早先false讀也同步，兩欄是同位置非各一份。|
|231 `ch13-seqcst-cycle.png`|藍A1寫X1→A2讀Y0、綠B1寫Y1→B2讀X0；灰各thread順序，紅依「都讀0」假設要求A2前B1、B2前A1而閉圈。這是反證矛盾，非實際執行軌跡。|

## 第12章真正恢復的操作規則

220：共同package／dependency設定要成員明確採用，點號true是採用不是把edition變bool；path起點依宣告manifest，workspace根profile與package／edition分開。rust-version宣最低需求而非安裝工具鏈。features可加能力；attribute cfg排除item／block，cfg!只給bool，普通if false仍查名字。host／target平台與Cargo建置target不能混，all-targets不是所有OS。完整兩member小workspace、tests及答案已讀，沒有以release相同數字稱效能改善。

221：build script先host編譯執行、stdout協定交下階段；env::var此時runtime，NotPresent與NotUnicode不同，LESSON_MODE有限驗證後才rustc-env。script cfg看host，CARGO_CFG_TARGET_OS看主要crate。env!/option_env!在編譯期嵌字串，直接成品runtime改環境不重寫內容；optional缺值None但非Unicode仍錯。rerun追蹤是輸入變化／可快取，不保每次重跑。OUT_DIR及build-dependencies有角色，但原生linker／ABI僅預告，本頁未用，不冒稱會實作。

222：root錯誤無法開始、link等政策略過、真正逐項問題三者分開。WalkDir Item本身Result，錯誤保留後可繼續，目錄打不開就不知其內。兩個link控制看不同位置；metadata與symlink_metadata、PathBuf owner與實體檔案、entry舊資訊與observe當前資料都分開。Snapshot僅len/modified觀察，非一致快照／鎖。Option map零或一份、原生Path Borrow/Hash/Ord身分及非Unicode保問題政策可對上完整source和八測。

223：大小與摘要都只是候選，真正bytes確認才是本次內容判準；hash digest／Hash值和Hash trait不混。短讀需填相同比較段或EOF，IO Err不能false；Interrupted重試。真檔案前／後按path與handle觀察，失敗保精確側path，未見差異仍非一致快照。Tuple size+digest候選鍵、hash_candidates首錯不回partial map；refine要逐各組代表，singleton也完整驗讀、同原生path去重。A/B/B人為同鍵分1/2兩組，非找到真collision。重複量按path、不等可回收空間，hard link仍兩名字；TryFrom和checked算術給具體失敗。四個階段全source與全部tests／同頁答案讀完，完整CLI／繼續政策明列後續整合未讀。

## 第13章真正恢復的操作規則

224：thread一條執行路徑，concurrency重疊與parallelism同時不同。Send marker／auto trait查值移交，Sync等價&T Send查shared borrow；不取代來源有效期。Cell／RefCell owner可Send而非Sync，Rc計數連移交都不Send。move capture不使Rc變Send、不延長reference；Fn呼叫能力另軸。此頁只有bound helpers，沒有實際thread。

225：spawn工作F與成果T各Send/static，JoinHandle是一次owned等待權限；即刻join不免bound。join blocking回threadResult，unwind panic帶Any+Send payload，abort不走Err。業務Result可正常回Err，與外threadErr不同。panic文字可能String／staticstr，owned downcast失敗保原Box再試，未知不假讀；stderr診斷與main stdout分開。普通handle drop是detach、不等不取消，兩工作都先join再判斷。

226：外層a只承諾一段caller借用，for<'a> HRTB使F接受helper自己之後選的每次合法借用；for非執行迴圈、不延長source。固定T在for外，可借外text而不可把局部number帶走；capture與Send/static、FnOnce/Mut/Fn各自獨立。projection把每次output明連同inputa，選哪欄由body。常見省略形式不等最早外層a。我的熟練界線仍是本頁這些案例，非任意複雜lifetime皆會推。

227：管理callback在當前thread，context.spawn才建worker；env覆scope、HRTB讓context借用由scope安排，管理F/T和workerF/T不同。scope callback返回後還自動等body；**本頁明列Rust1.99自動等待body，thread-local清理可能仍在，手動join才等完整清理**，未概括成一樣。drop scoped handle不取消也不立即等、scope仍追蹤。external view可回，scope-bound handle不能逃；scope回後仍有外借結果便仍需等最後借用。手動join panic自行處理，未處理由scope等全body後panic；等待不rollback修改。

228：channel是端點與訊息協定不建thread，ownedsend／recv各有T權限，失敗SendError.0才是回收owner。全部sender含main原端與clones都Drop後才不再新增，仍drain buffer再Err、再join。各producer順序保留，跨producer交錯未定。Sender/SyncSender clone不需T Clone；跨thread才查T Send，Receiver非Sync非Clone。bounded容量管buffer非總memory、先收再join避免等空位圈。容量0rendezvous確保已收不等已處理；drop Receiver只在producer再次send時協作停止，helper只還首次失敗值、不保存全部後續工作。

229：Mutex guard取得／Drop解鎖不同於Arc保活及worker完成；lock占用blocking不是poison Err，poison Err仍有guard。Arc原子計數不替內容同步，一般Arc內T需Send+Sync；Mutex內T只需Send可得Sync，因此Mutex<Cell>不同Arc<Cell>、Rc<Mutex>仍外Rc不Send。guard非Send、持鎖join可死鎖。poison是檢查提醒，不rollback／不證全部正確；into_inner拿guard不清標、clear不修資料，依本例Totals invariant重設再clear，而非萬用復原政策。

230：RwLock多read或單write，ownedguards皆非Send。try WouldBlock沒guard，Poisoned已有對應guard；readpanic不poison，writerpanic才可能。RwLock Sync內T需Send+Sync，不能沿Mutex<Cell>替換。read到write要釋放重取並在write下重查，downgrade是atomic write→read非upgrade。writerpanic後read錯誤內是readguard；本例完整追加可逐項確認保留，與Totals重設政策不同。平台不保fairness／不starve；所有相關路徑一致兩鎖順序只排該反序圈，不保無其他死鎖。

231：AtomicUsize非Copy、load值Copy，store替換、fetch_add回舊值並把讀改寫合一，wrapping不同普通檢查。Relaxed保單位置修改序，不發布別處也不保load每次最新；兩join完成後無其他寫才沿同步讀最終5。Release/Acquire需真讀自同atomic那次寫，唯一一次DATA/READY協定串P1→P2→R1→R2才保230；晚join不能倒替先前讀取解釋。yield不提供同步／期限進展。五Ordering有方法合法組合，SeqCst共同序排有限雙旗標(0,0)，卻仍不把split load/store變RMW。兩payload一次發布理由不適用後續反覆改欄。未用測試跑通替代記憶體模型理由，未稱已會任意lock-free設計。

## 留下的未知與本報告的用途

這些是實際已讀到、可用當前契約作小變化預測的普通前文。沒有把完整頁中較後source／答案倒充前面第一使用時已懂，也沒有做全書或章節 blanket pass。每概念的當時explained／inference／deferred及限制在records可查。

仍未知：本機依賴／平台／FS／排程的實際結果，強一致檔案快照、原生ABI整合、複雜HRTB熟練度、平台鎖優先策略、應用效能、複雜混合Ordering／多次發布／其他atomic API。未用外部或未釋出內容補答案。220–231原文沒有必要時，預告保持deferred；之後若真正需要仍要再讀。

已在明確STOP保存，等待root正式釋出第14 staged正文。
