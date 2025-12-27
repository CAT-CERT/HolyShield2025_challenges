#include "keyderiv.h"
#include <cstring>
#include <cstdint>

static uint32_t myhash32(const uint8_t* data, size_t len) {
    uint32_t h = 0x811C9DC5u;
    for (size_t i = 0; i < len; ++i) {
        h ^= data[i];
        h *= 0x01000193u;
        h ^= (h >> 16);
    }
    return h;
}

static const uint8_t enc_master[] = {
    0x51,0x14,0x38,0x88,0xF7,0xD3,0x5E,0x93,
    0x31,0x6E,0xD4,0xE6,0x7C,0x99,0x3B,0x1C
};
static const size_t enc_master_len = sizeof(enc_master);
static const uint8_t MASTER_MASK = 0x5A;

static inline uint8_t rol8(uint8_t v, unsigned r) {
    r &= 7u;
    return (uint8_t)((v << r) | (v >> (8 - r)));
}
static inline uint8_t ror8(uint8_t v, unsigned r) {
    r &= 7u;
    return (uint8_t)((v >> r) | (v << (8 - r)));
}

static void reconstruct_master(uint8_t* out, size_t out_len) {
    for (size_t i = 0; i < out_len; ++i) {
        uint8_t b_enc = enc_master[i % enc_master_len];
        uint8_t tmp = rol8(b_enc, 3);
        out[i] = tmp ^ MASTER_MASK;
    }
}

static void zero_mem(uint8_t* p, size_t n) {
    volatile uint8_t* vp = (volatile uint8_t*)p;
    while (n--) *vp++ = 0;
}

extern "C" void __cdecl kd_derive_key(const uint8_t* salt8, uint8_t* out_buf, size_t out_len) {
    if (!out_buf || out_len == 0) return;
    uint8_t master[64];
    size_t master_len = enc_master_len;
    if (master_len > sizeof(master)) master_len = sizeof(master);
    reconstruct_master(master, master_len);

    uint8_t counter = 0;
    for (size_t pos = 0; pos < out_len; ++pos) {
        uint8_t tmp[128];
        size_t tlen = 0;

        size_t copy = master_len;
        if (copy > 100) copy = 100;
        memcpy(tmp + tlen, master, copy); tlen += copy;
        if (salt8) { memcpy(tmp + tlen, salt8, 8); tlen += 8; }
        tmp[tlen++] = counter;
        uint32_t h = myhash32(tmp, tlen);
        out_buf[pos] = (uint8_t)(h & 0xFFu);
        counter++;
    }

    zero_mem(master, master_len);
}