# Boilerplate for Rust Service

This directory contains a __boilerplate code__ and __configuration__ to start developing a Rust service for [Datumbazo](https://github.com/engeniusua/datumbazo).

## Getting Started

Listed bellow are the steps you will need to take to get started with you Rust service:

1. First thing is to create a copy of this directory inside `./services` that will contain your source code. Change the name of the directory to the name of your service.

2. Make sure to change the information in the `Cargo.toml` file to the information of the service.

3. Next there's some changes that need to be done in the `Dockerfile`: Change every apperance of `rustboiler` to the name of your service.

4. To add the service to the platform some thing need to be taken in consideration. If you don't feel at ease in architecturing the integration in the platform feel free to contact the current dev ops platform maintiner or open and [issue](https://github.com/engeniusua/datumbazo/issues) on github. Different processes and deliberations need to be take in account:

    - The routing of the requests. Should they pass throught the reverse proxy server? If not, should you expose a port to the exterior? You can check the `default.conf` of the NGINX reverse proxy server in the `./services` directory.

    - Should you connect to an existing database? Should you create a new one? Wich database should we use?

    - You need to spawn the container running your service. Than can be added in the `docker-compose` file in the `./services` directory.

    - Your service depends on any other service? If you need to interact with an API of another service the answer is yes. Make sure to had that dependence in the `docker-compose` file.

