#include <stdlib.h>

#include "counts.h"

int main(void) {
    if (sum_values(NULL, 0) != 0) {
        return EXIT_FAILURE;
    }
    if (count_nonzero(NULL, 0) != 0) {
        return EXIT_FAILURE;
    }

    int32_t values[3] = {-80, 120, -40};
    size_t len = sizeof values / sizeof values[0];
    if (sum_values(values, len) != 0) {
        return EXIT_FAILURE;
    }
    if (sum_values(values, 2) != 40) {
        return EXIT_FAILURE;
    }

    uint8_t bytes[4] = {0, 10, 0, 20};
    size_t byte_len = sizeof bytes / sizeof bytes[0];
    if (count_nonzero(bytes, byte_len) != 2) {
        return EXIT_FAILURE;
    }
    if (count_nonzero(bytes, 1) != 0) {
        return EXIT_FAILURE;
    }

    int32_t amount = 80;
    set_value(&amount, -20);
    if (amount != -20) {
        return EXIT_FAILURE;
    }
    return EXIT_SUCCESS;
}
