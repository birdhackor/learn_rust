#include <stdint.h>

// --8<-- [start:version]
uint32_t tutorial_version(void);
// --8<-- [end:version]

// --8<-- [start:counts]
struct Counts {
    uint32_t zeros;
    uint32_t nonzero;
};

// len > 0：data 指向同一 array 內可讀、已初始化的 len 個 uint8_t。
// len == 0：不讀 data，也不做 pointer arithmetic。
// 同步只讀，不保存、寫入或釋放 data；counter 各不超過 len。
struct Counts tutorial_count_bytes(const uint8_t *data, uint32_t len);
// --8<-- [end:counts]

// --8<-- [start:label]
// word 是有效的 NUL 結尾 C string，含終止零的範圍可讀且仍存活。
// 同步只讀，不保存或釋放 word；完全等於 Rust 回 1，否則回 0。
int tutorial_is_rust(const char *word);
// --8<-- [end:label]
