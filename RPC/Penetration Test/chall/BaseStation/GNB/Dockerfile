FROM ubuntu:20.04

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      ca-certificates curl gnupg && \
    mkdir -p /usr/share/keyrings && \
    curl -fsSL https://apt.kitware.com/keys/kitware-archive-latest.asc | \
      gpg --dearmor -o /usr/share/keyrings/kitware-archive-keyring.gpg && \
    echo "deb [signed-by=/usr/share/keyrings/kitware-archive-keyring.gpg] https://apt.kitware.com/ubuntu/ focal main" \
      > /etc/apt/sources.list.d/kitware.list && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      build-essential cmake ninja-build ccache \
      libsctp-dev lksctp-tools iproute2 net-tools \
      openssh-server && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir -p /var/run/sshd

WORKDIR /app
COPY . /app

RUN chmod +x /app/announcer.sh || true

RUN cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_C_COMPILER_LAUNCHER=ccache \
      -DCMAKE_CXX_COMPILER_LAUNCHER=ccache && \
    cmake --build build --parallel

RUN groupadd -r appread && \
    useradd -m -d /home/Albert -s /bin/bash Albert && \
    echo "Albert:1q2w3e4r!" | chpasswd && \
    usermod -a -G appread Albert


RUN chown -R root:appread /app && \
    chmod -R g-rwx,o-rwx /app && \
    find /app -type d -exec chmod g+rx {} \; && \
    find /app -type f -exec chmod g+r {} \;

RUN chown root:Albert /home/Albert && \
    chmod 550 /home/Albert

RUN for d in / /bin /sbin /lib /lib64 /usr /etc /var /opt /srv /mnt /media; do \
      if [ -d "$d" ]; then \
        chmod o-rw "$d" || true; \
        chmod o+x "$d" || true; \
      fi; \
    done

RUN if [ -d /tmp ]; then chmod 755 /tmp; fi

RUN chmod o-x /usr/bin/apt /usr/bin/apt-get /usr/bin/dpkg 2>/dev/null || true

RUN sed -i 's/^#PasswordAuthentication yes/PasswordAuthentication yes/' /etc/ssh/sshd_config || true && \
    sed -i 's/^PasswordAuthentication no/PasswordAuthentication yes/' /etc/ssh/sshd_config || true && \
    sed -i 's/^#\?PubkeyAuthentication .*/PubkeyAuthentication yes/' /etc/ssh/sshd_config || true && \
    sed -i 's/^PermitRootLogin .*/PermitRootLogin no/' /etc/ssh/sshd_config || true && \
    echo "AllowUsers Albert" >> /etc/ssh/sshd_config && \
    sed -i 's/^#\?Port .*/Port 2222/' /etc/ssh/sshd_config

EXPOSE 2222

CMD ["/bin/bash","-lc","/usr/sbin/sshd; /app/announcer.sh & exec /app/build/nr-gnb -c /app/config/open5gs-gnb.yaml"]