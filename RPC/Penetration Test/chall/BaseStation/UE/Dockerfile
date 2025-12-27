# syntax=docker/dockerfile:1.4
FROM ubuntu:20.04

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      ca-certificates curl gnupg python3 python3-pip && \
    mkdir -p /usr/share/keyrings && \
    curl -fsSL https://apt.kitware.com/keys/kitware-archive-latest.asc | \
      gpg --dearmor -o /usr/share/keyrings/kitware-archive-keyring.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/kitware-archive-keyring.gpg] https://apt.kitware.com/ubuntu/ focal main" \
      > /etc/apt/sources.list.d/kitware.list && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      build-essential cmake ninja-build ccache libsctp-dev lksctp-tools iproute2 net-tools iputils-ping && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . /app

RUN python3 -m pip install --no-cache-dir -r /app/requirements.txt
RUN cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_C_COMPILER_LAUNCHER=ccache \
      -DCMAKE_CXX_COMPILER_LAUNCHER=ccache && \
    cmake --build build --parallel

# run request handler once, restart nr-ue every 30s
RUN chmod +x /app/entrypoint.sh
CMD ["/bin/bash","-lc","/app/entrypoint.sh"]
