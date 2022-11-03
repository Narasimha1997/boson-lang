#!/bin/bash

cp /usr/local/bin/boson-eval .
cp /usr/local/lib/boson/libsyscalls.so .

sudo heroku container:push web -a boson-demo

rm boson-eval
rm libsyscalls.so