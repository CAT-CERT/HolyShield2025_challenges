## 존재 취약점
1. CVE-2024-38819
2. command injection

## 풀이
1. CVE-2024-38819 이용해 JWT 생성에 사용하는 gen_key 파일 획득
2. 획득한 gen_key 파일 분석하여 JWT SECRET 획득 후 JWT 값 조작
3. 관리자 페이지 안 커멘드 인젝션으로 서버 /[anything] 로 향하는 symbolic link 생성 
4. 생성된 symbolic link 이용하여 flag 접근

## 핵심
1. CVE-2024-38819 이용한 gen_key 파일 획득
2. gen_key 파일 언패킹, 분석 후 jwt 생성
3. 관리자 페이지에서 command injection으로 symbolic link 생성