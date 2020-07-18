# RustGraphql

## Overview
The purpsose of rust-redis-pub-sub is to learn how to use redis with rust. 

## Install Dependencies
Install docker

## Build
### Network 
Run `docker network create -d bridge rust-redis-bridge`

### Dev
From this directory run: `docker build -t rust-redis-pub-sub:dev -f docker/Dockerfile.dev rust-redis-pub-sub`

## Start redis

This app depends on a redis server running on the same bridge network with the resovable hostname redis-broker. The easiest way to get redis up and running is with their Docker image. 
`docker run --network=rust-redis-bridge --name=redis-broker --rm redis:alpine`

## Development server
### Dev
Run `docker run --network=rust-redis-bridge --rm rust-redis-pub-sub:dev`

## Todo
* [x] use alpine
* [ ] build frontend