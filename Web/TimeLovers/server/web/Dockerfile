FROM openjdk:17.0.1-jdk-slim

COPY app /app

WORKDIR /app

COPY data /data

RUN apt-get update \
    && apt-get install -y iputils-ping

RUN echo "HolyShield{SyMbOlIC_1INK_bE7TEr_ThAN_5tar_l1NK}" > /flag && \
    chmod 755 /flag
    
RUN adduser timelovers
RUN mkdir /app/resources \
    && chown timelovers:timelovers /app
    
RUN ln -s /data/gamefile /app/resources/gamefile && \
        ln -s /data/images /app/resources/images && \
        ln -s /data/users /app/resources/users

RUN chmod 1733 /data/users
RUN chmod 755 /data/key/gen_key \
    && /data/key/gen_key

RUN chmod 755 /jwt_secret

USER timelovers
EXPOSE 9000
CMD ["java","-jar","TimeLovers-1.0-SNAPSHOT.jar"]