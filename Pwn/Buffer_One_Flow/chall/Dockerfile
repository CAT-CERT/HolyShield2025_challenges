FROM ubuntu:24.04@sha256:4f1db91d9560cf107b5832c0761364ec64f46777aa4ec637cca3008f287c975e

RUN apt update && apt install -y socat coreutils

RUN /usr/sbin/useradd --no-create-home -u 1001 user

COPY --chown=1001:1001 flag /
COPY --chown=1001:1001 chall /home/user/chall
COPY libc.so.6 /lib/x86_64-linux-gnu/libc.so.6

USER user

RUN chmod 400 /flag
RUN chmod 750 /home/user/chall

CMD ["socat", "TCP-LISTEN:1337,reuseaddr,fork", "EXEC:stdbuf -i0 -o0 -e0 /home/user/chall"]