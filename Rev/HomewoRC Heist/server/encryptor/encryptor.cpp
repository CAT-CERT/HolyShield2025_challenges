#include <windows.h>
#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <random>
#include <stdint.h>
#include "keyderiv.h"
#include "crypto_core.h"


using bytes = std::vector<uint8_t>;

//  유틸 / 더미 
static void noop_heavy(int x) {
	volatile int v = x;
	for (int i = 0; i < 3; ++i) { v = (v ^ (v << 5)) + 0x9e3779b9; }
	(void)v;
}

static bool opaque_predicate() {
	const uint64_t magic = 0xC0FFEE1234567890ull;
	volatile uint64_t t = magic;
	t ^= (t >> 13);
	t += 0xdeadbeef;
	return (t & 1) == 0;
}

//  파일 입출력
static bool read_file_raw(const std::string& path, bytes& out) {
	std::ifstream f(path, std::ios::binary | std::ios::ate);
	if (!f) return false;
	std::streamsize size = f.tellg();
	f.seekg(0, std::ios::beg);
	out.resize(size);
	if (!f.read((char*)out.data(), size)) return false;
	return true;
}
static bool read_file_wrapper1(const std::string& p, bytes& o) { noop_heavy(1); return read_file_raw(p, o); }
static bool read_file_wrapper2(const std::string& p, bytes& o) { if (opaque_predicate()) { noop_heavy(2); } return read_file_wrapper1(p, o); }

// write
static bool write_file_raw(const std::string& path, const bytes& data) {
	std::ofstream f(path, std::ios::binary);
	if (!f) return false;
	f.write((const char*)data.data(), data.size());
	return true;
}
static bool write_file_wrapper(const std::string& p, const bytes& d) { noop_heavy(3); return write_file_raw(p, d); }

// crypt
static uint64_t fnv1a64(const uint8_t* data, size_t len) {
	const uint64_t FNV_PRIME = 0x00000100000001B3ULL;
	uint64_t hash = 0xCBF29CE484222325ULL;
	for (size_t i = 0; i < len; ++i) {
		hash ^= (uint64_t)data[i];
		hash *= FNV_PRIME;
	}
	return hash;
}

static void derive_salt_from_data(const std::vector<uint8_t>& data, uint8_t out_salt[8]) {
	const uint8_t* ptr = data.empty() ? (const uint8_t*)"\0" : data.data();
	size_t len = data.empty() ? 1 : data.size();
	uint64_t h = fnv1a64(ptr, len);
	for (int i = 0; i < 8; ++i) out_salt[i] = (uint8_t)((h >> (8 * i)) & 0xFFu);
}

static void cc_init_state_wrapper(const uint8_t* key, size_t key_len) {
	typedef void (*cc_init_t)(const uint8_t*, size_t);
	cc_init_t tbl[2] = { &cc_init_state, &cc_init_state };
	size_t sel = (key_len % 2);
	tbl[sel](key, key_len);
	noop_heavy((int)sel);
}
static void cc_crypt_buffer_wrapper(uint8_t* buf, size_t len) {
	typedef void (*cc_crypt_t)(uint8_t*, size_t);
	cc_crypt_t fptr = &cc_crypt_buffer;
	if (len > 0 && (len & 7) == 0) { noop_heavy((int)len); }
	fptr(buf, len);
}

static void push_u32_le(bytes& out, uint32_t v) {
	out.push_back((uint8_t)(v & 0xFF));
	out.push_back((uint8_t)((v >> 8) & 0xFF));
	out.push_back((uint8_t)((v >> 16) & 0xFF));
	out.push_back((uint8_t)((v >> 24) & 0xFF));
}
static void push_bytes(bytes& out, const uint8_t* src, size_t n) {
	out.insert(out.end(), src, src + n);
}

static void build_magic_and_version(bytes& out) {
	out.insert(out.end(), { 'C','R','4','F' });
	out.push_back(0x01);
	out.push_back(0x00);
	out.push_back(0x00); out.push_back(0x00);
}
static void build_salt_field(bytes& out, const uint8_t* salt, size_t n) {
	push_bytes(out, salt, n);
}
static void build_length_field(bytes& out, uint32_t len) {
	push_u32_le(out, len);
}

static std::string derive_outfilename(const std::string& inf) {
	return inf + ".enc";
}

static void prepare_and_encrypt(bytes& data, uint8_t salt_out[8], bytes& out_blob) {
	derive_salt_from_data(data, salt_out);

	bytes key(32);
	kd_derive_key(salt_out, key.data(), key.size());

	cc_init_state_wrapper(key.data(), key.size());

	cc_crypt_buffer_wrapper(data.data(), data.size());

	out_blob.clear();
	build_magic_and_version(out_blob);
	build_salt_field(out_blob, salt_out, 8);
	build_length_field(out_blob, (uint32_t)data.size());
	out_blob.insert(out_blob.end(), data.begin(), data.end());
}

//  인자 검증
static void check_args_or_exit(int argc) {
	if (argc < 2) {
		std::cout << "Usage: encryptor.exe <infile>\n";
		exit(1);
	}
}
static std::string get_infile_from_argv(char** argv) {
	if (opaque_predicate()) { noop_heavy(7); }
	return std::string(argv[1]);
}

//  메인
int main(int argc, char** argv) {
	check_args_or_exit(argc);
	const std::string infile = get_infile_from_argv(argv);
	const std::string outfile = derive_outfilename(infile);

	bytes data;
	if (!read_file_wrapper2(infile, data)) {
		std::cerr << "Failed to read\n";
		return 4;
	}

	uint8_t salt[8];
	bytes out;
	prepare_and_encrypt(data, salt, out);

	if (!write_file_wrapper(outfile, out)) {
		std::cerr << "Failed to write\n";
		return 5;
	}

	std::cout << "Encrypted " << outfile << "\n";
	return 0;
}