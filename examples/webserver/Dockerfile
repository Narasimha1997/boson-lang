FROM ubuntu:latest

RUN mkdir -p /usr/local/bin
RUN mkdir -p /usr/local/lib/boson

COPY boson-eval /usr/local/bin/
COPY libsyscalls.so /usr/local/lib/boson/


COPY server.np /server.np
COPY data.html /data.html
WORKDIR /

ENTRYPOINT ["boson-eval", "server.np"]