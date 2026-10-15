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
