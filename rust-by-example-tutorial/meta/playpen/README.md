# MetaPlaypenRustTutorial

## Overview
The purpose of this app is to learn rust. This was made by following "Rust By Example"

## Install Dependencies
Install docker

## Build
### Dev
From this directory run: `docker build -t rust-meta-playpen:dev -f docker/Dockerfile.dev playpen`

## Development server
### Dev
Run `docker run -p 3000:3000  rust-meta-playpen:dev`