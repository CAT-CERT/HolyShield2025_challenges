ARG HTTPD_VERSION=2.4.55
FROM httpd:${HTTPD_VERSION}

RUN apt-get update \
 && apt-get install -y --no-install-recommends apache2-dev gcc make curl ca-certificates \
 && curl -L https://tn123.org/mod_xsendfile/mod_xsendfile.c -o /tmp/mod_xsendfile.c \
 && apxs -ci /tmp/mod_xsendfile.c \
 && rm -rf /var/lib/apt/lists/* /tmp/mod_xsendfile.c

COPY extra/holyshield-modules.conf /usr/local/apache2/conf/extra/holyshield-modules.conf
COPY extra/holyshield-vhost.conf   /usr/local/apache2/conf/extra/holyshield-vhost.conf

COPY ../secret/flag.txt /var/secret/flag
RUN chmod 644 /var/secret/flag

RUN printf "\n# --- HolyShield includes ---\nInclude conf/extra/holyshield-modules.conf\nInclude conf/extra/holyshield-vhost.conf\n" >> /usr/local/apache2/conf/httpd.conf
