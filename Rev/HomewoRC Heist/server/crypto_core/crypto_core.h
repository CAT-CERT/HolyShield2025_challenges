#ifndef CRYPTO_CORE_H
#define CRYPTO_CORE_H

#include <cstdint>
#include <cstddef>

extern "C" {
    void __cdecl cc_init_state(const uint8_t* key, size_t keylen);
    void __cdecl cc_crypt_buffer(uint8_t* buf, size_t buflen);
    void __cdecl cc_reset_state();
}

#endif
