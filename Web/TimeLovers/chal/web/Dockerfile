FROM openjdk:17.0.1-jdk-slim

COPY app /app

WORKDIR /app

COPY data /data

RUN apt-get update \
    && apt-get install -y iputils-ping

RUN echo "HolyShield{flag}" > /flag && \
    chmod 755 /flag
    
RUN adduser timelovers
RUN mkdir /app/resources \
    && chown timelovers:timelovers /app
    
RUN ln -s /data/gamefile /app/resources/gamefile && \
        ln -s /data/images /app/resources/images && \
        ln -s /data/users /app/resources/users

RUN chmod 1733 /data/users /data/users
RUN chmod 755 /data/key/gen_key 
RUN /data/key/gen_key || echo -n "6c5d95fcc469944ccac5e8f0cb07d0ee2aeee061e3c46f3c049dae147b227de7" > /jwt_secret

RUN chmod 755 /jwt_secret

USER timelovers
EXPOSE 9000
CMD ["java","-jar","TimeLovers-1.0-SNAPSHOT.jar"]