MAGIC_1, MAGIC_2, MAGIC_3, MAGIC_4 = 0x61707865, 0x3320646e, 0x79622d32, 0x6b206574
SHARD_ALPHA = bytes([0x48,0x6f,0x6c,0x79,0x53,0x68,0x69,0x65])
SHARD_BETA  = bytes([0x6c,0x64,0x5f,0x32,0x30,0x32,0x35,0x5f])
SHARD_GAMMA = bytes([0x43,0x54,0x46,0x5f,0x4b,0x65,0x79,0x21])
SHARD_DELTA = bytes([0x52,0x65,0x76,0x33,0x72,0x73,0x33,0x00])
IV_FRONT = bytes([0x4e,0x6f,0x6e,0x63,0x65,0x31])
IV_BACK  = bytes([0x32,0x30,0x32,0x35,0x21,0x00])

def rotl32(x,n): return ((x<<n)|(x>>(32-n))) & 0xffffffff
def qr(s,a,b,c,d):
    s[a]=(s[a]+s[b])&0xffffffff; s[d]^=s[a]; s[d]=rotl32(s[d],16)
    s[c]=(s[c]+s[d])&0xffffffff; s[b]^=s[c]; s[b]=rotl32(s[b],12)
    s[a]=(s[a]+s[b])&0xffffffff; s[d]^=s[a]; s[d]=rotl32(s[d],8)
    s[c]=(s[c]+s[d])&0xffffffff; s[b]^=s[c]; s[b]=rotl32(s[b],7)

def chacha20_block(key, nonce, counter):
    s=[0]*16
    s[0],s[1],s[2],s[3]=MAGIC_1,MAGIC_2,MAGIC_3,MAGIC_4
    for i in range(8): s[4+i]=int.from_bytes(key[i*4:(i+1)*4],'little')
    s[12]=counter
    for i in range(3): s[13+i]=int.from_bytes(nonce[i*4:(i+1)*4],'little')
    w=s[:]
    for _ in range(10):
        qr(w,0,4,8,12); qr(w,1,5,9,13); qr(w,2,6,10,14); qr(w,3,7,11,15)
        qr(w,0,5,10,15); qr(w,1,6,11,12); qr(w,2,7,8,13); qr(w,3,4,9,14)
    for i in range(16): w[i]=(w[i]+s[i])&0xffffffff
    out=b''.join(x.to_bytes(4,'little') for x in w)
    return out

def keystream_xor(data, key, nonce):
    res=bytearray(data); ctr=0; off=0
    while off<len(res):
        ks=chacha20_block(key, nonce, ctr)
        n=min(64, len(res)-off)
        for i in range(n): res[off+i]^=ks[i]
        ctr+=1; off+=n
    return bytes(res)

def derive_key():
    key=bytearray(SHARD_ALPHA+SHARD_BETA+SHARD_GAMMA+SHARD_DELTA)
    for i in range(32):
        key[i]^=key[(i+7)%32]
    for i in range(15,-1,-1):
        key[i],key[31-i]=key[31-i],key[i]
    for i in range(32):
        key[i]=(key[i]+i)%256
    return bytes(key)

def derive_nonce():
    n=bytearray(IV_FRONT+IV_BACK)
    for i in range(12): n[i]=(n[i]*3+7)%256
    return bytes(n)

given_hex="8cdd3b5ed03fe035e7695dcb92dab920abafae1b41a6846d4a7c37e7eafa52d6c9dd2b7f2c9bb3267f11ee7fd3a7e7b0"
cipher = bytes.fromhex(given_hex)          
key, nonce = derive_key(), derive_nonce()  
plain = keystream_xor(cipher, key, nonce)
print(plain.decode())
