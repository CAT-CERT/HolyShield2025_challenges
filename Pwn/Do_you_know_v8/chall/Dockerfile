FROM ubuntu:22.04@sha256:09506232a8004baa32c47d68f1e5c307d648fdd59f5e7eaa42aaf87914100db3

RUN sed -i 's|http://archive.ubuntu.com|http://mirror.kakao.com|g' /etc/apt/sources.list && \
    sed -i 's|http://security.ubuntu.com|http://mirror.kakao.com|g' /etc/apt/sources.list

RUN apt-get update && \
    apt-get install -y \
    socat \
    vim \
    python3 \
    netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash ctf
RUN useradd -M -s /bin/false pwned

WORKDIR /home/ctf

COPY ./flag ./flag
COPY ./flag_reader ./flag_reader
COPY ./deploy/runner/run.sh ./run.sh
COPY ./deploy/runner/runner.py ./runner.py
COPY ./deploy/out ./out

USER root

RUN chown pwned:pwned ./flag
RUN chown pwned:ctf ./flag_reader

RUN chmod 400 ./flag && \
    chmod 4550 ./flag_reader && \
    chmod 755 ./run.sh && \
    chmod 755 ./runner.py && \
    chmod 755 ./out

RUN HASH=$(md5sum ./flag_reader | awk '{print $1}') && \
    mv ./flag_reader ./flag_reader-${HASH}

USER ctf

EXPOSE 8080

CMD ["socat", "TCP-LISTEN:8080,reuseaddr,fork", "EXEC:/home/ctf/run.sh,pty,stderr,setsid,sigint,sane"]