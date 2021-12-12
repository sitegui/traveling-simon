#!/usr/bin/env bash

# Build and run the image locally

docker build -t traveling-simon .
docker run --rm -p 3000:3000 --env-file .env traveling-simon
