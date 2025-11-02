# Rust `build.rs` script demo

Demo for Rust `build.rs` script.
This repo makes use of [protobufs](protobuf.dev) and [flatbufs](flatbuffers.dev) in Rust as an example of a trade-off between the Cargo build scripts and traditional makefiles.

## Summary

This repo shows how a `build.rs`script runs on the host machine, generates assets/data, and embeds them into a Rust binary

## Rust template

This repo is based on my [rust template](https://github.com/fgr-17/rust-template) that contains:

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

## Usage

Initialize the dev container using the compose file:

~~~bash
docker compose up -d
~~~

Then jump into the container and start building:

~~~bash
docker exec -it serialization-libs-demo ash
cd hello
make all
~~~

## Build scripts and data generation

### Message schemas

The message schemas for proto and flat buffers can be found in [models](models):

* [flatdata.fbs](./serialization-libs-demo/models/flatbdata.fbs): describes a sensor-like message with a timestamp. The message can be `int`, `float` or complex number.
* [sensor.proto](./serialization-libs-demo/models/sensor.proto): describes a simpler sensor-like message, `double` value with some metadata.

### Rust source files generation

Both protobufs and flatbufs need to generate the source code from the respective message schema, in this case with some differences:

* Protobuf: proto rust source files can be generated using `protoc`command line tool or some rust lib. In this repo, I took the second option, making use of [prost](https://docs.rs/prost/latest/prost/). So, [build.rs](./serialization-libs-demo/build.rs) simlpy calls `gen_protobuf_src`, a function that searches for all the `*.proto` files in [models](./serialization-libs-demo/models) and pass the found files to `prost_build::compile_protos`. The protobuf source code will be stored somewhere inside `OUT_DIR`.

* Flatbuf: same options for protobuf - you can generate your rust source files using `flatc`CLI tool, or make use of some rust lib, like [flatc-rust](https://github.com/frol/flatc-rust) (that also needs `flatc`installed). In this case I took both options and compared them:

    * `flatc` CLI tool: check the `$(FLATB_GENERATED)` makefile target in [genflatb.mk](./serialization-libs-demo/genflatb.mk). Main option from flatbuf docs. The only drawback I see is that you have to manually put the output `*.rs` files in a `build/` dir you choose, and then use that path in `main.rs`.

    * `flatc-rust` lib: check `gen_flatbuf_src()` in [build.rs](./serialization-libs-demo/build.rs). This basically reads all the *.fbs files in [models](./serialization-libs-demo/models/) and generates the corresponding `*.rs` rust source code. As `build.rs`has access to `OUT_DIR` (*while Makefile doesn't*), this function can throw the output files there, a little bit more straightforward when including them (keep in mind that this is just an example of usage and tradeoff between Makefile and Cargo)

### Message binaries generation

Is frequently useful to embed data from the outside world into our program:

* Configuration data
* Shared data with other languages
* Mocking sensor inputs
* etc...?

In those cases, is tipically to have those files in human-readable format, so they can be easily checked and/or modified by QA, product people, etc (non geeks). For the case shown in this repo, with **protobufs** and **flatbufs**, the human readable formats can be `.pbtxt` or `.json` respectively.

In embedded systems that doesn't count with a filesystem, this is not so easy, as we have to find the way to include those files into the binary blob during compilation time, that is, parse the `.pbtxt` or `.json`, generate the binary data and embed that into runtime variables that will be loaded in our program.

For **protobufs** and **flatbufs**, this can be easily achieved using the CLI tools like `protoc` and `flatc`:

* [genproto.mk](./serialization-libs-demo/genproto.mk): the target `$(CONFIG_BIN)` picks the `.pbtxt` file in [config](./serialization-libs-demo/config/) dir, and generates the binary data by using the message schema `sensor.SensorData` to parse, described in [sensor.proto](./serialization-libs-demo/models/sensor.proto). Including the name of the schema in the command is not the most maintainable thing in the world, it should be automated somehow...
* [genflatb.mk](./serialization-libs-demo/genflatb.mk): the target `$(DATA_OUTPUT)` picks the `.json` files in [data](./serialization-libs-demo/data/) and generates the binary data by using the message schema described in [flatbdata.fbs](./serialization-libs-demo/models/flatbdata.fbs)to parse.

As both cases are based on CLI tools, it was a little bit more straightforward to solve on the Makefile side. But this could also be done in `build.rs` as shown in:

* `gen_protobuf_binaries`
* `gen_flatbuf_binaries`

Not a big fan of doing this CLI stuff in a rust function, but here it is for comparison. The drawbacks I see are:

* Doing the same in the Makefile is shorter
* Cleaning is also easier

The pros:

* Easier to detect and handle errors
* Integrated with the cargo toolchain

Note: to toggle generating binaries from Makefile or build.rs, uncomment [Makefile lines 3-4](./serialization-libs-demo/Makefile#L3-L4) and comment build.rs lines [#214](./serialization-libs-demo/build.rs#L214) and [#217](./serialization-libs-demo/build.rs#L217) and viceversa.

### Reading the messages in runtime

Once the binaries are generated from the `.pbtxt` or `.json` files, they can be easily read in [main.rs](./serialization-libs-demo/src/main.rs) using `include_bytes!` macro during build time (no filesystem needed in target!):

~~~rust
const CONFIG_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/build/sensor_config.pb"
));
~~~

Then, as shown in the `print_*_data` functions in `main()`, they can be loaded into a proto or flatb variable to be manipulated.

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

Pleaseee mantain the [CHANGELOG](./CHANGELOG) updated. Follow [keepachangelog](https://keepachangelog.com/en/1.0.0/) guidelines

## License

[Apache License 2.0](./LICENSE)

