# Oracle (RSA Coppersmith Challenge)

## 📝 문제 개요 (Description)
**Oracle**은 RSA 암호화 시스템을 기반으로 한 CTF 문제입니다.  
주어진 바이너리는 복잡한 비트 연산을 통해 RSA 모듈러 $N$을 생성하고, 특정 조건을 만족하는 Seed를 찾아 메시지의 앞부분(Prefix)을 구성합니다. 그 후 사용자가 입력한 Flag를 뒤에 붙여 암호화합니다.

플레이어는 바이너리를 리버싱하여 $N$과 Prefix 생성 로직을 파악하고, **RSA Coppersmith's Attack**을 통해 암호문으로부터 Flag를 복구해야 합니다.

## 🔍 상세 분석 (Analysis)

이 문제는 크게 **리버싱(Reversing)** 파트와 **암호학(Cryptography)** 파트로 나뉩니다.

### 1. 모듈러 N 복구 (Reversing)
`ClockworkEngine::ignite_steam_engine()` 함수는 하드코딩된 `u64` 배열을 `Vec<u64>`로 변환한 뒤, 복잡한 비트 연산(Rotate Right, XOR)을 수행하여 하나의 거대한 정수 $N$을 만듭니다.
*   **핵심:** 이 로직은 고정되어 있으므로, 파이썬이나 SageMath로 똑같이 구현하면 공개키 $N$을 얻을 수 있습니다.

### 2. Prefix 및 Seed 복구 (Brute-Force & Reversing)
프로그램은 실행 시 내부적으로 올바른 `TimeSeed`를 찾습니다.
*   **조건:** `SHA256(seed + "OrAcLe")`의 결과값 중 상위 3바이트가 `000000`이어야 합니다.
*   **Prefix 생성:** 찾아낸 Seed를 이용해 88바이트의 난수열(Prefix)을 생성합니다.
*   **구조:** 평문 $m$은 다음과 같이 구성됩니다.
    $$m = \text{Prefix (88 bytes)} \parallel \text{Flag (Unknown)}$$

### 3. 암호화 (RSA)
*   **공개키:** $N$ (생성됨), $e = 3$ (고정, 매우 작은 값)
*   **암호화:** $C \equiv m^e \pmod N$

---

## 💥 취약점 (Vulnerability)

이 문제는 **RSA Stereotyped Message Attack (Coppersmith's Attack)**에 취약합니다.

1.  **작은 공개 지수 ($e=3$):** $e$ 값이 매우 작습니다.
2.  **평문의 형태:** 평문의 대부분(상위 88바이트)을 공격자가 알고 있습니다(Prefix). 우리가 모르는 값(Flag)은 약 30~40바이트로, 전체 모듈러 $N$(1024비트)에 비해 매우 작습니다.

### 수학적 원리
우리는 평문 $m$을 다음과 같은 다항식 형태로 표현할 수 있습니다.
$$f(x) = (\text{Prefix} \times 2^{\text{shift}} + x)^3 - C \equiv 0 \pmod N$$

여기서 $x$는 우리가 찾고자 하는 **Flag**입니다. $x$가 $N^{1/e}$ 보다 작다면, **LLL 알고리즘**을 이용한 Coppersmith 정리로 $x$를 효율적으로 찾아낼 수 있습니다.

---

## 🚀 풀이 방법 (How to Solve)

제공된 `solve.sage` 스크립트는 다음 단계를 자동으로 수행합니다.

### 1. 환경 설정
SageMath가 설치된 환경이 필요합니다.
```bash
# SageMath 설치 (Ubuntu 예시)
sudo apt update
sudo apt install sagemath
```

### 2. 문제 실행 및 암호문 획득
먼저 Rust로 작성된 문제 파일을 실행하거나, 서버에 접속하여 임의의 문자열을 입력합니다. (입력값은 실제 Flag가 아니어도 암호문 생성 과정을 확인하기 위함이나, 실제 문제 환경에서는 서버가 암호문을 뱉어냅니다.)

```bash
# Rust 프로젝트 실행 (Flag가 파일 내부에 있거나 서버 환경일 경우)
cargo run
# 출력된 16진수 암호문(C)을 복사합니다.
```

### 3. Exploit 실행
`solve.sage`를 실행하고 복사한 암호문을 입력합니다.

```bash
sage solve.sage
```

**스크립트 내부 동작:**
1.  Rust 코드의 비트 연산을 파이썬으로 재구현하여 $N$을 계산합니다.
2.  SHA256 브루트포스를 수행하여 `Seed`를 찾고 `Prefix`를 생성합니다.
3.  $f(x) = (\text{Prefix} \cdot 2^{k} + x)^3 - C$ 다항식을 정의합니다.
4.  `f.small_roots()` 함수를 호출하여 방정식의 해(Flag)를 찾습니다.

---

## 🚩 Flag
```
HolyShield{f1nding_sm4ll_r00ts_w1th_LLL}
```
