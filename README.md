# rust-template
Basic Rust repo template

* Hello-world project
* Docker container with all deps
* Aliases for
    * lint
    * style
    * test
    * mock support with mockall
    * basic doc example
* A couple of scripts to automate building

[![Docs](https://docs.rs/my_crate/badge.svg)](https://docs.rs/my_crate)


**Author: Federico Roux (rouxfederico@gmail.com)**

## Installation

### Requirements

~~~
* docker
* docker compose
~~~

### Renaming the project

The basic demo project name is "hello". To rename it please follow the instructions:

1. Rename the dir called "hello":

~~~bash
mv hello my_new_project
~~~

2. Update the [Cargo.toml](./hello/Cargo.toml) file:

~~~
[package]
name = "my_new_project" # <--- CHANGE THIS LINE
version = "0.1.0"
edition = "2021"
~~~

3. Final cleanup (once inside the container)

~~~bash
cargo clean
cargo check
~~~

4. [Optional] Rename the docker container:

Change the name of the container in the [compose.yml](./compose.yml) file:

~~~
    container_name: rust-template # <--- CHANGE THIS NAME
~~~

## Usage

Initialize the dev container using the compose file:

~~~bash
docker compose up -d
~~~

Then jump into the container and start building:

~~~bash
docker exec -it rust-template sh
cd hello
make all
~~~

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

Pleaseee mantain the [CHANGELOG](./CHANGELOG) updated. Follo [keepachangelog](https://keepachangelog.com/en/1.0.0/) guidelines

## License

[Apache License 2.0](./LICENSE)
