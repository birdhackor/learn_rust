#include <stddef.h>
#include <stdint.h>

// target 必須指向仍有效、aligned、可寫的一個 int32_t。
void set_value(int32_t *target, int32_t value);

// len > 0：values 指向同一 array 內連續、有效、aligned、已初始化的 len 項。
// 呼叫期間只讀、沒有衝突寫入；每個加總中間結果都須可由 int32_t 表示。
// len == 0：允許 NULL，不存取元素或做 pointer arithmetic。
// 兩個 helper 都不保存 pointer，也不接手或釋放來源。
int32_t sum_values(const int32_t *values, size_t len);

// len > 0：bytes 指向同一 array 內連續、有效、aligned、已初始化的 len 項。
// 呼叫期間只讀，沒有衝突寫入。
// len == 0：允許 NULL。零值也在處理範圍內，並不終止掃描。
size_t count_nonzero(const uint8_t *bytes, size_t len);
