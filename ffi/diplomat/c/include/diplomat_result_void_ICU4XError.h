#ifndef diplomat_result_void_ICU4XError_H
#define diplomat_result_void_ICU4XError_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "ICU4XError.h"
typedef struct diplomat_result_void_ICU4XError {
    union {
        ICU4XError err;
    };
    bool is_ok;
} diplomat_result_void_ICU4XError;
#ifdef __cplusplus
}
#endif
#endif
