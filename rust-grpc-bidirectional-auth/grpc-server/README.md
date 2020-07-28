# GrpcServer

## Overview
The purpsose of grpc-server is to learn how to use grpc with rust. 

## Install Dependencies
Install docker

## Build
### Network 
Run `docker network create -d bridge rust-grpc-bridge`

### Dev
From this directory run: `docker build -t rust-grpc-bidirectional-auth-server:dev -f docker/Dockerfile.dev grpc-server`

## Development server
### Dev
Run `docker run --network=rust-grpc-bridge --name=grpc-server --rm rust-grpc-bidirectional-auth-server:dev`