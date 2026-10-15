// --8<-- [start:model]
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr;

pub struct PinnedNote {
    value: i32,
    saved: *const i32,
    _pin: PhantomPinned,
}

impl PinnedNote {
    pub fn new(value: i32) -> Pin<Box<Self>> {
        let mut pinned = Box::pin(Self {
            value,
            saved: ptr::null(),
            _pin: PhantomPinned,
        });
        // SAFETY: Box::pin 已把整份 Self 放在固定位置。這份獨佔借用只用來
        // 記下內部 i32 欄位的地址，不移出或取代 Self，不交出 &mut Self。
        // 所有欄位 private，其他公有方法與 Drop 也保持整份值的位置。
        let inner = unsafe { pinned.as_mut().get_unchecked_mut() };
        inner.saved = &raw const inner.value;
        pinned
    }

    pub fn value(self: Pin<&Self>) -> i32 {
        self.get_ref().value
    }

    pub fn address_matches(self: Pin<&Self>) -> bool {
        let inner = self.get_ref();
        ptr::eq(inner.saved, &raw const inner.value)
    }

    pub fn set_value(self: Pin<&mut Self>, value: i32) {
        // SAFETY: 只改既有 i32 欄位的內容，不移出或取代 Self，不修改 saved，
        // 也不回傳 &mut Self。這個模型沒有對任何欄位交出 Pin，
        // 沒有需移交的 owned 資源；整份值及 value 欄位位置一直保留。
        let inner = unsafe { self.get_unchecked_mut() };
        inner.value = value;
    }
}

impl Drop for PinnedNote {
    fn drop(&mut self) {
        // 即使 Drop 收到普通 &mut Self，也不移出或取代整份值。
        // 只讀正常 i32 欄位與比較地址；不透過 saved 讀取資料。
        println!(
            "清理 PinnedNote：value={}，地址仍一致={}",
            self.value,
            ptr::eq(self.saved, &raw const self.value)
        );
    }
}
// --8<-- [end:model]
