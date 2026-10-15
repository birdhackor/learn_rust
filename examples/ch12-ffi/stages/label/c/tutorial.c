#include <string.h>

#include "tutorial.h"

// --8<-- [start:version]
uint32_t tutorial_version(void) {
    return 1;
}
// --8<-- [end:version]

// --8<-- [start:counts]
struct Counts tutorial_count_bytes(const uint8_t *data, uint32_t len) {
    struct Counts result = {0, 0};
    for (uint32_t index = 0; index < len; index += 1) {
        if (data[index] == 0) {
            result.zeros += 1;
        } else {
            result.nonzero += 1;
        }
    }
    return result;
}
// --8<-- [end:counts]

// --8<-- [start:label]
int tutorial_is_rust(const char *word) {
    return strcmp(word, "Rust") == 0;
}
// --8<-- [end:label]
