FROM ubuntu:22.04
ENV DEBIAN_FRONTEND=noninteractive

RUN apt update && apt install -y socat ncat ffmpeg v4l-utils netcat curl && rm -rf /var/lib/apt/lists/*

RUN adduser --disabled-password --gecos '' --uid 1337 camera && usermod -aG video camera

WORKDIR /home/app

COPY ./func.js ./ 
COPY ./login.html ./ 
COPY ./server ./ 
COPY ./setlang.sh ./

RUN chmod 755 ./server ./setlang.sh
RUN chmod 644 ./func.js ./login.html
RUN chown -R camera:camera /home/app

USER root
CMD ["sh", "-c", "chmod a+rw /dev/video0 && su camera -c './server'"]

EXPOSE 8080