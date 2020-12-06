FROM php:7-fpm
LABEL maintainer "Phil"
COPY web /web
RUN chmod -R 777 /web
EXPOSE 9000
