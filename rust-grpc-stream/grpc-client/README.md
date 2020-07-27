# GrpcClient

## Overview
The purpsose of grpc-client is to learn how to use grpc with rust. 

## Install Dependencies
Install docker

## Build
### Network 
Run `docker network create -d bridge rust-grpc-bridge`

### Dev
From this directory run: `docker build -t rust-grpc-stream-client:dev -f docker/Dockerfile.dev grpc-client`

## Development client
### Dev
Run `docker run --network=rust-grpc-bridge --rm rust-grpc-stream-client:dev`