#include "counts.h"

// --8<-- [start:set-value]
void set_value(int32_t *target, int32_t value) {
    *target = value;
}
// --8<-- [end:set-value]

// --8<-- [start:sum]
int32_t sum_values(const int32_t *values, size_t len) {
    int32_t total = 0;
    for (size_t index = 0; index < len; index += 1) {
        total += values[index];
    }
    return total;
}
// --8<-- [end:sum]

// --8<-- [start:buffer]
size_t count_nonzero(const uint8_t *bytes, size_t len) {
    size_t count = 0;
    for (size_t index = 0; index < len; index += 1) {
        if (bytes[index] != 0) {
            count += 1;
        }
    }
    return count;
}
// --8<-- [end:buffer]
