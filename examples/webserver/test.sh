#!/bin/sh

# benchmark code taken from https://stackoverflow.com/questions/18215389/how-do-i-measure-request-and-response-times-at-once-using-curl
 
iterations=$1
url=$2
echo "Running $iterations iterations for curl $url"
totaltime=0.0
for run in $(seq 1 $iterations)
do
 time=$(curl $url \
    -s -o /dev/null -w "%{time_total}")
 totaltime=$(echo "$totaltime" + "$time" | bc)
done
avgtimeMs=$(echo "scale=4; 1000*$totaltime/$iterations" | bc)
echo "Averaged $avgtimeMs ms in $iterations iterations"