#ifndef ICU4XError_H
#define ICU4XError_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XError {
  ICU4XError_UnknownError = 0,
  ICU4XError_WriteableError = 1,
  ICU4XError_OutOfBoundsError = 2,
  ICU4XError_DataMissingResourceKeyError = 256,
  ICU4XError_DataMissingVariantError = 257,
  ICU4XError_DataMissingLocaleError = 258,
  ICU4XError_DataMissingResourceOptionsError = 259,
  ICU4XError_DataNeedsVariantError = 260,
  ICU4XError_DataNeedsLocaleError = 261,
  ICU4XError_DataExtraneousResourceOptionsError = 262,
  ICU4XError_DataFilteredResourceError = 263,
  ICU4XError_DataMismatchedTypeError = 264,
  ICU4XError_DataMissingPayloadError = 265,
  ICU4XError_DataInvalidStateError = 266,
  ICU4XError_DataCustomError = 267,
  ICU4XError_DataIoError = 268,
  ICU4XError_DataUnavailableBufferFormatError = 269,
  ICU4XError_LocaleUndefinedSubtagError = 512,
  ICU4XError_LocaleParserError = 513,
  ICU4XError_DataStructValidityError = 768,
  ICU4XError_PropertyUnknownScriptIdError = 1024,
  ICU4XError_PropertyUnknownGeneralCategoryGroupError = 41,
  ICU4XError_DecimalLimitError = 1280,
  ICU4XError_DecimalSyntaxError = 1281,
  ICU4XError_PluralParserError = 1536,
} ICU4XError;

void ICU4XError_destroy(ICU4XError* self);

#ifdef __cplusplus
}
#endif
#endif
