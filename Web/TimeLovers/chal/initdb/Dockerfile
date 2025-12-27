FROM mysql:8.0

ENV MYSQL_DATABASE=TL
ENV MYSQL_USER=timelovers
ENV MYSQL_PASSWORD=**redacted**
ENV MYSQL_ROOT_PASSWORD=**redacted**

COPY init.sql /docker-entrypoint-initdb.d
RUN chmod 755 -R /docker-entrypoint-initdb.d

EXPOSE 3306