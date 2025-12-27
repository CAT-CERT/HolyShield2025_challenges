# 빌드 방법

#### 1 단계
```bash
export USER=acetcom
export DIST=ubuntu
export TAG=latest
```

#### 2 단계
```bash
docker compose build base
```

#### 3 단계
```bash
docker compose build build mongodb webui gnb ue
```

#### 4 단계
```bash
docker compose up -d mongodb webui run gnb ue strapi frontend mailhog
```

###### 도커 버전에 따라 docker 명령어 쓰임새는 다름