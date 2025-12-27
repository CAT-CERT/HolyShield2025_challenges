#ifndef KEYDERIV_H
#define KEYDERIV_H

#include <cstdint>
#include <cstddef>

extern "C" {
    void __cdecl kd_derive_key(const uint8_t* salt8, uint8_t* out_buf, size_t out_len);
}

#endif
