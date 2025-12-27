FROM nginx:1.22.0-alpine

COPY nginx.conf /etc/nginx/nginx.conf

COPY dynamic_upstream.conf /etc/nginx/conf.d/dynamic_upstream.conf

RUN rm -f /etc/nginx/conf.d/default.conf
