#ifndef ICU4XFixedDecimal_H
#define ICU4XFixedDecimal_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XFixedDecimal ICU4XFixedDecimal;
#include "diplomat_result_box_ICU4XFixedDecimal_ICU4XError.h"
#include "ICU4XFixedDecimalRoundingMode.h"

ICU4XFixedDecimal* ICU4XFixedDecimal_create(int32_t v);

diplomat_result_box_ICU4XFixedDecimal_ICU4XError ICU4XFixedDecimal_create_from_f64_with_max_precision(double f);

diplomat_result_box_ICU4XFixedDecimal_ICU4XError ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(double f, int16_t precision, ICU4XFixedDecimalRoundingMode rounding_mode);

diplomat_result_box_ICU4XFixedDecimal_ICU4XError ICU4XFixedDecimal_create_from_f64_with_significant_digits(double f, uint8_t digits, ICU4XFixedDecimalRoundingMode rounding_mode);

diplomat_result_box_ICU4XFixedDecimal_ICU4XError ICU4XFixedDecimal_create_fromstr(const char* v_data, size_t v_len);

bool ICU4XFixedDecimal_multiply_pow10(ICU4XFixedDecimal* self, int16_t power);

void ICU4XFixedDecimal_negate(ICU4XFixedDecimal* self);

void ICU4XFixedDecimal_pad_left(ICU4XFixedDecimal* self, int16_t position);

void ICU4XFixedDecimal_truncate_left(ICU4XFixedDecimal* self, int16_t position);

void ICU4XFixedDecimal_pad_right(ICU4XFixedDecimal* self, int16_t position);

void ICU4XFixedDecimal_to_string(const ICU4XFixedDecimal* self, DiplomatWriteable* to);
void ICU4XFixedDecimal_destroy(ICU4XFixedDecimal* self);

#ifdef __cplusplus
}
#endif
#endif
