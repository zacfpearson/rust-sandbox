# RustGraphql

## Overview
The purpsose of rust-graphql is to learn how to use graphql with rust. This prject follows this (repo)[https://github.com/joshua-cooper/rust-graphql-intro]. 

## Install Dependencies
Install docker

## Build
### Network 
Run `docker network create -d bridge rust-graphql-bridge`

### Dev
From this directory run: `docker build -t rust-graphql:dev -f docker/Dockerfile.dev rust-graphql`

## Start postgres

This app depends on a postgres server running on the same network. The easiest way to get postgres server up and runnign is with their Docker image.
`docker run --network=rust-graphql-bridge --rm -it -e POSTGRES_PASSWORD=postgres -p 5432:5432 --name=postgres-server  postgres:alpine`

## Development server
### Dev
Run `docker run --network=rust-graphql-bridge -p 8000:8000 --rm rust-graphql:dev`

## Test
### Dev
Travel to `http://localhost:8000/graphiql`

## Todo
* [x] use alpine
* [ ] build frontend