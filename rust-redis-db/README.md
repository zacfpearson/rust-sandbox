# RustRedisDB

## Overview
The purpsose of rust-redis-db is to learn how to use redis with rust. 

## Install Dependencies
Install docker

## Build
### Network 
Run `docker network create -d bridge rust-redis-bridge`

### Dev
From this directory run: `docker build -t rust-redis-db:dev -f docker/Dockerfile.dev rust-redis-db`

## Start redis

This app depends on a redis server running on the same bridge network with the resovable hostname redis-broker. The easiest way to get redis up and running is with their Docker image. 
`docker run --network=rust-redis-bridge --name=redis-broker --rm redis:alpine`

## Development server
### Dev
Run `docker run --network=rust-redis-bridge --rm rust-redis-db:dev`