#!/bin/bash
PORT=${1:-8080}
while lsof -Pi :$PORT -sTCP:LISTEN -t >/dev/null ; do
    PORT=$((PORT + 1))
done
echo $PORT
