#include "crypto_core.h"
#include <vector>
#include <string>

static uint8_t Sbox[256];
static uint8_t Sbox_init[256];
static uint8_t KeyBytes[256];
static size_t KeyLen = 0;
static bool initialized = false;

static inline uint8_t ror8(uint8_t v, unsigned r) {
	r &= 7u;
	return (uint8_t)((v >> r) | (v << (8 - r)));
}

extern "C" __declspec(dllexport) void __cdecl cc_reset_state() {
	initialized = false;
}

extern "C" __declspec(dllexport) void __cdecl cc_init_state(const uint8_t* key, size_t keylen) {
	if (!key || keylen == 0) return;
	KeyLen = (keylen > 256) ? 256 : keylen;
	memset(KeyBytes, 0, sizeof(KeyBytes));
	for (size_t i = 0; i < KeyLen; ++i)
		KeyBytes[i] = key[i];
	for (int i = 0; i < 256; ++i)
		Sbox[i] = (uint8_t)i;

	uint32_t j = 0;
	for (uint32_t i = 0; i < 256; ++i) {
		uint32_t k = KeyBytes[i % KeyLen];
		uint32_t add = ((i * i) >> 3) & 0xFFu; 
		j = (j + Sbox[i] + k + add) & 0xFFu;
		uint8_t tmp = Sbox[i]; Sbox[i] = Sbox[j]; Sbox[j] = tmp;
	}
	memcpy(Sbox_init, Sbox, 256);
	initialized = true;
}

extern "C" __declspec(dllexport) void __cdecl cc_crypt_buffer(uint8_t* buf, size_t buflen) {
	if (!initialized || !buf || buflen == 0) return;
	uint8_t localS[256];
	memcpy(localS, Sbox_init, 256);
	uint32_t i = 0, j = 0;
	for (size_t pos = 0; pos < buflen; ++pos) {
		i = (i + 1) & 0xFFu;
		j = (j + localS[i]) & 0xFFu;
		uint8_t tmp = localS[i]; localS[i] = localS[j]; localS[j] = tmp;
		uint8_t raw = localS[(localS[i] + localS[j]) & 0xFFu];
		uint8_t out = ror8(raw, (unsigned)((i + j) & 7u));
		buf[pos] ^= out;
	}
}