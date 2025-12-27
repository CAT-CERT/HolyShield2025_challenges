FROM node:19

MAINTAINER Sukchan Lee <acetcom@gmail.com>

COPY webui /usr/src/open5gs/webui

WORKDIR /usr/src/open5gs/webui
RUN npm clean-install && \
    npm run build

ADD https://github.com/ufoscout/docker-compose-wait/releases/download/2.9.0/wait /wait
RUN chmod +x /wait

RUN apt-get update && \
    apt-get install -y curl && \
    rm -rf /var/lib/apt/lists/*

COPY docker/webui/curl.sh /usr/local/bin/seed-subscriber.sh
RUN chmod +x /usr/local/bin/seed-subscriber.sh

COPY docker/webui/entrypoint.sh /usr/local/bin/webui-entrypoint.sh
RUN chmod +x /usr/local/bin/webui-entrypoint.sh

ENTRYPOINT ["/usr/local/bin/webui-entrypoint.sh"]

EXPOSE 9999
