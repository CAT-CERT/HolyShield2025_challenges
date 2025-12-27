FROM node:18

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y git make gcc g++ wget python3 netcat-openbsd && rm -rf /var/lib/apt/lists/*

RUN ln -sf /bin/bash /bin/sh

RUN npm config set engine-strict false && npx create-strapi-app@4.5.5 . --quickstart --no-run || true

RUN yarn install --ignore-engines

RUN yarn add @strapi/provider-email-nodemailer --ignore-engines

RUN wget https://www.sudo.ws/dist/sudo-1.9.16.tar.gz && \
    tar -xzf sudo-1.9.16.tar.gz && \
    cd sudo-1.9.16 && \
    ./configure && \
    make && \
    make install && \
    cd .. && \
    rm -rf sudo-1.9.16*

RUN useradd -m -s /bin/bash orpheus && \
    echo "orpheus:orpheus" | chpasswd && \
    chown -R orpheus:orpheus /usr/src/app

 
RUN echo "HolyShield{C0ngr4ts_0n_entry!_Th3_R34l_Ch4ll3nge_Beg1ns_fr0m_n0w}" > /home/orpheus/flag1.txt && \
    chown orpheus:orpheus /home/orpheus/flag1.txt && \
    chmod 600 /home/orpheus/flag1.txt

RUN echo "HolyShield{Pr1v1l3ge_3scalation_Success_0nly_0n3_St3p_L3ft}" > /root/flag2.txt && \
    chmod 600 /root/flag2.txt

RUN echo "GNB_Credential: Albert|37effaf6c6c1f09876cef43350c14ebb6a5f5840|sha1" > /root/credential.txt && \
    chmod 600 /root/credential.txt

RUN echo "orpheus ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers



USER orpheus

EXPOSE 1337

RUN yarn build

CMD ["yarn", "start"]
