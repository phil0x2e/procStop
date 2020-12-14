FROM php:7-fpm
LABEL maintainer "Phil"
ENV TZ=Europe/Berlin
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN printf '[PHP]\ndate.timezone = "Europe/Berlin"\n' > /usr/local/etc/php/conf.d/tzone.ini
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y sqlite3
COPY web /var/www/html
RUN chmod -R 777 /var/www/html/database.sqlite3
EXPOSE 9000
