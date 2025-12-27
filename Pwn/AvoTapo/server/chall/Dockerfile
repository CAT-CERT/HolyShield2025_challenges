FROM ubuntu:22.04

RUN apt-get update && apt-get install -y socat curl netcat net-tools

RUN adduser -u 1337 pwn

WORKDIR /home/pwn

COPY ./prob ./prob
COPY ./testing ./testing

RUN chmod 755 ./prob
RUN chmod 755 ./testing

USER pwn

EXPOSE 9000
CMD socat TCP-LISTEN:9000,reuseaddr,fork EXEC:./prob,stderr
