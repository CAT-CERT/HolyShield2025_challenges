from sage.all import *
from Crypto.Util.number import *
from hashlib import *

FLAG = int.from_bytes(b'HolyShield{baby_ECDSA_using_lattice_and_a_bit_of_CRT_and_bruteforcing__}')
assert FLAG.bit_length()==575
p = 0xaadd9db8dbe9c48b3fd4e6ae33c9fc07cb308db3b3c9d20ed6639cca703308717d4d9b009bc66842aecda12ae6a380e62881ff2f2d82c68528aa6056583a48f3
a = 0x7830a3318b603b89e2327145ac234cc594cbdd8d3df91610a83441caea9863bc2ded5d5aa8253aa10a2ef1c98b9ac8b57f1117a72bf2c7b9e7c1ac4d77fc94ca
b = 0x3df91610a83441caea9863bc2ded5d5aa8253aa10a2ef1c98b9ac8b57f1117a72bf2c7b9e7c1ac4d77fc94cadc083e67984050b75ebae5dd2809bd638016f723
E = EllipticCurve(GF(p), [a, b])
n = 0xaadd9db8dbe9c48b3fd4e6ae33c9fc07cb308db3b3c9d20ed6639cca70330870553e5c414ca92619418661197fac10471db1d381085ddaddb58796829ca90069
d = FLAG%n

G = E(0x81aee4bdd82ed9645a21322e9c4c6a9385ed9f70b5d916c1b43b62eef4d0098eff3b1f78e2d0d48d50d1687b93b97d5f7c6d5047406a5e688b352209bcb9f822, 0x7dde385d566332ecc0eabfa9cf7822fdf209f70024a57b1aa000c55b881f8111b2dcde494a5f485e5bca4bd88a2763aed1ca2b2fa8f0540678cd1e0f3ad80892)

def sign(msg):
    z = bytes_to_long(sha512(long_to_bytes(msg)).digest())
    k = bytes_to_long(sha256(long_to_bytes(FLAG^msg)).digest()+sha256(long_to_bytes(msg)).digest())
    r = int((k*G).x())
    s = pow(k, -1, n)*(z+r*d)%n

    return (r, s)

msg = 0x1337
print(sign(msg))

msg = 0x13371337
print(sign(msg))

msg = 0x133713371337
print(sign(msg))

print("If you have many flag candidates, just in case:")
print(hash(FLAG))