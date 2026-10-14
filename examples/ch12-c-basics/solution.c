#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

#include "counts.h"

int main(void) {
    int32_t values[3] = {80, 120, 40};
    size_t len = sizeof values / sizeof values[0];
    int32_t *owned = malloc(len * sizeof *owned);
    if (owned == NULL) {
        return EXIT_FAILURE;
    }
    for (size_t index = 0; index < len; index += 1) {
        owned[index] = values[index];
    }

    set_value(owned, 100);
    int32_t original_total = sum_values(values, len);
    int32_t updated_total = sum_values(owned, len);
    free(owned);

    if (original_total != 240) {
        return EXIT_FAILURE;
    }
    if (updated_total != 260) {
        return EXIT_FAILURE;
    }
    printf("original=%" PRId32 "; updated=%" PRId32 "\n",
           original_total, updated_total);
    return EXIT_SUCCESS;
}
