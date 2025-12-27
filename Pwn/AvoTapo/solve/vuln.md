# Chall
1. FSB
    - Libc, PIE Base 구할 수 있습니다.
    - PIE Base를 통해서 fini_array와 main을 구할 수 있습니다.

2. fini_array
    - fini_array에서 실행되는 last함수를 main으로 덮어 한번 더 main을 실행시킵니다.
    - 한번 더 pwn 함수로 진입하면 8바이트를 덮을 수 있으므로 puts@got에 원가젯을 덮으면 됩니다.

3. Connect to Camera_Server
    - chall문제에는 testing 실행파일을 실행시킬 수 있는데, 이 실행파일을 실행시키고 나면 logfile.txt가 생성됩니다.
    - 따라서 testing 실행파일을 최소 한번 이상 실행시켜 logfile.txt를 보고 카메라 서버의 스트리밍 도구, 주소 등을 알아내야 합니다.
    - 이 부분은 Dockerfile이나 docker-compose.yml 파일을 보고 충분히 유추할 수도 있습니다.


# Camera Server
1. CVE-2021-4045의 취약점을 재현한 문제입니다. (https://github.com/hacefresko/CVE-2021-4045)
2. exec_and_read_json 함수에서 popen을 사용하여 명령어를 실행하고 있습니다.
3. 이때 setLanguage 함수에서 싱글 쿼터를 제대로 이스케이프 처리를 하지 않아 Command Injection 이 발생할 수 있습니다.
4. 카메라 서버에 접근하고 나면 앞선 정보를 이용해 ffmpeg와 curl을 이용하여 카메라를 녹화하고, 본인 서버로 옮길 수 있습니다.