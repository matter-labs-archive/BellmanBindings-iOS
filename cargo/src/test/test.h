#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

VerificationResult test_verify(const char *file_with_vk,
                               const uint8_t *inputs_array,
                               uintptr_t inputs_array_size,
                               EngineType engine,
                               const uint8_t *proof_vec,
                               uintptr_t proof_vec_size);