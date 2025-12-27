#include "crypto_core.h"
#include <vector>
#include <string>
#include <cstdint>
#include <windows.h>

struct ObfStr { const unsigned char* data; size_t len; unsigned char mask; };
static inline std::string unobf(const ObfStr& s) {
	std::string tmp; tmp.resize(s.len);
	for (size_t i = 0; i < s.len; ++i) tmp[i] = (char)(s.data[i] ^ (unsigned char)((s.mask + i) & 0xFF));
	return tmp;
}

static const unsigned char dllname_enc[] = { 0xF3,0xCF,0xE8,0x88,0xA1,0xE6,0xCC,0x8E,0xF2,0xC9,0xD1,0x99,0xA0,0xC7,0xD9 };
static const ObfStr dllname_obf = { dllname_enc, sizeof(dllname_enc), 0x33 };

static const uint8_t secret_table[] = { 0x5A,0x1F,0xC3,0x77,0x9E,0x2D,0xA1,0x4B, 0x11, 0x22, 0x33, 0x44 };

static inline bool opaque_key_dep(const uint8_t* secret, size_t secret_len, uint32_t seed) {
	uint32_t acc = seed ^ 0xA5A5A5A5u;
	for (size_t i = 0; i < secret_len; i += 4) {
		uint32_t w = 0;
		for (size_t b = 0; b < 4; b++) w = (w << 8) | (uint32_t)secret[(i + b) % secret_len];
		acc = _rotl(acc ^ w, (int)(i & 31));
		acc ^= (acc << 13) | (acc >> 19);
	}
	return (acc % 3) == 0;
}

static uint8_t Sbox[256];
static uint8_t Sbox_init[256];
static uint8_t KeyBytes[256];
static size_t KeyLen = 0;
static bool initialized = false;

static inline uint8_t ror8(uint8_t v, unsigned r) {
	r &= 7u;
	return (uint8_t)((v >> r) | (v << (8 - r)));
}

extern "C" void __cdecl cc_reset_state() {
	initialized = false;
}

extern "C" void __cdecl cc_init_state(const uint8_t* key, size_t keylen) {
	if (!key || keylen == 0) return;
	KeyLen = (keylen > 256) ? 256 : keylen;
	memset(KeyBytes, 0, sizeof(KeyBytes));
	for (size_t i = 0; i < KeyLen; ++i) KeyBytes[i] = key[i];

	for (int i = 0; i < 256; ++i) Sbox[i] = (uint8_t)i;

	uint32_t j = 0;
	int state = 0; uint32_t idx = 0;
	while (true) {
		switch (state) {
		case 0:
			idx = 0;
			state = 1;
			break;
		case 1:
			if (idx >= 256) { state = 4; break; }
			{
				uint32_t i = idx;
				uint32_t k = KeyBytes[i % KeyLen];
				uint32_t add = ((i * i) >> 3) & 0xFFu;
				if (opaque_key_dep(secret_table, sizeof(secret_table), 0x1234)) {
					j = (j + Sbox[i] + k + add) & 0xFFu;
					uint8_t tmp = Sbox[i]; Sbox[i] = Sbox[j]; Sbox[j] = tmp;
				}
				else {
					j = (j + Sbox[i] + k + ((i * 7) & 0xFFu)) & 0xFFu;
					uint8_t tmp = Sbox[(i + 1) & 0xFF]; Sbox[(i + 1) & 0xFF] = Sbox[j]; Sbox[j] = tmp;
				}
			}
			idx++;
			state = 1; 
			break;
		case 4:
			memcpy(Sbox_init, Sbox, 256);
			initialized = true;
			return;
		}
	}
}

extern "C" void __cdecl cc_crypt_buffer(uint8_t* buf, size_t buflen) {
	if (!initialized || !buf || buflen == 0) return;
	uint8_t localS[256]; memcpy(localS, Sbox_init, 256);
	uint32_t i = 0, j = 0;
	for (size_t pos = 0; pos < buflen; ++pos) {
		i = (i + 1) & 0xFFu;
		j = (j + localS[i]) & 0xFFu;

		if (opaque_key_dep(secret_table, sizeof(secret_table), (uint32_t)pos)) {
			uint8_t tmp = localS[i]; localS[i] = localS[j]; localS[j] = tmp;
		}
		else {
			uint8_t tmp = localS[j]; localS[j] = localS[i]; localS[i] = tmp;
		}
		uint8_t raw = localS[(localS[i] + localS[j]) & 0xFFu];
		uint8_t out = ror8(raw, (unsigned)((i + j) & 7u));
		buf[pos] ^= out;
	}
}