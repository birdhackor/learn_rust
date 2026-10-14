#include <stddef.h>
#include <stdlib.h>
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

// --8<-- [start:greeting]
char *tutorial_greeting(void) {
    char word[] = "Rust";
    char *data = malloc(sizeof word);
    if (data == NULL) {
        return NULL;
    }
    for (size_t index = 0; index < sizeof word; index += 1) {
        data[index] = word[index];
    }
    return data;
}

void tutorial_free_greeting(char *data) {
    free(data);
}
// --8<-- [end:greeting]
