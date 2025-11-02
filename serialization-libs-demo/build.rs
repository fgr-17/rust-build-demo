use anyhow::Result;
use dotenvy::dotenv;
use glob::glob;
use std::fs;
use std::path::Path;
use std::process::Command;

#[path = "build/flatbdata_generated.rs"]
#[allow(
    dead_code,
    unused_imports,
    clippy::all,
    warnings,
    unsafe_op_in_unsafe_fn
)]
mod flatbdata_generated;

use flatbdata_generated::example::*;

fn print_flatb(message: &Message) {
    println!("  Timestamp: {}", message.timestamp());
    match message.data_type() {
        Data::IntValue => {
            if let Some(int_val) = message.data_as_int_value() {
                println!("  Type: IntValue");
                println!("  Value: {}", int_val.value());
            }
        }
        Data::FloatValue => {
            if let Some(float_val) = message.data_as_float_value() {
                println!("  Type: FloatValue");
                println!("  Value: {}", float_val.value());
            }
        }
        Data::ComplexNumber => {
            if let Some(complex) = message.data_as_complex_number() {
                println!("  Type: ComplexNumber");
                println!("  Real: {}", complex.re());
                println!("  Imaginary: {}", complex.im());
            }
        }
        _ => {
            println!("  Type: None");
        }
    }
}

fn load_and_process_flatbuf_data() -> Result<()> {
    let data_output = std::env::var("DATA_OUTPUT").unwrap_or_else(|_| "build".to_string());

    let bin_files = get_file_list(&format!("{}/*.bin", data_output))?;

    println!("\n=== Loading FlatBuffer binaries ===");

    for bin_file in &bin_files {
        println!("\nProcessing: {}", bin_file);

        let data = fs::read(bin_file)?;

        let message = match root_as_message(&data) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Failed to parse {}: {:?}", bin_file, e);
                continue;
            }
        };

        print_flatb(&message);
    }

    println!("\n=== Finished processing FlatBuffers ===");
    Ok(())
}

fn get_file_list(pattern: &str) -> Result<Vec<String>> {
    let file_list: Vec<String> = glob(pattern)
        .expect("Failed to read glob pattern")
        .filter_map(|path| path.ok())
        .map(|path| path.to_string_lossy().to_string())
        .collect();

    if file_list.is_empty() {
        anyhow::bail!("No files found matching pattern: {}", pattern);
    }

    for file in &file_list {
        println!("cargo:rerun-if-changed={}", file);
    }
    Ok(file_list)
}

fn gen_protobuf_src(models_dir: &str) -> Result<()> {
    let proto_list = get_file_list(&format!("{}/*.proto", &models_dir))?;
    prost_build::compile_protos(&proto_list, &[&models_dir])?;
    Ok(())
}

fn gen_flatbuf_src(models_dir: &str) -> Result<()> {
    let fbs_list = get_file_list(&format!("{}/*.fbs", &models_dir))?;
    let out_dir = std::env::var("OUT_DIR")?;

    println!(
        "Generating FlatBuffers Rust source from {} files",
        fbs_list.len()
    );

    let fbs_paths: Vec<std::path::PathBuf> =
        fbs_list.into_iter().map(std::path::PathBuf::from).collect();

    let path_refs: Vec<&Path> = fbs_paths.iter().map(|p| p.as_path()).collect();

    flatc_rust::run(flatc_rust::Args {
        inputs: &path_refs,
        out_dir: Path::new(&out_dir),
        ..Default::default()
    })?;

    println!("✓ Generated to: {}", out_dir);

    Ok(())
}

fn gen_protobuf_binaries(models_dir: &str, config_dir: &str, output_dir: &str) -> Result<()> {
    let proto_file = format!("{}/sensor.proto", models_dir);
    let config_txt = format!("{}/sensor_config.pbtxt", config_dir);
    let config_bin = format!("{}/sensor_config.pb", output_dir);

    println!("Converting protobuf text to binary...");

    if !Path::new(&proto_file).exists() {
        anyhow::bail!("Proto file not found: {}", proto_file);
    }
    if !Path::new(&config_txt).exists() {
        anyhow::bail!("Config text file not found: {}", config_txt);
    }

    fs::create_dir_all(output_dir)?;

    let status = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "protoc --proto_path={} --encode=sensor.SensorData {} < {} > {}",
            models_dir, proto_file, config_txt, config_bin
        ))
        .status()?;

    if !status.success() {
        anyhow::bail!("protoc encode failed");
    }

    println!("Generated: {}", config_bin);
    println!("cargo:rerun-if-changed={}", proto_file);
    println!("cargo:rerun-if-changed={}", config_txt);

    Ok(())
}

fn gen_flatbuf_binaries(models_dir: &str, data_input: &str, data_output: &str) -> Result<()> {
    let fbs_file = format!("{}/flatbdata.fbs", models_dir);

    if !Path::new(&fbs_file).exists() {
        anyhow::bail!("FlatBuffers schema not found: {}", fbs_file);
    }

    let json_files = get_file_list(&format!("{}/*.json", data_input))?;

    fs::create_dir_all(data_output)?;

    println!(
        "Converting {} JSON files to FlatBuffers binaries...",
        json_files.len()
    );

    for json_file in &json_files {
        let filename = Path::new(json_file).file_stem().unwrap().to_str().unwrap();
        let output_bin = format!("{}/{}.bin", data_output, filename);

        println!("  {} -> {}", json_file, output_bin);

        let output = Command::new("flatc")
            .arg("--binary")
            .arg("--force-defaults")
            .arg("-o")
            .arg(data_output)
            .arg(&fbs_file)
            .arg(json_file)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("flatc failed for {}: {}", json_file, stderr);
        }

        println!("cargo:rerun-if-changed={}", json_file);
    }

    println!("Generated {} FlatBuffers binaries", json_files.len());
    println!("cargo:rerun-if-changed={}", fbs_file);

    Ok(())
}

fn main() -> Result<()> {
    println!("============ init build.rs ===============");
    dotenv().ok();
    println!("cargo:rerun-if-changed=.env");

    let models_dir = std::env::var("MODELS_DIR").unwrap_or_else(|_| "models".to_string());
    let config_dir = std::env::var("CONFIG_DIR").unwrap_or_else(|_| "config".to_string());
    let data_input = std::env::var("DATA_INPUT").unwrap_or_else(|_| "data".to_string());
    let data_output = std::env::var("DATA_OUTPUT").unwrap_or_else(|_| "build".to_string());

    gen_protobuf_src(&models_dir).expect("Failed to generate rust helpers from protos schemas");
    gen_protobuf_binaries(&models_dir, &config_dir, &data_output)?;

    gen_flatbuf_src(&models_dir).expect("Failed to generate rust helpers from flatbufs schemas");
    gen_flatbuf_binaries(&models_dir, &data_input, &data_output)?;
    load_and_process_flatbuf_data()?;

    println!("============ end build.rs ===============");
    Ok(())
}
