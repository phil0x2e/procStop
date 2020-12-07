FROM php:7-fpm
LABEL maintainer "Phil"
COPY web /var/www/html
RUN chmod -R 777 /var/www/html/database.sqlite3
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y sqlite3
EXPOSE 9000
