use colored::*;
use prost::Message;

const SEPARATOR: &str = "------------------------------------------------";

pub mod sensor {
    include!(concat!(env!("OUT_DIR"), "/sensor.rs"));
}

// Flatbuf helpers generated with CLI flatc tool in genflatc.mk
pub mod flatc_helpers {
    #![allow(clippy::all, warnings, unsafe_op_in_unsafe_fn)]
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/build/flatbdata_generated.rs"
    ));
}

// Flatbuf helpers generated with flatc_rust cargo tool
pub mod flatc_rust_lib_helpers {
    #![allow(clippy::all, warnings, unsafe_op_in_unsafe_fn)]
    include!(concat!(env!("OUT_DIR"), "/flatbdata_generated.rs"));
}

const CONFIG_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/build/sensor_config.pb"
));

// Directly embed binary files - NO code generation!
const FLATBDATA_INT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/build/flatbdata-int.bin"
));

const FLATBDATA_FLOAT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/build/flatbdata-float.bin"
));

const FLATBDATA_COMPLEX: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/build/flatbdata-complex.bin"
));

fn print_protob_data() {
    println!("{}", SEPARATOR.yellow().bold());
    println!("{}", "Protobufs using prost lib:".yellow().bold());
    println!("{}", SEPARATOR.yellow().bold());

    let config = sensor::SensorData::decode(CONFIG_DATA).unwrap();
    println!(
        "Loaded bin data from protobuf parsed messages in build.rs:\n {:#?}",
        config
    );
}

fn print_flatb_data_from_cli_flac() {
    println!("{}", SEPARATOR.yellow().bold());
    println!(
        "{}",
        "Flatbufs data using CLI flatc generated helpers:"
            .yellow()
            .bold()
    );
    println!("{}", "helpers in makefile (genflatb.mk):".yellow().bold());
    println!("{}", SEPARATOR.yellow().bold());
    println!("Loaded bin data from flatbuf parsed messages in build.rs:\n");

    let int_msg = flatc_helpers::example::root_as_message(FLATBDATA_INT).unwrap();
    println!(
        "Int value: {}",
        int_msg.data_as_int_value().unwrap().value()
    );

    let float_msg = flatc_helpers::example::root_as_message(FLATBDATA_FLOAT).unwrap();
    println!(
        "Float value: {}",
        float_msg.data_as_float_value().unwrap().value()
    );

    let complex_msg = flatc_helpers::example::root_as_message(FLATBDATA_COMPLEX).unwrap();
    let complex = complex_msg.data_as_complex_number().unwrap();
    println!("Complex: {} + {}i", complex.re(), complex.im());
}

fn print_flatb_data_from_flatc_rust_lib_helpers() {
    println!("{}", SEPARATOR.yellow().bold());
    println!("{}", "Flatbufs data using cargo flatc lib".yellow().bold());
    println!("{}", "generated helpers:".yellow().bold());
    println!("{}", SEPARATOR.yellow().bold());
    println!("Loaded bin data from flatbuf parsed messages in build.rs:\n");

    let int_msg = flatc_rust_lib_helpers::example::root_as_message(FLATBDATA_INT).unwrap();
    println!(
        "Int value: {}",
        int_msg.data_as_int_value().unwrap().value()
    );

    let float_msg = flatc_rust_lib_helpers::example::root_as_message(FLATBDATA_FLOAT).unwrap();
    println!(
        "Float value: {}",
        float_msg.data_as_float_value().unwrap().value()
    );

    let complex_msg = flatc_rust_lib_helpers::example::root_as_message(FLATBDATA_COMPLEX).unwrap();
    let complex = complex_msg.data_as_complex_number().unwrap();
    println!("Complex: {} + {}i", complex.re(), complex.im());
}

fn main() {
    print_protob_data();
    print_flatb_data_from_cli_flac();
    print_flatb_data_from_flatc_rust_lib_helpers();
}
