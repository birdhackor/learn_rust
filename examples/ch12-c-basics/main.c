#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "counts.h"

int main(void) {
    // --8<-- [start:local-array]
    int32_t values[3] = {80, 120, 40};
    size_t len = sizeof values / sizeof values[0];
    // --8<-- [end:local-array]

    // --8<-- [start:pointer]
    int32_t amount = 80;
    int32_t *pointer = &amount;
    set_value(pointer, 100);
    // --8<-- [end:pointer]

    // --8<-- [start:string-buffer]
    char word[] = "Rust";
    uint8_t bytes[3] = {10, 0, 20};
    size_t byte_len = sizeof bytes / sizeof bytes[0];
    // --8<-- [end:string-buffer]

    // --8<-- [start:allocated-array]
    int32_t *owned = malloc(len * sizeof *owned);
    if (owned == NULL) {
        return EXIT_FAILURE;
    }
    for (size_t index = 0; index < len; index += 1) {
        owned[index] = values[index];
    }
    int32_t owned_total = sum_values(owned, len);
    free(owned);
    // --8<-- [end:allocated-array]

    printf("amount=%" PRId32 "\n", amount);
    printf("local total=%" PRId32 "; empty total=%" PRId32 "\n",
           sum_values(values, len), sum_values(NULL, 0));
    printf("text=%s; text bytes=%zu; storage bytes=%zu\n",
           word, strlen(word), sizeof word);
    printf("buffer length=%zu; nonzero=%zu\n",
           byte_len, count_nonzero(bytes, byte_len));
    printf("owned total=%" PRId32 "\n", owned_total);
    return EXIT_SUCCESS;
}
