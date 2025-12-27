# vm_defs.py
CHUNK_SIZE = 16
INPUT_SIZE = 64
NUM_CHUNKS = 4

OP_ADD              = 0x20
OP_SUB              = 0x21
OP_XOR              = 0x22
OP_ROL              = 0x23
OP_ROR              = 0x24
OP_MUL              = 0x25
OP_END              = 0xff

OP_PUSH_INPUT_CHUNK = 0x10
OP_STORE_CHUNK      = 0x11
OP_XOR_CHUNK = 0x30
OP_ADD_CHUNK = 0x31
OP_SUB_CHUNK = 0x32
