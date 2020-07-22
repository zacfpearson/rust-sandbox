# RedisPub

## Overview
The purpsose of redis-pub is to learn how to use redis with rust. 

## Install Dependencies
Install docker

## Build
### Network 
Run `docker network create -d bridge rust-redis-bridge`

### Dev
From this directory run: `docker build -t redis-pub:dev -f docker/Dockerfile.dev redis-pub`

## Start redis

This app depends on a redis server running on the same bridge network with the resovable hostname redis-broker. The easiest way to get redis up and running is with their Docker image. 
`docker run --network=rust-redis-bridge --name=redis-broker --rm redis:alpine`

## Development server
### Dev
Run `docker run --network=rust-redis-bridge --rm redis-pub:dev`