#!/bin/bash

cp /usr/local/bin/boson-eval .
cp /usr/local/lib/boson/libsyscalls.so .

sudo docker build . -t boson-ws:latest 

rm boson-eval
rm libsyscalls.so